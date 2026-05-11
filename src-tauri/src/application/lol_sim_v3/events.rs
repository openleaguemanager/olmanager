use std::collections::HashMap;

use super::{LolSimV3Event, LolSimV3EventKind, LolSimV3Vec2};

const EVENT_CAP: usize = 240;

#[derive(Debug, Clone, Default)]
pub struct LolSimV3EventQueue {
    events: Vec<LolSimV3Event>,
}

impl LolSimV3EventQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(
        &mut self,
        tick: u64,
        time_sec: f64,
        kind: LolSimV3EventKind,
        actor_id: Option<String>,
        target_id: Option<String>,
        amount: Option<f64>,
        from_pos: Option<LolSimV3Vec2>,
        to_pos: Option<LolSimV3Vec2>,
    ) {
        self.push_with_context(
            tick,
            time_sec,
            kind,
            actor_id,
            target_id,
            None,
            None,
            amount,
            from_pos,
            to_pos,
            HashMap::new(),
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn push_with_context(
        &mut self,
        tick: u64,
        time_sec: f64,
        kind: LolSimV3EventKind,
        actor_id: Option<String>,
        target_id: Option<String>,
        team: Option<String>,
        lane: Option<String>,
        amount: Option<f64>,
        from_pos: Option<LolSimV3Vec2>,
        to_pos: Option<LolSimV3Vec2>,
        metadata: HashMap<String, serde_json::Value>,
    ) {
        let normalized_metadata = normalize_event_metadata(
            &kind,
            target_id.as_deref(),
            team.as_deref(),
            lane.as_deref(),
            metadata,
        );
        let event = LolSimV3Event {
            id: format!("v3-{}-{}", tick, self.events.len() + 1),
            t: time_sec,
            kind,
            actor_id,
            target_id,
            team,
            lane,
            amount,
            from_state: None,
            to_state: None,
            from_pos,
            to_pos,
            metadata: normalized_metadata,
        };
        self.events.push(event);

        if self.events.len() > EVENT_CAP {
            let extra = self.events.len() - EVENT_CAP;
            self.events.drain(0..extra);
        }
    }

    pub fn into_events(self) -> Vec<LolSimV3Event> {
        self.events
    }
}

fn normalize_event_metadata(
    kind: &LolSimV3EventKind,
    target_id: Option<&str>,
    team: Option<&str>,
    lane: Option<&str>,
    mut metadata: HashMap<String, serde_json::Value>,
) -> HashMap<String, serde_json::Value> {
    metadata
        .entry("v".to_string())
        .or_insert_with(|| serde_json::json!(1));
    if let Some(team) = team {
        metadata
            .entry("team".to_string())
            .or_insert_with(|| serde_json::json!(team));
    }
    if let Some(lane) = lane {
        metadata
            .entry("lane".to_string())
            .or_insert_with(|| serde_json::json!(lane));
    }
    if let Some(key) = metadata
        .get("key")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
    {
        if key.trim().is_empty() {
            if let Some(target_id) = target_id {
                metadata.insert("key".to_string(), serde_json::json!(target_id));
            }
        }
    } else if let Some(target_id) = target_id {
        metadata.insert("key".to_string(), serde_json::json!(target_id));
    }

    match kind {
        LolSimV3EventKind::WaveSpawned => {
            metadata
                .entry("overlayType".to_string())
                .or_insert_with(|| serde_json::json!("wave-spawn"));
            metadata
                .entry("source".to_string())
                .or_insert_with(|| serde_json::json!("wave"));
            metadata
                .entry("importance".to_string())
                .or_insert_with(|| serde_json::json!("low"));
        }
        LolSimV3EventKind::NeutralCampSpawned => {
            metadata
                .entry("overlayType".to_string())
                .or_insert_with(|| serde_json::json!("neutral-spawn"));
            metadata
                .entry("source".to_string())
                .or_insert_with(|| serde_json::json!("timer"));
            metadata
                .entry("importance".to_string())
                .or_insert_with(|| serde_json::json!("low"));
        }
        LolSimV3EventKind::NeutralCampTaken => {
            metadata
                .entry("overlayType".to_string())
                .or_insert_with(|| serde_json::json!("neutral-taken"));
            metadata
                .entry("source".to_string())
                .or_insert_with(|| serde_json::json!("jungle-camp"));
            metadata
                .entry("importance".to_string())
                .or_insert_with(|| serde_json::json!("low"));
        }
        LolSimV3EventKind::TowerDamaged => {
            metadata
                .entry("overlayType".to_string())
                .or_insert_with(|| serde_json::json!("structure-pressure"));
            metadata
                .entry("source".to_string())
                .or_insert_with(|| serde_json::json!("push"));
            metadata
                .entry("importance".to_string())
                .or_insert_with(|| serde_json::json!("normal"));
        }
        LolSimV3EventKind::TowerDestroyed => {
            metadata
                .entry("overlayType".to_string())
                .or_insert_with(|| serde_json::json!("structure-destroyed"));
            metadata
                .entry("source".to_string())
                .or_insert_with(|| serde_json::json!("push"));
            metadata
                .entry("importance".to_string())
                .or_insert_with(|| serde_json::json!("high"));
        }
        _ => {}
    }

    metadata
}
