//! Unified command dispatch — single match statement for ALL game commands.
//!
//! Both Tauri and the web server call this function. The caller is responsible
//! for extracting typed arguments from JSON and persisting the game when
//! `DispatchResult::GameModified` is returned.
//!
//! Platform-specific concerns (file I/O, auth, StateManager) live in the
//! caller — not here.

use serde_json::{json, Value};
use std::collections::HashSet;

use crate::domain::team::TeamKind;
use crate::game::Game;
use crate::commands;

// ── Helpers ──────────────────────────────────────────────────

fn string_arg(args: &Value, names: &[&str]) -> Result<String, String> {
    for name in names {
        if let Some(v) = args.get(name).and_then(|v| v.as_str()) {
            return Ok(v.to_string());
        }
    }
    Err(format!("Missing required argument: {}", names.join("/")))
}

fn optional_string_arg(args: &Value, names: &[&str]) -> Option<String> {
    for name in names {
        if let Some(v) = args.get(name).and_then(|v| v.as_str()) {
            return Some(v.to_string());
        }
    }
    None
}

fn string_vec_arg(args: &Value, names: &[&str]) -> Result<Vec<String>, String> {
    for name in names {
        if let Some(v) = args.get(name).and_then(|v| v.as_array()) {
            return Ok(v.iter().filter_map(|x| x.as_str().map(String::from)).collect());
        }
    }
    Err(format!("Missing required array argument: {}", names.join("/")))
}

fn manager_team_id(game: &Game) -> Result<String, String> {
    game.manager.team_id.clone().ok_or_else(|| "No team assigned".to_string())
}

// ── Result ───────────────────────────────────────────────────

#[derive(Debug)]
pub enum DispatchResult {
    /// Game was mutated — caller should persist and return game to frontend
    GameModified(Value),
    /// Read-only query — no persistence needed
    Query(Value),
}

// ── Dispatch ─────────────────────────────────────────────────

