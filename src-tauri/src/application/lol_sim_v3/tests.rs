#![cfg(test)]

use super::*;
use std::collections::HashMap;

#[test]
fn run_to_completion_is_deterministic_for_same_seed() {
    let store = LolSimV3StoreState::default();
    let request = LolSimV3RunToCompletionRequest {
        seed: "seed-123".to_string(),
        snapshot: serde_json::Value::Null,
        champion_by_player_id: std::collections::HashMap::new(),
        tick_dt_sec: 0.1,
        max_steps: 2000,
    };

    let first = run_to_completion(&store, request.clone()).expect("first run");
    let second = run_to_completion(&store, request).expect("second run");

    assert_eq!(first.winner, second.winner);
    assert_eq!(first.steps, second.steps);
    assert_eq!(
        first.snapshot.scoreboard.blue.kills,
        second.snapshot.scoreboard.blue.kills
    );
    assert_eq!(
        first.snapshot.scoreboard.red.kills,
        second.snapshot.scoreboard.red.kills
    );
}

#[test]
fn emitted_events_are_time_ordered() {
    let store = LolSimV3StoreState::default();
    let response = run_to_completion(
        &store,
        LolSimV3RunToCompletionRequest {
            seed: "events-ordered".to_string(),
            snapshot: serde_json::Value::Null,
            champion_by_player_id: std::collections::HashMap::new(),
            tick_dt_sec: 0.1,
            max_steps: 300,
        },
    )
    .expect("run to completion");

    let mut prev = f64::NEG_INFINITY;
    for event in response.events {
        assert!(event.t >= prev, "event time moved backwards");
        prev = event.t;
    }
}

#[test]
fn run_to_completion_finishes_before_max_steps() {
    let store = LolSimV3StoreState::default();
    let max_steps = 5000;
    let response = run_to_completion(
        &store,
        LolSimV3RunToCompletionRequest {
            seed: "bounded-sim".to_string(),
            snapshot: serde_json::Value::Null,
            champion_by_player_id: std::collections::HashMap::new(),
            tick_dt_sec: 0.1,
            max_steps,
        },
    )
    .expect("run to completion");

    assert!(response.steps < max_steps);
    assert!(response.winner.is_some());
}

#[test]
fn combat_kill_updates_scoreboard_kills() {
    let mut world = create_minimal_world_state("kills", 0.1);
    // Make one red champion easy to kill
    if let Some(red) = world
        .champions
        .iter_mut()
        .find(|champion| champion.team == LolSimV3Team::Red)
    {
        red.hp = 1.0;
    }

    let intention = LolSimV3Intention {
        champion_id: "blue-top".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::Fighting,
        kind: LolSimV3IntentKind::TradeWithEnemy,
        priority: 90,
        reason: "test",
    };

    let report = resolve_intentions_by_systems(&mut world, &[intention]);

    assert!(report.combat_actions >= 1);
    assert!(world.scoreboard.blue.kills >= 1);
}

#[test]
fn destroyed_nexus_sets_winner() {
    let mut world = create_minimal_world_state("nexus", 0.1);
    if let Some(red_nexus) = world
        .structures
        .iter_mut()
        .find(|structure| structure.id == "red-nexus")
    {
        red_nexus.alive = false;
    }

    recompute_winner(&mut world);

    assert_eq!(world.winner, Some(LolSimV3Team::Blue));
}

#[test]
fn lightweight_snapshot_does_not_expose_internal_fields() {
    let world = create_minimal_world_state("snapshot", 0.1);
    let snapshot = world_snapshot(&world);
    let serialized = serde_json::to_value(snapshot).expect("serialize snapshot");
    let obj = serialized.as_object().expect("snapshot object");

    assert!(!obj.contains_key("seed"));
    assert!(!obj.contains_key("rngState"));
    assert!(obj.contains_key("timeSec"));
    assert!(obj.contains_key("units"));
    assert!(obj.contains_key("minions"));
    assert!(obj.contains_key("structures"));
    assert!(obj.contains_key("objectives"));
    assert!(obj.contains_key("neutralCamps"));
    assert!(obj.contains_key("lanePressure"));
    assert!(obj.contains_key("towerTargets"));
    assert!(obj.contains_key("neutralTimers"));
    assert!(obj.contains_key("phaseContributions"));
    assert!(obj.contains_key("roleLaneContributions"));
    assert!(obj.contains_key("objectivePressureSummary"));
}

#[test]
fn snapshot_exposes_additive_debug_telemetry_consistently() {
    let mut world = create_minimal_world_state("snapshot-telemetry", 1.0);
    let red_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");
    world.minions.push(LolSimV3MinionState {
        id: "blue-top-telemetry".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "melee".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: red_top_outer_pos,
        lane_progress: 0.95,
    });

    resolve_intentions_by_systems(&mut world, &[]);
    let snapshot = world_snapshot(&world);

    assert_eq!(snapshot.lane_pressure.len(), 3);
    assert!(snapshot
        .lane_pressure
        .iter()
        .any(|entry| entry.lane == "top"));
    assert!(snapshot.lane_pressure.iter().all(|entry| entry.blue >= -1.0
        && entry.blue <= 1.0
        && entry.red >= -1.0
        && entry.red <= 1.0));
    assert!(snapshot.neutral_timers.camps_alive >= 0);
    assert!(snapshot.neutral_timers.camps_respawning >= 0);
    assert!(!snapshot.phase_contributions.is_empty());
    assert_eq!(snapshot.phase_contributions.len(), 14);
    assert_eq!(snapshot.role_lane_contributions.len(), 10);
    assert!(snapshot
        .phase_contributions
        .iter()
        .all(|entry| entry.value >= 0.0 && entry.value <= 1.0));
    assert!(snapshot
        .role_lane_contributions
        .iter()
        .all(|entry| entry.pressure >= -1.0 && entry.pressure <= 1.0));
    assert!(
        snapshot.objective_pressure_summary.blue >= 0.0
            && snapshot.objective_pressure_summary.blue <= 1.0
    );
    assert!(
        snapshot.objective_pressure_summary.red >= 0.0
            && snapshot.objective_pressure_summary.red <= 1.0
    );
}

#[test]
fn additive_telemetry_is_deterministic_for_same_seed() {
    let mut first = create_minimal_world_state("telemetry-deterministic", 1.0);
    let mut second = create_minimal_world_state("telemetry-deterministic", 1.0);

    for _ in 0..90 {
        resolve_intentions_by_systems(&mut first, &[]);
        resolve_intentions_by_systems(&mut second, &[]);
    }

    let first_snapshot = world_snapshot(&first);
    let second_snapshot = world_snapshot(&second);
    assert_eq!(
        serde_json::to_value(&first_snapshot.phase_contributions)
            .expect("serialize phase contributions"),
        serde_json::to_value(&second_snapshot.phase_contributions)
            .expect("serialize phase contributions")
    );
    assert_eq!(
        serde_json::to_value(&first_snapshot.role_lane_contributions)
            .expect("serialize role/lane contributions"),
        serde_json::to_value(&second_snapshot.role_lane_contributions)
            .expect("serialize role/lane contributions")
    );
    assert_eq!(
        serde_json::to_value(&first_snapshot.objective_pressure_summary)
            .expect("serialize objective pressure summary"),
        serde_json::to_value(&second_snapshot.objective_pressure_summary)
            .expect("serialize objective pressure summary")
    );
}

