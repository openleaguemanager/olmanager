use std::collections::HashMap;

use axum::http::StatusCode;
use olm_core::domain::team::TeamKind;
use olm_core::game::Game;
use olm_core::generator::definitions::LeagueSelectionData;
use serde_json::{json, Value};

use crate::data;

pub struct CommandResult { pub value: Value, pub persist: bool }
pub struct CommandError { pub status: StatusCode, pub message: String }

impl CommandError {
    fn bad_request(m: impl Into<String>) -> Self { Self { status: StatusCode::BAD_REQUEST, message: m.into() } }
    fn not_found(m: impl Into<String>) -> Self { Self { status: StatusCode::NOT_FOUND, message: m.into() } }
}

pub fn dispatch(command: &str, args: Value, game: &mut Game) -> Result<CommandResult, CommandError> {
    match command {
        "debug_log" => {
            if let Some(msg) = args.get("message").and_then(|v| v.as_str()) { tracing::debug!("{msg}"); }
            ok(Value::Null, false)
        }
        "get_active_game" => {
            let rf = data::repair_player_financials(game);
            let rc = data::repair_active_competition(game);
            olm_core::champions::bootstrap_champion_state(game);
            ok(json!(game), rf || rc)
        }
        "save_game" => ok(Value::Null, true),
        "advance_time" => { olm_core::turn::process_day(game); ok(json!(game), true) }
        "advance_time_with_mode" => { olm_core::turn::process_day(game); ok(json!({"action":"advanced","game":game}), true) }
        "select_team" => {
            let team_id = string_arg(&args, &["teamId","team_id"])?;
            data::select_team(game, &team_id).map_err(CommandError::bad_request)?;
            ok(json!(game), true)
        }
        "get_team_selection_data" => ok(json!({
            "manager":game.manager,
            "teams":game.teams.iter().filter(|t|t.team_kind!=TeamKind::Academy).cloned().collect::<Vec<_>>(),
            "players":game.players
        }), false),
        "get_league_selection_data" => { ok(json!(league_selection_data().map_err(CommandError::bad_request)?), false) }
        "check_blocking_actions" => ok(json!(crate::time_blockers::compute_blocking_actions(game)), false),
        "relocalize_social_feed" => {
            let lang = optional_string_arg(&args, &["language","locale"]).unwrap_or_else(||"en".to_string());
            olm_core::social::relocalize_social_posts(game, &lang);
            ok(json!(game), true)
        }
        "get_champions" => ok(json!(champions_catalog()), false),

        // ── Training ────────────────────────────────────
        "set_training" => {
            let focus = string_arg(&args, &["focus"])?;
            let intensity = string_arg(&args, &["intensity"])?;
            olm_core::commands::set_training(game, &manager_team_id(game)?, &focus, &intensity);
            ok(json!(game), true)
        }
        "set_training_schedule" => { ok(json!(game), true) }
        "set_training_groups" => { ok(json!(game), true) }
        "set_player_training_focus" => {
            let pid = string_arg(&args, &["playerId","player_id"]).unwrap_or_default();
            olm_core::commands::set_player_training_focus(game, &pid, optional_string_arg(&args, &["focus"]).as_deref());
            ok(json!(game), true)
        }

        // ── Lineup ──────────────────────────────────────
        "set_active_lineup"|"set_starting_xi" => {
            let ids = string_vec_arg(&args, &["playerIds","player_ids"])?;
            olm_core::commands::set_active_lineup(game, &manager_team_id(game)?, ids);
            ok(json!(game), true)
        }

        // ── Draft Strategy ──────────────────────────────
        "set_draft_strategy" => {
            let v = string_arg(&args, &["draftStrategy","draft_strategy"])?;
            olm_core::commands::set_draft_strategy(game, &manager_team_id(game)?, &v);
            ok(json!(game), true)
        }

        // ── Tactics ─────────────────────────────────────
        "set_lol_tactics" => {
            let tactics: olm_core::domain::team::LolTactics = serde_json::from_value(
                get_arg(&args, &["lolTactics","lol_tactics"])?.clone()
            ).map_err(|e| CommandError::bad_request(format!("invalid lol_tactics: {e}")))?;
            olm_core::commands::set_lol_tactics(game, &manager_team_id(game)?, tactics);
            ok(json!(game), true)
        }

        // ── Team Roles ──────────────────────────────────
        "set_team_roles" => {
            let roles: olm_core::domain::team::TeamRoles = serde_json::from_value(
                get_arg(&args, &["teamRoles","team_roles"])?.clone()
            ).map_err(|e| CommandError::bad_request(format!("invalid team_roles: {e}")))?;
            olm_core::commands::set_team_roles(game, &manager_team_id(game)?, roles);
            ok(json!(game), true)
        }

        // ── Scrims ──────────────────────────────────────
        "set_weekly_scrims" => {
            let ids = args.get("opponentTeamIds").and_then(|v|v.as_array())
                .map(|a| a.iter().filter_map(|v|v.as_str().map(String::from)).collect()).unwrap_or_default();
            olm_core::commands::set_weekly_scrims(game, &manager_team_id(game)?, ids);
            ok(json!(game), true)
        }
        "set_weekly_scrim_plans" => {
            let plans = args.get("plans").and_then(|v|v.as_array())
                .map(|a| a.iter().map(|slot|slot.as_array().map(|s|s.iter().filter_map(|v|v.as_str().map(String::from)).collect()).unwrap_or_default()).collect()).unwrap_or_default();
            olm_core::commands::set_weekly_scrim_plans(game, &manager_team_id(game)?, plans);
            ok(json!(game), true)
        }
        "set_weekly_scrim_slots" => {
            let slots = args.get("slots").and_then(|v|v.as_u64()).unwrap_or(0) as u8;
            olm_core::commands::set_weekly_scrim_slots(game, &manager_team_id(game)?, slots);
            ok(json!(game), true)
        }
        "set_weekly_scrim_objective" => {
            let obj = optional_string_arg(&args, &["objective"]);
            let focus = obj.and_then(|o| serde_json::from_str(&format!("\"{o}\"")).ok());
            olm_core::commands::set_weekly_scrim_objective(game, &manager_team_id(game)?, focus);
            ok(json!(game), true)
        }
        "finalize_weekly_scrim_setup"|"auto_configure_weekly_scrim_setup"|"cancel_todays_scrims"
        |"choose_post_scrim_decision"|"choose_daily_scrim_action"|"delegate_scrim_decision" => ok(json!(game), true),
        "get_scrim_context" => ok(json!({
            "today":{"state":"NoScrimToday","slot_index":null,"opponent_team_id":null,"resolved_opponent_team_id":null,"objective":null,"report":null,
                "can_edit_plan":true,"can_cancel":false,"can_review":false,"can_view_weekly_plan":true,"has_official_match":false,"primary_action":null,"push_through_recommended":false},
            "week":{"week_key":"","objective":null,"capacity":0,"planned":0,"reputation":0,"cancellations":0,"played":0,"wins":0,"losses":0,
                "loss_streak":0,"avg_quality":0.0,"top_focus":null,"top_issue":null,"next_official_rival_team_id":null,"next_official_rival_competition":null,"setup_locked":false,"setup_locked_reason":null,"can_finalize_setup":true,
                "slots":[],"latest_reports":[]}
        }), false),

        // ── Staff ────────────────────────────────────────
        "hire_staff" => {
            let sid = string_arg(&args, &["staffId","staff_id"])?;
            olm_core::commands::hire_staff(game, &sid, &manager_team_id(game)?);
            ok(json!(game), true)
        }
        "release_staff" => {
            let sid = string_arg(&args, &["staffId","staff_id"])?;
            olm_core::commands::release_staff(game, &sid, &manager_team_id(game)?);
            ok(json!(game), true)
        }

        // ── Inbox ────────────────────────────────────────
        "mark_message_read" => {
            let mid = string_arg(&args, &["messageId","message_id"])?;
            olm_core::commands::mark_message_read(game, &mid);
            ok(json!(game), true)
        }
        "mark_all_messages_read" => { olm_core::commands::mark_all_messages_read(game); ok(json!(game), true) }
        "clear_old_messages" => { olm_core::commands::clear_old_messages(game); ok(json!(game), true) }
        "delete_message" => {
            let mid = string_arg(&args, &["messageId","message_id"])?;
            olm_core::commands::delete_message(game, &mid);
            ok(json!(game), true)
        }
        "delete_messages" => {
            if let Some(ids) = args.get("messageIds").and_then(|v|v.as_array()) {
                let set: std::collections::HashSet<String> = ids.iter().filter_map(|v|v.as_str().map(String::from)).collect();
                olm_core::commands::delete_messages(game, &set);
            }
            ok(json!(game), true)
        }
        "resolve_message_action" => ok(json!(game), true),

        // ── Social ───────────────────────────────────────
        "get_social_feed" => ok(json!(game.social_posts), false),
        "create_manager_social_post" => {
            if let Some(text) = args.get("text").and_then(|v|v.as_str()) {
                olm_core::commands::create_social_post(game, text);
            }
            ok(json!(game), true)
        }
        "get_social_accounts" => ok(json!(game.social_accounts), false),
        "save_social_accounts" => {
            if let Some(a) = args.get("accounts").and_then(|v|serde_json::from_value(v.clone()).ok()) { game.social_accounts = a; }
            ok(json!(game), true)
        }
        "get_social_templates" => ok(json!(game.social_templates), false),
        "save_social_templates" => {
            if let Some(t) = args.get("templates").and_then(|v|serde_json::from_value(v.clone()).ok()) { game.social_templates = t; }
            ok(json!(game), true)
        }

        // ── Scouting ──────────────────────────────────────
        "send_scout" => {
            let pid = string_arg(&args, &["playerId","player_id"]).unwrap_or_default();
            let sid = string_arg(&args, &["scoutId","scout_id"]).unwrap_or_default();
            olm_core::commands::send_scout(game, &sid, &pid);
            ok(json!(game), true)
        }
        "start_potential_research" => {
            let pid = string_arg(&args, &["playerId","player_id"])?;
            olm_core::commands::start_potential_research(game, &pid);
            ok(json!(game), true)
        }

        // ── Champions ─────────────────────────────────────
        "set_player_champion_training_target" => {
            let pid = string_arg(&args, &["playerId","player_id"]).unwrap_or_default();
            let ck = string_arg(&args, &["championKey","champion_key"]).unwrap_or_default();
            olm_core::commands::set_player_champion_training_target(game, &pid, &ck);
            ok(json!(game), true)
        }
        "delegate_champion_training" => { olm_core::commands::delegate_champion_training(game); ok(json!(game), true) }

        // ── Academies ─────────────────────────────────────
        "get_academy_acquisition_options" => {
            let ptid = string_arg(&args, &["parentTeamId","parent_team_id"])?;
            olm_core::commands::bootstrap_academy_pool(game);
            let parent = game.teams.iter().find(|t|t.id==ptid).cloned()
                .ok_or_else(|| CommandError::bad_request(format!("Team '{ptid}' not found")))?;
            let occupied: std::collections::HashSet<String> = game.teams.iter()
                .filter(|t|t.team_kind==TeamKind::Academy&&t.parent_team_id.is_some())
                .flat_map(|t|{let mut ids=vec![t.id.clone()];if let Some(ref m)=t.academy{ids.push(m.source_team_id.clone())};ids}).collect();
            let taken: std::collections::HashSet<String> = game.teams.iter()
                .filter(|t|t.team_kind==TeamKind::Academy&&t.parent_team_id.is_some())
                .filter_map(|t|t.academy.as_ref().map(|m|norm(&m.original_name))).collect();
            let options: Vec<olm_core::academy::AcademyAcquisitionOption> =
                olm_core::academy::eligible_academy_acquisition_options(&parent.country,
                    olm_core::academy::academy_erl_catalog(), olm_core::academy::academy_candidate_catalog())
                .into_iter().filter(|o|!occupied.contains(&o.source_team_id)&&!taken.contains(&norm(&o.name))).collect();
            let blocked = if!parent.is_main(){Some("Academy can only be acquired for a main team".to_string())}
                else if parent.academy_team_id.is_some(){Some("Parent team already has academy".to_string())}
                else if options.is_empty(){Some("No free academy candidates available".to_string())}
                else if options.iter().all(|o|parent.finance<o.acquisition_cost){Some("Insufficient funds".to_string())}
                else{None};
            ok(json!({"parent_team_id":ptid,"acquisition_allowed":blocked.is_none(),"blocked_reason":blocked,"options":options}), false)
        }
        "acquire_academy_team" => {
            let req = args.get("request").cloned().unwrap_or(args);
            let pid = string_arg(&req, &["parentTeamId","parent_team_id"])?;
            let sid = string_arg(&req, &["sourceTeamId","source_team_id"])?;
            let cn = optional_string_arg(&req, &["customName","custom_name"]);
            let cs = optional_string_arg(&req, &["customShortName","custom_short_name"]);
            let pidx = game.teams.iter().position(|t|t.id==pid).ok_or_else(||CommandError::bad_request("Parent team not found"))?;
            olm_core::commands::bootstrap_academy_pool(game);
            let a = olm_core::academy::eligible_academy_acquisition_options(&game.teams[pidx].country,
                    olm_core::academy::academy_erl_catalog(), olm_core::academy::academy_candidate_catalog())
                .into_iter().find(|o|o.source_team_id==sid);
            if let Some(opt) = a {
                game.teams[pidx].finance -= opt.acquisition_cost;
                game.teams[pidx].season_expenses += opt.acquisition_cost;
                game.teams[pidx].academy_team_id = Some(sid.clone());
                if let Some(idx) = game.teams.iter().position(|t|t.id==sid&&t.team_kind==TeamKind::Academy) {
                    game.teams[idx].name = cn.unwrap_or_else(||opt.name.clone());
                    game.teams[idx].short_name = cs.unwrap_or_else(||opt.short_name.clone());
                    game.teams[idx].parent_team_id = Some(pid.clone());
                    game.teams[idx].academy = Some(olm_core::domain::team::AcademyMetadata {
                        lifecycle: olm_core::domain::team::AcademyLifecycle::Active,
                        erl_assignment: olm_core::domain::team::ErlAssignment {
                            erl_league_id: opt.erl_league_id.clone(), country_rule: olm_core::domain::team::ErlAssignmentRule::Domestic,
                            fallback_reason: Some("Acquired by user".to_string()), reputation: opt.reputation,
                            acquisition_cost: opt.acquisition_cost, acquired_at: game.clock.current_date.to_rfc3339(),
                            creation_cost: 0, created_at: game.clock.current_date.to_rfc3339(),
                        },
                        source_team_id: sid.clone(), original_name: opt.name, original_short_name: opt.short_name,
                        original_logo_url: opt.logo_url, current_logo_url: None,
                        acquisition_cost: opt.acquisition_cost, acquired_at: game.clock.current_date.to_rfc3339(),
                    });
                }
            }
            ok(json!(game), true)
        }
        "promote_academy_player" => {
            let pid = string_arg(&args, &["playerId","player_id"])?;
            olm_core::commands::promote_academy_player(game, &pid, &manager_team_id(game)?);
            ok(json!(game), true)
        }
        "demote_main_player_to_academy" => {
            let pid = string_arg(&args, &["playerId","player_id"])?;
            let tid = manager_team_id(game)?;
            let aid = game.teams.iter().find(|t|t.parent_team_id.as_deref()==Some(&tid)&&t.team_kind==TeamKind::Academy).map(|t|t.id.clone()).unwrap_or_default();
            olm_core::commands::demote_academy_player(game, &pid, &aid);
            ok(json!(game), true)
        }
        "get_academy_creation_options" => dispatch("get_academy_acquisition_options", args, game),

        // ── Transfers ─────────────────────────────────────
        "make_transfer_bid"|"respond_to_offer"|"counter_offer"|"negotiate_player_wage" => ok(json!(game), true),
        "release_player_contract" => {
            let pid = string_arg(&args, &["playerId","player_id"])?;
            olm_core::commands::release_player_contract(game, &pid);
            ok(json!(game), true)
        }
        "toggle_transfer_list" => {
            let pid = string_arg(&args, &["playerId","player_id"])?;
            olm_core::commands::toggle_transfer_list(game, &pid);
            ok(json!(game), true)
        }
        "toggle_loan_list" => {
            let pid = string_arg(&args, &["playerId","player_id"])?;
            olm_core::commands::toggle_loan_list(game, &pid);
            ok(json!(game), true)
        }
        "get_transfer_history_cmd" => ok(json!(olm_core::commands::get_transfer_history(game)), false),
        "preview_transfer_bid_financial_impact" => ok(json!({"bid":{},"can_afford":false}), false),

        // ── Manager ───────────────────────────────────────
        "update_manager_profile" => {
            olm_core::commands::update_manager_profile(game,
                optional_string_arg(&args, &["firstName","first_name"]).as_deref(),
                optional_string_arg(&args, &["lastName","last_name"]).as_deref(),
                optional_string_arg(&args, &["nickname"]).as_deref(),
                optional_string_arg(&args, &["nationality"]).as_deref());
            ok(json!(game), true)
        }
        "reroll_player_lol_role" => {
            let pid = string_arg(&args, &["playerId","player_id"])?;
            olm_core::commands::reroll_player_role(game, &pid);
            ok(json!(game), true)
        }

        // ── Stats ─────────────────────────────────────────
        "get_player_match_history" => {
            let pid = string_arg(&args, &["playerId","player_id"]).unwrap_or_default();
            let h: Vec<Value> = game.stats_state.player_matches.iter().filter(|m|m.player_id==pid).map(|m|json!(m)).collect();
            ok(json!(h), false)
        }
        "get_team_match_history" => {
            let tid = string_arg(&args, &["teamId","team_id"]).unwrap_or_default();
            let h: Vec<Value> = game.stats_state.team_matches.iter().filter(|m|m.team_id==tid).map(|m|json!(m)).collect();
            ok(json!(h), false)
        }
        "get_player_stats_overview" | "get_team_stats_overview" => ok(json!({"matchesPlayed":0,"metrics":{}}), false),

        // ── Jobs ───────────────────────────────────────────
        "get_available_jobs" => ok(json!([]), false),
        "apply_for_job" => ok(json!({"success":false,"message":"Not available"}), false),

        // ── Season ─────────────────────────────────────────
        "advance_to_next_season" => ok(json!(game), true),
        "check_season_complete" => ok(json!({"complete":false}), false),
        "get_season_awards" => ok(json!({}), false),

        // ── Facilities ─────────────────────────────────────
        "upgrade_main_facility_module"|"expand_main_facility_hub"|"upgrade_facility" 
        |"propose_renewal"|"delegate_renewals"|"preview_renewal_financial_impact" => ok(json!(game), true),

        // ── Match / Sim ───────────────────────────────────
        "start_live_match"|"step_live_match"|"apply_match_command"|"get_match_snapshot"|"finish_live_match"
        |"record_fixture_champion_picks"|"apply_champion_mastery_from_draft"|"apply_team_talk"|"submit_press_conference"
        |"lol_sim_v2_init"|"lol_sim_v2_tick"|"lol_sim_v2_reset"|"lol_sim_v2_dispose"|"lol_sim_v2_run_to_completion"
        |"lol_sim_v2_skip_to_end" => ok(json!(game), true),

        "skip_to_match_day" => ok(json!({"action":"skipped","game":game}), true),
        "export_bug_report" => ok(json!({"path":""}), false),

        _ => Err(CommandError::not_found(format!("unsupported command: {command}"))),
    }
}

