use std::collections::HashMap;

use axum::http::StatusCode;
use domain::stats::LolRole;
use domain::team::{DraftStrategy, TeamKind, TrainingFocus, TrainingIntensity};
use ofm_core::game::Game;
use ofm_core::generator::definitions::LeagueSelectionData;
use serde_json::{json, Value};

use crate::data;

pub struct CommandResult {
    pub value: Value,
    pub persist: bool,
}

pub struct CommandError {
    pub status: StatusCode,
    pub message: String,
}

impl CommandError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self { status: StatusCode::BAD_REQUEST, message: message.into() }
    }
    fn not_found(message: impl Into<String>) -> Self {
        Self { status: StatusCode::NOT_FOUND, message: message.into() }
    }
}

pub fn dispatch(command: &str, args: Value, game: &mut Game) -> Result<CommandResult, CommandError> {
    match command {
        "debug_log" => {
            if let Some(message) = args.get("message").and_then(|v| v.as_str()) {
                tracing::debug!("{message}");
            }
            ok(Value::Null, false)
        }
        "get_active_game" => {
            let repaired_financials = data::repair_player_financials(game);
            let repaired_active_competition = data::repair_active_competition(game);
            ofm_core::champions::bootstrap_champion_state(game);
            ok(json!(game), repaired_financials || repaired_active_competition)
        }
        "save_game" => ok(Value::Null, true),
        "advance_time" => { ofm_core::turn::process_day(game); ok(json!(game), true) }
        "advance_time_with_mode" => {
            ofm_core::turn::process_day(game);
            ok(json!({"action":"advanced","game":game}), true)
        }
        "select_team" => {
            let team_id = string_arg(&args, &["teamId", "team_id"])?;
            data::select_team(game, &team_id).map_err(CommandError::bad_request)?;
            ok(json!(game), true)
        }
        "set_active_lineup" | "set_starting_xi" => {
            let player_ids = string_vec_arg(&args, &["playerIds", "player_ids"])?;
            let team = managed_team_mut(game, &manager_team_id(game)?)?;
            team.active_lineup_ids = player_ids;
            ok(json!(game), true)
        }
        "set_draft_strategy" => {
            let value = string_arg(&args, &["draftStrategy", "draft_strategy"])?;
            let strategy = match value.as_str() {
                "Attacking"|"HighPress" => DraftStrategy::Aggressive,
                "Defensive" => DraftStrategy::Passive,
                "Possession" => DraftStrategy::Scaling,
                "Counter" => DraftStrategy::CounterPick,
                "PriorityBans" => DraftStrategy::PriorityBans,
                _ => DraftStrategy::Balanced,
            };
            managed_team_mut(game, &manager_team_id(game)?)?.draft_strategy = strategy;
            ok(json!(game), true)
        }
        "set_lol_tactics" => {
            let tactics: domain::team::LolTactics = serde_json::from_value(
                get_arg(&args, &["lolTactics", "lol_tactics"])?.clone()
            ).map_err(|e| CommandError::bad_request(format!("invalid lol_tactics: {e}")))?;
            managed_team_mut(game, &manager_team_id(game)?)?.lol_tactics = tactics;
            ok(json!(game), true)
        }
        "set_training" => {
            let focus = string_arg(&args, &["focus"])?;
            let intensity = string_arg(&args, &["intensity"])?;
            let team = managed_team_mut(game, &manager_team_id(game)?)?;
            team.training_focus = TrainingFocus::from_id(&focus).unwrap_or_default();
            team.training_intensity = match intensity.as_str() { "Low"=>TrainingIntensity::Low, "High"=>TrainingIntensity::High, _=>TrainingIntensity::Medium };
            ok(json!(game), true)
        }
        "get_team_selection_data" => ok(json!({"manager":game.manager,"teams":game.teams.iter().filter(|t|t.team_kind!=TeamKind::Academy).cloned().collect::<Vec<_>>(),"players":game.players}), false),
        "get_league_selection_data" => {
            let data = league_selection_data().map_err(CommandError::bad_request)?;
            ok(json!(data), false)
        }
        "mark_message_read" => {
            let message_id = string_arg(&args, &["messageId","message_id"])?;
            if let Some(m) = game.messages.iter_mut().find(|m| m.id == message_id) { m.read = true; }
            ok(json!(game), true)
        }
        "mark_all_messages_read" => {
            for m in game.messages.iter_mut() { m.read = true; }
            ok(json!(game), true)
        }
        "relocalize_social_feed" => {
            let lang = optional_string_arg(&args, &["language","locale"]).unwrap_or_else(|| "en".to_string());
            ofm_core::social::relocalize_social_posts(game, &lang);
            ok(json!(game), true)
        }
        "check_blocking_actions" => {
            let blockers = crate::time_blockers::compute_blocking_actions(game);
            ok(json!(blockers), false)
        }

        // ── Academies ──────────────────────────────────────
        "get_academy_acquisition_options" => {
            let parent_team_id = string_arg(&args, &["parentTeamId","parent_team_id"])?;
            let bootstrap_date = game.clock.current_date.format("%Y-%m-%d").to_string();
            ofm_core::game_setup::bootstrap_example_academy_pool_from_example(&mut game.teams, &mut game.players, &bootstrap_date);
            let parent = game.teams.iter().find(|t| t.id == parent_team_id).cloned()
                .ok_or_else(|| CommandError::bad_request(format!("Team '{}' not found", parent_team_id)))?;

            let occupied: std::collections::HashSet<String> = game.teams.iter()
                .filter(|t| t.team_kind == TeamKind::Academy && t.parent_team_id.is_some())
                .flat_map(|t| {
                    let mut ids = vec![t.id.clone()];
                    if let Some(ref m) = t.academy { ids.push(m.source_team_id.clone()); }
                    ids
                }).collect();
            let taken: std::collections::HashSet<String> = game.teams.iter()
                .filter(|t| t.team_kind == TeamKind::Academy && t.parent_team_id.is_some())
                .filter_map(|t| t.academy.as_ref().map(|m| norm(&m.original_name))).collect();
            let mut options: Vec<ofm_core::academy::AcademyAcquisitionOption> =
                ofm_core::academy::eligible_academy_acquisition_options(&parent.country,
                    ofm_core::academy::academy_erl_catalog(),
                    ofm_core::academy::academy_candidate_catalog())
                .into_iter().filter(|o| !occupied.contains(&o.source_team_id) && !taken.contains(&norm(&o.name))).collect();
            let blocked = if !parent.is_main() { Some("Academy can only be acquired for a main team".to_string()) }
                else if parent.academy_team_id.is_some() { Some("Parent team already has academy".to_string()) }
                else if options.is_empty() { Some("No free academy candidates available".to_string()) }
                else if options.iter().all(|o| parent.finance < o.acquisition_cost) { Some("Insufficient funds".to_string()) }
                else { None };
            ok(json!({"parent_team_id":parent_team_id,"acquisition_allowed":blocked.is_none(),"blocked_reason":blocked,"options":options}), false)
        }
        "acquire_academy_team" => {
            let req: serde_json::Value = args.get("request").cloned().unwrap_or(args);
            let parent_id = string_arg(&req, &["parentTeamId","parent_team_id"])?;
            let source_id = string_arg(&req, &["sourceTeamId","source_team_id"])?;
            let custom_name = optional_string_arg(&req, &["customName","custom_name"]);
            let custom_short = optional_string_arg(&req, &["customShortName","custom_short_name"]);

            let parent_idx = game.teams.iter().position(|t| t.id == parent_id)
                .ok_or_else(|| CommandError::bad_request("Parent team not found"))?;
            let bootstrap_date = game.clock.current_date.format("%Y-%m-%d").to_string();
            ofm_core::game_setup::bootstrap_example_academy_pool_from_example(&mut game.teams, &mut game.players, &bootstrap_date);

            let academy_idx = game.teams.iter().position(|t| t.id == source_id && t.team_kind == TeamKind::Academy);
            let option = ofm_core::academy::eligible_academy_acquisition_options(
                &game.teams[parent_idx].country,
                ofm_core::academy::academy_erl_catalog(),
                ofm_core::academy::academy_candidate_catalog()
            ).into_iter().find(|o| o.source_team_id == source_id);

            if let Some(opt) = option {
                game.teams[parent_idx].finance -= opt.acquisition_cost;
                game.teams[parent_idx].season_expenses += opt.acquisition_cost;
                game.teams[parent_idx].academy_team_id = Some(source_id.clone());

                if let Some(idx) = academy_idx {
                    game.teams[idx].name = custom_name.unwrap_or_else(|| opt.name.clone());
                    game.teams[idx].short_name = custom_short.unwrap_or_else(|| opt.short_name.clone());
                    game.teams[idx].parent_team_id = Some(parent_id.clone());
                    game.teams[idx].academy = Some(domain::team::AcademyMetadata {
                        lifecycle: domain::team::AcademyLifecycle::Active,
                        erl_assignment: domain::team::ErlAssignment {
                            erl_league_id: opt.erl_league_id.clone(),
                            country_rule: domain::team::ErlAssignmentRule::Domestic,
                            fallback_reason: Some("Acquired by user".to_string()),
                            reputation: opt.reputation,
                            acquisition_cost: opt.acquisition_cost,
                            acquired_at: game.clock.current_date.to_rfc3339(),
                            creation_cost: 0,
                            created_at: game.clock.current_date.to_rfc3339(),
                        },
                        source_team_id: source_id.clone(),
                        original_name: opt.name,
                        original_short_name: opt.short_name,
                        original_logo_url: opt.logo_url,
                        current_logo_url: None,
                        acquisition_cost: opt.acquisition_cost,
                        acquired_at: game.clock.current_date.to_rfc3339(),
                    });
                }
            }
            ok(json!(game), true)
        }
        "promote_academy_player" => {
            let player_id = string_arg(&args, &["playerId","player_id"])?;
            let team_id = manager_team_id(game)?;
            if let Some(p) = game.players.iter_mut().find(|p| p.id == player_id) { p.team_id = Some(team_id); }
            ok(json!(game), true)
        }
        "demote_main_player_to_academy" => {
            let player_id = string_arg(&args, &["playerId","player_id"])?;
            let parent_id = manager_team_id(game)?;
            let academy_id = game.teams.iter().find(|t| t.parent_team_id.as_deref() == Some(&parent_id) && t.team_kind == TeamKind::Academy)
                .map(|t| t.id.clone());
            if let Some(aid) = academy_id {
                if let Some(p) = game.players.iter_mut().find(|p| p.id == player_id) { p.team_id = Some(aid); }
            }
            ok(json!(game), true)
        }
        "get_academy_creation_options" => {
            return dispatch("get_academy_acquisition_options", args, game);
        }

        // ── Scrims ────────────────────────────────────────
        "set_weekly_scrims" => {
            if let Some(ids) = args.get("opponentTeamIds").and_then(|v| v.as_array()) {
                let tid = manager_team_id(game)?;
                let team = managed_team_mut(game, &tid)?;
                team.weekly_scrim_opponent_ids = ids.iter().filter_map(|v| v.as_str().map(String::from)).collect();
            }
            ok(json!(game), true)
        }
        "set_weekly_scrim_plans" => {
            if let Some(plans) = args.get("plans").and_then(|v| v.as_array()) {
                let tid = manager_team_id(game)?;
                let team = managed_team_mut(game, &tid)?;
                team.weekly_scrim_plan_team_ids = plans.iter().map(|slot| {
                    slot.as_array().map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect()).unwrap_or_default()
                }).collect();
            }
            ok(json!(game), true)
        }
        "set_weekly_scrim_slots" => {
            if let Some(slots) = args.get("slots").and_then(|v| v.as_u64()) {
                let tid = manager_team_id(game)?;
                managed_team_mut(game, &tid)?.scrim_weekly_slots = slots as u8;
            }
            ok(json!(game), true)
        }
        "set_weekly_scrim_objective" => {
            let obj = optional_string_arg(&args, &["objective"]);
            let tid = manager_team_id(game)?;
            let team = managed_team_mut(game, &tid)?;
            team.scrim_weekly_objective = obj.and_then(|o| serde_json::from_str(&format!("\"{o}\"")).ok());
            ok(json!(game), true)
        }
        "finalize_weekly_scrim_setup" => { ok(json!(game), true) }
        "auto_configure_weekly_scrim_setup" => { ok(json!(game), true) }
        "cancel_todays_scrims" => { ok(json!(game), true) }
        "choose_post_scrim_decision" => { ok(json!(game), true) }
        "choose_daily_scrim_action" => { ok(json!(game), true) }
        "delegate_scrim_decision" => { ok(json!(game), true) }
        "get_scrim_context" => {
            let team_id = manager_team_id(game).unwrap_or_default();
            ok(json!({
                "today":{"state":"NoScrimToday","slot_index":null,"opponent_team_id":null,"resolved_opponent_team_id":null,"objective":null,"report":null,
                    "can_edit_plan":true,"can_cancel":false,"can_review":false,"can_view_weekly_plan":true,"has_official_match":false,"primary_action":null,"push_through_recommended":false},
                "week":{"week_key":"","objective":null,"capacity":0,"planned":0,"reputation":0,"cancellations":0,"played":0,"wins":0,"losses":0,
                    "loss_streak":0,"avg_quality":0.0,"top_focus":null,"top_issue":null,"next_official_rival_team_id":null,"next_official_rival_competition":null,"setup_locked":false,"setup_locked_reason":null,"can_finalize_setup":true,
                    "slots":(if team_id.is_empty() {vec![]} else {
                        let t = game.teams.iter().find(|t|t.id==team_id).map(|t|t.scrim_weekly_slots).unwrap_or(0);
                        (0..t).map(|i| json!({"slot_index":i as i64,"weekday":0,"label":"","label_day":0,"label_suffix":"","plan":[],"resolved_opponent_team_id":null,"result_won":null,"report":null,"status":"Open","can_edit":true})).collect::<Vec<_>>()
                    }),"latest_reports":[]
                }
            }), false)
        }

        // ── Staff ──────────────────────────────────────────
        "hire_staff" => {
            let staff_id = string_arg(&args, &["staffId","staff_id"])?;
            let team_id = manager_team_id(game)?;
            if let Some(s) = game.staff.iter_mut().find(|s| s.id == staff_id) { s.team_id = Some(team_id); }
            ok(json!(game), true)
        }
        "release_staff" => {
            let staff_id = string_arg(&args, &["staffId","staff_id"])?;
            let team_id = manager_team_id(game)?;
            if let Some(s) = game.staff.iter_mut().find(|s| s.id == staff_id && s.team_id.as_deref() == Some(&team_id)) { s.team_id = Some("fa".to_string()); }
            ok(json!(game), true)
        }

        // ── Training ───────────────────────────────────────
        "set_training_schedule" => { ok(json!(game), true) }
        "set_training_groups" => { ok(json!(game), true) }
        "set_player_training_focus" => { ok(json!(game), true) }

        // ── Social ─────────────────────────────────────────
        "get_social_feed" => ok(json!(game.social_posts), false),
        "create_manager_social_post" => {
            ok(json!(game), true)
        }
        "get_social_accounts" => ok(json!(game.social_accounts), false),
        "save_social_accounts" => {
            if let Some(a) = args.get("accounts").and_then(|v| serde_json::from_value(v.clone()).ok()) { game.social_accounts = a; }
            ok(json!(game), true)
        }
        "get_social_templates" => ok(json!(game.social_templates), false),
        "save_social_templates" => {
            if let Some(t) = args.get("templates").and_then(|v| serde_json::from_value(v.clone()).ok()) { game.social_templates = t; }
            ok(json!(game), true)
        }

        // ── Scouting ────────────────────────────────────────
        "send_scout" => {
            ok(json!(game), true)
        }
        "start_potential_research" => {
            let player_id = string_arg(&args, &["playerId","player_id"])?;
            if let Some(p) = game.players.iter_mut().find(|p| p.id == player_id) {
                p.potential_revealed = None;
                p.potential_research_started_on = Some(game.clock.current_date.to_rfc3339());
                p.potential_research_eta_days = Some(7);
            }
            ok(json!(game), true)
        }

        // ── Champions ──────────────────────────────────────
        "set_player_champion_training_target" => { ok(json!(game), true) }
        "delegate_champion_training" => { ok(json!(game), true) }

        // ── Inbox ──────────────────────────────────────────
        "resolve_message_action" => { ok(json!(game), true) }
        "clear_old_messages" => { game.messages.clear(); ok(json!(game), true) }
        "delete_message" => {
            let id = string_arg(&args, &["messageId","message_id"])?;
            game.messages.retain(|m| m.id != id);
            ok(json!(game), true)
        }
        "delete_messages" => {
            if let Some(ids) = args.get("messageIds").and_then(|v| v.as_array()) {
                let set: std::collections::HashSet<&str> = ids.iter().filter_map(|v| v.as_str()).collect();
                game.messages.retain(|m| !set.contains(m.id.as_str()));
            }
            ok(json!(game), true)
        }

        // ── Transfers ──────────────────────────────────────
        "make_transfer_bid" => {
            ok(json!(game), true)
        }
        "respond_to_offer" => {
            ok(json!(game), true)
        }
        "counter_offer" => {
            ok(json!(game), true)
        }
        "negotiate_player_wage" => {
            ok(json!(game), true)
        }
        "release_player_contract" => {
            let player_id = string_arg(&args, &["playerId","player_id"])?;
            if let Some(p) = game.players.iter_mut().find(|p| p.id == player_id) { p.team_id = None; p.transfer_listed = false; }
            ok(json!(game), true)
        }
        "get_transfer_history_cmd" => ok(json!(game.transfer_history.entries), false),
        "toggle_transfer_list" => {
            let player_id = string_arg(&args, &["playerId","player_id"])?;
            if let Some(p) = game.players.iter_mut().find(|p| p.id == player_id) { p.transfer_listed = !p.transfer_listed; }
            ok(json!(game), true)
        }
        "toggle_loan_list" => {
            let player_id = string_arg(&args, &["playerId","player_id"])?;
            if let Some(p) = game.players.iter_mut().find(|p| p.id == player_id) { p.loan_listed = !p.loan_listed; }
            ok(json!(game), true)
        }
        "preview_transfer_bid_financial_impact" => {
            ok(json!({"bid":{},"can_afford":false}), false)
        }

        // ── Jobs ───────────────────────────────────────────
        "get_available_jobs" => {
            ok(json!([]), false)
        }
        "apply_for_job" => {
            ok(json!({"success":false,"message":"Not available in web version"}), false)
        }

        // ── Season ─────────────────────────────────────────
        "advance_to_next_season" => {
            ok(json!(game), true)
        }
        "check_season_complete" => {
            ok(json!({"complete": false}), false)
        }
        "get_season_awards" => {
            let awards = ofm_core::season_awards::compute_season_awards(game);
            ok(json!(awards), false)
        }

        // ── Facilities ──────────────────────────────────────
        "upgrade_main_facility_module" => { ok(json!(game), true) }
        "expand_main_facility_hub" => { ok(json!(game), true) }
        "upgrade_facility" => { ok(json!(game), true) }

        // ── Contracts / Renewals ───────────────────────────
        "propose_renewal" => { ok(json!(game), true) }
        "delegate_renewals" => { ok(json!(game), true) }
        "preview_renewal_financial_impact" => { ok(json!(game), true) }

        // ── Team Roles ─────────────────────────────────────
        "set_team_roles" => {
            let roles_val = get_arg(&args, &["teamRoles","team_roles"])?.clone();
            let roles: domain::team::TeamRoles = serde_json::from_value(roles_val)
                .map_err(|e| CommandError::bad_request(format!("invalid team_roles: {e}")))?;
            let team = managed_team_mut(game, &manager_team_id(game)?)?;
            team.team_roles = roles;
            ok(json!(game), true)
        }

        // ── Manager Profile ───────────────────────────────
        "update_manager_profile" => {
            let first_name = optional_string_arg(&args, &["firstName","first_name"]);
            let last_name = optional_string_arg(&args, &["lastName","last_name"]);
            let nickname = optional_string_arg(&args, &["nickname"]);
            let nationality = optional_string_arg(&args, &["nationality"]);
            if let Some(v) = first_name { game.manager.first_name = v; }
            if let Some(v) = last_name { game.manager.last_name = v; }
            if let Some(v) = nickname { game.manager.nickname = v; }
            if let Some(v) = nationality { game.manager.nationality = v; }
            ok(json!(game), true)
        }
        "reroll_player_lol_role" => {
            let player_id = string_arg(&args, &["playerId","player_id"])?;
            if let Some(p) = game.players.iter_mut().find(|p| p.id == player_id) {
                p.position = LolRole::Unknown;
            }
            ok(json!(game), true)
        }

        // ── Stats ──────────────────────────────────────────
        "get_player_match_history" => {
            let player_id = string_arg(&args, &["playerId","player_id"]).unwrap_or_default();
            let history: Vec<Value> = game.stats_state.player_matches.iter()
                .filter(|m| m.player_id == player_id)
                .map(|m| json!(m))
                .collect();
            ok(json!(history), false)
        }
        "get_team_match_history" => {
            let team_id = string_arg(&args, &["teamId","team_id"]).unwrap_or_default();
            let history: Vec<Value> = game.stats_state.team_matches.iter()
                .filter(|m| m.team_id == team_id)
                .map(|m| json!(m))
                .collect();
            ok(json!(history), false)
        }
        "get_player_stats_overview" => {
            let player_id = string_arg(&args, &["playerId","player_id"]).unwrap_or_default();
            let matches: Vec<_> = game.stats_state.player_matches.iter().filter(|m| m.player_id == player_id).collect();
            let total = matches.len() as u64;
            if total == 0 { return ok(json!(empty_player_stats_overview()), false); }
            let mut kills = 0u64; let mut deaths = 0u64; let mut assists = 0u64; let mut cs = 0u64; let mut vision = 0u64; let mut wards = 0u64;
            for m in &matches { kills += m.kills as u64; deaths += m.deaths as u64; assists += m.assists as u64; cs += m.creep_score as u64; vision += m.vision_score as u64; wards += m.wards_placed as u64; }
            ok(json!({
                "percentileEligible":false,"matchesPlayed":total,
                "metrics":{"kills":{"total":kills,"perMatch":(kills as f64/total as f64).round() as u64},"deaths":{"total":deaths,"perMatch":(deaths as f64/total as f64).round() as u64},
                    "assists":{"total":assists,"perMatch":(assists as f64/total as f64).round() as u64},"creepScore":{"total":cs,"perMatch":(cs as f64/total as f64).round() as u64},
                    "visionScore":{"total":vision,"perMatch":(vision as f64/total as f64).round() as u64},"wardsPlaced":{"total":wards,"perMatch":(wards as f64/total as f64).round() as u64}}
            }), false)
        }
        "get_team_stats_overview" => {
            let team_id = string_arg(&args, &["teamId","team_id"]).unwrap_or_default();
            let matches: Vec<_> = game.stats_state.team_matches.iter().filter(|m| m.team_id == team_id).collect();
            let total = matches.len() as u64;
            if total == 0 { return ok(json!(empty_team_stats_overview()), false); }
            let mut kills = 0u64; let mut deaths = 0u64; let mut gold = 0u64; let mut dmg = 0u64; let mut objs = 0u64; let mut duration = 0u64; let mut wins = 0u64;
            for m in &matches { kills += m.kills as u64; deaths += m.deaths as u64; gold += m.gold_earned as u64; dmg += m.damage_dealt as u64; objs += m.objectives as u64;
                duration += m.duration_seconds as u64; if m.result == domain::stats::MatchOutcome::Win { wins += 1; } }
            ok(json!({
                "matchesPlayed":total,"wins":wins,"losses":(total - wins),
                "metrics":{"kills":{"total":kills,"perMatch":(kills as f64/total as f64).round() as u64},"deaths":{"total":deaths,"perMatch":(deaths as f64/total as f64).round() as u64},
                    "goldEarned":{"total":gold,"perMatch":(gold as f64/total as f64).round() as u64},"damageToChampions":{"total":dmg,"perMatch":(dmg as f64/total as f64).round() as u64},
                    "objectives":{"total":objs,"perMatch":(objs as f64/total as f64).round() as u64},"averageGameDurationSeconds":{"total":duration,"perMatch":(duration as f64/total as f64).round() as u64}}
            }), false)
        }
        "get_champions" => ok(json!(champions_catalog()), false),

        // ── Live Match ─────────────────────────────────────
        "start_live_match" => { ok(json!(game), true) }
        "step_live_match" => { ok(json!(game), true) }
        "apply_match_command" => { ok(json!(game), true) }
        "get_match_snapshot" => { ok(json!(game), true) }
        "finish_live_match" => { ok(json!(game), true) }

        // ── Match / Draft ─────────────────────────────────
        "record_fixture_champion_picks" => { ok(json!(game), true) }
        "apply_champion_mastery_from_draft" => { ok(json!(game), true) }

        // ── Team Talk / Press ─────────────────────────────
        "apply_team_talk" => { ok(json!(game), true) }
        "submit_press_conference" => { ok(json!(game), true) }

        // ── Simulator ─────────────────────────────────────
        "lol_sim_v2_init" => { ok(json!(game), true) }
        "lol_sim_v2_tick" => { ok(json!(game), true) }
        "lol_sim_v2_reset" => { ok(json!(game), true) }
        "lol_sim_v2_dispose" => { ok(json!(game), true) }
        "lol_sim_v2_run_to_completion" => { ok(json!(game), true) }
        "lol_sim_v2_skip_to_end" => { ok(json!(game), true) }

        // ── Skip / Misc ───────────────────────────────────
        "skip_to_match_day" => { ok(json!({"action":"skipped","game":game}), true) }
        "export_bug_report" => { ok(json!({"path":""}), false) }
        "apply_weekly_scrim_setup" => { ok(json!(game), true) }
        "get_academy_acquisition_options_for_game" => { ok(json!(game), false) }

        _ => Err(CommandError::not_found(format!("unsupported command: {command}"))),
    }
}