#[test]
fn epic_objectives_spawn_deterministically_by_cadence() {
    let mut world = create_minimal_world_state("epic-cadence", 1.0);
    while world.time_sec < 305.0 {
        let decisions = apply_agent_states(&mut world);
        let intentions = intentions_from_decisions(&world, &decisions);
        resolve_intentions_by_systems(&mut world, &intentions);
    }

    let dragon = world
        .objectives
        .iter()
        .find(|objective| objective.key == "dragon")
        .expect("dragon exists");
    let baron = world
        .objectives
        .iter()
        .find(|objective| objective.key == "baron")
        .expect("baron exists");
    assert!(dragon.alive, "dragon should be spawned after 5:00");
    assert!(!baron.alive, "baron should not be spawned before 20:00");
}

#[test]
fn dragon_take_sets_respawn_and_reappears_after_timer() {
    let mut world = create_minimal_world_state("dragon-respawn", 1.0);
    while world.time_sec < 301.0 {
        resolve_intentions_by_systems(&mut world, &[]);
    }

    let blue_jgl_pos = LolSimV3Vec2 { x: 0.67, y: 0.70 };
    if let Some(blue_jgl) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-jgl")
    {
        blue_jgl.pos = blue_jgl_pos;
    }
    if let Some(blue_sup) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-sup")
    {
        blue_sup.pos = blue_jgl_pos;
    }

    let before_dragons = world.scoreboard.blue.dragons;
    let take = LolSimV3Intention {
        champion_id: "blue-jgl".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::ObjectiveSetup,
        kind: LolSimV3IntentKind::TakeDragon,
        priority: 100,
        reason: "test",
    };
    let report = resolve_intentions_by_systems(&mut world, &[take]);

    let dragon = world
        .objectives
        .iter()
        .find(|objective| objective.key == "dragon")
        .expect("dragon exists");
    assert!(!dragon.alive, "dragon should despawn when taken");
    assert!(dragon.next_spawn_at_sec.is_some());
    assert_eq!(world.scoreboard.blue.dragons, before_dragons + 1);
    assert!(report
        .events
        .iter()
        .any(|event| event.kind == LolSimV3EventKind::DragonTaken));

    while world.time_sec < 605.0 {
        resolve_intentions_by_systems(&mut world, &[]);
    }
    let dragon = world
        .objectives
        .iter()
        .find(|objective| objective.key == "dragon")
        .expect("dragon exists");
    assert!(
        dragon.alive,
        "dragon should respawn on deterministic cadence"
    );
    assert!(dragon.next_spawn_at_sec.is_none());
}

#[test]
fn neutral_camp_take_triggers_gold_and_respawn_timer() {
    let mut world = create_minimal_world_state("camp-respawn", 1.0);
    while world.time_sec < 95.0 {
        resolve_intentions_by_systems(&mut world, &[]);
    }

    if let Some(blue_jgl) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-jgl")
    {
        blue_jgl.pos = LolSimV3Vec2 { x: 0.20, y: 0.63 };
    }

    let before_gold = world.scoreboard.blue.gold;
    let report = resolve_intentions_by_systems(&mut world, &[]);
    let blue_gromp = world
        .neutral_camps
        .iter()
        .find(|camp| camp.key == "blue-gromp")
        .expect("blue gromp exists");

    assert!(!blue_gromp.alive);
    assert!(blue_gromp.next_spawn_at_sec.is_some());
    assert!(world.scoreboard.blue.gold >= before_gold + 28);
    assert!(report
        .events
        .iter()
        .any(|event| event.kind == LolSimV3EventKind::NeutralCampTaken));
}

#[test]
fn minion_waves_spawn_after_expected_time() {
    let store = LolSimV3StoreState::default();
    let _ = init(
        &store,
        LolSimV3InitRequest {
            session_id: "waves-spawn".to_string(),
            seed: "waves-spawn-seed".to_string(),
            snapshot: serde_json::Value::Null,
            champion_by_player_id: HashMap::new(),
            tick_dt_sec: 1.0,
        },
    )
    .expect("init");

    let response = tick(
        &store,
        LolSimV3TickRequest {
            session_id: "waves-spawn".to_string(),
            running: true,
            steps: 66,
        },
    )
    .expect("tick");

    assert!(response.snapshot.minions.iter().any(|minion| minion.alive));
    assert!(response
        .events
        .iter()
        .any(|event| event.kind == LolSimV3EventKind::WaveSpawned));
}

#[test]
fn neutral_spawn_and_take_events_include_metadata() {
    let mut world = create_minimal_world_state("neutral-events", 1.0);

    let mut spawn_report = None;
    while world.time_sec < 95.0 {
        let report = resolve_intentions_by_systems(&mut world, &[]);
        if report
            .events
            .iter()
            .any(|event| event.kind == LolSimV3EventKind::NeutralCampSpawned)
        {
            spawn_report = Some(report);
            break;
        }
    }

    let spawn_report = spawn_report.expect("neutral spawn report");
    let spawn_event = spawn_report
        .events
        .iter()
        .find(|event| event.kind == LolSimV3EventKind::NeutralCampSpawned)
        .expect("spawn event");
    assert_eq!(
        spawn_event.metadata.get("source").and_then(|v| v.as_str()),
        Some("timer")
    );
    assert!(spawn_event.metadata.contains_key("key"));
    assert!(spawn_event.metadata.contains_key("v"));
    assert!(spawn_event.metadata.contains_key("overlayType"));
    assert!(spawn_event.metadata.contains_key("importance"));

    if let Some(blue_jgl) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-jgl")
    {
        blue_jgl.pos = LolSimV3Vec2 { x: 0.20, y: 0.63 };
    }
    let take_report = resolve_intentions_by_systems(&mut world, &[]);
    let take_event = take_report
        .events
        .iter()
        .find(|event| event.kind == LolSimV3EventKind::NeutralCampTaken)
        .expect("take event");
    assert_eq!(take_event.team.as_deref(), Some("blue"));
    assert_eq!(
        take_event.metadata.get("source").and_then(|v| v.as_str()),
        Some("jungle-camp")
    );
    assert!(take_event.metadata.contains_key("v"));
    assert!(take_event.metadata.contains_key("overlayType"));
    assert!(take_event.metadata.contains_key("importance"));
}

