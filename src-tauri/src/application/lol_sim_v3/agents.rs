use super::{LolSimV3AgentState, LolSimV3ChampionState, LolSimV3Team, LolSimV3WorldState};

#[derive(Debug, Clone)]
pub struct LolSimV3AgentDecision {
    pub champion_id: String,
    pub current_state: LolSimV3AgentState,
    pub next_state: LolSimV3AgentState,
    pub reason: &'static str,
}

/// Step 3: explicit agent states
///
/// This pass only decides state transitions + semantic intent.
/// It does NOT apply movement/combat/objective resolution yet.
pub fn evaluate_agents(world: &LolSimV3WorldState) -> Vec<LolSimV3AgentDecision> {
    world
        .champions
        .iter()
        .enumerate()
        .map(|(idx, champion)| evaluate_one(world, champion, idx as u64))
        .collect()
}

/// Applies the new state to each champion from agent decisions.
pub fn apply_agent_states(world: &mut LolSimV3WorldState) -> Vec<LolSimV3AgentDecision> {
    let decisions = evaluate_agents(world);

    for decision in &decisions {
        if let Some(champion) = world
            .champions
            .iter_mut()
            .find(|champion| champion.id == decision.champion_id)
        {
            champion.state = decision.next_state.clone();
        }
    }

    decisions
}

fn evaluate_one(
    world: &LolSimV3WorldState,
    champion: &LolSimV3ChampionState,
    idx_salt: u64,
) -> LolSimV3AgentDecision {
    let hp_ratio = (champion.hp / champion.max_hp.max(1.0)).clamp(0.0, 1.0);

    if !champion.alive {
        return decide(champion, LolSimV3AgentState::Dead, "champion_dead");
    }

    if hp_ratio <= 0.22 {
        return decide(
            champion,
            LolSimV3AgentState::Recalling,
            "critical_hp_recall",
        );
    }

    let has_early_dragon_window = world.time_sec >= 4.5 * 60.0 && world.time_sec <= 8.5 * 60.0;
    let objective_bias = champion.role == "JGL" || champion.role == "SUP";

    if has_early_dragon_window && objective_bias && hp_ratio >= 0.45 {
        return decide(
            champion,
            LolSimV3AgentState::ObjectiveSetup,
            "early_dragon_setup",
        );
    }

    let pressure_roll = deterministic_roll(world.rng_state, world.tick, idx_salt);

    if hp_ratio >= 0.72 && pressure_roll > 0.72 {
        return decide(champion, LolSimV3AgentState::Pushing, "high_hp_push_window");
    }

    if hp_ratio >= 0.58 && pressure_roll > 0.48 {
        return decide(champion, LolSimV3AgentState::Fighting, "trade_window");
    }

    if champion.role == "JGL" && pressure_roll > 0.35 {
        return decide(champion, LolSimV3AgentState::Roaming, "jungle_roam_cycle");
    }

    decide(champion, LolSimV3AgentState::Laning, "default_lane_hold")
}

fn decide(
    champion: &LolSimV3ChampionState,
    next_state: LolSimV3AgentState,
    reason: &'static str,
) -> LolSimV3AgentDecision {
    LolSimV3AgentDecision {
        champion_id: champion.id.clone(),
        current_state: champion.state.clone(),
        next_state,
        reason,
    }
}

fn deterministic_roll(rng_state: u64, tick: u64, idx_salt: u64) -> f64 {
    // Small deterministic pseudo-random roll in [0,1).
    let mixed = rng_state
        ^ tick.wrapping_mul(6364136223846793005)
        ^ idx_salt.wrapping_mul(1442695040888963407)
        ^ 0x9E3779B97F4A7C15;
    let bucket = (mixed % 10_000) as f64;
    bucket / 10_000.0
}

pub fn team_alive_counts(world: &LolSimV3WorldState) -> (usize, usize) {
    let blue_alive = world
        .champions
        .iter()
        .filter(|champion| champion.alive && champion.team == LolSimV3Team::Blue)
        .count();
    let red_alive = world
        .champions
        .iter()
        .filter(|champion| champion.alive && champion.team == LolSimV3Team::Red)
        .count();
    (blue_alive, red_alive)
}
