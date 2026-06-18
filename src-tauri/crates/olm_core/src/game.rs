use crate::champions::{ChampionMasteryEntry, ChampionPatchState};
use crate::clock::GameClock;
use crate::domain::league::League;
use crate::domain::manager::Manager;
use crate::domain::message::InboxMessage;
use crate::domain::news::NewsArticle;
use crate::domain::player::Player;
use crate::domain::season::SeasonContext;
use crate::domain::social::{SocialAccount, SocialPost, SocialTemplate};
use crate::domain::staff::Staff;
use crate::domain::stats::StatsState;
use crate::domain::team::Team;
use crate::domain::tournament_state::ScheduledTournament;
use crate::domain::transfer_history::TransferHistory;
#[cfg(feature = "typescript")]
use ts_rs::TS;

use crate::generator::definitions::CompetitionManifest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub enum DayPhase {
    #[default]
    Morning,
    ScrimBlock,
    ReviewBlock,
    TrainingBlock,
    Evening,
}

impl DayPhase {
    pub fn as_id(&self) -> &'static str {
        match self {
            Self::Morning => "Morning",
            Self::ScrimBlock => "ScrimBlock",
            Self::ReviewBlock => "ReviewBlock",
            Self::TrainingBlock => "TrainingBlock",
            Self::Evening => "Evening",
        }
    }

    pub fn from_id(value: &str) -> Self {
        match value {
            "ScrimBlock" => Self::ScrimBlock,
            "ReviewBlock" => Self::ReviewBlock,
            "TrainingBlock" => Self::TrainingBlock,
            "Evening" => Self::Evening,
            _ => Self::Morning,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Morning => Self::ScrimBlock,
            Self::ScrimBlock => Self::ReviewBlock,
            Self::ReviewBlock => Self::TrainingBlock,
            Self::TrainingBlock => Self::Evening,
            Self::Evening => Self::Evening,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub enum ObjectiveType {
    LeaguePosition,
    Wins,
    GoalsScored,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct BoardObjective {
    pub id: String,
    pub description: String,
    pub target: u32,
    pub objective_type: ObjectiveType,
    pub met: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct ScoutingAssignment {
    pub id: String,
    pub scout_id: String,
    pub player_id: String,
    pub days_remaining: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct Game {
    pub clock: GameClock,
    #[serde(default)]
    pub day_phase: DayPhase,
    pub manager: Manager,
    pub teams: Vec<Team>,
    pub players: Vec<Player>,
    pub staff: Vec<Staff>,
    pub messages: Vec<InboxMessage>,
    #[serde(default)]
    pub news: Vec<NewsArticle>,
    #[serde(default)]
    pub social_posts: Vec<SocialPost>,
    #[serde(default)]
    pub social_accounts: Vec<SocialAccount>,
    #[serde(default)]
    pub social_templates: Vec<SocialTemplate>,
    /// Multi-league storage. The first element is the player's active league.
    #[serde(default)]
    pub leagues: Vec<League>,
    /// The competition_id of the player's active league.
    #[serde(default)]
    pub user_competition_id: Option<String>,
    #[serde(default)]
    pub scouting_assignments: Vec<ScoutingAssignment>,
    #[serde(default)]
    pub board_objectives: Vec<BoardObjective>,
    #[serde(default)]
    pub season_context: SeasonContext,
    #[serde(default)]
    pub days_since_last_job_offer: Option<u32>,
    #[serde(default)]
    pub champion_masteries: Vec<ChampionMasteryEntry>,
    #[serde(default)]
    pub champion_patch: ChampionPatchState,
    #[serde(default)]
    pub stats_state: StatsState,
    #[serde(default, deserialize_with = "deserialize_competition_configs_lenient")]
    pub competition_configs: HashMap<String, CompetitionManifest>,
    #[serde(default)]
    pub transfer_history: TransferHistory,
    /// When the user team qualifies for an international tournament, this is set
    /// to the tournament's competition_id so the active context temporarily switches.
    #[serde(default)]
    pub active_tournament_id: Option<String>,
    /// True while the tournament is running so the turn loop knows to simulate
    /// tournament fixtures before the user's regional league.
    #[serde(default)]
    pub tournament_queuing: bool,
    /// Tournaments scheduled to start after a split ends.
    /// Materialized into a League when current_date >= start_date.
    #[serde(default)]
    pub scheduled_tournaments: Vec<ScheduledTournament>,
}

/// Lenient deserializer for `competition_configs`.
///
/// Saves created before the `ScheduleConfig` -> `CompetitionManifest` migration
/// stored values without the fields a manifest requires (e.g. `id`), which would
/// otherwise fail the entire save load with "missing field `id`". Parse each
/// entry independently and drop any that no longer match the manifest shape; the
/// map is repopulated from the on-disk manifests when the game is loaded.
fn deserialize_competition_configs_lenient<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, CompetitionManifest>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw = HashMap::<String, serde_json::Value>::deserialize(deserializer)?;
    let mut configs = HashMap::with_capacity(raw.len());
    for (key, value) in raw {
        if let Ok(manifest) = serde_json::from_value::<CompetitionManifest>(value) {
            configs.insert(key, manifest);
        }
    }
    Ok(configs)
}

impl Game {
    pub fn new(
        clock: GameClock,
        manager: Manager,
        teams: Vec<Team>,
        players: Vec<Player>,
        staff: Vec<Staff>,
        messages: Vec<InboxMessage>,
    ) -> Self {
        let mut game = Self {
            clock,
            day_phase: DayPhase::Morning,
            manager,
            teams,
            players,
            staff,
            messages,
            news: vec![],
            social_posts: vec![],
            social_accounts: vec![],
            social_templates: vec![],
            leagues: vec![],
            user_competition_id: None,
            scouting_assignments: vec![],
            board_objectives: vec![],
            season_context: SeasonContext::default(),
            days_since_last_job_offer: None,
            champion_masteries: vec![],
            champion_patch: ChampionPatchState::default(),
            stats_state: StatsState::default(),
            competition_configs: HashMap::new(),
            transfer_history: TransferHistory::default(),
            active_tournament_id: None,
            tournament_queuing: false,
            scheduled_tournaments: vec![],
        };
        crate::identity_upgrade::upgrade_game_football_identities(&mut game);
        crate::season_context::refresh_game_context(&mut game);
        game.refresh_lol_ovrs();
        game
    }

    /// Returns a reference to the player's active league, identified by
    /// `user_competition_id`. Falls back to `leagues.first()` if not set.
    pub fn active_league(&self) -> Option<&League> {
        self.user_competition_id
            .as_ref()
            .and_then(|cid| self.leagues.iter().find(|l| l.competition_id.as_deref() == Some(cid)))
            .or_else(|| self.leagues.first())
    }

    /// Returns a mutable reference to the player's active league.
    pub fn active_league_mut(&mut self) -> Option<&mut League> {
        let cid = self.user_competition_id.clone();
        if let Some(ref cid) = cid {
            if let Some(pos) = self.leagues.iter().position(|l| l.competition_id.as_deref() == Some(cid)) {
                return self.leagues.get_mut(pos);
            }
        }
        self.leagues.first_mut()
    }

    /// Recompute `lol_ovr` for every player from their current attributes.
    pub fn refresh_lol_ovrs(&mut self) {
        for player in &mut self.players {
            player.lol_ovr = crate::potential::calculate_lol_ovr(player);
        }
    }

    /// Returns the index of the player's active league.
    pub fn active_league_index(&self) -> usize {
        self.user_competition_id
            .as_ref()
            .and_then(|cid| self.leagues.iter().position(|l| l.competition_id.as_deref() == Some(cid)))
            .unwrap_or(0)
    }

    /// Returns a reference to the currently active tournament league, if any.
    pub fn active_tournament_league(&self) -> Option<&League> {
        self.active_tournament_id
            .as_ref()
            .and_then(|cid| self.leagues.iter().find(|l| l.competition_id.as_deref() == Some(cid)))
    }

    /// Returns a mutable reference to the currently active tournament league, if any.
    pub fn active_tournament_league_mut(&mut self) -> Option<&mut League> {
        let cid = self.active_tournament_id.clone();
        if let Some(ref cid) = cid {
            if let Some(pos) = self.leagues.iter().position(|l| l.competition_id.as_deref() == Some(cid)) {
                return self.leagues.get_mut(pos);
            }
        }
        None
    }

    /// Returns the league that should be simulated today: the active tournament
    /// when `tournament_queuing` is true, otherwise the user's regional league.
    pub fn active_simulation_league(&self) -> Option<&League> {
        if self.tournament_queuing {
            self.active_tournament_league()
                .or_else(|| self.active_league())
        } else {
            self.active_league()
        }
    }

    /// Mutable version of `active_simulation_league`.
    pub fn active_simulation_league_mut(&mut self) -> Option<&mut League> {
        if self.tournament_queuing {
            let cid = self.active_tournament_id.clone();
            if let Some(ref cid) = cid {
                if let Some(pos) = self.leagues.iter().position(|l| l.competition_id.as_deref() == Some(cid)) {
                    return self.leagues.get_mut(pos);
                }
            }
            self.active_league_mut()
        } else {
            self.active_league_mut()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::GameClock;
    use crate::domain::manager::Manager;

    fn empty_game() -> Game {
        let clock = GameClock::new(chrono::Utc::now());
        let manager = Manager::new(
            "mgr".to_string(),
            "Test".to_string(),
            "Manager".to_string(),
            "1980-01-01".to_string(),
            "ES".to_string(),
        );
        Game::new(clock, manager, vec![], vec![], vec![], vec![])
    }

    #[test]
    fn test_active_tournament_id_defaults_to_none() {
        let game = empty_game();
        assert!(game.active_tournament_id.is_none());
        assert!(!game.tournament_queuing);
    }

    #[test]
    fn test_active_tournament_league_returns_none_when_not_set() {
        let game = empty_game();
        assert!(game.active_tournament_league().is_none());
    }

    #[test]
    fn test_active_tournament_league_returns_league_when_set() {
        let mut game = empty_game();
        let mut league = crate::domain::league::League::new(
            "fst".to_string(),
            "First Stand".to_string(),
            2026,
            &["t1".to_string(), "t2".to_string()],
            Some("fst".to_string()),
        );
        league.active = true;
        game.leagues.push(league);
        game.active_tournament_id = Some("fst".to_string());

        let active = game.active_tournament_league();
        assert!(active.is_some());
        assert_eq!(active.unwrap().id, "fst");
    }

    #[test]
    fn test_active_tournament_league_mut_returns_mutable_league() {
        let mut game = empty_game();
        let league = crate::domain::league::League::new(
            "fst".to_string(),
            "First Stand".to_string(),
            2026,
            &["t1".to_string(), "t2".to_string()],
            Some("fst".to_string()),
        );
        game.leagues.push(league);
        game.active_tournament_id = Some("fst".to_string());

        if let Some(l) = game.active_tournament_league_mut() {
            l.name = "FST 2026".to_string();
        }
        assert_eq!(game.leagues[0].name, "FST 2026");
    }

    #[test]
    fn test_game_serde_roundtrip_with_new_fields() {
        let mut game = empty_game();
        game.active_tournament_id = Some("msi".to_string());
        game.tournament_queuing = true;

        let json = serde_json::to_string(&game).unwrap();
        let deserialized: Game = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.active_tournament_id, Some("msi".to_string()));
        assert!(deserialized.tournament_queuing);
    }

    #[test]
    fn test_game_deserialize_missing_fields_defaults() {
        let json = r#"{
            "clock": {"current_date": "2026-01-01T00:00:00Z"},
            "manager": {"id": "mgr", "first_name": "T", "last_name": "M", "date_of_birth": "1980-01-01", "nationality": "ES", "career_stats": {"matches_managed": 0, "wins": 0, "losses": 0, "trophies": 0}, "satisfaction": 50, "career_history": []},
            "teams": [], "players": [], "staff": [], "messages": [],
            "leagues": [], "user_competition_id": null
        }"#;
        let game: Game = serde_json::from_str(json).unwrap();
        assert!(game.active_tournament_id.is_none());
        assert!(!game.tournament_queuing);
    }

    #[test]
    fn test_active_simulation_league_returns_tournament_when_queuing() {
        let mut game = empty_game();
        let regional = crate::domain::league::League::new(
            "lec".to_string(),
            "LEC".to_string(),
            2026,
            &["t1".to_string()],
            Some("lec".to_string()),
        );
        let tournament = crate::domain::league::League::new(
            "fst".to_string(),
            "First Stand".to_string(),
            2026,
            &["t2".to_string()],
            Some("fst".to_string()),
        );
        game.leagues.push(regional);
        game.leagues.push(tournament);
        game.user_competition_id = Some("lec".to_string());
        game.active_tournament_id = Some("fst".to_string());
        game.tournament_queuing = true;

        let sim = game.active_simulation_league();
        assert!(sim.is_some());
        assert_eq!(sim.unwrap().id, "fst");
    }

    #[test]
    fn test_active_simulation_league_returns_regional_when_not_queuing() {
        let mut game = empty_game();
        let regional = crate::domain::league::League::new(
            "lec".to_string(),
            "LEC".to_string(),
            2026,
            &["t1".to_string()],
            Some("lec".to_string()),
        );
        game.leagues.push(regional);
        game.user_competition_id = Some("lec".to_string());
        game.active_tournament_id = Some("fst".to_string());
        game.tournament_queuing = false;

        let sim = game.active_simulation_league();
        assert!(sim.is_some());
        assert_eq!(sim.unwrap().id, "lec");
    }
}