pub fn dispatch(command: &str, args: &Value, game: &mut Game) -> Result<DispatchResult, String> {
    match command {
        // ── Debug ───────────────────────────────────────────
        "debug_log" => {
            if let Some(msg) = args.get("message").and_then(|v| v.as_str()) {
                log::debug!("{msg}");
            }
            Ok(DispatchResult::Query(Value::Null))
        }

        // ── Game lifecycle ──────────────────────────────────
        "get_active_game" => {
            crate::champions::bootstrap_champion_state(game);
            Ok(DispatchResult::Query(json!(game)))
        }
        "save_game" => Ok(DispatchResult::GameModified(Value::Null)),
        "advance_time" => {
            crate::turn::process_day(game);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "advance_time_with_mode" => {
            crate::turn::process_day(game);
            Ok(DispatchResult::GameModified(json!({"action":"advanced","game":game})))
        }

        // ── World data ──────────────────────────────────────
        "get_team_selection_data" => Ok(DispatchResult::Query(json!({
            "manager": game.manager,
            "teams": game.teams.iter().filter(|t| t.team_kind != TeamKind::Academy).cloned().collect::<Vec<_>>(),
            "players": game.players,
        }))),
        "check_blocking_actions" => {
            Ok(DispatchResult::Query(json!(crate::time_blockers::compute_blocking_actions(game))))
        }
        "relocalize_social_feed" => {
            let lang = optional_string_arg(args, &["language", "locale"]).unwrap_or_else(|| "en".to_string());
            crate::social::relocalize_social_posts(game, &lang);
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Training ────────────────────────────────────────
        "set_training" => {
            let focus = string_arg(args, &["focus"])?;
            let intensity = string_arg(args, &["intensity"])?;
            commands::set_training(game, &manager_team_id(game)?, &focus, &intensity);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "set_training_schedule" => {
            let schedule = string_arg(args, &["schedule"])?;
            commands::set_training_schedule(game, &manager_team_id(game)?, &schedule);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "set_training_groups" => {
            let groups = args.get("groups").and_then(|v| v.as_array()).cloned().unwrap_or_default();
            commands::set_training_groups(game, &manager_team_id(game)?, &groups);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "set_player_training_focus" => {
            let pid = string_arg(args, &["playerId", "player_id"]).unwrap_or_default();
            commands::set_player_training_focus(game, &pid, optional_string_arg(args, &["focus"]).as_deref());
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Lineup ──────────────────────────────────────────
        "set_active_lineup" | "set_starting_xi" => {
            let ids = string_vec_arg(args, &["playerIds", "player_ids"])?;
            commands::set_active_lineup(game, &manager_team_id(game)?, ids);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "set_lol_tactics" => {
            let tactics: crate::domain::team::LolTactics = serde_json::from_value(args.get("tactics").cloned().unwrap_or_default())
                .map_err(|e| format!("invalid tactics: {e}"))?;
            commands::set_lol_tactics(game, &manager_team_id(game)?, tactics);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "set_team_roles" => {
            let roles: crate::domain::team::TeamRoles = serde_json::from_value(args.get("roles").cloned().unwrap_or_default())
                .map_err(|e| format!("invalid roles: {e}"))?;
            commands::set_team_roles(game, &manager_team_id(game)?, roles);
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Draft ───────────────────────────────────────────
        "set_draft_strategy" => {
            let value = string_arg(args, &["value", "strategy"])?;
            commands::set_draft_strategy(game, &manager_team_id(game)?, &value);
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Scrims ──────────────────────────────────────────
        "set_weekly_scrims" => {
            let ids = string_vec_arg(args, &["opponentIds", "opponent_ids"])?;
            commands::set_weekly_scrims(game, &manager_team_id(game)?, ids);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "set_weekly_scrim_plans" => {
            let plans: Vec<Vec<String>> = serde_json::from_value(args.get("plans").cloned().unwrap_or_default())
                .map_err(|e| format!("invalid scrim plans: {e}"))?;
            commands::set_weekly_scrim_plans(game, &manager_team_id(game)?, plans);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "set_weekly_scrim_slots" => {
            let slots = args.get("slots").and_then(|v| v.as_u64()).unwrap_or(3) as u8;
            commands::set_weekly_scrim_slots(game, &manager_team_id(game)?, slots);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "set_weekly_scrim_objective" => {
            let obj = optional_string_arg(args, &["objective", "focus"]);
            let scrim_focus = obj.as_deref().and_then(|s| {
                use crate::domain::team::ScrimFocus;
                match s {
                    "ChampionPool" => Some(ScrimFocus::ChampionPool),
                    "DraftPrep" => Some(ScrimFocus::DraftPrep),
                    "EarlyGame" => Some(ScrimFocus::EarlyGame),
                    "Macro" => Some(ScrimFocus::Macro),
                    "Mental" => Some(ScrimFocus::Mental),
                    "Teamfighting" => Some(ScrimFocus::Teamfighting),
                    _ => None,
                }
            });
            commands::set_weekly_scrim_objective(game, &manager_team_id(game)?, scrim_focus);
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Inbox ───────────────────────────────────────────
        "mark_message_read" => {
            let mid = string_arg(args, &["messageId", "message_id"])?;
            commands::mark_message_read(game, &mid);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "mark_all_messages_read" => {
            commands::mark_all_messages_read(game);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "delete_message" => {
            let mid = string_arg(args, &["messageId", "message_id"])?;
            commands::delete_message(game, &mid);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "delete_messages" => {
            let ids: HashSet<String> = args.get("ids").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                .unwrap_or_default();
            commands::delete_messages(game, &ids);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "clear_old_messages" => {
            commands::clear_old_messages(game);
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Staff ───────────────────────────────────────────
        "hire_staff" => {
            let sid = string_arg(args, &["staffId", "staff_id"])?;
            commands::hire_staff(game, &sid, &manager_team_id(game)?);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "release_staff" => {
            let sid = string_arg(args, &["staffId", "staff_id"])?;
            commands::release_staff(game, &sid, &manager_team_id(game)?);
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Scouting ────────────────────────────────────────
        "send_scout" => {
            let sid = string_arg(args, &["scoutId", "scout_id"])?;
            let pid = string_arg(args, &["playerId", "player_id"])?;
            commands::send_scout(game, &sid, &pid);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "start_potential_research" => {
            let pid = string_arg(args, &["playerId", "player_id"])?;
            commands::start_potential_research(game, &pid);
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Contracts ───────────────────────────────────────
        "release_player_contract" => {
            let pid = string_arg(args, &["playerId", "player_id"])?;
            commands::release_player_contract(game, &pid);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "toggle_transfer_list" => {
            let pid = string_arg(args, &["playerId", "player_id"])?;
            commands::toggle_transfer_list(game, &pid);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "toggle_loan_list" => {
            let pid = string_arg(args, &["playerId", "player_id"])?;
            commands::toggle_loan_list(game, &pid);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "get_transfer_history" => {
            let history = commands::get_transfer_history(game);
            Ok(DispatchResult::Query(json!(history)))
        }

        // ── Academies ───────────────────────────────────────
        "get_academy_acquisition_options" => {
            let ptid = string_arg(args, &["parentTeamId", "parent_team_id"])?;
            commands::bootstrap_academy_pool(game);
            let (options, blocked) = crate::academy::get_acquisition_options(game, &ptid);
            Ok(DispatchResult::Query(json!({
                "parent_team_id": ptid,
                "acquisition_allowed": blocked.is_none(),
                "blocked_reason": blocked,
                "options": options,
            })))
        }
        "acquire_academy_team" => {
            let req = args.get("request").cloned().unwrap_or(args.clone());
            let pid = string_arg(&req, &["parentTeamId", "parent_team_id"])?;
            let sid = string_arg(&req, &["sourceTeamId", "source_team_id"])?;
            let cn = optional_string_arg(&req, &["customName", "custom_name"]);
            let cs = optional_string_arg(&req, &["customShortName", "custom_short_name"]);
            commands::bootstrap_academy_pool(game);
            crate::academy::acquire_academy(game, &pid, &sid, cn.as_deref(), cs.as_deref())?;
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "promote_academy_player" => {
            let pid = string_arg(args, &["playerId", "player_id"])?;
            commands::promote_academy_player(game, &pid, &manager_team_id(game)?);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "demote_main_player_to_academy" => {
            let pid = string_arg(args, &["playerId", "player_id"])?;
            let tid = manager_team_id(game)?;
            let aid = game.teams.iter()
                .find(|t| t.parent_team_id.as_deref() == Some(&tid) && t.team_kind == TeamKind::Academy)
                .map(|t| t.id.clone()).unwrap_or_default();
            commands::demote_academy_player(game, &pid, &aid);
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Champions ───────────────────────────────────────
        "set_player_champion_training_target" => {
            let pid = string_arg(args, &["playerId", "player_id"]).unwrap_or_default();
            let ck = string_arg(args, &["championKey", "champion_key"]).unwrap_or_default();
            commands::set_player_champion_training_target(game, &pid, &ck);
            Ok(DispatchResult::GameModified(json!(game)))
        }
        "delegate_champion_training" => {
            commands::delegate_champion_training(game);
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Manager ─────────────────────────────────────────
        "update_manager_profile" => {
            let first_name = optional_string_arg(args, &["firstName", "first_name"]);
            let last_name = optional_string_arg(args, &["lastName", "last_name"]);
            let nickname = optional_string_arg(args, &["nickname"]);
            let nationality = optional_string_arg(args, &["nationality"]);
            commands::update_manager_profile(game, first_name.as_deref(), last_name.as_deref(),
                nickname.as_deref(), nationality.as_deref());
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Social ──────────────────────────────────────────
        "create_social_post" => {
            let text = string_arg(args, &["text", "content"])?;
            commands::create_social_post(game, &text);
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Player ──────────────────────────────────────────
        "reroll_player_role" => {
            let pid = string_arg(args, &["playerId", "player_id"])?;
            commands::reroll_player_role(game, &pid);
            Ok(DispatchResult::GameModified(json!(game)))
        }

        // ── Unknown ─────────────────────────────────────────
        _ => Err(format!("Unknown command: {command}")),
    }
}
