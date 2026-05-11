use serde_json::Value;

use super::{RuntimeEvent, RuntimeState, EVENT_CAP, SKIP_FAST_MODE_EXTRA_KEY};

pub fn push_event(events: &mut Vec<RuntimeEvent>, at: f64, text: &str, kind: &str) {
    events.push(RuntimeEvent {
        t: at,
        text: text.to_string(),
        kind: kind.to_string(),
        metadata: None,
    });

    if events.len() > EVENT_CAP {
        let drain = events.len() - EVENT_CAP;
        events.drain(0..drain);
    }
}

pub fn log_event(runtime: &mut RuntimeState, text: &str, kind: &str) {
    if runtime_is_skip_fast_mode(runtime)
        && !matches!(kind, "kill" | "tower" | "dragon" | "baron" | "nexus")
    {
        return;
    }
    push_event(&mut runtime.events, runtime.time_sec, text, kind);
}

pub fn log_event_with_metadata(
    runtime: &mut RuntimeState,
    text: &str,
    kind: &str,
    metadata: Value,
) {
    if runtime_is_skip_fast_mode(runtime)
        && !matches!(kind, "kill" | "tower" | "dragon" | "baron" | "nexus")
    {
        return;
    }

    runtime.events.push(RuntimeEvent {
        t: runtime.time_sec,
        text: text.to_string(),
        kind: kind.to_string(),
        metadata: Some(metadata),
    });

    if runtime.events.len() > EVENT_CAP {
        let drain = runtime.events.len() - EVENT_CAP;
        runtime.events.drain(0..drain);
    }
}

fn runtime_is_skip_fast_mode(runtime: &RuntimeState) -> bool {
    runtime
        .extra
        .get(SKIP_FAST_MODE_EXTRA_KEY)
        .and_then(Value::as_bool)
        .unwrap_or(false)
}