fn ok(value: Value, persist: bool) -> Result<CommandResult, CommandError> { Ok(CommandResult { value, persist }) }
fn norm(s: &str) -> String { s.to_lowercase().chars().filter(|c|c.is_ascii_alphanumeric()).collect() }

fn get_arg<'a>(args: &'a Value, names: &[&str]) -> Result<&'a Value, CommandError> {
    names.iter().find_map(|n|args.get(*n)).ok_or_else(||CommandError::bad_request(format!("missing: {}",names[0])))
}
fn string_arg(args: &Value, names: &[&str]) -> Result<String, CommandError> {
    get_arg(args, names)?.as_str().map(String::from).ok_or_else(||CommandError::bad_request(format!("not a string: {}",names[0])))
}
fn optional_string_arg(args: &Value, names: &[&str]) -> Option<String> {
    names.iter().find_map(|n|args.get(*n)).and_then(Value::as_str).map(String::from)
}
fn string_vec_arg(args: &Value, names: &[&str]) -> Result<Vec<String>, CommandError> {
    let arr = get_arg(args, names)?.as_array().ok_or_else(||CommandError::bad_request(format!("not an array: {}",names[0])))?;
    arr.iter().map(|i|i.as_str().map(String::from).ok_or_else(||CommandError::bad_request(format!("not strings: {}",names[0])))).collect()
}
fn manager_team_id(game: &Game) -> Result<String, CommandError> {
    game.manager.team_id.clone().ok_or_else(||CommandError::bad_request("No team assigned"))
}
fn league_selection_data() -> Result<LeagueSelectionData, String> { Ok(olm_core::competitions::build_league_selection(&data::data_dir())) }

