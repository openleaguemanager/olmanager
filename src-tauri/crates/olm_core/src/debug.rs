use crate::domain::league::{FixtureStatus, League, MatchEndReason, MatchResult};
use crate::game::Game;
use log::info;
use rand::RngExt;
use rand::rngs::StdRng;
use rand::SeedableRng;

/// Debug helper: instantly completes every remaining scheduled fixture in the
/// active league's current split.
///
/// - The user's team wins every match (target_wins vs 0).
/// - All other fixtures get a random 50/50 result.
/// - Standings are updated but player stats / stamina / morale are skipped
///   (this is a debug fast-forward, not a real simulation).
///
/// Returns `true` if any fixture was completed.
pub fn debug_complete_all_split_fixtures(game: &mut Game) -> bool {
    let user_team_id = match game.manager.team_id.as_deref() {
        Some(id) => id.to_string(),
        None => return false,
    };

    let league = match game.active_simulation_league_mut() {
        Some(l) => l,
        None => return false,
    };

    let mut completed_count = 0usize;

    for idx in 0..league.fixtures.len() {
        if league.fixtures[idx].status != FixtureStatus::Scheduled {
            continue;
        }

        let home_id = league.fixtures[idx].home_team_id.clone();
        let away_id = league.fixtures[idx].away_team_id.clone();
        let best_of = league.fixtures[idx].best_of.max(1);
        let target_wins = (best_of / 2) + 1;

        let (home_wins, away_wins) = if home_id == user_team_id {
            (target_wins, 0)
        } else if away_id == user_team_id {
            (0, target_wins)
        } else {
            // Random 50/50 for other teams
            let mut rng = StdRng::seed_from_u64(42);
            if rng.random_bool(0.5) {
                (target_wins, 0)
            } else {
                (0, target_wins)
            }
        };

        let result = MatchResult {
            home_wins,
            away_wins,
            ended_by: MatchEndReason::NexusDestroyed,
            game_duration_seconds: 1800,
            report: None,
        };

        // Update standings for league fixtures
        if league.fixtures[idx].counts_for_league_standings() {
            if let Some(entry) = league.standings.iter_mut().find(|e| e.team_id == home_id) {
                entry.record_result(home_wins, away_wins);
            }
            if let Some(entry) = league.standings.iter_mut().find(|e| e.team_id == away_id) {
                entry.record_result(away_wins, home_wins);
            }
        }

        league.fixtures[idx].status = FixtureStatus::Completed;
        league.fixtures[idx].result = Some(result);
        completed_count += 1;
    }

    if completed_count > 0 {
        info!(
            "[debug] completed {} fixtures for split in league '{}'",
            completed_count, league.name
        );
    }

    completed_count > 0
}

/// Debug helper: completes remaining scheduled fixtures for the active league
/// AND all background leagues so that their playoffs/champions are resolved.
pub fn debug_complete_all_leagues(game: &mut Game) {
    let original_league_idx = game
        .user_competition_id
        .as_deref()
        .and_then(|cid| game.leagues.iter().position(|l| l.competition_id.as_deref() == Some(cid)))
        .unwrap_or(0);

    // Complete user's league first
    if let Some(league) = game.leagues.get_mut(original_league_idx) {
        debug_complete_league_fixtures(league);
    }

    // Complete all background leagues
    for idx in 0..game.leagues.len() {
        if idx == original_league_idx {
            continue;
        }
        if let Some(league) = game.leagues.get_mut(idx) {
            debug_complete_league_fixtures(league);
        }
    }
}

fn debug_complete_league_fixtures(league: &mut League) {
    let mut rng = StdRng::seed_from_u64(42);

    for idx in 0..league.fixtures.len() {
        if league.fixtures[idx].status != FixtureStatus::Scheduled {
            continue;
        }

        let home_id = league.fixtures[idx].home_team_id.clone();
        let away_id = league.fixtures[idx].away_team_id.clone();
        let best_of = league.fixtures[idx].best_of.max(1);
        let target_wins = (best_of / 2) + 1;

        let (home_wins, away_wins) = if rng.random_bool(0.5) {
            (target_wins, 0)
        } else {
            (0, target_wins)
        };

        let result = MatchResult {
            home_wins,
            away_wins,
            ended_by: MatchEndReason::NexusDestroyed,
            game_duration_seconds: 1800,
            report: None,
        };

        if league.fixtures[idx].counts_for_league_standings() {
            if let Some(entry) = league.standings.iter_mut().find(|e| e.team_id == home_id) {
                entry.record_result(home_wins, away_wins);
            }
            if let Some(entry) = league.standings.iter_mut().find(|e| e.team_id == away_id) {
                entry.record_result(away_wins, home_wins);
            }
        }

        league.fixtures[idx].status = FixtureStatus::Completed;
        league.fixtures[idx].result = Some(result);
    }
}
