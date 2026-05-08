use crate::types::{DraftStrategy, MatchConfig, PlayerData, Side};

// ---------------------------------------------------------------------------
// PlayerSnap — lightweight snapshot of a player to avoid borrow conflicts
// ---------------------------------------------------------------------------

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct PlayerSnap {
    pub id: String,
    pub mechanics: u8,
    pub laning: u8,
    pub teamfighting: u8,
    pub macro_play: u8,
    pub consistency: u8,
    pub shotcalling: u8,
    pub champion_pool: u8,
    pub discipline: u8,
    pub mental_resilience: u8,
    pub traits: Vec<String>,
}

impl PlayerSnap {
    pub fn from(p: &PlayerData) -> Self {
        Self {
            id: p.id.clone(),
            mechanics: p.mechanics,
            laning: p.laning,
            teamfighting: p.teamfighting,
            macro_play: p.macro_play,
            consistency: p.consistency,
            shotcalling: p.shotcalling,
            champion_pool: p.champion_pool,
            discipline: p.discipline,
            mental_resilience: p.mental_resilience,
            traits: p.traits.clone(),
        }
    }

    pub fn has_trait(&self, name: &str) -> bool {
        self.traits.iter().any(|t| t == name)
    }
}

// ---------------------------------------------------------------------------
// TraitContext — which game action context we're computing a bonus for
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub(crate) enum TraitContext {
    Shooting,
    Dribbling,
    Passing,
    Tackling,
    Goalkeeping,
    Foul,
    Midfield,
}

/// Compute a multiplicative trait bonus for a specific action context.
/// Temporarily dummied out to return 1.0 until LoL trait system is designed.
#[allow(unused_variables)]
pub(crate) fn trait_bonus(snap: &PlayerSnap, context: TraitContext) -> f64 {
    1.0
}

// ---------------------------------------------------------------------------
// Draft-strategy modifiers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub(crate) enum DraftStrategyPhase {
    Midfield,
    Attack,
    Defense,
    Press,
}

pub(crate) fn draft_strategy_modifier(
    strategy: DraftStrategy,
    phase: DraftStrategyPhase,
    is_own_phase: bool,
) -> f64 {
    if !is_own_phase {
        return 1.0;
    }
    match (strategy, phase) {
        // Aggressive (formerly Attacking + HighPress) — uses HighPress values
        (DraftStrategy::Aggressive, DraftStrategyPhase::Attack) => 1.12,
        (DraftStrategy::Aggressive, DraftStrategyPhase::Defense) => 0.95,
        (DraftStrategy::Aggressive, DraftStrategyPhase::Press) => 1.20,
        // Passive (formerly Defensive)
        (DraftStrategy::Passive, DraftStrategyPhase::Defense) => 1.12,
        (DraftStrategy::Passive, DraftStrategyPhase::Attack) => 0.93,
        // Scaling (formerly Possession)
        (DraftStrategy::Scaling, DraftStrategyPhase::Midfield) => 1.15,
        (DraftStrategy::Scaling, DraftStrategyPhase::Attack) => 0.97,
        // CounterPick (formerly Counter)
        (DraftStrategy::CounterPick, DraftStrategyPhase::Attack) => 1.18,
        (DraftStrategy::CounterPick, DraftStrategyPhase::Midfield) => 0.92,
        // Balanced and PriorityBans use default 1.0
        _ => 1.0,
    }
}

// ---------------------------------------------------------------------------
// Home advantage modifier
// ---------------------------------------------------------------------------

pub(crate) fn home_mod(side: Side, config: &MatchConfig) -> f64 {
    match side {
        Side::Home => config.home_advantage,
        Side::Away => 1.0,
    }
}