fn ok(value: Value, persist: bool) -> Result<CommandResult, CommandError> {
    Ok(CommandResult { value, persist })
}

fn norm(s: &str) -> String {
    s.to_lowercase().chars().filter(|ch| ch.is_ascii_alphanumeric()).collect()
}

fn empty_metric(include_percentile: bool) -> Value {
    if include_percentile { json!({"total":0,"perMatch":null,"percentile":null}) }
    else { json!({"total":0,"perMatch":null}) }
}

fn empty_player_stats_overview() -> Value {
    json!({"percentileEligible":false,"matchesPlayed":0,
        "metrics":{"kills":empty_metric(true),"deaths":empty_metric(true),"assists":empty_metric(true),
            "creepScore":empty_metric(true),"visionScore":empty_metric(true),"wardsPlaced":empty_metric(true)}})
}

fn empty_team_stats_overview() -> Value {
    json!({"matchesPlayed":0,"wins":0,"losses":0,
        "metrics":{"kills":empty_metric(false),"deaths":empty_metric(false),"goldEarned":empty_metric(false),
            "damageToChampions":empty_metric(false),"objectives":empty_metric(false),"averageGameDurationSeconds":empty_metric(false)}})
}

fn get_arg<'a>(args: &'a Value, names: &[&str]) -> Result<&'a Value, CommandError> {
    names.iter().find_map(|n| args.get(*n)).ok_or_else(|| CommandError::bad_request(format!("missing argument: {}", names[0])))
}

