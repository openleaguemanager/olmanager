use std::collections::HashMap;

use super::decision_layer::{classify_decision_intent, AgentState, IntentKind};
use super::test_helpers::{
    empty_neutral, test_champion, test_minion, test_neutral_timer, test_runtime, test_structure,
};
use super::*;

#[test]
fn decision_layer_classifies_lane_farming_intent() {
    let champion = test_champion("blue-mid", "blue", "MID", "mid", Vec2 { x: 0.4, y: 0.6 });

    let intent = classify_decision_intent(
        &champion,
        120.0,
        None,
        &RuntimeTeamTactics::default(),
        &RuntimeTeamBuffState::default(),
    );

    assert_eq!(intent.state, AgentState::Laning);
    assert_eq!(intent.intent, IntentKind::FarmLane);
    assert_eq!(intent.team, "blue");
    assert_eq!(intent.lane, "mid");
}

#[test]
fn decision_layer_classifies_recall_and_dead_as_high_priority() {
    let mut recalling = test_champion("blue-adc", "blue", "ADC", "bot", Vec2 { x: 0.5, y: 0.5 });
    recalling.state = "recall".to_string();
    let recall_intent = classify_decision_intent(
        &recalling,
        180.0,
        None,
        &RuntimeTeamTactics::default(),
        &RuntimeTeamBuffState::default(),
    );

    let mut dead = test_champion("red-top", "red", "TOP", "top", Vec2 { x: 0.7, y: 0.3 });
    dead.alive = false;
    let dead_intent = classify_decision_intent(
        &dead,
        180.0,
        None,
        &RuntimeTeamTactics::default(),
        &RuntimeTeamBuffState::default(),
    );

    assert_eq!(recall_intent.state, AgentState::Recalling);
    assert_eq!(recall_intent.intent, IntentKind::Recall);
    assert_eq!(dead_intent.state, AgentState::Dead);
    assert_eq!(dead_intent.intent, IntentKind::WaitRespawn);
    assert!(dead_intent.priority >= recall_intent.priority);
}

#[test]
fn move_champions_records_decision_layer_debug_intent() {
    let mut runtime = test_runtime(
        vec![
            test_champion("blue-mid", "blue", "MID", "mid", Vec2 { x: 0.35, y: 0.65 }),
            test_champion("red-mid", "red", "MID", "mid", Vec2 { x: 0.65, y: 0.35 }),
        ],
        Vec::new(),
        Vec::new(),
        empty_neutral(),
    );
    runtime.time_sec = 120.0;

    move_champions(&mut runtime, 0.1);

    let debug = &runtime.champions[0].debug_ai_decision;
    assert!(debug.contains("agent:"));
    assert!(debug.contains("intent:"));
}

#[test]
fn jungler_with_live_dragon_records_take_dragon_and_paths_to_it() {
    let jungler = test_champion(
        "blue-jgl",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.30, y: 0.55 },
    );
    let dragon_pos = Vec2 { x: 0.58, y: 0.58 };
    let mut entities = HashMap::new();
    entities.insert(
        "dragon".to_string(),
        test_neutral_timer("dragon", dragon_pos, true),
    );
    entities.insert(
        "gromp-blue".to_string(),
        test_neutral_timer("gromp-blue", Vec2 { x: 0.16, y: 0.43 }, true),
    );
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };
    let mut runtime = test_runtime(vec![jungler], Vec::new(), Vec::new(), neutral);
    runtime.time_sec = 360.0;

    move_champions(&mut runtime, 0.1);

    let champion = &runtime.champions[0];
    assert!(champion.debug_ai_decision.contains("intent:take_dragon"));
    assert_eq!(champion.state, "objective");
    let target = champion
        .target_path
        .last()
        .copied()
        .unwrap_or(Vec2 { x: 0.0, y: 0.0 });
    assert!(dist(target, dragon_pos) <= 0.002);
}