#[test]
fn tower_damage_and_destroyed_events_are_emitted_in_sequence() {
    let mut world = create_minimal_world_state("tower-events", 1.0);
    let target_idx = world
        .structures
        .iter()
        .position(|structure| structure.id == "red-top-outer")
        .expect("red top outer exists");
    world.structures[target_idx].hp = 40.0;

    world.minions.push(LolSimV3MinionState {
        id: "blue-top-siege-test".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "siege".to_string(),
        alive: true,
        hp: 600.0,
        max_hp: 600.0,
        pos: world.structures[target_idx].pos,
        lane_progress: 1.0,
    });

    let mut saw_damage = false;
    let mut saw_destroyed = false;
    for _ in 0..10 {
        let report = resolve_intentions_by_systems(&mut world, &[]);
        if report
            .events
            .iter()
            .any(|event| event.kind == LolSimV3EventKind::TowerDamaged)
        {
            saw_damage = true;
        }
        if let Some(destroyed) = report
            .events
            .iter()
            .find(|event| event.kind == LolSimV3EventKind::TowerDestroyed)
        {
            saw_destroyed = true;
            assert_eq!(destroyed.lane.as_deref(), Some("top"));
            assert_eq!(destroyed.team.as_deref(), Some("red"));
            break;
        }
    }

    assert!(
        saw_damage,
        "expected at least one throttled tower_damaged event"
    );
    assert!(saw_destroyed, "expected tower_destroyed event after damage");
}

#[test]
fn major_event_families_keep_consistent_metadata_contract() {
    let mut world = create_minimal_world_state("event-metadata-consistency", 1.0);
    let mut events = Vec::new();

    while world.time_sec < 100.0 {
        let report = resolve_intentions_by_systems(&mut world, &[]);
        events.extend(report.events);
    }

    let target_idx = world
        .structures
        .iter()
        .position(|structure| structure.id == "red-top-outer")
        .expect("red top outer exists");
    world.structures[target_idx].hp = 20.0;
    world.minions.push(LolSimV3MinionState {
        id: "blue-top-siege-contract".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "siege".to_string(),
        alive: true,
        hp: 600.0,
        max_hp: 600.0,
        pos: world.structures[target_idx].pos,
        lane_progress: 1.0,
    });

    for _ in 0..6 {
        let report = resolve_intentions_by_systems(&mut world, &[]);
        events.extend(report.events);
    }

    for event in events.iter().filter(|event| {
        matches!(
            event.kind,
            LolSimV3EventKind::WaveSpawned
                | LolSimV3EventKind::NeutralCampSpawned
                | LolSimV3EventKind::NeutralCampTaken
                | LolSimV3EventKind::TowerDamaged
                | LolSimV3EventKind::TowerDestroyed
        )
    }) {
        assert!(
            event.metadata.contains_key("v"),
            "missing v for {:?}",
            event.kind
        );
        assert!(
            event.metadata.contains_key("key"),
            "missing key for {:?}",
            event.kind
        );
        assert!(
            event.metadata.contains_key("overlayType"),
            "missing overlayType for {:?}",
            event.kind
        );
        assert!(
            event.metadata.contains_key("source"),
            "missing source for {:?}",
            event.kind
        );
        assert!(
            event.metadata.contains_key("importance"),
            "missing importance for {:?}",
            event.kind
        );
    }
}

#[test]
fn spawned_minions_are_distributed_across_lanes() {
    let mut world = create_minimal_world_state("waves-lanes", 1.0);
    while world.time_sec < 66.0 {
        let decisions = apply_agent_states(&mut world);
        let intentions = intentions_from_decisions(&world, &decisions);
        resolve_intentions_by_systems(&mut world, &intentions);
    }

    assert!(world.minions.iter().any(|minion| minion.lane == "top"));
    assert!(world.minions.iter().any(|minion| minion.lane == "mid"));
    assert!(world.minions.iter().any(|minion| minion.lane == "bot"));
}

#[test]
fn minion_waves_are_deterministic_for_same_seed() {
    let mut first = create_minimal_world_state("waves-deterministic", 1.0);
    let mut second = create_minimal_world_state("waves-deterministic", 1.0);

    for _ in 0..80 {
        let first_decisions = apply_agent_states(&mut first);
        let first_intentions = intentions_from_decisions(&first, &first_decisions);
        resolve_intentions_by_systems(&mut first, &first_intentions);

        let second_decisions = apply_agent_states(&mut second);
        let second_intentions = intentions_from_decisions(&second, &second_decisions);
        resolve_intentions_by_systems(&mut second, &second_intentions);
    }

    assert_eq!(first.minions.len(), second.minions.len());
    for (a, b) in first.minions.iter().zip(second.minions.iter()) {
        assert_eq!(a.id, b.id);
        assert_eq!(a.team, b.team);
        assert_eq!(a.lane, b.lane);
        assert_eq!(a.kind, b.kind);
        assert!((a.pos.x - b.pos.x).abs() < 1e-9);
        assert!((a.pos.y - b.pos.y).abs() < 1e-9);
    }
}

#[test]
fn full_map_snapshot_includes_v2_structure_baseline_and_champion_ids() {
    let world = create_minimal_world_state("map-baseline", 0.1);
    let snapshot = world_snapshot(&world);
    let serialized = serde_json::to_value(&snapshot).expect("serialize snapshot");

    assert_eq!(snapshot.structures.len(), 30);
    assert!(snapshot
        .structures
        .iter()
        .any(|structure| structure.id == "blue-nexus"));
    assert!(snapshot
        .structures
        .iter()
        .any(|structure| structure.id == "red-inhib-bot"));
    for id in [
        "blue-bot-outer",
        "blue-bot-inner",
        "blue-bot-inhib-tower",
        "red-bot-outer",
        "red-bot-inner",
        "red-bot-inhib-tower",
    ] {
        assert!(
            snapshot
                .structures
                .iter()
                .any(|structure| structure.id == id),
            "missing {id}"
        );
    }
    assert!(snapshot
        .units
        .iter()
        .all(|unit| !unit.champion_id.is_empty()));
    assert!(serialized["units"][0].get("championId").is_some());
}

#[test]
fn live_init_preserves_selected_champion_ids_in_snapshot() {
    let store = LolSimV3StoreState::default();
    let mut champion_by_player_id = HashMap::new();
    champion_by_player_id.insert("blue-top".to_string(), "Riven".to_string());
    champion_by_player_id.insert("blue-jgl".to_string(), "Sejuani".to_string());
    champion_by_player_id.insert("blue-mid".to_string(), "Ahri".to_string());
    champion_by_player_id.insert("blue-adc".to_string(), "Jinx".to_string());
    champion_by_player_id.insert("blue-sup".to_string(), "Thresh".to_string());
    champion_by_player_id.insert("red-top".to_string(), "Garen".to_string());
    champion_by_player_id.insert("red-jgl".to_string(), "LeeSin".to_string());
    champion_by_player_id.insert("red-mid".to_string(), "Lux".to_string());
    champion_by_player_id.insert("red-adc".to_string(), "Ezreal".to_string());
    champion_by_player_id.insert("red-sup".to_string(), "Leona".to_string());

    let response = init(
        &store,
        LolSimV3InitRequest {
            session_id: "champion-identity".to_string(),
            seed: "champion-identity-seed".to_string(),
            snapshot: serde_json::Value::Null,
            champion_by_player_id,
            tick_dt_sec: 0.1,
        },
    )
    .expect("v3 init");

    for (unit_id, champion_id) in [
        ("blue-top", "Riven"),
        ("blue-jgl", "Sejuani"),
        ("blue-mid", "Ahri"),
        ("blue-adc", "Jinx"),
        ("blue-sup", "Thresh"),
        ("red-top", "Garen"),
        ("red-jgl", "LeeSin"),
        ("red-mid", "Lux"),
        ("red-adc", "Ezreal"),
        ("red-sup", "Leona"),
    ] {
        assert_eq!(
            response
                .snapshot
                .units
                .iter()
                .find(|unit| unit.id == unit_id)
                .map(|unit| unit.champion_id.as_str()),
            Some(champion_id),
            "champion identity mismatch for {unit_id}",
        );
    }
    assert!(response
        .snapshot
        .units
        .iter()
        .all(|unit| !unit.champion_id.trim().is_empty()));
}

