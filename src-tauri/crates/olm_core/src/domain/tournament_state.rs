use crate::tournament_qualification::TournamentFormat;
use serde::{Deserialize, Serialize};
#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub enum TournamentPhase {
    Group,
    PlayIn,
    Swiss,
    Knockout,
    Complete,
}

impl Default for TournamentPhase {
    fn default() -> Self {
        TournamentPhase::Group
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[serde(default)]
pub struct SwissRecord {
    pub team_id: String,
    pub wins: u32,
    pub losses: u32,
    pub buchholz: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[serde(default)]
pub struct GslGroupState {
    pub teams: Vec<String>,
    pub opening_winners: Vec<String>,
    pub opening_losers: Vec<String>,
    pub winners_match_winner: Option<String>,
    pub losers_match_winner: Option<String>,
    pub decider_winner: Option<String>,
    pub advanced_teams: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[serde(default)]
pub struct TournamentState {
    pub format: TournamentFormat,
    pub current_phase: TournamentPhase,
    pub current_round: u32,
    pub swiss_records: Vec<SwissRecord>,
    pub gsl_groups: Vec<GslGroupState>,
    pub advancing_teams: Vec<String>,
    pub start_date: String,
    pub is_complete: bool,
}

impl Default for TournamentState {
    fn default() -> Self {
        Self {
            format: TournamentFormat::Fst2026,
            current_phase: TournamentPhase::Group,
            current_round: 1,
            swiss_records: Vec::new(),
            gsl_groups: Vec::new(),
            advancing_teams: Vec::new(),
            start_date: String::new(),
            is_complete: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct ScheduledTournament {
    pub competition_id: String,
    pub start_date: String,
    pub format: TournamentFormat,
    pub qualified_teams: Vec<String>,
}

/// Read winners of a completed round from a tournament league.
/// Returns team IDs in fixture order.
pub fn read_round_winners(league: &crate::domain::league::League, round_matchday: u32) -> Vec<String> {
    use crate::domain::league::{FixtureStatus, MatchType};
    let mut winners = Vec::new();
    for fixture in &league.fixtures {
        if fixture.matchday == round_matchday
            && fixture.status == FixtureStatus::Completed
            && matches!(
                fixture.match_type,
                MatchType::TournamentGroup
                    | MatchType::TournamentPlayIn
                    | MatchType::TournamentSwiss
                    | MatchType::TournamentKnockout
            )
        {
            if let Some(ref result) = fixture.result {
                if result.home_wins > result.away_wins {
                    winners.push(fixture.home_team_id.clone());
                } else if result.away_wins > result.home_wins {
                    winners.push(fixture.away_team_id.clone());
                }
            }
        }
    }
    winners
}

/// Read losers of a completed round from a tournament league.
/// Returns team IDs in fixture order.
pub fn read_round_losers(league: &crate::domain::league::League, round_matchday: u32) -> Vec<String> {
    use crate::domain::league::{FixtureStatus, MatchType};
    let mut losers = Vec::new();
    for fixture in &league.fixtures {
        if fixture.matchday == round_matchday
            && fixture.status == FixtureStatus::Completed
            && matches!(
                fixture.match_type,
                MatchType::TournamentGroup
                    | MatchType::TournamentPlayIn
                    | MatchType::TournamentSwiss
                    | MatchType::TournamentKnockout
            )
        {
            if let Some(ref result) = fixture.result {
                if result.home_wins > result.away_wins {
                    losers.push(fixture.away_team_id.clone());
                } else if result.away_wins > result.home_wins {
                    losers.push(fixture.home_team_id.clone());
                }
            }
        }
    }
    losers
}

/// Determine if every fixture for the given round matchday is completed.
pub fn is_round_complete(league: &crate::domain::league::League, round_matchday: u32) -> bool {
    use crate::domain::league::{FixtureStatus, MatchType};
    let round_fixtures: Vec<_> = league
        .fixtures
        .iter()
        .filter(|f| {
            f.matchday == round_matchday
                && matches!(
                    f.match_type,
                    MatchType::TournamentGroup
                        | MatchType::TournamentPlayIn
                        | MatchType::TournamentSwiss
                        | MatchType::TournamentKnockout
                )
        })
        .collect();
    !round_fixtures.is_empty()
        && round_fixtures
            .iter()
            .all(|f| f.status == FixtureStatus::Completed)
}