#[test]
fn objective_assist_intent_reinforces_contested_objective_path() {
    let adc = test_champion("blue-adc", "blue", "ADC", "bot", Vec2 { x: 0.60, y: 0.73 });
    let jungler = test_champion(
        "blue-jgl",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.66, y: 0.70 },
    );
    let enemy = test_champion("red-mid", "red", "MID", "mid", Vec2 { x: 0.69, y: 0.70 });
    let dragon_pos = Vec2 { x: 0.67, y: 0.70 };

    let mut entities = HashMap::new();
    let mut dragon = test_neutral_timer("dragon", dragon_pos, true);
    dragon.hp = dragon.max_hp * 0.80;
    entities.insert("dragon".to_string(), dragon);
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };
    let mut runtime = test_runtime(vec![adc, jungler, enemy], Vec::new(), Vec::new(), neutral);
    runtime.time_sec = 360.0;

    move_champions(&mut runtime, 0.1);

    let adc = &runtime.champions[0];
    assert_eq!(adc.state, "objective");
    assert!(adc.debug_ai_decision.contains("intent:rotate_to_objective"));
    let target = adc
        .target_path
        .last()
        .copied()
        .unwrap_or(Vec2 { x: 0.0, y: 0.0 });
    assert!(dist(target, dragon_pos) <= 0.002);
}

#[test]
fn low_hp_objective_assist_intent_does_not_override_recall() {
    let mut adc = test_champion("blue-adc", "blue", "ADC", "bot", Vec2 { x: 0.60, y: 0.73 });
    adc.hp = 12.0;
    let jungler = test_champion(
        "blue-jgl",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.66, y: 0.70 },
    );
    let enemy = test_champion("red-mid", "red", "MID", "mid", Vec2 { x: 0.69, y: 0.70 });
    let dragon_pos = Vec2 { x: 0.67, y: 0.70 };

    let mut entities = HashMap::new();
    entities.insert(
        "dragon".to_string(),
        test_neutral_timer("dragon", dragon_pos, true),
    );
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };
    let mut runtime = test_runtime(vec![adc, jungler, enemy], Vec::new(), Vec::new(), neutral);
    runtime.time_sec = 360.0;

    move_champions(&mut runtime, 0.1);

    assert_eq!(runtime.champions[0].state, "recall");
    assert!(!runtime.champions[0]
        .debug_ai_decision
        .contains("intent:rotate_to_objective"));
}

#[test]
fn purchase_recall_waits_when_objective_assist_intent_is_active() {
    let mut adc = test_champion("blue-adc", "blue", "ADC", "bot", Vec2 { x: 0.60, y: 0.73 });
    adc.has_left_base_once = true;
    adc.gold = 1200;
    let jungler = test_champion(
        "blue-jgl",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.66, y: 0.70 },
    );
    let enemy = test_champion("red-mid", "red", "MID", "mid", Vec2 { x: 0.69, y: 0.70 });
    let dragon_pos = Vec2 { x: 0.67, y: 0.70 };

    let mut entities = HashMap::new();
    let mut dragon = test_neutral_timer("dragon", dragon_pos, true);
    dragon.hp = dragon.max_hp * 0.80;
    entities.insert("dragon".to_string(), dragon);
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };
    let mut runtime = test_runtime(vec![adc, jungler, enemy], Vec::new(), Vec::new(), neutral);
    runtime.time_sec = 360.0;

    move_champions(&mut runtime, 0.1);

    let adc = &runtime.champions[0];
    assert_eq!(adc.state, "objective");
    assert!(adc.debug_ai_decision.contains("intent:rotate_to_objective"));
    assert!(!adc.debug_ai_decision.contains("intent:recall"));
}

#[test]
fn purchase_recall_starts_when_farm_lane_intent_is_active() {
    let mut adc = test_champion("blue-adc", "blue", "ADC", "bot", Vec2 { x: 0.60, y: 0.73 });
    adc.has_left_base_once = true;
    adc.gold = 1200;
    let mut runtime = test_runtime(vec![adc], Vec::new(), Vec::new(), empty_neutral());
    runtime.time_sec = 360.0;

    move_champions(&mut runtime, 0.1);

    let adc = &runtime.champions[0];
    assert_eq!(adc.state, "recall");
    assert!(adc.debug_ai_decision.contains("intent:recall"));
    assert!(adc
        .debug_ai_decision
        .contains("reason:recall_purchase_economy"));
}

