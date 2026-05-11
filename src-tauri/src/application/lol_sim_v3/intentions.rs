use super::{LolSimV3AgentDecision, LolSimV3AgentState, LolSimV3Team, LolSimV3WorldState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LolSimV3IntentKind {
    FarmLane,
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

#[derive(Debug, Clone)]
pub struct LolSimV3Intention {
    pub champion_id: String,
    pub team: LolSimV3Team,
    pub state: LolSimV3AgentState,
    pub kind: LolSimV3IntentKind,
    pub priority: u8,
    pub reason: &'static str,
}

/// Step 4: explicit intention system
///
/// Converts agent decisions into intentions without mutating world state.
pub fn intentions_from_decisions(
    world: &LolSimV3WorldState,
    decisions: &[LolSimV3AgentDecision],
) -> Vec<LolSimV3Intention> {
    decisions
        .iter()
        .filter_map(|decision| {
            let champion = world
                .champions
                .iter()
                .find(|champion| champion.id == decision.champion_id)?;

            let hp_ratio = (champion.hp / champion.max_hp.max(1.0)).clamp(0.0, 1.0);
            let kind = intention_kind_for(
                decision.next_state.clone(),
                champion.role.as_str(),
                world.time_sec,
            );
            let tactics = match champion.team {
                LolSimV3Team::Blue => &world.team_tactics.blue,
                LolSimV3Team::Red => &world.team_tactics.red,
            };
            let priority = priority_for(kind, hp_ratio, tactics);

            Some(LolSimV3Intention {
                champion_id: champion.id.clone(),
                team: champion.team,
                state: decision.next_state.clone(),
                kind,
                priority,
                reason: decision.reason,
            })
        })
        .collect()
}

fn intention_kind_for(state: LolSimV3AgentState, role: &str, time_sec: f64) -> LolSimV3IntentKind {
    match state {
        LolSimV3AgentState::Dead => LolSimV3IntentKind::WaitRespawn,
        LolSimV3AgentState::Recalling => LolSimV3IntentKind::Recall,
        LolSimV3AgentState::Laning => LolSimV3IntentKind::FarmLane,
        LolSimV3AgentState::Pushing => LolSimV3IntentKind::PushTower,
        LolSimV3AgentState::Roaming => {
            if role == "JGL" && time_sec >= 20.0 * 60.0 {
                LolSimV3IntentKind::TakeBaron
            } else if role == "JGL" && time_sec >= 5.0 * 60.0 {
                LolSimV3IntentKind::TakeDragon
            } else {
                LolSimV3IntentKind::RoamLane
            }
        }
        LolSimV3AgentState::ObjectiveSetup => {
            if role == "JGL" {
                LolSimV3IntentKind::TakeDragon
            } else {
                LolSimV3IntentKind::RotateToObjective
            }
        }
        LolSimV3AgentState::Fighting => {
            if role == "SUP" {
                LolSimV3IntentKind::DefendBase
            } else {
                LolSimV3IntentKind::TradeWithEnemy
            }
        }
    }
}

fn priority_for(
    kind: LolSimV3IntentKind,
    hp_ratio: f64,
    tactics: &super::LolSimV3ManagerTactics,
) -> u8 {
    let base = match kind {
        LolSimV3IntentKind::WaitRespawn => 100,
        LolSimV3IntentKind::Recall => 95,
        LolSimV3IntentKind::TakeBaron => 90,
        LolSimV3IntentKind::TakeDragon => 85,
        LolSimV3IntentKind::DefendBase => 82,
        LolSimV3IntentKind::RoamLane => 78,
        LolSimV3IntentKind::RotateToObjective => 75,
        LolSimV3IntentKind::TradeWithEnemy => {
            if hp_ratio >= 0.7 {
                72
            } else {
                60
            }
        }
        LolSimV3IntentKind::PushTower => 58,
        LolSimV3IntentKind::FarmLane => 45,
    } as f64;

    let tactical_bonus = match kind {
        LolSimV3IntentKind::TradeWithEnemy => (tactics.aggression - 0.5) * 18.0,
        LolSimV3IntentKind::TakeDragon
        | LolSimV3IntentKind::TakeBaron
        | LolSimV3IntentKind::RotateToObjective => (tactics.objective_priority - 0.5) * 22.0,
        LolSimV3IntentKind::Recall | LolSimV3IntentKind::DefendBase => {
            (tactics.safety_bias - 0.5) * 20.0
        }
        LolSimV3IntentKind::PushTower => {
            ((tactics.aggression + tactics.objective_priority) / 2.0 - 0.5) * 14.0
        }
        LolSimV3IntentKind::FarmLane
        | LolSimV3IntentKind::RoamLane
        | LolSimV3IntentKind::WaitRespawn => 0.0,
    };

    (base + tactical_bonus).clamp(1.0, 100.0).round() as u8
}