#[test]
fn initial_positions_are_lane_distributed_for_core_roles() {
    let world = create_minimal_world_state("lane-distribution", 0.1);
    let blue_top = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-top")
        .expect("blue top exists");
    let blue_mid = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-mid")
        .expect("blue mid exists");
    let blue_adc = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-adc")
        .expect("blue adc exists");

    assert!(
        blue_top.pos.y < blue_mid.pos.y,
        "top should start above mid"
    );
    assert!(
        blue_mid.pos.y < blue_adc.pos.y,
        "mid should start above bot/adc"
    );
    assert!(blue_top.lane == "top" && blue_mid.lane == "mid" && blue_adc.lane == "bot");
}

#[test]
fn structure_pressure_respects_lane_gate_before_nexus() {
    let mut world = create_minimal_world_state("structure-gates", 0.1);
    let red_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");
    if let Some(blue_top) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-top")
    {
        blue_top.pos = red_top_outer_pos;
    }
    world.minions.push(LolSimV3MinionState {
        id: "blue-top-siege-test".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "siege".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: red_top_outer_pos,
        lane_progress: 0.95,
    });
    let nexus_hp = world
        .structures
        .iter_mut()
        .find(|structure| structure.id == "red-nexus")
        .map(|structure| {
            structure.hp = 1.0;
            structure.hp
        })
        .expect("red nexus exists");

    let intention = LolSimV3Intention {
        champion_id: "blue-top".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::Pushing,
        kind: LolSimV3IntentKind::PushTower,
        priority: 90,
        reason: "test",
    };

    resolve_intentions_by_systems(&mut world, &[intention]);

    let red_top_outer = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .expect("red top outer exists");
    let red_nexus = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-nexus")
        .expect("red nexus exists");

    assert!(red_top_outer.hp < red_top_outer.max_hp);
    assert!(red_nexus.alive);
    assert_eq!(red_nexus.hp, nexus_hp);
    assert!(world.winner.is_none());
}

#[test]
fn structure_pressure_requires_enemy_structure_proximity() {
    let mut world = create_minimal_world_state("structure-proximity", 0.1);
    let red_top_outer_hp = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.hp)
        .expect("red top outer exists");

    let intention = LolSimV3Intention {
        champion_id: "blue-top".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::Pushing,
        kind: LolSimV3IntentKind::PushTower,
        priority: 90,
        reason: "test",
    };

    resolve_intentions_by_systems(&mut world, &[intention]);

    let red_top_outer = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .expect("red top outer exists");
    assert_eq!(red_top_outer.hp, red_top_outer_hp);
}

#[test]
fn structure_pressure_never_damages_own_team_structures() {
    let mut world = create_minimal_world_state("own-structure-invariant", 0.1);
    let blue_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "blue-top-outer")
        .map(|structure| structure.pos)
        .expect("blue top outer exists");
    if let Some(blue_top) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-top")
    {
        blue_top.pos = blue_outer_pos;
    }
    let blue_structure_hp: HashMap<String, f64> = world
        .structures
        .iter()
        .filter(|structure| structure.team == LolSimV3Team::Blue)
        .map(|structure| (structure.id.clone(), structure.hp))
        .collect();

    let intention = LolSimV3Intention {
        champion_id: "blue-top".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::Pushing,
        kind: LolSimV3IntentKind::PushTower,
        priority: 90,
        reason: "test",
    };

    resolve_intentions_by_systems(&mut world, &[intention]);

    for structure in world
        .structures
        .iter()
        .filter(|structure| structure.team == LolSimV3Team::Blue)
    {
        assert_eq!(
            Some(&structure.hp),
            blue_structure_hp.get(&structure.id),
            "own structure changed: {}",
            structure.id
        );
        assert!(structure.alive, "own structure destroyed: {}", structure.id);
    }
}

#[test]
fn structure_pressure_is_blocked_without_allied_wave_near_target() {
    let mut world = create_minimal_world_state("wave-gate-block", 0.1);
    let red_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");
    let red_top_outer_hp = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.hp)
        .expect("red top outer exists");
    if let Some(blue_top) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-top")
    {
        blue_top.pos = red_top_outer_pos;
    }

    let intention = LolSimV3Intention {
        champion_id: "blue-top".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::Pushing,
        kind: LolSimV3IntentKind::PushTower,
        priority: 90,
        reason: "test",
    };

    resolve_intentions_by_systems(&mut world, &[intention]);

    let red_top_outer = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .expect("red top outer exists");
    assert_eq!(red_top_outer.hp, red_top_outer_hp);
}

#[test]
fn structure_pressure_damages_when_allied_wave_is_near_target() {
    let mut world = create_minimal_world_state("wave-gate-allow", 0.1);
    let red_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");
    if let Some(blue_top) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-top")
    {
        blue_top.pos = red_top_outer_pos;
    }
    world.minions.push(LolSimV3MinionState {
        id: "blue-top-siege-near".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "siege".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: red_top_outer_pos,
        lane_progress: 0.95,
    });
    let initial_hp = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.hp)
        .expect("red top outer exists");

    let intention = LolSimV3Intention {
        champion_id: "blue-top".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::Pushing,
        kind: LolSimV3IntentKind::PushTower,
        priority: 90,
        reason: "test",
    };

    for _ in 0..3 {
        resolve_intentions_by_systems(&mut world, std::slice::from_ref(&intention));
    }

    let red_top_outer = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .expect("red top outer exists");
    assert!(red_top_outer.hp < initial_hp);
}

