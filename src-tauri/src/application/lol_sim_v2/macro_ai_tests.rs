use std::collections::HashMap;

use super::macro_ai::{jungler_macro_jungle_priority_for_team, pick_macro_objective_pos};
use super::test_helpers::{empty_neutral, test_champion, test_neutral_timer, test_runtime};
use super::*;

#[test]
fn jgl_disengage_prefers_jungle_camp_fallback() {
    let jungler = test_champion("jgl-blue", "blue", "JGL", "bot", Vec2 { x: 0.46, y: 0.61 });
    let mut entities = HashMap::new();
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
    let mut runtime = test_runtime(vec![jungler], vec![], vec![], neutral);

    issue_lane_disengage(&mut runtime, 0, Vec2 { x: 0.52, y: 0.65 });

    let target = runtime.champions[0].target_path.last().copied();
    assert!(target.is_some());
    let p = target.unwrap_or(Vec2 { x: 0.0, y: 0.0 });
    assert!(dist(p, Vec2 { x: 0.16, y: 0.43 }) <= 0.02);
}

#[test]
fn red_jungler_macro_prefers_own_side_buffs_first() {
    let red_jgl = test_champion("jgl-red", "red", "JGL", "bot", Vec2 { x: 0.75, y: 0.55 });
    let blue_jgl = test_champion("jgl-blue", "blue", "JGL", "bot", Vec2 { x: 0.25, y: 0.46 });

    let mut entities = HashMap::new();
    entities.insert(
        "blue-buff-blue".to_string(),
        test_neutral_timer("blue-buff-blue", Vec2 { x: 0.25, y: 0.46 }, true),
    );
    entities.insert(
        "blue-buff-red".to_string(),
        test_neutral_timer("blue-buff-red", Vec2 { x: 0.48, y: 0.26 }, true),
    );

    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };

    let default_tactics = RuntimeTeamTactics::default();
    let champions = vec![red_jgl.clone(), blue_jgl.clone()];
    let red_pick =
        pick_macro_objective_pos(&red_jgl, &champions, &neutral, 120.0, &default_tactics);
    let blue_pick =
        pick_macro_objective_pos(&blue_jgl, &champions, &neutral, 120.0, &default_tactics);

    assert_eq!(red_pick.map(|p| (p.x, p.y)), Some((0.48, 0.26)));
    assert_eq!(blue_pick.map(|p| (p.x, p.y)), Some((0.25, 0.46)));
}

#[test]
fn jungle_pathing_bot_to_top_invades_enemy_top_side_first_for_both_teams() {
    let blue_order = jungler_macro_jungle_priority_for_team("blue", "BotToTop");
    let red_order = jungler_macro_jungle_priority_for_team("red", "BotToTop");

    assert_eq!(blue_order[8], "blue-buff-red");
    assert_eq!(red_order[8], "blue-buff-blue");
}

#[test]
fn jungle_pathing_top_to_bot_invades_enemy_bot_side_first_for_both_teams() {
    let blue_order = jungler_macro_jungle_priority_for_team("blue", "TopToBot");
    let red_order = jungler_macro_jungle_priority_for_team("red", "TopToBot");

    assert_eq!(blue_order[8], "red-buff-red");
    assert_eq!(red_order[8], "red-buff-blue");
}

#[test]
fn jungle_disengage_fallback_honors_pathing_start_side_for_blue_and_red() {
    let blue_bot_to_top = jungle_disengage_fallback_order_for_team("blue", "BotToTop");
    let blue_top_to_bot = jungle_disengage_fallback_order_for_team("blue", "TopToBot");
    let red_bot_to_top = jungle_disengage_fallback_order_for_team("red", "BotToTop");
    let red_top_to_bot = jungle_disengage_fallback_order_for_team("red", "TopToBot");

    assert_eq!(blue_bot_to_top[0], "raptors-blue");
    assert_eq!(blue_top_to_bot[0], "gromp-blue");
    assert_eq!(red_bot_to_top[0], "raptors-red");
    assert_eq!(red_top_to_bot[0], "gromp-red");
}

#[test]
fn jungler_macro_prioritizes_viable_dragon_over_camp() {
    let jungler = test_champion(
        "blue-jgl",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.48, y: 0.56 },
    );
    let mid = test_champion("blue-mid", "blue", "MID", "mid", Vec2 { x: 0.54, y: 0.55 });
    let adc = test_champion("blue-adc", "blue", "ADC", "bot", Vec2 { x: 0.62, y: 0.67 });
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
    let mut runtime = test_runtime(vec![jungler, mid, adc], vec![], vec![], neutral);
    runtime.time_sec = 360.0;

    move_champions(&mut runtime, 0.1);

    let jungler = &runtime.champions[0];
    assert_eq!(jungler.state, "objective");
    assert!(jungler.debug_ai_decision.contains("macro:objective_setup"));
    let target = jungler
        .target_path
        .last()
        .copied()
        .unwrap_or(Vec2 { x: 0.0, y: 0.0 });
    assert!(dist(target, dragon_pos) <= 0.002);
}

