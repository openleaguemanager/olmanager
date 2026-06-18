use chrono::TimeZone;
use log::info;
use tauri::State;

use olm_core::end_of_season::EndOfSeasonSummary;
use olm_core::generator::definitions::CompetitionManifest;
use olm_core::state::StateManager;

/// Debug command: instantly complete all remaining fixtures in the current split,
/// put the user's team in first place, and advance to the next split/season.
/// This is useful for QA/testing international tournament flows.
#[tauri::command]
pub fn debug_skip_split(state: State<'_, StateManager>) -> Result<serde_json::Value, String> {
    info!("[cmd] debug_skip_split");

    let mut game = state
        .get_game(|g| g.clone())
        .ok_or("No active game session".to_string())?;

    // 1. Complete all fixtures in the active league (user wins everything)
    olm_core::debug::debug_complete_all_split_fixtures(&mut game);

    // 2. Generate playoffs if the regular season just finished
    olm_core::turn::maybe_schedule_playoffs(&mut game);

    // 3. Complete any newly-generated playoff fixtures
    olm_core::debug::debug_complete_all_split_fixtures(&mut game);

    // 4. Also complete background leagues so their champions are resolved
    olm_core::debug::debug_complete_all_leagues(&mut game);

    // 5. Save the intermediate state
    state.set_game(game.clone());

    // 6. Check if season is complete and advance
    if !olm_core::end_of_season::is_season_complete(&game) {
        return Ok(serde_json::json!({
            "success": true,
            "message": "Fixtures completed but season not fully complete (playoffs may remain).",
            "game": game,
        }));
    }

    // 7. Resolve manifest and advance
    let competition_id = game.user_competition_id.as_deref().unwrap_or("");
    let manifest = if competition_id.is_empty() {
        None
    } else {
        let data_dir = olm_core::state::RESOURCE_DATA_DIR.get();
        data_dir.and_then(|dir| {
            let path = dir.join("competitions").join(competition_id).join("manifest.json");
            std::fs::read_to_string(path).ok()
                .and_then(|json| serde_json::from_str::<CompetitionManifest>(&json).ok())
        })
    };

    let summary = if let Some(ref manifest) = manifest {
        let is_wrapping = game
            .active_league()
            .is_some_and(|league| league.split_index + 1 >= manifest.schedule.splits.len());

        if is_wrapping {
            info!("[cmd] debug_skip_split: season complete, running full end-of-season");
            if let Some(scheduled) = olm_core::end_of_season::schedule_tournament_for_next_split(&game, manifest) {
                game.scheduled_tournaments.push(scheduled);
            }
            let summary = olm_core::end_of_season::process_end_of_season_with_config(
                &mut game, Some(manifest),
            );
            olm_core::end_of_season::process_end_of_split(&mut game, manifest);
            summary
        } else {
            info!("[cmd] debug_skip_split: split complete, advancing to next split");
            let pre_advance_league = game.active_league().cloned();
            let pre_advance_standings = pre_advance_league
                .as_ref()
                .map(|l| l.sorted_standings())
                .unwrap_or_default();
            let pre_advance_user_team_id = game.manager.team_id.clone();
            let pre_advance_user_pos = pre_advance_user_team_id
                .as_deref()
                .and_then(|tid| pre_advance_standings.iter().position(|s| s.team_id == tid))
                .map(|i| i + 1)
                .unwrap_or(0);
            let pre_advance_user_st = pre_advance_user_pos
                .checked_sub(1)
                .and_then(|i| pre_advance_standings.get(i));
            let pre_advance_champion = pre_advance_standings.first();
            let pre_advance_champion_name = pre_advance_champion
                .and_then(|c| game.teams.iter().find(|t| t.id == c.team_id))
                .map(|t| t.name.clone())
                .unwrap_or_default();

            if let Some(scheduled) = olm_core::end_of_season::schedule_tournament_for_next_split(&game, manifest) {
                game.scheduled_tournaments.push(scheduled);
            }

            if let Some(l) = game.active_league_mut() {
                l.split_index += 1;
            }
            olm_core::end_of_season::process_end_of_split(&mut game, manifest);
            EndOfSeasonSummary {
                season: pre_advance_league.as_ref().map(|l| l.season).unwrap_or(0),
                league_name: pre_advance_league.as_ref().map(|l| l.name.clone()).unwrap_or_default(),
                champion_id: pre_advance_champion.map(|c| c.team_id.clone()).unwrap_or_default(),
                champion_name: pre_advance_champion_name,
                user_position: pre_advance_user_pos as u32,
                user_points: pre_advance_user_st.map(|s| s.points).unwrap_or(0),
                user_won: pre_advance_user_st.map(|s| s.won).unwrap_or(0),
                user_lost: pre_advance_user_st.map(|s| s.lost).unwrap_or(0),
                user_maps_won: pre_advance_user_st.map(|s| s.maps_won).unwrap_or(0),
                user_maps_lost: pre_advance_user_st.map(|s| s.maps_lost).unwrap_or(0),
                total_teams: pre_advance_standings.len() as u32,
                ..Default::default()
            }
        }
    } else {
        info!("[cmd] debug_skip_split: using legacy schedule");
        olm_core::end_of_season::process_end_of_season(&mut game)
    };

    // NOTE: We do NOT call process_background_seasons here because it replaces
    // background leagues and loses their historical standings, which breaks
    // international tournament qualification. Background leagues are left
    // completed (fixtures + standings intact) so qualification can read them.

    // Advance the game clock to the first scheduled tournament start date,
    // or the first fixture date of the next split, so that the daily simulation
    // loop can actually hit those fixtures instead of leaving them in the future.
    let target_date = game
        .scheduled_tournaments
        .first()
        .map(|st| st.start_date.clone())
        .or_else(|| {
            game.leagues
                .iter()
                .filter(|l| {
                    l.fixtures
                        .iter()
                        .any(|f| f.status == olm_core::domain::league::FixtureStatus::Scheduled)
                })
                .flat_map(|l| {
                    l.fixtures
                        .iter()
                        .filter(|f| f.status == olm_core::domain::league::FixtureStatus::Scheduled)
                        .map(|f| f.date.clone())
                })
                .min()
        });

    if let Some(first_date) = target_date {
        if let Ok(naive) = chrono::NaiveDate::parse_from_str(&first_date, "%Y-%m-%d") {
            let new_date = chrono::Utc.from_utc_datetime(&naive.and_hms_opt(0, 0, 0).unwrap());
            if new_date > game.clock.current_date {
                game.clock.current_date = new_date;
                info!("[cmd] debug_skip_split: advanced clock to {}", first_date);
            }
        }
    }

    // Reset tournament queuing flag, but KEEP active_tournament_id if one was set
    // (so the UI shows the tournament schedule immediately).
    game.tournament_queuing = false;
    if game.active_tournament_id.is_none() {
        // Only clear if no tournament was injected; otherwise it stays active.
        info!("[cmd] debug_skip_split: no tournament injected, clearing active_tournament_id");
    }

    // Save final state
    state.set_game(game.clone());

    let has_tournament = game.active_tournament_id.is_some();

    Ok(serde_json::json!({
        "success": true,
        "message": if has_tournament { "Split skipped. Tournament injected and active." } else { "Split skipped. No tournament for this window." },
        "game": game,
        "summary": summary,
    }))
}