#[test]
fn enemy_wave_and_champion_cannot_damage_own_team_structure() {
    let mut world = create_minimal_world_state("own-structure-wave-guard", 0.1);
    let blue_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "blue-top-outer")
        .map(|structure| structure.pos)
        .expect("blue top outer exists");
    if let Some(red_top) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "red-top")
    {
        red_top.pos = blue_top_outer_pos;
    }
    world.minions.push(LolSimV3MinionState {
        id: "red-top-siege-near-own".to_string(),
        team: LolSimV3Team::Red,
        lane: "top".to_string(),
        kind: "siege".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: blue_top_outer_pos,
        lane_progress: 0.95,
    });
    let blue_hp = world
        .structures
        .iter()
        .find(|structure| structure.id == "blue-top-outer")
        .map(|structure| structure.hp)
        .expect("blue top outer exists");

    let red_push = LolSimV3Intention {
        champion_id: "red-top".to_string(),
        team: LolSimV3Team::Red,
        state: LolSimV3AgentState::Pushing,
        kind: LolSimV3IntentKind::PushTower,
        priority: 90,
        reason: "test",
    };
    let blue_push = LolSimV3Intention {
        champion_id: "blue-top".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::Pushing,
        kind: LolSimV3IntentKind::PushTower,
        priority: 90,
        reason: "test",
    };

    resolve_intentions_by_systems(&mut world, &[red_push, blue_push]);

    let blue_outer = world
        .structures
        .iter()
        .find(|structure| structure.id == "blue-top-outer")
        .expect("blue top outer exists");
    assert_eq!(blue_outer.hp, blue_hp);
}

#[test]
fn tower_attacks_enemy_minion_in_lane_when_present() {
    let mut world = create_minimal_world_state("tower-shots-minion", 1.0);
    let red_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");
    world.minions.push(LolSimV3MinionState {
        id: "blue-top-minion-under-tower".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "melee".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: red_top_outer_pos,
        lane_progress: 0.95,
    });

    resolve_intentions_by_systems(&mut world, &[]);

    let minion = world
        .minions
        .iter()
        .find(|minion| minion.id == "blue-top-minion-under-tower")
        .expect("minion still tracked after one shot");
    assert!(minion.hp < 420.0);
}

#[test]
fn tower_prefers_minion_over_champion_when_both_are_valid() {
    let mut world = create_minimal_world_state("tower-target-priority", 1.0);
    let red_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");

    world.minions.push(LolSimV3MinionState {
        id: "blue-top-minion-priority".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "melee".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: red_top_outer_pos,
        lane_progress: 0.95,
    });
    let blue_top_hp_before = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-top")
        .map(|champion| champion.hp)
        .expect("blue top exists");
    if let Some(blue_top) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-top")
    {
        blue_top.pos = red_top_outer_pos;
    }

    resolve_intentions_by_systems(&mut world, &[]);

    let minion = world
        .minions
        .iter()
        .find(|minion| minion.id == "blue-top-minion-priority")
        .expect("minion still tracked after one shot");
    let blue_top = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-top")
        .expect("blue top exists");

    assert!(minion.hp < 420.0);
    assert_eq!(blue_top.hp, blue_top_hp_before);
}

#[test]
fn tower_damages_enemy_champion_when_no_enemy_minion_available() {
    let mut world = create_minimal_world_state("tower-shots-champion", 1.0);
    let red_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");
    if let Some(blue_top) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-top")
    {
        blue_top.pos = red_top_outer_pos;
    }
    let hp_before = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-top")
        .map(|champion| champion.hp)
        .expect("blue top exists");

    resolve_intentions_by_systems(&mut world, &[]);

    let blue_top = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-top")
        .expect("blue top exists");
    assert!(blue_top.hp < hp_before);
}

#[test]
fn tower_shot_resolution_remains_deterministic_for_same_seed() {
    let mut first = create_minimal_world_state("tower-determinism", 1.0);
    let mut second = create_minimal_world_state("tower-determinism", 1.0);
    let red_top_outer_pos = first
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");

    for world in [&mut first, &mut second] {
        world.minions.push(LolSimV3MinionState {
            id: "blue-top-minion-determinism".to_string(),
            team: LolSimV3Team::Blue,
            lane: "top".to_string(),
            kind: "melee".to_string(),
            alive: true,
            hp: 420.0,
            max_hp: 420.0,
            pos: red_top_outer_pos,
            lane_progress: 0.95,
        });
    }

    for _ in 0..4 {
        resolve_intentions_by_systems(&mut first, &[]);
        resolve_intentions_by_systems(&mut second, &[]);
    }

    assert_eq!(first.tick, second.tick);
    assert_eq!(first.time_sec, second.time_sec);
    assert_eq!(first.minions.len(), second.minions.len());
    assert_eq!(
        first
            .champions
            .iter()
            .find(|champion| champion.id == "blue-top")
            .map(|champion| champion.hp),
        second
            .champions
            .iter()
            .find(|champion| champion.id == "blue-top")
            .map(|champion| champion.hp)
    );
}

#[test]
fn tower_keeps_locked_target_across_short_cadence_window() {
    let mut world = create_minimal_world_state("tower-lock-window", 1.0);
    let red_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");

    world.minions.push(LolSimV3MinionState {
        id: "blue-top-minion-lock-a".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "melee".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: red_top_outer_pos,
        lane_progress: 0.95,
    });
    world.minions.push(LolSimV3MinionState {
        id: "blue-top-minion-lock-b".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "melee".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: lane_path_position(LolSimV3Team::Blue, "top", 0.88),
        lane_progress: 0.88,
    });

    resolve_intentions_by_systems(&mut world, &[]);
    if let Some(minion_a) = world
        .minions
        .iter_mut()
        .find(|minion| minion.id == "blue-top-minion-lock-a")
    {
        minion_a.pos = lane_path_position(LolSimV3Team::Blue, "top", 0.80);
    }

    let hp_a_before = world
        .minions
        .iter()
        .find(|minion| minion.id == "blue-top-minion-lock-a")
        .map(|minion| minion.hp)
        .expect("minion a exists");
    let hp_b_before = world
        .minions
        .iter()
        .find(|minion| minion.id == "blue-top-minion-lock-b")
        .map(|minion| minion.hp)
        .expect("minion b exists");

    resolve_intentions_by_systems(&mut world, &[]);

    let hp_a_after = world
        .minions
        .iter()
        .find(|minion| minion.id == "blue-top-minion-lock-a")
        .map(|minion| minion.hp)
        .expect("minion a exists");
    let hp_b_after = world
        .minions
        .iter()
        .find(|minion| minion.id == "blue-top-minion-lock-b")
        .map(|minion| minion.hp)
        .expect("minion b exists");

    assert!(hp_a_after < hp_a_before);
    assert_eq!(hp_b_after, hp_b_before);
}

