use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::{
    full_structure_layout, role_start_position, LolSimV3AgentState, LolSimV3Scoreboard,
    LolSimV3Vec2,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LolSimV3TowerThreatState {
    pub tower_id: String,
    pub target_id: Option<String>,
    pub target_kind: Option<String>,
    pub lock_until_sec: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LolSimV3AggressionRecord {
    pub attacker_id: String,
    pub victim_team: LolSimV3Team,
    pub lane: String,
    pub at_sec: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LolSimV3TowerDamageTelemetryState {
    pub tower_id: String,
    pub last_emit_at_sec: f64,
    pub last_hp_bucket: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LolSimV3Team {
    Blue,
    Red,
}

impl LolSimV3Team {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blue => "blue",
            Self::Red => "red",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LolSimV3ChampionState {
    pub id: String,
    pub name: String,
    pub champion_id: String,
    pub team: LolSimV3Team,
    pub role: String,
    pub lane: String,
    pub alive: bool,
    pub hp: f64,
    pub max_hp: f64,
    pub respawn_at_sec: f64,
    pub pos: LolSimV3Vec2,
    pub state: LolSimV3AgentState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LolSimV3StructureState {
    pub id: String,
    pub team: LolSimV3Team,
    pub lane: String,
    pub kind: String,
    pub alive: bool,
    pub hp: f64,
    pub max_hp: f64,
    pub pos: LolSimV3Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LolSimV3ObjectiveState {
    pub key: String,
    pub alive: bool,
    pub next_spawn_at_sec: Option<f64>,
    pub pos: LolSimV3Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LolSimV3NeutralCampState {
    pub key: String,
    pub team: LolSimV3Team,
    pub alive: bool,
    pub next_spawn_at_sec: Option<f64>,
    pub pos: LolSimV3Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LolSimV3MinionState {
    pub id: String,
    pub team: LolSimV3Team,
    pub lane: String,
    pub kind: String,
    pub alive: bool,
    pub hp: f64,
    pub max_hp: f64,
    pub pos: LolSimV3Vec2,
    pub lane_progress: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LolSimV3ManagerTactics {
    pub aggression: f64,
    pub objective_priority: f64,
    pub safety_bias: f64,
}

impl Default for LolSimV3ManagerTactics {
    fn default() -> Self {
        Self {
            aggression: 0.5,
            objective_priority: 0.5,
            safety_bias: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LolSimV3TeamTacticsState {
    pub blue: LolSimV3ManagerTactics,
    pub red: LolSimV3ManagerTactics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LolSimV3WorldState {
    pub tick: u64,
    pub time_sec: f64,
    pub tick_dt_sec: f64,
    pub running: bool,
    pub winner: Option<LolSimV3Team>,
    pub seed: String,
    pub rng_state: u64,
    pub champions: Vec<LolSimV3ChampionState>,
    pub minions: Vec<LolSimV3MinionState>,
    pub structures: Vec<LolSimV3StructureState>,
    pub objectives: Vec<LolSimV3ObjectiveState>,
    pub neutral_camps: Vec<LolSimV3NeutralCampState>,
    pub scoreboard: LolSimV3Scoreboard,
    pub team_tactics: LolSimV3TeamTacticsState,
    pub next_wave_spawn_at_sec: f64,
    pub next_minion_serial: u64,
    #[serde(default)]
    pub tower_threat: Vec<LolSimV3TowerThreatState>,
    #[serde(default)]
    pub recent_aggressions: Vec<LolSimV3AggressionRecord>,
    #[serde(default)]
    pub tower_damage_telemetry: Vec<LolSimV3TowerDamageTelemetryState>,
}

pub fn create_minimal_world_state(seed: &str, tick_dt_sec: f64) -> LolSimV3WorldState {
    let roles = ["TOP", "JGL", "MID", "ADC", "SUP"];
    let mut champions = Vec::with_capacity(10);
    for role in roles {
        champions.push(LolSimV3ChampionState {
            id: format!("blue-{}", role.to_lowercase()),
            name: format!("Blue {}", role),
            champion_id: fallback_champion_id(LolSimV3Team::Blue, role).to_string(),
            team: LolSimV3Team::Blue,
            role: role.to_string(),
            lane: lane_for_role(role).to_string(),
            alive: true,
            hp: 100.0,
            max_hp: 100.0,
            respawn_at_sec: 0.0,
            pos: role_start_position(LolSimV3Team::Blue, role),
            state: LolSimV3AgentState::Laning,
        });

        champions.push(LolSimV3ChampionState {
            id: format!("red-{}", role.to_lowercase()),
            name: format!("Red {}", role),
            champion_id: fallback_champion_id(LolSimV3Team::Red, role).to_string(),
            team: LolSimV3Team::Red,
            role: role.to_string(),
            lane: lane_for_role(role).to_string(),
            alive: true,
            hp: 100.0,
            max_hp: 100.0,
            respawn_at_sec: 0.0,
            pos: role_start_position(LolSimV3Team::Red, role),
            state: LolSimV3AgentState::Laning,
        });
    }

    LolSimV3WorldState {
        tick: 0,
        time_sec: 0.0,
        tick_dt_sec: tick_dt_sec.max(0.01),
        running: true,
        winner: None,
        seed: seed.to_string(),
        rng_state: seed_to_u64(seed),
        champions,
        minions: Vec::new(),
        structures: full_structure_layout(),
        objectives: vec![
            LolSimV3ObjectiveState {
                key: "dragon".to_string(),
                alive: false,
                next_spawn_at_sec: Some(5.0 * 60.0),
                pos: LolSimV3Vec2 { x: 0.67, y: 0.70 },
            },
            LolSimV3ObjectiveState {
                key: "baron".to_string(),
                alive: false,
                next_spawn_at_sec: Some(20.0 * 60.0),
                pos: LolSimV3Vec2 { x: 0.33, y: 0.30 },
            },
        ],
        neutral_camps: vec![
            LolSimV3NeutralCampState {
                key: "blue-gromp".to_string(),
                team: LolSimV3Team::Blue,
                alive: false,
                next_spawn_at_sec: Some(90.0),
                pos: LolSimV3Vec2 { x: 0.20, y: 0.63 },
            },
            LolSimV3NeutralCampState {
                key: "blue-red".to_string(),
                team: LolSimV3Team::Blue,
                alive: false,
                next_spawn_at_sec: Some(90.0),
                pos: LolSimV3Vec2 { x: 0.30, y: 0.76 },
            },
            LolSimV3NeutralCampState {
                key: "red-gromp".to_string(),
                team: LolSimV3Team::Red,
                alive: false,
                next_spawn_at_sec: Some(90.0),
                pos: LolSimV3Vec2 { x: 0.80, y: 0.37 },
            },
            LolSimV3NeutralCampState {
                key: "red-blue".to_string(),
                team: LolSimV3Team::Red,
                alive: false,
                next_spawn_at_sec: Some(90.0),
                pos: LolSimV3Vec2 { x: 0.70, y: 0.24 },
            },
        ],
        scoreboard: LolSimV3Scoreboard::default(),
        team_tactics: LolSimV3TeamTacticsState::default(),
        next_wave_spawn_at_sec: 65.0,
        next_minion_serial: 0,
        tower_threat: Vec::new(),
        recent_aggressions: Vec::new(),
        tower_damage_telemetry: Vec::new(),
    }
}

fn lane_for_role(role: &str) -> &'static str {
    match role {
        "TOP" => "top",
        "MID" => "mid",
        _ => "bot",
    }
}

fn fallback_champion_id(team: LolSimV3Team, role: &str) -> &'static str {
    match (team, role) {
        (LolSimV3Team::Blue, "TOP") => "Aatrox",
        (LolSimV3Team::Blue, "JGL") => "Sejuani",
        (LolSimV3Team::Blue, "MID") => "Ahri",
        (LolSimV3Team::Blue, "ADC") => "Jinx",
        (LolSimV3Team::Blue, "SUP") => "Thresh",
        (LolSimV3Team::Red, "TOP") => "Garen",
        (LolSimV3Team::Red, "JGL") => "LeeSin",
        (LolSimV3Team::Red, "MID") => "Lux",
        (LolSimV3Team::Red, "ADC") => "Caitlyn",
        (LolSimV3Team::Red, "SUP") => "Leona",
        _ => "Ryze",
    }
}

pub(crate) fn ratio(current: f64, max: f64) -> f64 {
    if max <= 0.0 {
        return 0.0;
    }
    (current / max).clamp(0.0, 1.0)
}

fn seed_to_u64(seed: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    hasher.finish()
}