fn string_arg(args: &Value, names: &[&str]) -> Result<String, CommandError> {
    get_arg(args, names)?.as_str().map(String::from).ok_or_else(|| CommandError::bad_request(format!("argument must be a string: {}", names[0])))
}

fn optional_string_arg(args: &Value, names: &[&str]) -> Option<String> {
    names.iter().find_map(|n| args.get(*n)).and_then(Value::as_str).map(String::from)
}

fn string_vec_arg(args: &Value, names: &[&str]) -> Result<Vec<String>, CommandError> {
    let arr = get_arg(args, names)?.as_array().ok_or_else(|| CommandError::bad_request(format!("argument must be an array: {}", names[0])))?;
    arr.iter().map(|i| i.as_str().map(String::from).ok_or_else(|| CommandError::bad_request(format!("array must contain strings: {}", names[0])))).collect()
}

fn manager_team_id(game: &Game) -> Result<String, CommandError> {
    game.manager.team_id.clone().ok_or_else(|| CommandError::bad_request("No team assigned"))
}

fn managed_team_mut<'a>(game: &'a mut Game, team_id: &str) -> Result<&'a mut domain::team::Team, CommandError> {
    game.teams.iter_mut().find(|t| t.id == team_id).ok_or_else(|| CommandError::bad_request("Team not found"))
}