#[test]
fn jungler_macro_takes_camp_when_objective_is_not_viable() {
    let mut jungler = test_champion(
        "blue-jgl",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.48, y: 0.56 },
    );
    jungler.hp = 40.0;
    let camp_pos = Vec2 { x: 0.16, y: 0.43 };
    let mut entities = HashMap::new();
    entities.insert(
        "baron".to_string(),
        test_neutral_timer("baron", Vec2 { x: 0.42, y: 0.42 }, true),
    );
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
    let mut runtime = test_runtime(vec![jungler], vec![], vec![], neutral);
    runtime.time_sec = 360.0;

    move_champions(&mut runtime, 0.1);

    let jungler = &runtime.champions[0];
    assert_eq!(jungler.state, "jungle");
    assert!(jungler.debug_ai_decision.contains("macro:jungle_path"));
    let target = jungler
        .target_path
        .last()
        .copied()
        .unwrap_or(Vec2 { x: 0.0, y: 0.0 });
    assert!(dist(target, camp_pos) <= 0.002);
}

#[test]
fn support_macro_does_not_roam_when_adc_low_hp() {
    let support = test_champion("blue-sup", "blue", "SUP", "bot", Vec2 { x: 0.52, y: 0.70 });
    let mut adc = test_champion("blue-adc", "blue", "ADC", "bot", Vec2 { x: 0.62, y: 0.72 });
    adc.hp = 30.0;
    let mid = test_champion("blue-mid", "blue", "MID", "mid", Vec2 { x: 0.46, y: 0.50 });
    let mut runtime = test_runtime(
        vec![support, adc, mid],
        Vec::new(),
        Vec::new(),
        empty_neutral(),
    );
    runtime.time_sec = SUPPORT_OPEN_ROAM_AT_SEC + 20.0;

    move_champions(&mut runtime, 0.1);

    let support = &runtime.champions[0];
    assert_eq!(support.state, "lane");
    assert!(!support.debug_ai_decision.contains("macro:support_roam"));
}

#[test]
fn support_macro_roams_to_near_objective_when_adc_safe() {
    let support = test_champion("blue-sup", "blue", "SUP", "bot", Vec2 { x: 0.52, y: 0.70 });
    let adc = test_champion("blue-adc", "blue", "ADC", "bot", Vec2 { x: 0.62, y: 0.72 });
    let jungler = test_champion(
        "blue-jgl",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.54, y: 0.58 },
    );
    let dragon_pos = Vec2 { x: 0.58, y: 0.58 };
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
    let mut runtime = test_runtime(vec![support, adc, jungler], Vec::new(), Vec::new(), neutral);
    runtime.time_sec = SUPPORT_OPEN_ROAM_AT_SEC + 20.0;

    move_champions(&mut runtime, 0.1);

    let support = &runtime.champions[0];
    assert_eq!(support.state, "objective");
    assert!(support.debug_ai_decision.contains("macro:support_roam"));
    let target = support
        .target_path
        .last()
        .copied()
        .unwrap_or(Vec2 { x: 0.0, y: 0.0 });
    assert!(dist(target, dragon_pos) <= 0.002);
}

#[test]
fn carry_macro_avoids_solo_river_before_critical_objective() {
    let adc_pos = Vec2 { x: 0.58, y: 0.58 };
    let adc = test_champion("blue-adc", "blue", "ADC", "bot", adc_pos);
    let mut dragon = test_neutral_timer("dragon", adc_pos, false);
    dragon.next_spawn_at = Some(390.0);
    let mut entities = HashMap::new();
    entities.insert("dragon".to_string(), dragon);
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };
    let mut runtime = test_runtime(vec![adc], Vec::new(), Vec::new(), neutral);
    runtime.time_sec = 360.0;

    move_champions(&mut runtime, 0.1);

    let target = runtime.champions[0]
        .target_path
        .last()
        .copied()
        .unwrap_or(adc_pos);
    assert!(dist(target, adc_pos) > 0.04);
    assert!(target.x < adc_pos.x || target.y > adc_pos.y);
}

#[test]
fn adc_alone_in_river_without_vision_is_anchored_to_lane() {
    let adc_pos = Vec2 { x: 0.58, y: 0.58 };
    let mut adc = test_champion("blue-adc", "blue", "ADC", "bot", adc_pos);
    adc.state = "objective".to_string();
    let mut runtime = test_runtime(vec![adc], Vec::new(), Vec::new(), empty_neutral());
    runtime.time_sec = 360.0;

    move_champions(&mut runtime, 0.1);

    let adc = &runtime.champions[0];
    assert_eq!(adc.state, "lane");
    assert!(adc.debug_ai_decision.contains("guard:no_lonely_river"));
    let target = adc.target_path.last().copied().unwrap_or(adc_pos);
    assert!(dist(target, adc_pos) > 0.04);
}

#[test]
fn recall_is_safe_against_far_irrelevant_enemy() {
    let recalling = test_champion("blue-adc", "blue", "ADC", "bot", Vec2 { x: 0.62, y: 0.72 });
    let far_enemy = test_champion("red-top", "red", "TOP", "top", Vec2 { x: 0.25, y: 0.25 });

    assert!(super::macro_ai::should_recall_in_place(
        &recalling,
        &[recalling.clone(), far_enemy]
    ));
}