#[test]
fn jungler_with_available_camp_records_clear_jungle_and_keeps_camp_path() {
    let jungler = test_champion(
        "blue-jgl",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.30, y: 0.55 },
    );
    let camp_pos = Vec2 { x: 0.16, y: 0.43 };
    let mut entities = HashMap::new();
    entities.insert(
        "gromp-blue".to_string(),
        test_neutral_timer("gromp-blue", camp_pos, true),
    );
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };
    let mut runtime = test_runtime(vec![jungler], Vec::new(), Vec::new(), neutral);
    runtime.time_sec = 180.0;

    move_champions(&mut runtime, 0.1);

    let champion = &runtime.champions[0];
    assert!(champion.debug_ai_decision.contains("intent:clear_jungle"));
    assert_eq!(champion.state, "jungle");
    let target = champion
        .target_path
        .last()
        .copied()
        .unwrap_or(Vec2 { x: 0.0, y: 0.0 });
    assert!(dist(target, camp_pos) <= 0.002);
}

#[test]
fn decision_intent_target_does_not_override_recall_or_dead_jungler() {
    let sentinel = Vec2 { x: 0.42, y: 0.42 };
    let dragon_pos = Vec2 { x: 0.58, y: 0.58 };
    let mut recalling = test_champion(
        "blue-jgl-recall",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.30, y: 0.55 },
    );
    recalling.state = "recall".to_string();
    recalling.recall_channel_until = 500.0;
    recalling.target_path = vec![sentinel];

    let mut dead = test_champion(
        "red-jgl-dead",
        "red",
        "JGL",
        "jungle",
        Vec2 { x: 0.70, y: 0.45 },
    );
    dead.alive = false;
    dead.respawn_at = 500.0;
    dead.target_path = vec![sentinel];

    let mut entities = HashMap::new();
    entities.insert(
        "dragon".to_string(),
        test_neutral_timer("dragon", dragon_pos, true),
    );
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };
    let mut runtime = test_runtime(vec![recalling, dead], Vec::new(), Vec::new(), neutral);
    runtime.time_sec = 360.0;

    move_champions(&mut runtime, 0.1);

    assert_eq!(runtime.champions[0].state, "recall");
    assert_eq!(
        runtime.champions[0].target_path.last().map(|p| (p.x, p.y)),
        Some((sentinel.x, sentinel.y))
    );
    assert!(!runtime.champions[1].alive);
    assert_eq!(
        runtime.champions[1].target_path.last().map(|p| (p.x, p.y)),
        Some((sentinel.x, sentinel.y))
    );
}

#[test]
fn support_roam_intent_preserves_v2_roam_path_and_debug() {
    let support = test_champion("blue-sup", "blue", "SUP", "bot", Vec2 { x: 0.52, y: 0.70 });
    let mut mid = test_champion("blue-mid", "blue", "MID", "mid", Vec2 { x: 0.46, y: 0.50 });
    mid.hp = 35.0;
    let adc = test_champion("blue-adc", "blue", "ADC", "bot", Vec2 { x: 0.72, y: 0.80 });
    let mut runtime = test_runtime(
        vec![support, mid.clone(), adc],
        Vec::new(),
        Vec::new(),
        empty_neutral(),
    );
    runtime.time_sec = SUPPORT_OPEN_ROAM_AT_SEC + 20.0;

    move_champions(&mut runtime, 0.1);

    let support = &runtime.champions[0];
    assert_eq!(support.state, "objective");
    assert!(support.debug_ai_decision.contains("intent:roam_lane"));
    assert_eq!(support.support_last_roam_role, "MID");
    assert!(support.support_roam_cd_until > runtime.time_sec);
    let target = support
        .target_path
        .last()
        .copied()
        .unwrap_or(Vec2 { x: 0.0, y: 0.0 });
    assert!(dist(target, mid.pos) <= 0.08);
}