fn champions_catalog() -> &'static Vec<Value> {
    static CATALOG: std::sync::OnceLock<Vec<Value>> = std::sync::OnceLock::new();
    CATALOG.get_or_init(|| {
        let raw = include_str!("../../../../assets/simulation/champions.json");
        let json: Value = match serde_json::from_str(raw) { Ok(v) => v, Err(_) => return Vec::new() };
        let data = json.get("data");
        let roles = match data.and_then(|d|d.get("roles")).and_then(|r|r.as_object()) { Some(m) => m, None => return Vec::new() };
        let counterpicks = data.and_then(|d|d.get("counterpicks")).and_then(|c|c.as_array());
        let synergies = data.and_then(|d|d.get("synergies")).and_then(|s|s.as_array());
        let mut k2n: HashMap<String,String> = HashMap::new();
        if let Some(a) = data.and_then(|d|d.get("display_aliases")).and_then(|a|a.as_object()) {
            for (alias,value) in a { if let Some(key) = value.as_str() { k2n.insert(key.to_string(), alias.clone()); } }
        }
        let filter = |arr: Option<&Vec<Value>>, key: &str| -> Option<String> {
            arr?.iter().filter(|i|i.get("a").and_then(|v|v.as_str())==Some(key)).collect::<Vec<_>>().into_iter()
                .filter(|i|i.get("a").and_then(|v|v.as_str())==Some(key)).next().and_then(|_|None)
        };
        let mut keys: Vec<&String> = roles.keys().collect();
        keys.sort();
        keys.iter().enumerate().map(|(i,k)|{
            let ck = k.as_str();
            json!({"id":(i as i64)+1,"name":k2n.get(ck).cloned().unwrap_or_else(||{
                let mut n=String::with_capacity(ck.len()+4);
                for (j,c) in ck.chars().enumerate(){if j>0&&c.is_uppercase(){n.push(' ')} n.push(c);} n
            }),"champion_key":ck,"roles_json":roles.get(ck).map(|v|v.to_string()).unwrap_or_else(||"[]".to_string()),
            "image_tile_url":format!("/champion-tiles/{ck}.webp"),"image_splash_url":format!("/champion-splash/{ck}.webp")})
        }).collect()
    })
}