#[test]
fn tower_retargets_recent_diver_when_minion_shield_is_missing() {
    let mut world = create_minimal_world_state("tower-retarget-diver", 1.0);
    let red_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");

    if let Some(blue_top) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-top")
    {
        blue_top.pos = red_top_outer_pos;
    }
    if let Some(blue_jgl) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-jgl")
    {
        blue_jgl.lane = "top".to_string();
        blue_jgl.pos = red_top_outer_pos;
    }
    if let Some(red_top) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "red-top")
    {
        red_top.pos = red_top_outer_pos;
    }

    let blue_top_hp_before = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-top")
        .map(|champion| champion.hp)
        .expect("blue top exists");
    let blue_jgl_hp_before = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-jgl")
        .map(|champion| champion.hp)
        .expect("blue jgl exists");

    let dive = LolSimV3Intention {
        champion_id: "blue-top".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::Fighting,
        kind: LolSimV3IntentKind::TradeWithEnemy,
        priority: 95,
        reason: "test",
    };
    resolve_intentions_by_systems(&mut world, &[dive]);

    let blue_top_hp_after = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-top")
        .map(|champion| champion.hp)
        .expect("blue top exists");
    let blue_jgl_hp_after = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-jgl")
        .map(|champion| champion.hp)
        .expect("blue jgl exists");

    assert!(blue_top_hp_after < blue_top_hp_before);
    assert_eq!(blue_jgl_hp_after, blue_jgl_hp_before);
}

#[test]
fn tower_resets_and_retargets_when_previous_target_becomes_invalid() {
    let mut world = create_minimal_world_state("tower-retarget-reset", 1.0);
    let red_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");

    world.minions.push(LolSimV3MinionState {
        id: "blue-top-minion-reset".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "melee".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: red_top_outer_pos,
        lane_progress: 0.95,
    });
    if let Some(blue_top) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "blue-top")
    {
        blue_top.pos = red_top_outer_pos;
    }

    resolve_intentions_by_systems(&mut world, &[]);
    world.minions.clear();

    let hp_before = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-top")
        .map(|champion| champion.hp)
        .expect("blue top exists");

    resolve_intentions_by_systems(&mut world, &[]);

    let hp_after = world
        .champions
        .iter()
        .find(|champion| champion.id == "blue-top")
        .map(|champion| champion.hp)
        .expect("blue top exists");
    assert!(hp_after < hp_before);
}

#[test]
fn opposing_minion_waves_trade_damage_in_same_lane() {
    let mut world = create_minimal_world_state("minion-vs-minion", 1.0);

    world.minions.push(LolSimV3MinionState {
        id: "blue-top-melee-a".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "melee".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: lane_path_position(LolSimV3Team::Blue, "top", 0.5),
        lane_progress: 0.5,
    });
    world.minions.push(LolSimV3MinionState {
        id: "red-top-melee-a".to_string(),
        team: LolSimV3Team::Red,
        lane: "top".to_string(),
        kind: "melee".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: lane_path_position(LolSimV3Team::Red, "top", 0.5),
        lane_progress: 0.5,
    });

    for _ in 0..3 {
        resolve_intentions_by_systems(&mut world, &[]);
    }

    let blue = world
        .minions
        .iter()
        .find(|minion| minion.id == "blue-top-melee-a")
        .expect("blue minion exists");
    let red = world
        .minions
        .iter()
        .find(|minion| minion.id == "red-top-melee-a")
        .expect("red minion exists");

    assert!(blue.hp < 420.0, "blue minion should take damage");
    assert!(red.hp < 420.0, "red minion should take damage");
}

#[test]
fn stronger_minion_wave_survives_and_pushes_further() {
    let mut world = create_minimal_world_state("minion-wave-strength", 1.0);
    for minion in world
        .minions
        .iter_mut()
        .filter(|minion| minion.lane == "top")
    {
        minion.alive = false;
    }
    world.minions.retain(|minion| minion.alive);

    for idx in 0..3 {
        world.minions.push(LolSimV3MinionState {
            id: format!("blue-top-melee-{idx}"),
            team: LolSimV3Team::Blue,
            lane: "top".to_string(),
            kind: "melee".to_string(),
            alive: true,
            hp: 420.0,
            max_hp: 420.0,
            pos: lane_path_position(LolSimV3Team::Blue, "top", 0.48),
            lane_progress: 0.48,
        });
    }
    world.minions.push(LolSimV3MinionState {
        id: "red-top-melee-weak".to_string(),
        team: LolSimV3Team::Red,
        lane: "top".to_string(),
        kind: "melee".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: lane_path_position(LolSimV3Team::Red, "top", 0.5),
        lane_progress: 0.5,
    });

    for _ in 0..12 {
        resolve_intentions_by_systems(&mut world, &[]);
    }

    let blue_alive = world
        .minions
        .iter()
        .filter(|minion| minion.team == LolSimV3Team::Blue && minion.lane == "top" && minion.alive)
        .count();
    let red_alive = world
        .minions
        .iter()
        .filter(|minion| minion.team == LolSimV3Team::Red && minion.lane == "top" && minion.alive)
        .count();
    let blue_front = world
        .minions
        .iter()
        .filter(|minion| minion.team == LolSimV3Team::Blue && minion.lane == "top" && minion.alive)
        .map(|minion| minion.lane_progress)
        .fold(0.0, f64::max);

    assert!(blue_alive >= 1, "stronger blue wave should keep survivors");
    assert_eq!(red_alive, 0, "weaker red wave should be cleared");
    assert!(
        blue_front > 0.5,
        "surviving blue wave should keep pushing lane"
    );
}

#[test]
fn minion_lane_combat_is_deterministic_for_same_seed() {
    let mut first = create_minimal_world_state("minion-combat-deterministic", 1.0);
    let mut second = create_minimal_world_state("minion-combat-deterministic", 1.0);

    for world in [&mut first, &mut second] {
        world.minions.push(LolSimV3MinionState {
            id: "blue-top-melee-det".to_string(),
            team: LolSimV3Team::Blue,
            lane: "top".to_string(),
            kind: "melee".to_string(),
            alive: true,
            hp: 420.0,
            max_hp: 420.0,
            pos: lane_path_position(LolSimV3Team::Blue, "top", 0.5),
            lane_progress: 0.5,
        });
        world.minions.push(LolSimV3MinionState {
            id: "red-top-melee-det".to_string(),
            team: LolSimV3Team::Red,
            lane: "top".to_string(),
            kind: "melee".to_string(),
            alive: true,
            hp: 420.0,
            max_hp: 420.0,
            pos: lane_path_position(LolSimV3Team::Red, "top", 0.5),
            lane_progress: 0.5,
        });
    }

    for _ in 0..10 {
        resolve_intentions_by_systems(&mut first, &[]);
        resolve_intentions_by_systems(&mut second, &[]);
    }

    assert_eq!(first.tick, second.tick);
    assert_eq!(first.time_sec, second.time_sec);
    assert_eq!(first.minions.len(), second.minions.len());
    for (a, b) in first.minions.iter().zip(second.minions.iter()) {
        assert_eq!(a.id, b.id);
        assert_eq!(a.alive, b.alive);
        assert!((a.hp - b.hp).abs() < 1e-9);
        assert!((a.lane_progress - b.lane_progress).abs() < 1e-9);
    }
}

