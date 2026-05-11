use serde::Serialize;

use super::{
    champion_can_afford_next_item, normalized_lane, normalized_team, ChampionRuntime,
    NeutralTimersRuntime, RuntimeTeamBuffState, RuntimeTeamTactics,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum AgentState {
    Laning,
    Jungling,
    Pushing,
    Roaming,
    ObjectiveSetup,
    Fighting,
    Recalling,
    Dead,
}

impl AgentState {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Laning => "laning",
            Self::Jungling => "jungling",
            Self::Pushing => "pushing",
            Self::Roaming => "roaming",
            Self::ObjectiveSetup => "objective_setup",
            Self::Fighting => "fighting",
            Self::Recalling => "recalling",
            Self::Dead => "dead",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum IntentKind {
    FarmLane,
    ClearJungle,
    TradeWithEnemy,
    RotateToObjective,
    TakeDragon,
    TakeBaron,
    RoamLane,
    PushTower,
    Recall,
    DefendBase,
    WaitRespawn,
}

impl IntentKind {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::FarmLane => "farm_lane",
            Self::ClearJungle => "clear_jungle",
            Self::TradeWithEnemy => "trade_with_enemy",
            Self::RotateToObjective => "rotate_to_objective",
            Self::TakeDragon => "take_dragon",
            Self::TakeBaron => "take_baron",
            Self::RoamLane => "roam_lane",
            Self::PushTower => "push_tower",
            Self::Recall => "recall",
            Self::DefendBase => "defend_base",
            Self::WaitRespawn => "wait_respawn",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DecisionIntent {
    pub champion_id: String,
    pub team: String,
    pub role: String,
    pub lane: String,
    pub state: AgentState,
    pub intent: IntentKind,
    pub priority: u8,
    pub reason: &'static str,
}

impl DecisionIntent {
    pub(super) fn debug_label(&self) -> String {
        let macro_signal = macro_signal_for(self).unwrap_or("macro:none");
        format!(
            "agent:{}|intent:{}|priority:{}|reason:{}",
            self.state.as_str(),
            self.intent.as_str(),
            self.priority,
            self.reason
        ) + &format!("|{}|score:{}", macro_signal, self.priority)
    }
}

fn macro_signal_for(intent: &DecisionIntent) -> Option<&'static str> {
    match intent.intent {
        IntentKind::TakeDragon | IntentKind::TakeBaron | IntentKind::RotateToObjective => {
            Some("macro:objective_setup")
        }
        IntentKind::ClearJungle if intent.role == "JGL" => Some("macro:jungle_path"),
        IntentKind::RoamLane if intent.role == "SUP" => Some("macro:support_roam"),
        _ => None,
    }
}

pub(super) fn classify_decision_intent(
    champion: &ChampionRuntime,
    now: f64,
    neutral_timers: Option<&NeutralTimersRuntime>,
    team_tactics: &RuntimeTeamTactics,
    team_buffs: &RuntimeTeamBuffState,
) -> DecisionIntent {
    let state = agent_state_for(champion);
    let (intent, reason) = intent_for(champion, state, now, neutral_timers, team_buffs);
    let priority = priority_for(champion, intent, team_tactics);

    DecisionIntent {
        champion_id: champion.id.clone(),
        team: normalized_team(&champion.team).to_string(),
        role: champion.role.clone(),
        lane: normalized_lane(&champion.lane).to_string(),
        state,
        intent,
        priority,
        reason,
    }
}

fn agent_state_for(champion: &ChampionRuntime) -> AgentState {
    if !champion.alive {
        return AgentState::Dead;
    }

    match champion.state.as_str() {
        "recall" => AgentState::Recalling,
        "jungle" => AgentState::Jungling,
        "objective" => AgentState::ObjectiveSetup,
        "fight" => AgentState::Fighting,
        "push" => AgentState::Pushing,
        "roam" => AgentState::Roaming,
        _ => AgentState::Laning,
    }
}

fn intent_for(
    champion: &ChampionRuntime,
    state: AgentState,
    now: f64,
    neutral_timers: Option<&NeutralTimersRuntime>,
    team_buffs: &RuntimeTeamBuffState,
) -> (IntentKind, &'static str) {
    match state {
        AgentState::Dead => (IntentKind::WaitRespawn, "dead_wait_respawn"),
        AgentState::Recalling if champion_can_afford_next_item(champion) => {
            (IntentKind::Recall, "recall_purchase_economy")
        }
        AgentState::Recalling => (IntentKind::Recall, "recall_channel_or_path"),
        AgentState::Jungling => (IntentKind::ClearJungle, "jungle_clear_cycle"),
        AgentState::Fighting => (IntentKind::TradeWithEnemy, "combat_state"),
        AgentState::Roaming => (IntentKind::RoamLane, "roam_state"),
        AgentState::Pushing => (IntentKind::PushTower, "push_state"),
        AgentState::ObjectiveSetup => {
            objective_intent_for(champion, now, neutral_timers, team_buffs)
        }
        AgentState::Laning => {
            if team_buffs.baron_until > now {
                (IntentKind::PushTower, "baron_siege_lane")
            } else {
                (IntentKind::FarmLane, "lane_default")
            }
        }
    }
}

fn objective_intent_for(
    champion: &ChampionRuntime,
    now: f64,
    neutral_timers: Option<&NeutralTimersRuntime>,
    team_buffs: &RuntimeTeamBuffState,
) -> (IntentKind, &'static str) {
    if champion.role == "JGL" {
        if let Some(timers) = neutral_timers {
            if timers
                .entities
                .get("baron")
                .map(|timer| timer.alive && now >= timer.first_spawn_at)
                .unwrap_or(false)
            {
                return (IntentKind::TakeBaron, "jungler_live_baron");
            }
            if timers
                .entities
                .get("dragon")
                .map(|timer| timer.alive && now >= timer.first_spawn_at)
                .unwrap_or(false)
            {
                return (IntentKind::TakeDragon, "jungler_live_dragon");
            }
        }
    }

    if team_buffs.baron_until > now {
        (IntentKind::PushTower, "baron_group_siege")
    } else if champion.role == "SUP" {
        (IntentKind::RoamLane, "support_objective_roam")
    } else {
        (IntentKind::RotateToObjective, "objective_rotation")
    }
}

fn priority_for(
    champion: &ChampionRuntime,
    intent: IntentKind,
    tactics: &RuntimeTeamTactics,
) -> u8 {
    let hp_ratio = if champion.max_hp <= 0.0 {
        1.0
    } else {
        champion.hp / champion.max_hp
    }
    .clamp(0.0, 1.0);

    let base = match intent {
        IntentKind::WaitRespawn => 100.0,
        IntentKind::Recall => 94.0,
        IntentKind::TakeBaron => 90.0,
        IntentKind::TakeDragon => 86.0,
        IntentKind::DefendBase => 84.0,
        IntentKind::RotateToObjective => 78.0,
        IntentKind::RoamLane => 72.0,
        IntentKind::TradeWithEnemy => 62.0 + (hp_ratio * 16.0),
        IntentKind::PushTower => 58.0,
        IntentKind::ClearJungle => 50.0,
        IntentKind::FarmLane => 42.0,
    };

    let aggression_bias = match tactics.fight_plan.as_str() {
        "Dive" | "Skirmish" => 0.12,
        "Peel" | "FrontToBack" => -0.03,
        _ => 0.0,
    };
    let objective_bias = match tactics.jungle_style.as_str() {
        "Objective" | "Enabler" => 0.10,
        "Carry" => 0.04,
        _ => 0.0,
    };
    let safety_bias = match tactics.game_timing.as_str() {
        "Late" => 0.08,
        "Early" => -0.03,
        _ => 0.0,
    };

    let tactic_bonus = match intent {
        IntentKind::TradeWithEnemy => aggression_bias * 18.0,
        IntentKind::TakeDragon
        | IntentKind::TakeBaron
        | IntentKind::RotateToObjective
        | IntentKind::ClearJungle => objective_bias * 18.0,
        IntentKind::Recall | IntentKind::DefendBase => safety_bias * 18.0,
        IntentKind::PushTower => ((aggression_bias + objective_bias) / 2.0) * 14.0,
        IntentKind::FarmLane | IntentKind::RoamLane | IntentKind::WaitRespawn => 0.0,
    };

    (base + tactic_bonus).clamp(1.0, 100.0).round() as u8
}
