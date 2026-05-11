use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LolSimV3AgentState {
    Laning,
    Pushing,
    Roaming,
    ObjectiveSetup,
    Fighting,
    Recalling,
    Dead,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LolSimV3EventKind {
    UnitMoved,
    AgentStateChanged,
    TradeStarted,
    DamageApplied,
    ChampionKilled,
    TowerDestroyed,
    DragonTaken,
    BaronTaken,
    WaveSpawned,
    NeutralCampSpawned,
    NeutralCampTaken,
    TowerDamaged,
    NexusDestroyed,
    GoldChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3UnitView {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub champion_id: String,
    pub team: String,
    pub role: String,
    pub lane: String,
    pub alive: bool,
    pub pos: LolSimV3Vec2,
    pub hp_ratio: f64,
    pub state: LolSimV3AgentState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3MinionView {
    pub id: String,
    pub team: String,
    pub lane: String,
    pub kind: String,
    pub alive: bool,
    pub hp_ratio: f64,
    pub pos: LolSimV3Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3StructureView {
    pub id: String,
    pub team: String,
    pub lane: String,
    pub kind: String,
    pub alive: bool,
    pub hp_ratio: f64,
    pub pos: LolSimV3Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3ObjectiveView {
    pub key: String,
    pub alive: bool,
    pub next_spawn_at_sec: Option<f64>,
    pub pos: LolSimV3Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3NeutralCampView {
    pub key: String,
    pub team: String,
    pub alive: bool,
    pub next_spawn_at_sec: Option<f64>,
    pub pos: LolSimV3Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3ScoreboardTeam {
    pub kills: i64,
    pub towers: i64,
    pub dragons: i64,
    pub gold: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3Scoreboard {
    pub blue: LolSimV3ScoreboardTeam,
    pub red: LolSimV3ScoreboardTeam,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3Snapshot {
    pub tick: u64,
    pub time_sec: f64,
    pub running: bool,
    pub winner: Option<String>,
    pub units: Vec<LolSimV3UnitView>,
    pub minions: Vec<LolSimV3MinionView>,
    pub structures: Vec<LolSimV3StructureView>,
    pub objectives: Vec<LolSimV3ObjectiveView>,
    #[serde(default)]
    pub neutral_camps: Vec<LolSimV3NeutralCampView>,
    pub scoreboard: LolSimV3Scoreboard,
    #[serde(default)]
    pub lane_pressure: Vec<LolSimV3LanePressureView>,
    #[serde(default)]
    pub tower_targets: Vec<LolSimV3TowerTargetView>,
    #[serde(default)]
    pub neutral_timers: LolSimV3NeutralTimerSummaryView,
    #[serde(default)]
    pub phase_contributions: Vec<LolSimV3PhaseContributionView>,
    #[serde(default)]
    pub role_lane_contributions: Vec<LolSimV3RoleLaneContributionView>,
    #[serde(default)]
    pub objective_pressure_summary: LolSimV3ObjectivePressureSummaryView,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3PhaseContributionView {
    pub team: String,
    pub phase: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3RoleLaneContributionView {
    pub team: String,
    pub role: String,
    pub lane: String,
    pub pressure: f64,
    pub objective_pressure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3ObjectivePressureSummaryView {
    pub blue: f64,
    pub red: f64,
    pub contested: bool,
    pub delta: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3LanePressureView {
    pub lane: String,
    pub blue: f64,
    pub red: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3TowerTargetView {
    pub tower_id: String,
    pub target_id: Option<String>,
    pub target_kind: Option<String>,
    pub lock_until_sec: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3NeutralTimerSummaryView {
    pub next_dragon_at_sec: Option<f64>,
    pub next_baron_at_sec: Option<f64>,
    pub camps_alive: i64,
    pub camps_respawning: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3Event {
    pub id: String,
    pub t: f64,
    pub kind: LolSimV3EventKind,
    #[serde(default)]
    pub actor_id: Option<String>,
    #[serde(default)]
    pub target_id: Option<String>,
    #[serde(default)]
    pub team: Option<String>,
    #[serde(default)]
    pub lane: Option<String>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub from_state: Option<LolSimV3AgentState>,
    #[serde(default)]
    pub to_state: Option<LolSimV3AgentState>,
    #[serde(default)]
    pub from_pos: Option<LolSimV3Vec2>,
    #[serde(default)]
    pub to_pos: Option<LolSimV3Vec2>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3InitRequest {
    pub session_id: String,
    pub seed: String,
    pub snapshot: serde_json::Value,
    #[serde(default)]
    pub champion_by_player_id: HashMap<String, String>,
    #[serde(default = "default_tick_dt_sec")]
    pub tick_dt_sec: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3TickRequest {
    pub session_id: String,
    pub running: bool,
    #[serde(default = "default_steps")]
    pub steps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3TickResponse {
    pub session_id: String,
    pub snapshot: LolSimV3Snapshot,
    pub events: Vec<LolSimV3Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3ResetRequest {
    pub session_id: String,
    pub seed: String,
    #[serde(default = "default_tick_dt_sec")]
    pub tick_dt_sec: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3DisposeRequest {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3DisposeResponse {
    pub session_id: String,
    pub disposed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3RunToCompletionRequest {
    pub seed: String,
    pub snapshot: serde_json::Value,
    #[serde(default)]
    pub champion_by_player_id: HashMap<String, String>,
    #[serde(default = "default_tick_dt_sec")]
    pub tick_dt_sec: f64,
    #[serde(default = "default_max_steps")]
    pub max_steps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimV3RunToCompletionResponse {
    pub winner: Option<String>,
    pub steps: u32,
    pub elapsed_simulated_sec: f64,
    pub snapshot: LolSimV3Snapshot,
    pub events: Vec<LolSimV3Event>,
}

fn default_tick_dt_sec() -> f64 {
    0.1
}

fn default_steps() -> u32 {
    1
}

fn default_max_steps() -> u32 {
    1800
}