fn league_selection_data() -> Result<LeagueSelectionData, String> {
    Ok(ofm_core::competitions::build_league_selection(&data::data_dir()))
}

fn split_camel_case(key: &str) -> String {
    let mut name = String::with_capacity(key.len() + 4);
    for (i, ch) in key.chars().enumerate() {
        if i > 0 && ch.is_uppercase() { name.push(' '); }
        name.push(ch);
    }
    name
}

fn champions_catalog() -> &'static Vec<Value> {
    static CATALOG: std::sync::OnceLock<Vec<Value>> = std::sync::OnceLock::new();
    CATALOG.get_or_init(|| {
        let raw = include_str!("../../../../assets/simulation/champions.json");
        let json: Value = match serde_json::from_str(raw) { Ok(v) => v, Err(_) => return Vec::new() };
        let data = json.get("data");
        let roles = match data.and_then(|d| d.get("roles")).and_then(|r| r.as_object()) { Some(m) => m, None => return Vec::new() };
        let counterpicks = data.and_then(|d| d.get("counterpicks")).and_then(|c| c.as_array());
        let synergies = data.and_then(|d| d.get("synergies")).and_then(|s| s.as_array());

        let mut key_to_name: HashMap<String, String> = HashMap::new();
        if let Some(aliases) = data.and_then(|d| d.get("display_aliases")).and_then(|a| a.as_object()) {
            for (alias, value) in aliases { if let Some(key) = value.as_str() { key_to_name.insert(key.to_string(), alias.clone()); } }
        }

        let filter = |arr: Option<&Vec<Value>>, key: &str| -> Option<String> {
            let items = arr?;
            let filtered: Vec<&Value> = items.iter().filter(|i| i.get("a").and_then(|v| v.as_str()) == Some(key)).collect();
            if filtered.is_empty() { None } else { serde_json::to_string(&filtered).ok() }
        };

        let mut keys: Vec<&String> = roles.keys().collect();
        keys.sort();
        keys.iter().enumerate().map(|(i, key)| {
            let ck = key.as_str();
            json!({"id":(i as i64)+1,"name":key_to_name.get(ck).cloned().unwrap_or_else(|| split_camel_case(ck)),
                "champion_key":ck,"roles_json":roles.get(ck).map(|v|v.to_string()).unwrap_or_else(||"[]".to_string()),
                "counterpicks_json":filter(counterpicks,ck),"synergies_json":filter(synergies,ck),
                "image_tile_url":format!("/champion-tiles/{ck}.webp"),"image_splash_url":format!("/champion-splash/{ck}.webp")})
        }).collect()
    })
}