#[test]
fn profile_slice_directional_combat_and_durability() {
    let mut tank_world = create_minimal_world_state("profile-combat-tank", 1.0);
    let mut squishy_world = create_minimal_world_state("profile-combat-squishy", 1.0);

    if let Some(target) = tank_world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "red-top")
    {
        target.champion_id = "TankOrnn".to_string();
        target.role = "TOP".to_string();
    }
    if let Some(target) = squishy_world
        .champions
        .iter_mut()
        .find(|champion| champion.id == "red-top")
    {
        target.champion_id = "AkaliAssassin".to_string();
        target.role = "MID".to_string();
    }

    let intention = LolSimV3Intention {
        champion_id: "blue-top".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::Fighting,
        kind: LolSimV3IntentKind::TradeWithEnemy,
        priority: 90,
        reason: "profile directional",
    };

    let tank_before = tank_world
        .champions
        .iter()
        .find(|champion| champion.id == "red-top")
        .expect("red top exists")
        .hp;
    let squishy_before = squishy_world
        .champions
        .iter()
        .find(|champion| champion.id == "red-top")
        .expect("red top exists")
        .hp;

    resolve_intentions_by_systems(&mut tank_world, &[intention.clone()]);
    resolve_intentions_by_systems(&mut squishy_world, &[intention]);

    let tank_after = tank_world
        .champions
        .iter()
        .find(|champion| champion.id == "red-top")
        .expect("red top exists")
        .hp;
    let squishy_after = squishy_world
        .champions
        .iter()
        .find(|champion| champion.id == "red-top")
        .expect("red top exists")
        .hp;

    let tank_damage = tank_before - tank_after;
    let squishy_damage = squishy_before - squishy_after;
    assert!(
        squishy_damage > tank_damage,
        "durability should reduce incoming damage"
    );
    assert!(
        squishy_damage < 20.0,
        "combat multipliers must stay bounded"
    );
}

#[test]
fn profile_slice_directional_push_and_objectives() {
    let mut push_world = create_minimal_world_state("profile-push", 1.0);
    for champion in &mut push_world.champions {
        if champion.id == "blue-adc" {
            champion.champion_id = "JinxMarksman".to_string();
            champion.pos = lane_path_position(LolSimV3Team::Blue, "bot", 0.88);
        }
        if champion.id == "blue-sup" {
            champion.champion_id = "TankSion".to_string();
            champion.pos = lane_path_position(LolSimV3Team::Blue, "bot", 0.88);
        }
    }
    push_world.minions.push(LolSimV3MinionState {
        id: "blue-bot-push-profile".to_string(),
        team: LolSimV3Team::Blue,
        lane: "bot".to_string(),
        kind: "melee".to_string(),
        alive: true,
        hp: 420.0,
        max_hp: 420.0,
        pos: lane_path_position(LolSimV3Team::Blue, "bot", 0.90),
        lane_progress: 0.90,
    });

    let tower_before = push_world
        .structures
        .iter()
        .find(|structure| structure.id == "red-bot-outer")
        .expect("red bot outer tower")
        .hp;

    let adc_push = LolSimV3Intention {
        champion_id: "blue-adc".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::Pushing,
        kind: LolSimV3IntentKind::PushTower,
        priority: 90,
        reason: "profile push adc",
    };
    resolve_intentions_by_systems(&mut push_world, &[adc_push]);
    let tower_after_adc = push_world
        .structures
        .iter()
        .find(|structure| structure.id == "red-bot-outer")
        .expect("red bot outer tower")
        .hp;

    let sup_push = LolSimV3Intention {
        champion_id: "blue-sup".to_string(),
        team: LolSimV3Team::Blue,
        state: LolSimV3AgentState::Pushing,
        kind: LolSimV3IntentKind::PushTower,
        priority: 90,
        reason: "profile push sup",
    };
    resolve_intentions_by_systems(&mut push_world, &[sup_push]);
    let tower_after_sup = push_world
        .structures
        .iter()
        .find(|structure| structure.id == "red-bot-outer")
        .expect("red bot outer tower")
        .hp;

    let adc_damage = tower_before - tower_after_adc;
    let sup_damage = tower_after_adc - tower_after_sup;
    assert!(
        adc_damage > sup_damage,
        "adc marksman profile should push harder than support"
    );

    let mut jgl_objective_world = create_minimal_world_state("profile-obj-jgl", 1.0);
    while jgl_objective_world.time_sec < 301.0 {
        resolve_intentions_by_systems(&mut jgl_objective_world, &[]);
    }
    for champion in &mut jgl_objective_world.champions {
        if champion.id == "blue-jgl" {
            champion.champion_id = "TankSejuani".to_string();
            champion.pos = LolSimV3Vec2 { x: 0.67, y: 0.70 };
        }
        if champion.id == "blue-sup" {
            champion.pos = LolSimV3Vec2 { x: 0.67, y: 0.70 };
        }
    }
    let jgl_before = jgl_objective_world.scoreboard.blue.gold;
    resolve_intentions_by_systems(
        &mut jgl_objective_world,
        &[LolSimV3Intention {
            champion_id: "blue-jgl".to_string(),
            team: LolSimV3Team::Blue,
            state: LolSimV3AgentState::ObjectiveSetup,
            kind: LolSimV3IntentKind::TakeDragon,
            priority: 100,
            reason: "profile objective jgl",
        }],
    );
    let jgl_gain = jgl_objective_world.scoreboard.blue.gold - jgl_before;

    let mut adc_objective_world = create_minimal_world_state("profile-obj-adc", 1.0);
    while adc_objective_world.time_sec < 301.0 {
        resolve_intentions_by_systems(&mut adc_objective_world, &[]);
    }
    for champion in &mut adc_objective_world.champions {
        if champion.id == "blue-adc" {
            champion.champion_id = "JinxMarksman".to_string();
            champion.pos = LolSimV3Vec2 { x: 0.67, y: 0.70 };
        }
        if champion.id == "blue-jgl" {
            champion.pos = LolSimV3Vec2 { x: 0.67, y: 0.70 };
        }
    }
    let adc_before = adc_objective_world.scoreboard.blue.gold;
    resolve_intentions_by_systems(
        &mut adc_objective_world,
        &[LolSimV3Intention {
            champion_id: "blue-adc".to_string(),
            team: LolSimV3Team::Blue,
            state: LolSimV3AgentState::ObjectiveSetup,
            kind: LolSimV3IntentKind::TakeDragon,
            priority: 100,
            reason: "profile objective adc",
        }],
    );
    let adc_gain = adc_objective_world.scoreboard.blue.gold - adc_before;

    assert!(
        jgl_gain > adc_gain,
        "jungle profile should contribute more objective value than adc profile"
    );
}