#[test]
fn decision_intent_target_does_not_override_recall_or_dead_support() {
    let sentinel = Vec2 { x: 0.42, y: 0.42 };
    let mut recalling = test_champion(
        "blue-sup-recall",
        "blue",
        "SUP",
        "bot",
        Vec2 { x: 0.52, y: 0.70 },
    );
    recalling.state = "recall".to_string();
    recalling.recall_channel_until = 500.0;
    recalling.target_path = vec![sentinel];

    let mut dead = test_champion(
        "red-sup-dead",
        "red",
        "SUP",
        "bot",
        Vec2 { x: 0.70, y: 0.45 },
    );
    dead.alive = false;
    dead.respawn_at = 500.0;
    dead.target_path = vec![sentinel];

    let recall_intent = classify_decision_intent(
        &recalling,
        SUPPORT_OPEN_ROAM_AT_SEC + 20.0,
        None,
        &RuntimeTeamTactics::default(),
        &RuntimeTeamBuffState::default(),
    );
    apply_decision_intent_target(
        &mut recalling,
        &recall_intent,
        &[],
        None,
        &RuntimeTeamTactics::default(),
        SUPPORT_OPEN_ROAM_AT_SEC + 20.0,
    );

    let dead_intent = classify_decision_intent(
        &dead,
        SUPPORT_OPEN_ROAM_AT_SEC + 20.0,
        None,
        &RuntimeTeamTactics::default(),
        &RuntimeTeamBuffState::default(),
    );
    apply_decision_intent_target(
        &mut dead,
        &dead_intent,
        &[],
        None,
        &RuntimeTeamTactics::default(),
        SUPPORT_OPEN_ROAM_AT_SEC + 20.0,
    );

    assert_eq!(recalling.state, "recall");
    assert_eq!(
        recalling.target_path.last().map(|p| (p.x, p.y)),
        Some((sentinel.x, sentinel.y))
    );
    assert!(!dead.alive);
    assert_eq!(
        dead.target_path.last().map(|p| (p.x, p.y)),
        Some((sentinel.x, sentinel.y))
    );
}

#[test]
fn trade_guardrail_blocks_defensive_intent_but_keeps_normal_trade_open() {
    let mut low_hp_laner =
        test_champion("blue-low", "blue", "MID", "mid", Vec2 { x: 0.50, y: 0.50 });
    low_hp_laner.hp = 19.0;
    let mut enemy = test_champion("red-mid", "red", "MID", "mid", Vec2 { x: 0.53, y: 0.50 });
    enemy.hp = 85.0;
    let allied_minion = test_minion("blue-m1", "blue", "mid", Vec2 { x: 0.525, y: 0.50 });
    let enemy_minion = test_minion("red-m1", "red", "mid", Vec2 { x: 0.535, y: 0.50 });
    let champions = vec![low_hp_laner.clone(), enemy.clone()];
    let minions = vec![allied_minion.clone(), enemy_minion.clone()];
    let policy = SimulatorPolicyConfig::default();

    let blocked = evaluate_open_trade_window(
        &low_hp_laner,
        &enemy,
        180.0,
        &champions,
        &minions,
        &[],
        &HashMap::new(),
        SimulatorAiMode::Rules,
        &policy,
    );
    assert!(!blocked.decision);
    assert_eq!(blocked.confidence, 1.0);

    let healthy_laner = test_champion(
        "blue-healthy",
        "blue",
        "MID",
        "mid",
        Vec2 { x: 0.50, y: 0.50 },
    );
    assert!(decision_intent_trade_guardrail(&healthy_laner, 180.0, &policy).is_none());
}

#[test]
fn tower_chase_guard_is_observable_in_decision_debug() {
    let mut blue = test_champion(
        "top-blue",
        "blue",
        "TOP",
        "top",
        Vec2 { x: 0.276, y: 0.075 },
    );
    blue.state = "fight".to_string();
    let red = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.274, y: 0.073 });
    let tower = test_structure("red-top-outer", "red", "top", Vec2 { x: 0.275, y: 0.072 });
    let mut runtime = test_runtime(vec![blue, red], Vec::new(), vec![tower], empty_neutral());

    move_champions(&mut runtime, 0.1);

    assert!(runtime.champions[0]
        .debug_ai_decision
        .contains("guard:no_tower_chase"));
}

#[test]
fn bad_dive_guard_is_observable_in_decision_debug() {
    let mut blue = test_champion(
        "top-blue",
        "blue",
        "TOP",
        "top",
        Vec2 { x: 0.276, y: 0.075 },
    );
    blue.hp = 42.0;
    blue.state = "fight".to_string();
    let red = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.274, y: 0.073 });
    let tower = test_structure("red-top-outer", "red", "top", Vec2 { x: 0.275, y: 0.072 });
    let mut runtime = test_runtime(vec![blue, red], Vec::new(), vec![tower], empty_neutral());

    move_champions(&mut runtime, 0.1);

    assert!(runtime.champions[0]
        .debug_ai_decision
        .contains("guard:no_bad_dive"));
}
