use crate::game::Game;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TournamentFormat {
    Fst2026,
    Msi2026,
    Worlds2026,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotType {
    Champion,
    Finalist,
    SubChampion,
    MsiWinner,
    PlayInWinner,
    Top3,
}

#[derive(Debug, Clone)]
pub struct QualificationSlot {
    pub tournament_format: TournamentFormat,
    pub source_competition_id: String,
    pub split_index: usize,
    pub slot_type: SlotType,
    pub seed: u8,
}

/// Extract standings from a background league by competition_id and split_index.
/// Returns team IDs in order of placement (1st → Nth).
/// If the exact split_index is not found, falls back to the most recent split
/// with completed standings (played > 0).
pub fn get_regional_standings(
    game: &Game,
    competition_id: &str,
    split_index: usize,
) -> Option<Vec<String>> {
    // Try exact match first
    if let Some(league) = game.leagues.iter().find(|l| {
        l.competition_id.as_deref() == Some(competition_id) && l.split_index == split_index
    }) {
        let sorted = league.sorted_standings();
        if !sorted.is_empty() && sorted.iter().any(|s| s.played > 0) {
            return Some(sorted.iter().map(|s| s.team_id.clone()).collect());
        }
    }

    // Fallback: find the league with this competition_id that has the most
    // completed standings (highest split_index where played > 0)
    let mut best_league: Option<&crate::domain::league::League> = None;
    for league in game.leagues.iter() {
        if league.competition_id.as_deref() == Some(competition_id) {
            let has_played = league.standings.iter().any(|s| s.played > 0);
            if has_played {
                if let Some(current_best) = best_league {
                    if league.split_index > current_best.split_index {
                        best_league = Some(league);
                    }
                } else {
                    best_league = Some(league);
                }
            }
        }
    }

    best_league.map(|l| {
        let sorted = l.sorted_standings();
        sorted.iter().map(|s| s.team_id.clone()).collect()
    })
}

/// Determine which region a team belongs to by looking up its competition.
pub fn team_region(game: &Game, team_id: &str) -> Option<String> {
    let team = game.teams.iter().find(|t| t.id == team_id)?;
    let cid = team.competition_id.as_deref()?;
    let manifest = game.competition_configs.get(cid)?;
    Some(manifest.region.clone())
}

/// Determine the competition_id for a team by looking up its current assignment.
pub fn team_competition_id(game: &Game, team_id: &str) -> Option<String> {
    game.teams
        .iter()
        .find(|t| t.id == team_id)
        .and_then(|t| t.competition_id.clone())
}

/// Build the list of qualified team IDs for a given tournament format.
/// Call this after the prerequisite split has completed so standings exist.
pub fn qualify_teams_for_tournament(game: &Game, format: TournamentFormat) -> Vec<String> {
    match format {
        TournamentFormat::Fst2026 => qualify_fst(game),
        TournamentFormat::Msi2026 => qualify_msi(game),
        TournamentFormat::Worlds2026 => qualify_worlds(game),
    }
}

// ---------------------------------------------------------------------------
// FST 2026 — 8 teams
// Split 1 finalists:
//   LCK × 2, LPL × 2, LEC × 1, LCS × 1, CBLOL × 1, LCP × 1
// ---------------------------------------------------------------------------
fn qualify_fst(game: &Game) -> Vec<String> {
    let mut qualified = Vec::new();
    let regions: Vec<(&str, usize)> = vec![
        ("lck", 2),
        ("lpl", 2),
        ("lec", 1),
        ("lcs", 1),
        ("cblol", 1),
        ("lcp", 1),
    ];
    for (cid, slots) in regions {
        if let Some(standings) = get_regional_standings(game, cid, 0) {
            for team_id in standings.iter().take(slots) {
                if !qualified.contains(team_id) {
                    qualified.push(team_id.clone());
                }
            }
        }
    }
    qualified
}

// ---------------------------------------------------------------------------
// MSI 2026 — 11 teams
// Split 2 champions + subchampions from major regions.
// FST winner region gets an extra slot (2 teams from that region).
// ---------------------------------------------------------------------------
fn qualify_msi(game: &Game) -> Vec<String> {
    let mut qualified = Vec::new();
    // 6 champions from split 2
    let champion_regions = vec!["lck", "lpl", "lec", "lcs", "cblol", "lcp"];
    for cid in &champion_regions {
        if let Some(standings) = get_regional_standings(game, cid, 1) {
            if let Some(first) = standings.first() {
                if !qualified.contains(first) {
                    qualified.push(first.clone());
                }
            }
        }
    }

    // Subchampions from the 4 major regions (LCK, LPL, LEC, LCS)
    let major_regions = vec!["lck", "lpl", "lec", "lcs"];
    for cid in &major_regions {
        if let Some(standings) = get_regional_standings(game, cid, 1) {
            if let Some(second) = standings.get(1) {
                if !qualified.contains(second) {
                    qualified.push(second.clone());
                }
            }
        }
    }

    // FST winner region bonus slot (2nd team from that region)
    if let Some(fst_winner) = get_tournament_winner(game, "fst") {
        if let Some(region) = team_competition_id(game, &fst_winner) {
            if let Some(standings) = get_regional_standings(game, &region, 1) {
                if let Some(second) = standings.get(1) {
                    if !qualified.contains(second) {
                        qualified.push(second.clone());
                    }
                }
            }
        }
    }

    qualified
}

// ---------------------------------------------------------------------------
// Worlds 2026 — 19 teams
// Top 3 from each region (split 2) + MSI winner + play-in winner.
// MSI winner region gets 2 byes (represented as the first two seeds).
// ---------------------------------------------------------------------------
fn qualify_worlds(game: &Game) -> Vec<String> {
    let mut qualified = Vec::new();
    let regions = vec!["lck", "lpl", "lec", "lcs", "cblol", "lcp"];
    for cid in &regions {
        if let Some(standings) = get_regional_standings(game, cid, 1) {
            for team_id in standings.iter().take(3) {
                if !qualified.contains(team_id) {
                    qualified.push(team_id.clone());
                }
            }
        }
    }

    // MSI winner
    if let Some(msi_winner) = get_tournament_winner(game, "msi") {
        if !qualified.contains(&msi_winner) {
            qualified.push(msi_winner);
        }
    }

    // 2nd best MSI region: use MSI runner-up region
    if let Some(msi_runner) = get_tournament_runner_up(game, "msi") {
        if let Some(cid) = team_competition_id(game, &msi_runner) {
            if let Some(standings) = get_regional_standings(game, &cid, 1) {
                if let Some(third) = standings.get(2) {
                    if !qualified.contains(third) {
                        qualified.push(third.clone());
                    }
                }
            }
        }
    }

    // CBLOL extra slot (already included in top-3, but ensure distinct)
    // In 2026 format CBLOL may have 2 slots; if top-3 already gave 3,
    // we still keep the distinct set.
    if let Some(standings) = get_regional_standings(game, "cblol", 1) {
        if let Some(fourth) = standings.get(3) {
            if !qualified.contains(fourth) {
                qualified.push(fourth.clone());
            }
        }
    }

    // Play-in winner placeholder — if a play-in league already exists and
    // has a winner, add them. Otherwise the slot will be filled later.
    if let Some(pi_winner) = get_tournament_winner(game, "worlds_play_in") {
        if !qualified.contains(&pi_winner) {
            qualified.push(pi_winner);
        }
    }

    qualified
}

/// Look for a completed tournament league and return the champion team ID.
fn get_tournament_winner(game: &Game, competition_id: &str) -> Option<String> {
    let league = game.leagues.iter().find(|l| {
        l.competition_id.as_deref() == Some(competition_id)
    })?;
    // Find the final fixture (max matchday) in the KO / PlayIn / Group stage
    let final_fixture = league
        .fixtures
        .iter()
        .filter(|f| {
            matches!(
                f.match_type,
                crate::domain::league::MatchType::TournamentKnockout
                    | crate::domain::league::MatchType::TournamentPlayIn
                    | crate::domain::league::MatchType::TournamentGroup
            )
        })
        .max_by_key(|f| f.matchday)?;
    let result = final_fixture.result.as_ref()?;
    if result.home_wins > result.away_wins {
        Some(final_fixture.home_team_id.clone())
    } else if result.away_wins > result.home_wins {
        Some(final_fixture.away_team_id.clone())
    } else {
        None
    }
}

/// Look for a completed tournament and return the runner-up team ID.
fn get_tournament_runner_up(game: &Game, competition_id: &str) -> Option<String> {
    let league = game.leagues.iter().find(|l| {
        l.competition_id.as_deref() == Some(competition_id)
    })?;
    let final_fixture = league
        .fixtures
        .iter()
        .filter(|f| {
            matches!(
                f.match_type,
                crate::domain::league::MatchType::TournamentKnockout
                    | crate::domain::league::MatchType::TournamentPlayIn
                    | crate::domain::league::MatchType::TournamentGroup
            )
        })
        .max_by_key(|f| f.matchday)?;
    let result = final_fixture.result.as_ref()?;
    if result.home_wins > result.away_wins {
        Some(final_fixture.away_team_id.clone())
    } else if result.away_wins > result.home_wins {
        Some(final_fixture.home_team_id.clone())
    } else {
        None
    }
}

/// Seed the qualified teams for a tournament.
/// Uses the split standings order to assign seed numbers (1 = best).
pub fn seed_qualified_teams(qualified: &[String], game: &Game, split_index: usize) -> Vec<(String, u8)> {
    let mut seeded: Vec<(String, u8, usize)> = Vec::new();
    let mut seed = 1u8;
    for team_id in qualified {
        let cid = team_competition_id(game, team_id);
        let region_pos = if let Some(ref c) = cid {
            get_regional_standings(game, c, split_index)
                .and_then(|s| s.iter().position(|id| id == team_id))
                .unwrap_or(999)
        } else {
            999
        };
        seeded.push((team_id.clone(), seed, region_pos));
        seed += 1;
    }
    // Sort by region position (ascending) so higher seeds are first
    seeded.sort_by(|(a_id, _, a_pos), (b_id, _, b_pos)| {
        a_pos.cmp(b_pos).then(a_id.cmp(b_id))
    });
    // Reassign seeds after sort
    for (idx, entry) in seeded.iter_mut().enumerate() {
        entry.1 = (idx + 1) as u8;
    }
    seeded.into_iter().map(|(id, seed, _)| (id, seed)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::league::{Fixture, FixtureStatus, League, MatchResult, MatchType, StandingEntry};
    use crate::domain::team::Team;
    use crate::game::Game;
    use crate::clock::GameClock;
    use crate::domain::manager::Manager;
    use chrono::TimeZone;

    fn make_team(id: &str, competition_id: &str) -> Team {
        let mut t = Team::new(
            id.to_string(),
            format!("Team {id}"),
            id.to_string(),
            "KR".to_string(),
            competition_id.to_string(),
            "Arena".to_string(),
            1000,
        );
        t.competition_id = Some(competition_id.to_string());
        t
    }

    fn make_league_with_results(competition_id: &str, split_index: usize, team_ids: &[String], results: &[(usize, usize)]) -> League {
        let mut league = League::new(
            competition_id.to_string(),
            competition_id.to_string(),
            2026,
            team_ids,
            Some(competition_id.to_string()),
        );
        league.split_index = split_index;
        for (idx, (w, l)) in results.iter().enumerate() {
            if let Some(entry) = league.standings.iter_mut().find(|e| e.team_id == team_ids[*w]) {
                entry.won += 1;
                entry.points += 3;
                entry.maps_won += 2;
                entry.maps_lost += 1;
                entry.played += 1;
            }
            if let Some(entry) = league.standings.iter_mut().find(|e| e.team_id == team_ids[*l]) {
                entry.lost += 1;
                entry.maps_won += 1;
                entry.maps_lost += 2;
                entry.played += 1;
            }
        }
        league
    }

    fn empty_game_with_teams(teams: Vec<Team>) -> Game {
        let clock = GameClock::new(chrono::Utc::now());
        let manager = Manager::new(
            "mgr".to_string(),
            "Test".to_string(),
            "Manager".to_string(),
            "1980-01-01".to_string(),
            "ES".to_string(),
        );
        Game::new(clock, manager, teams, vec![], vec![], vec![])
    }

    #[test]
    fn fst_qualification_gathers_top_2_from_lck_lpl_and_top_1_from_others() {
        let teams = vec![
            make_team("lck-t1", "lck"),
            make_team("lck-gen", "lck"),
            make_team("lck-hle", "lck"),
            make_team("lpl-blg", "lpl"),
            make_team("lpl-tes", "lpl"),
            make_team("lpl-jdg", "lpl"),
            make_team("lec-g2", "lec"),
            make_team("lec-fnc", "lec"),
            make_team("lcs-100", "lcs"),
            make_team("cblol-png", "cblol"),
            make_team("lcp-100", "lcp"),
        ];
        let mut game = empty_game_with_teams(teams);
        let tids: Vec<String> = (0..11).map(|i| format!("team_{i}")).collect();
        let lck_results = vec![(0,1),(0,2),(1,2)]; // t1 > gen > hle
        let lpl_results = vec![(3,4),(3,5),(4,5)]; // blg > tes > jdg
        let lec_results = vec![(6,7)]; // g2 > fnc
        let lcs_results = vec![(8, 8)]; // dummy
        let cblol_results = vec![(9, 9)]; // dummy
        let lcp_results = vec![(10, 10)]; // dummy
        // Actually we need real team_ids matching
        let lck_tids = vec!["lck-t1".to_string(), "lck-gen".to_string(), "lck-hle".to_string()];
        let lpl_tids = vec!["lpl-blg".to_string(), "lpl-tes".to_string(), "lpl-jdg".to_string()];
        let lec_tids = vec!["lec-g2".to_string(), "lec-fnc".to_string()];
        let lcs_tids = vec!["lcs-100".to_string()];
        let cblol_tids = vec!["cblol-png".to_string()];
        let lcp_tids = vec!["lcp-100".to_string()];
        game.leagues.push(make_league_with_results("lck", 0, &lck_tids, &[(0,1),(0,2),(1,2)]));
        game.leagues.push(make_league_with_results("lpl", 0, &lpl_tids, &[(0,1),(0,2),(1,2)]));
        game.leagues.push(make_league_with_results("lec", 0, &lec_tids, &[(0,1)]));
        game.leagues.push(make_league_with_results("lcs", 0, &lcs_tids, &[]));
        game.leagues.push(make_league_with_results("cblol", 0, &cblol_tids, &[]));
        game.leagues.push(make_league_with_results("lcp", 0, &lcp_tids, &[]));

        let fst = qualify_teams_for_tournament(&game, TournamentFormat::Fst2026);
        assert_eq!(fst.len(), 8, "FST should qualify 8 teams");
        assert!(fst.contains(&"lck-t1".to_string()));
        assert!(fst.contains(&"lck-gen".to_string()));
        assert!(fst.contains(&"lpl-blg".to_string()));
        assert!(fst.contains(&"lpl-tes".to_string()));
        assert!(fst.contains(&"lec-g2".to_string()));
        assert!(fst.contains(&"lcs-100".to_string()));
        assert!(fst.contains(&"cblol-png".to_string()));
        assert!(fst.contains(&"lcp-100".to_string()));
    }

    #[test]
    fn get_regional_standings_returns_ordered_team_ids() {
        let teams = vec![
            make_team("t1", "lck"),
            make_team("gen", "lck"),
        ];
        let mut game = empty_game_with_teams(teams);
        let tids = vec!["t1".to_string(), "gen".to_string()];
        game.leagues.push(make_league_with_results("lck", 0, &tids, &[(0,1)]));
        let standings = get_regional_standings(&game, "lck", 0).unwrap();
        assert_eq!(standings[0], "t1");
        assert_eq!(standings[1], "gen");
    }
}