#[test]
fn profile_slice_deterministic_and_regression_safe() {
    let mut first = create_minimal_world_state("profile-deterministic", 1.0);
    let mut second = create_minimal_world_state("profile-deterministic", 1.0);

    let intentions = vec![
        LolSimV3Intention {
            champion_id: "blue-mid".to_string(),
            team: LolSimV3Team::Blue,
            state: LolSimV3AgentState::Fighting,
            kind: LolSimV3IntentKind::TradeWithEnemy,
            priority: 88,
            reason: "deterministic combat",
        },
        LolSimV3Intention {
            champion_id: "blue-adc".to_string(),
            team: LolSimV3Team::Blue,
            state: LolSimV3AgentState::Pushing,
            kind: LolSimV3IntentKind::PushTower,
            priority: 86,
            reason: "deterministic push",
        },
        LolSimV3Intention {
            champion_id: "blue-sup".to_string(),
            team: LolSimV3Team::Blue,
            state: LolSimV3AgentState::Roaming,
            kind: LolSimV3IntentKind::RoamLane,
            priority: 70,
            reason: "deterministic roam",
        },
    ];

    let report_a = resolve_intentions_by_systems(&mut first, &intentions);
    let report_b = resolve_intentions_by_systems(&mut second, &intentions);

    assert_eq!(
        serde_json::to_value(&first.champions).expect("serialize champions"),
        serde_json::to_value(&second.champions).expect("serialize champions")
    );
    assert_eq!(
        serde_json::to_value(&first.structures).expect("serialize structures"),
        serde_json::to_value(&second.structures).expect("serialize structures")
    );
    assert_eq!(report_a.events, report_b.events);

    let red_outer = first
        .structures
        .iter()
        .find(|structure| structure.id == "red-bot-outer")
        .expect("red outer exists");
    assert!(
        red_outer.hp > 0.0,
        "single tick should not create extreme structural swings"
    );
}

#[test]
fn minion_structure_pressure_damages_lane_target_with_advancing_wave() {
    let mut world = create_minimal_world_state("minion-structure-advance", 1.0);
    let red_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("red top outer exists");
    let hp_before = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.hp)
        .expect("red top outer exists");

    world.minions.push(LolSimV3MinionState {
        id: "blue-top-siege-structure-dps".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "siege".to_string(),
        alive: true,
        hp: 600.0,
        max_hp: 600.0,
        pos: red_top_outer_pos,
        lane_progress: 0.95,
    });

    for _ in 0..4 {
        resolve_intentions_by_systems(&mut world, &[]);
    }

    let hp_after = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.hp)
        .expect("red top outer exists");
    assert!(hp_after < hp_before);
}

#[test]
fn minion_structure_pressure_requires_nearby_allied_wave() {
    let mut world = create_minimal_world_state("minion-structure-no-wave", 1.0);
    let hp_before = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.hp)
        .expect("red top outer exists");

    for _ in 0..4 {
        resolve_intentions_by_systems(&mut world, &[]);
    }

    let hp_after = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.hp)
        .expect("red top outer exists");
    assert_eq!(hp_after, hp_before);
}

#[test]
fn minion_structure_pressure_cannot_damage_own_team_structures() {
    let mut world = create_minimal_world_state("minion-own-structure-guard", 1.0);
    let blue_top_outer_pos = world
        .structures
        .iter()
        .find(|structure| structure.id == "blue-top-outer")
        .map(|structure| structure.pos)
        .expect("blue top outer exists");
    let hp_before = world
        .structures
        .iter()
        .find(|structure| structure.id == "blue-top-outer")
        .map(|structure| structure.hp)
        .expect("blue top outer exists");

    world.minions.push(LolSimV3MinionState {
        id: "blue-top-own-structure-check".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "siege".to_string(),
        alive: true,
        hp: 600.0,
        max_hp: 600.0,
        pos: blue_top_outer_pos,
        lane_progress: 0.95,
    });

    for _ in 0..4 {
        resolve_intentions_by_systems(&mut world, &[]);
    }

    let hp_after = world
        .structures
        .iter()
        .find(|structure| structure.id == "blue-top-outer")
        .map(|structure| structure.hp)
        .expect("blue top outer exists");
    assert_eq!(hp_after, hp_before);
}

#[test]
fn minion_structure_pressure_anti_snowball_scales_up_later() {
    let mut early = create_minimal_world_state("minion-anti-snowball-early", 1.0);
    let mut late = create_minimal_world_state("minion-anti-snowball-late", 1.0);

    let early_target_idx = early
        .structures
        .iter()
        .position(|structure| structure.id == "red-top-outer")
        .expect("early target");
    let late_target_idx = late
        .structures
        .iter()
        .position(|structure| structure.id == "red-top-outer")
        .expect("late target");
    let early_target_pos = early.structures[early_target_idx].pos;
    let late_target_pos = late.structures[late_target_idx].pos;

    early.time_sec = 120.0;
    late.time_sec = 1200.0;

    early.minions.push(LolSimV3MinionState {
        id: "blue-top-siege-early".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "siege".to_string(),
        alive: true,
        hp: 600.0,
        max_hp: 600.0,
        pos: early_target_pos,
        lane_progress: 0.95,
    });
    late.minions.push(LolSimV3MinionState {
        id: "blue-top-siege-late".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "siege".to_string(),
        alive: true,
        hp: 600.0,
        max_hp: 600.0,
        pos: late_target_pos,
        lane_progress: 0.95,
    });

    let early_before = early.structures[early_target_idx].hp;
    let late_before = late.structures[late_target_idx].hp;
    for _ in 0..2 {
        resolve_intentions_by_systems(&mut early, &[]);
        resolve_intentions_by_systems(&mut late, &[]);
    }
    let early_damage = early_before - early.structures[early_target_idx].hp;
    let late_damage = late_before - late.structures[late_target_idx].hp;
    assert!(early_damage > 0.0);
    assert!(
        late_damage > early_damage,
        "late damage should exceed early damage under same setup"
    );
}

#[test]
fn minion_structure_pressure_anti_snowball_is_deterministic() {
    let mut first = create_minimal_world_state("minion-anti-snowball-deterministic", 1.0);
    let mut second = create_minimal_world_state("minion-anti-snowball-deterministic", 1.0);

    first.time_sec = 900.0;
    second.time_sec = 900.0;

    let first_target_pos = first
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("first target");
    let second_target_pos = second
        .structures
        .iter()
        .find(|structure| structure.id == "red-top-outer")
        .map(|structure| structure.pos)
        .expect("second target");

    first.minions.push(LolSimV3MinionState {
        id: "blue-top-siege-deterministic".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "siege".to_string(),
        alive: true,
        hp: 600.0,
        max_hp: 600.0,
        pos: first_target_pos,
        lane_progress: 0.95,
    });
    second.minions.push(LolSimV3MinionState {
        id: "blue-top-siege-deterministic".to_string(),
        team: LolSimV3Team::Blue,
        lane: "top".to_string(),
        kind: "siege".to_string(),
        alive: true,
        hp: 600.0,
        max_hp: 600.0,
        pos: second_target_pos,
        lane_progress: 0.95,
    });

    let report_a = resolve_intentions_by_systems(&mut first, &[]);
    let report_b = resolve_intentions_by_systems(&mut second, &[]);

    assert_eq!(
        serde_json::to_value(&first.structures).expect("serialize first structures"),
        serde_json::to_value(&second.structures).expect("serialize second structures")
    );
    assert_eq!(report_a.events, report_b.events);
}
