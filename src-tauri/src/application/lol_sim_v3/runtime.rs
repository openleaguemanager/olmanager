use super::*;

pub fn init(
    store: &LolSimV3StoreState,
    request: LolSimV3InitRequest,
) -> Result<LolSimV3TickResponse, String> {
    if request.session_id.trim().is_empty() {
        return Err("sessionId is required".to_string());
    }

    let mut world = create_minimal_world_state(&request.seed, request.tick_dt_sec);
    apply_champion_identity(&mut world, &request.champion_by_player_id);
    let snapshot = world_snapshot(&world);
    let session = LolSimV3Session {
        id: request.session_id.clone(),
        world,
    };

    let mut sessions = store
        .sessions
        .lock()
        .map_err(|_| "lol sim v3 session store lock poisoned".to_string())?;
    sessions.insert(session.id.clone(), session);

    Ok(LolSimV3TickResponse {
        session_id: request.session_id,
        snapshot,
        events: Vec::new(),
    })
}

pub fn tick(
    store: &LolSimV3StoreState,
    request: LolSimV3TickRequest,
) -> Result<LolSimV3TickResponse, String> {
    let mut sessions = store
        .sessions
        .lock()
        .map_err(|_| "lol sim v3 session store lock poisoned".to_string())?;

    let session = sessions
        .get_mut(&request.session_id)
        .ok_or_else(|| format!("lol sim v3 session not found: {}", request.session_id))?;

    session.world.running = request.running;
    if !session.world.running {
        return Ok(LolSimV3TickResponse {
            session_id: request.session_id,
            snapshot: world_snapshot(&session.world),
            events: Vec::new(),
        });
    }

    let mut all_events = Vec::new();
    let steps = request.steps.max(1);
    for _ in 0..steps {
        if !session.world.running || session.world.winner.is_some() {
            break;
        }
        let decisions = apply_agent_states(&mut session.world);
        let intentions = intentions_from_decisions(&session.world, &decisions);
        let report = resolve_intentions_by_systems(&mut session.world, &intentions);
        all_events.extend(report.events);
    }

    Ok(LolSimV3TickResponse {
        session_id: request.session_id,
        snapshot: world_snapshot(&session.world),
        events: all_events,
    })
}

pub fn reset(
    store: &LolSimV3StoreState,
    request: LolSimV3ResetRequest,
) -> Result<LolSimV3TickResponse, String> {
    let mut sessions = store
        .sessions
        .lock()
        .map_err(|_| "lol sim v3 session store lock poisoned".to_string())?;

    let session = sessions
        .get_mut(&request.session_id)
        .ok_or_else(|| format!("lol sim v3 session not found: {}", request.session_id))?;

    session.world = create_minimal_world_state(&request.seed, request.tick_dt_sec);

    Ok(LolSimV3TickResponse {
        session_id: request.session_id,
        snapshot: world_snapshot(&session.world),
        events: Vec::new(),
    })
}

pub fn dispose(
    store: &LolSimV3StoreState,
    request: LolSimV3DisposeRequest,
) -> Result<LolSimV3DisposeResponse, String> {
    let mut sessions = store
        .sessions
        .lock()
        .map_err(|_| "lol sim v3 session store lock poisoned".to_string())?;
    let removed = sessions.remove(&request.session_id).is_some();

    Ok(LolSimV3DisposeResponse {
        session_id: request.session_id,
        disposed: removed,
    })
}

pub fn run_to_completion(
    _store: &LolSimV3StoreState,
    request: LolSimV3RunToCompletionRequest,
) -> Result<LolSimV3RunToCompletionResponse, String> {
    let mut world = create_minimal_world_state(&request.seed, request.tick_dt_sec);
    apply_champion_identity(&mut world, &request.champion_by_player_id);
    let mut all_events = Vec::new();
    let mut steps = 0u32;

    while steps < request.max_steps {
        if !world.running || world.winner.is_some() {
            break;
        }
        let decisions = apply_agent_states(&mut world);
        let intentions = intentions_from_decisions(&world, &decisions);
        let report = resolve_intentions_by_systems(&mut world, &intentions);
        all_events.extend(report.events);
        steps = steps.saturating_add(1);
    }

    let snapshot = world_snapshot(&world);
    Ok(LolSimV3RunToCompletionResponse {
        winner: snapshot.winner.clone(),
        steps,
        elapsed_simulated_sec: snapshot.time_sec,
        snapshot,
        events: all_events,
    })
}

fn apply_champion_identity(
    world: &mut LolSimV3WorldState,
    champion_by_player_id: &std::collections::HashMap<String, String>,
) {
    fn normalize_key(value: &str) -> String {
        value
            .trim()
            .to_lowercase()
            .replace('_', "-")
            .replace(' ', "-")
    }

    fn role_slot_key(team: LolSimV3Team, role: &str) -> String {
        let role_key = match role {
            "TOP" => "top".to_string(),
            "JGL" | "JUNGLE" => "jgl".to_string(),
            "MID" => "mid".to_string(),
            "ADC" | "BOT" => "adc".to_string(),
            "SUP" | "SUPPORT" => "sup".to_string(),
            _ => role.to_lowercase(),
        };
        format!("{}-{role_key}", team.as_str())
    }

    let normalized_map: std::collections::HashMap<String, String> = champion_by_player_id
        .iter()
        .map(|(key, value)| (normalize_key(key), value.clone()))
        .collect();

    for champion in &mut world.champions {
        let direct = normalized_map.get(&normalize_key(&champion.id)).cloned();
        let role_slot = role_slot_key(champion.team, champion.role.as_str());
        let by_slot = normalized_map.get(&role_slot).cloned().or_else(|| {
            let alias = role_slot
                .replace("-jgl", "-jungle")
                .replace("-sup", "-support");
            normalized_map.get(&alias).cloned()
        });

        if let Some(champion_id) = direct.or(by_slot).filter(|id| !id.trim().is_empty()) {
            champion.champion_id = champion_id;
        }
    }
}
