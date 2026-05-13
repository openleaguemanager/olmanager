use std::collections::HashMap;

use super::combat::{
    decision_intent_objective_chase_guardrail_allows, fight_debug_for_trade, pick_combat_target,
};
use super::test_helpers::{
    empty_neutral, test_champion, test_minion, test_neutral_timer, test_runtime, test_structure,
};
use super::*;

fn decode_neutral_for_tests(runtime: &RuntimeState) -> NeutralTimersRuntime {
    decode_neutral_timers_state(&runtime.neutral_timers)
        .unwrap_or_else(neutral_timers_default_runtime_state)
}

fn batch_ultimate_champion(id: &str, champion_id: &str, pos: Vec2) -> ChampionRuntime {
    let mut champion = test_champion(id, "blue", "MID", "mid", pos);
    champion.champion_id = champion_id.to_string();
    champion.level = 6;
    champion.ultimate = Some(RuntimeUltimateSlot {
        archetype: "burst".to_string(),
        icon: String::new(),
        cd_until: 0.0,
        ..Default::default()
    });
    champion
}

fn last_ultimate_metadata(runtime: &RuntimeState) -> serde_json::Value {
    runtime
        .events
        .iter()
        .rev()
        .find_map(|event| event.metadata.clone())
        .unwrap_or_else(|| panic!("missing ultimate metadata"))
}

#[test]
fn batch_one_ultimates_emit_real_metadata_not_generic_damage_only() {
    for champion_id in [
        "Aatrox", "Ahri", "Akali", "Akshan", "Alistar", "Ambessa", "Amumu", "Anivia",
        "Annie", "Aphelios",
    ] {
        let caster = batch_ultimate_champion("mid-blue", champion_id, Vec2 { x: 0.50, y: 0.50 });
        let target = test_champion("mid-red", "red", "MID", "mid", Vec2 { x: 0.56, y: 0.50 });
        let mut runtime = test_runtime(vec![caster, target], vec![], vec![], empty_neutral());

        resolve_champion_combat(&mut runtime);

        let metadata = last_ultimate_metadata(&runtime);
        assert_eq!(metadata["event"], "champion_ultimate_cast", "{champion_id}");
        assert_ne!(metadata["ultimateIdentity"]["signatureId"], serde_json::Value::Null, "{champion_id}");
        assert!(
            metadata["bespokeKind"].is_string()
                || metadata["sequenceKind"].is_string()
                || metadata["persistent"].as_bool().unwrap_or(false)
                || metadata["destinationPos"].is_object(),
            "{champion_id} fell back to generic-only metadata: {metadata}"
        );
    }
}

#[test]
fn amumu_ultimate_affects_multiple_nearby_targets() {
    let caster = batch_ultimate_champion("sup-blue", "Amumu", Vec2 { x: 0.50, y: 0.50 });
    let enemy_1 = test_champion("red-1", "red", "MID", "mid", Vec2 { x: 0.55, y: 0.50 });
    let enemy_2 = test_champion("red-2", "red", "JGL", "mid", Vec2 { x: 0.51, y: 0.56 });
    let mut runtime = test_runtime(vec![caster, enemy_1, enemy_2], vec![], vec![], empty_neutral());

    resolve_champion_combat(&mut runtime);

    assert!(runtime.champions[1].hp < runtime.champions[1].max_hp);
    assert!(runtime.champions[2].hp < runtime.champions[2].max_hp);
    let metadata = last_ultimate_metadata(&runtime);
    assert_eq!(metadata["bespokeKind"], "aoe_bandage_lockdown");
    assert!(metadata["affectedTargetIds"].as_array().unwrap().len() >= 2);
}

#[test]
fn anivia_ultimate_persists_and_ticks_zone_damage() {
    let caster = batch_ultimate_champion("mid-blue", "Anivia", Vec2 { x: 0.50, y: 0.50 });
    let enemy_1 = test_champion("red-1", "red", "MID", "mid", Vec2 { x: 0.56, y: 0.50 });
    let enemy_2 = test_champion("red-2", "red", "JGL", "mid", Vec2 { x: 0.58, y: 0.52 });
    let mut runtime = test_runtime(vec![caster, enemy_1, enemy_2], vec![], vec![], empty_neutral());

    resolve_champion_combat(&mut runtime);

    assert!(runtime.champions[1].hp <= runtime.champions[1].max_hp - 15.0);
    let metadata = last_ultimate_metadata(&runtime);
    assert_eq!(metadata["shape"], "zone");
    assert_eq!(metadata["persistent"], true);
    assert!(metadata["pulseCount"].as_u64().unwrap_or(0) >= 8);
}

#[test]
fn annie_ultimate_spawns_tibbers_and_impacts_area() {
    let caster = batch_ultimate_champion("mid-blue", "Annie", Vec2 { x: 0.50, y: 0.50 });
    let enemy = test_champion("red-1", "red", "MID", "mid", Vec2 { x: 0.515, y: 0.51 });
    let mut runtime = test_runtime(vec![caster, enemy], vec![], vec![], empty_neutral());

    resolve_champion_combat(&mut runtime);

    assert!(runtime.minions.iter().any(|minion| minion.summon_kind.as_deref() == Some("tibbers")));
    assert!(runtime.champions[1].hp < runtime.champions[1].max_hp);
    assert_eq!(last_ultimate_metadata(&runtime)["bespokeKind"], "tibbers_drop_burst_pet");
}

#[test]
fn batch_one_lock_channel_and_dash_metadata_is_explicit() {
    for (champion_id, expected) in [
        ("Akshan", "lock_on_multi_shot_channel"),
        ("Ahri", "recast_dash_charges"),
        ("Akali", "execute_recast_dash"),
    ] {
        let caster = batch_ultimate_champion("mid-blue", champion_id, Vec2 { x: 0.50, y: 0.50 });
        let mut target = test_champion("mid-red", "red", "MID", "mid", Vec2 { x: 0.56, y: 0.50 });
        target.hp = 35.0;
        let mut runtime = test_runtime(vec![caster, target], vec![], vec![], empty_neutral());
        resolve_champion_combat(&mut runtime);
        let metadata = last_ultimate_metadata(&runtime);
        assert_eq!(metadata["sequenceKind"], expected, "{champion_id}");
        assert!(metadata["targetId"].is_string(), "{champion_id}");
    }

    let caster = batch_ultimate_champion("mid-blue", "Ambessa", Vec2 { x: 0.50, y: 0.50 });
    let target = test_champion("mid-red", "red", "MID", "mid", Vec2 { x: 0.56, y: 0.50 });
    let mut runtime = test_runtime(vec![caster, target], vec![], vec![], empty_neutral());
    resolve_champion_combat(&mut runtime);
    let metadata = last_ultimate_metadata(&runtime);
    assert_eq!(metadata["bespokeKind"], "noxian_execution_dash");
    assert!(metadata["destinationPos"].is_object());
}

#[test]
fn heal_spell_casts_when_self_is_low_hp() {
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities: HashMap::new(),
        extra: HashMap::new(),
    };

    let mut champion = test_champion("adc-blue", "blue", "ADC", "bot", Vec2 { x: 0.50, y: 0.50 });
    champion.hp = 20.0;
    champion.summoner_spells = vec![RuntimeSummonerSpellSlot {
        key: "Heal".to_string(),
        cd_until: 0.0,
    }];

    let mut runtime = test_runtime(vec![champion], vec![], vec![], neutral);
    let hp_before = runtime.champions[0].hp;

    resolve_champion_combat(&mut runtime);

    assert!(runtime.champions[0].hp > hp_before);
    let heal_cd = runtime.champions[0]
        .summoner_spells
        .iter()
        .find(|spell| spell.key == "Heal")
        .map(|spell| spell.cd_until)
        .unwrap_or(0.0);
    assert!(heal_cd > runtime.time_sec);
}

#[test]
fn smite_executes_low_hp_dragon_for_jungler() {
    let mut entities = HashMap::new();
    let mut dragon = test_neutral_timer(
        "dragon",
        Vec2 {
            x: 0.6738,
            y: 0.7031,
        },
        true,
    );
    dragon.hp = 520.0;
    dragon.max_hp = 3600.0;
    entities.insert("dragon".to_string(), dragon);

    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };

    let mut jgl = test_champion("jgl-blue", "blue", "JGL", "bot", Vec2 { x: 0.67, y: 0.70 });
    jgl.summoner_spells = vec![RuntimeSummonerSpellSlot {
        key: "Smite".to_string(),
        cd_until: 0.0,
    }];

    let mut runtime = test_runtime(vec![jgl], vec![], vec![], neutral);
    resolve_champion_combat(&mut runtime);

    assert_eq!(runtime.stats.blue.dragons, 1);
    let decoded = decode_neutral_timers_state(&runtime.neutral_timers)
        .unwrap_or_else(|| panic!("failed to decode timers"));
    let dragon_after = decoded
        .entities
        .get("dragon")
        .unwrap_or_else(|| panic!("dragon missing"));
    assert!(!dragon_after.alive);
}

#[test]
fn jungle_micro_damage_no_longer_has_role_penalty() {
    let top = test_champion("top-blue", "blue", "TOP", "top", Vec2 { x: 0.40, y: 0.40 });
    let jgl = test_champion("jgl-blue", "blue", "JGL", "top", Vec2 { x: 0.42, y: 0.40 });

    assert_eq!(
        champion_micro_damage_multiplier(&top),
        champion_micro_damage_multiplier(&jgl)
    );
}

#[test]
fn jungler_retaliates_against_recent_attacker_even_with_camps_up() {
    let mut entities = HashMap::new();
    entities.insert(
        "wolves-blue".to_string(),
        test_neutral_timer("wolves-blue", Vec2 { x: 0.25, y: 0.25 }, true),
    );
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };

    let mut jgl = test_champion("jgl-blue", "blue", "JGL", "top", Vec2 { x: 0.50, y: 0.50 });
    jgl.attack_range = 0.08;
    jgl.last_damaged_by_champion_id = Some("top-red".to_string());
    jgl.last_damaged_by_champion_at = LANE_COMBAT_UNLOCK_AT + 1.0;
    jgl.last_damaged_at = LANE_COMBAT_UNLOCK_AT + 1.0;

    let enemy = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.55, y: 0.50 });

    let mut runtime = test_runtime(vec![jgl, enemy], vec![], vec![], neutral);
    let enemy_hp_before = runtime.champions[1].hp;

    resolve_champion_combat(&mut runtime);

    assert!(runtime.champions[1].hp < enemy_hp_before);
}

#[test]
fn ultimate_burst_casts_when_level_six_enemy_nearby() {
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities: HashMap::new(),
        extra: HashMap::new(),
    };

    let mut caster = test_champion("mid-blue", "blue", "MID", "mid", Vec2 { x: 0.50, y: 0.50 });
    caster.level = 6;
    caster.ultimate = Some(RuntimeUltimateSlot {
        archetype: "burst".to_string(),
        icon: String::new(),
        cd_until: 0.0,
        ..Default::default()
    });

    let target = test_champion("mid-red", "red", "MID", "mid", Vec2 { x: 0.55, y: 0.50 });
    let mut runtime = test_runtime(vec![caster, target], vec![], vec![], neutral);
    let hp_before = runtime.champions[1].hp;

    resolve_champion_combat(&mut runtime);

    assert!(runtime.champions[1].hp < hp_before);
    let cd = runtime.champions[0]
        .ultimate
        .as_ref()
        .map(|ultimate| ultimate.cd_until)
        .unwrap_or(0.0);
    assert!(cd > runtime.time_sec);
}

#[test]
fn execute_ultimate_requires_low_hp_target() {
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities: HashMap::new(),
        extra: HashMap::new(),
    };

    let mut adc = test_champion("adc-blue", "blue", "ADC", "bot", Vec2 { x: 0.50, y: 0.50 });
    adc.level = 7;
    adc.ultimate = Some(RuntimeUltimateSlot {
        archetype: "execute".to_string(),
        icon: String::new(),
        cd_until: 0.0,
        ..Default::default()
    });

    let mut target = test_champion("adc-red", "red", "ADC", "bot", Vec2 { x: 0.55, y: 0.50 });
    target.hp = 90.0;
    let mut runtime = test_runtime(vec![adc, target], vec![], vec![], neutral);

    resolve_champion_combat(&mut runtime);

    let cd = runtime.champions[0]
        .ultimate
        .as_ref()
        .map(|ultimate| ultimate.cd_until)
        .unwrap_or(0.0);
    assert_eq!(cd, 0.0);
}

#[test]
fn annie_ultimate_summons_tibbers_with_scaled_stats() {
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities: HashMap::new(),
        extra: HashMap::new(),
    };

    let mut annie = test_champion("mid-blue", "blue", "MID", "mid", Vec2 { x: 0.50, y: 0.50 });
    annie.champion_id = "Annie".to_string();
    annie.level = 6;
    annie.ultimate = Some(RuntimeUltimateSlot {
        archetype: "burst".to_string(),
        icon: String::new(),
        cd_until: 0.0,
        ..Default::default()
    });

    let mut runtime = test_runtime(vec![annie], vec![], vec![], neutral);
    resolve_champion_combat(&mut runtime);

    let summon = runtime.minions.iter().find(|minion| {
        minion.id.contains("tibbers") && minion.owner_champion_id.as_deref() == Some("mid-blue")
    });
    assert!(summon.is_some());
}

#[test]
fn shen_ultimate_shields_ally_and_teleports() {
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities: HashMap::new(),
        extra: HashMap::new(),
    };

    let mut shen = test_champion("top-blue", "blue", "TOP", "top", Vec2 { x: 0.30, y: 0.30 });
    shen.champion_id = "Shen".to_string();
    shen.level = 6;
    shen.ultimate = Some(RuntimeUltimateSlot {
        archetype: "defensive".to_string(),
        icon: String::new(),
        cd_until: 0.0,
        ..Default::default()
    });

    let mut ally = test_champion("adc-blue", "blue", "ADC", "bot", Vec2 { x: 0.72, y: 0.78 });
    ally.hp = 25.0;

    let mut runtime = test_runtime(vec![shen, ally], vec![], vec![], neutral);
    let hp_before = runtime.champions[1].hp;
    let ally_pos = runtime.champions[1].pos;

    resolve_champion_combat(&mut runtime);

    assert!(runtime.champions[1].hp > hp_before);
    assert!(dist(runtime.champions[0].pos, ally_pos) < 0.0001);
}

#[test]
fn mordekaiser_ultimate_banishes_both_champions_temporarily() {
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities: HashMap::new(),
        extra: HashMap::new(),
    };

    let mut morde = test_champion("top-blue", "blue", "TOP", "top", Vec2 { x: 0.50, y: 0.50 });
    morde.champion_id = "Mordekaiser".to_string();
    morde.level = 6;
    morde.ultimate = Some(RuntimeUltimateSlot {
        archetype: "burst".to_string(),
        icon: String::new(),
        cd_until: 0.0,
        ..Default::default()
    });

    let enemy = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.54, y: 0.50 });
    let mut runtime = test_runtime(vec![morde, enemy], vec![], vec![], neutral);

    resolve_champion_combat(&mut runtime);

    assert!(runtime.champions[0].realm_banished_until > runtime.time_sec);
    assert!(runtime.champions[1].realm_banished_until > runtime.time_sec);
}

#[test]
fn summon_expires_after_configured_duration() {
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities: HashMap::new(),
        extra: HashMap::new(),
    };

    let mut annie = test_champion("mid-blue", "blue", "MID", "mid", Vec2 { x: 0.50, y: 0.50 });
    annie.champion_id = "Annie".to_string();
    annie.level = 6;
    annie.ultimate = Some(RuntimeUltimateSlot {
        archetype: "burst".to_string(),
        icon: String::new(),
        cd_until: 0.0,
        ..Default::default()
    });

    let mut runtime = test_runtime(vec![annie], vec![], vec![], neutral);
    resolve_champion_combat(&mut runtime);
    assert!(runtime
        .minions
        .iter()
        .any(|minion| minion.alive && minion.kind == "summon"));

    runtime.time_sec += 46.0;
    move_minions(&mut runtime, 0.1);

    assert!(!runtime
        .minions
        .iter()
        .any(|minion| minion.alive && minion.kind == "summon"));
}

#[test]
fn mordekaiser_realm_returns_positions_after_duration() {
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities: HashMap::new(),
        extra: HashMap::new(),
    };

    let mut morde = test_champion("top-blue", "blue", "TOP", "top", Vec2 { x: 0.50, y: 0.50 });
    morde.champion_id = "Mordekaiser".to_string();
    morde.level = 6;
    morde.ultimate = Some(RuntimeUltimateSlot {
        archetype: "burst".to_string(),
        icon: String::new(),
        cd_until: 0.0,
        ..Default::default()
    });

    let enemy = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.54, y: 0.50 });
    let mut runtime = test_runtime(vec![morde, enemy], vec![], vec![], neutral);
    let morde_pos = runtime.champions[0].pos;
    let enemy_pos = runtime.champions[1].pos;

    resolve_champion_combat(&mut runtime);
    runtime.time_sec += ULTIMATE_MORDE_REALM_DURATION_SEC + 0.5;
    move_champions(&mut runtime, 0.1);

    assert!(dist(runtime.champions[0].pos, morde_pos) < 0.0001);
    assert!(dist(runtime.champions[1].pos, enemy_pos) < 0.0001);
}

#[test]
fn global_ultimate_requires_team_vision() {
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities: HashMap::new(),
        extra: HashMap::new(),
    };

    let mut caster = test_champion("jgl-blue", "blue", "JGL", "mid", Vec2 { x: 0.40, y: 0.40 });
    caster.level = 8;
    caster.ultimate = Some(RuntimeUltimateSlot {
        archetype: "global".to_string(),
        icon: String::new(),
        cd_until: 0.0,
        ..Default::default()
    });
    let target = test_champion("mid-red", "red", "MID", "mid", Vec2 { x: 0.56, y: 0.40 });

    let mut runtime = test_runtime(
        vec![caster.clone(), target.clone()],
        vec![],
        vec![],
        neutral.clone(),
    );
    resolve_champion_combat(&mut runtime);
    let cd_without_vision = runtime.champions[0]
        .ultimate
        .as_ref()
        .map(|u| u.cd_until)
        .unwrap_or(0.0);
    assert_eq!(cd_without_vision, 0.0);

    let mut runtime_with_ward = test_runtime(vec![caster, target], vec![], vec![], neutral);
    runtime_with_ward.wards.push(WardRuntime {
        id: "w1".to_string(),
        team: "blue".to_string(),
        owner_champion_id: "jgl-blue".to_string(),
        pos: Vec2 { x: 0.56, y: 0.40 },
        expires_at: runtime_with_ward.time_sec + 30.0,
    });
    resolve_champion_combat(&mut runtime_with_ward);
    let cd_with_vision = runtime_with_ward.champions[0]
        .ultimate
        .as_ref()
        .map(|u| u.cd_until)
        .unwrap_or(0.0);
    assert!(cd_with_vision > runtime_with_ward.time_sec);
}

#[test]
fn pick_combat_target_without_entities_returns_none() {
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities: HashMap::new(),
        extra: HashMap::new(),
    };
    let runtime = test_runtime(vec![], vec![], vec![], neutral.clone());
    let neutral = decode_neutral_for_tests(&runtime);
    let selected = pick_combat_target(&runtime, 0, runtime.time_sec, &neutral);
    assert!(selected.is_none());
}

#[test]
fn objective_assist_prioritizes_objective_over_farm_lock() {
    let adc = test_champion("adc-blue", "blue", "ADC", "bot", Vec2 { x: 0.62, y: 0.73 });
    let jungler = test_champion("jgl-blue", "blue", "JGL", "bot", Vec2 { x: 0.64, y: 0.71 });
    let mut enemy = test_champion("mid-red", "red", "MID", "mid", Vec2 { x: 0.82, y: 0.70 });
    enemy.attack_damage = 1.0;

    let minion = test_minion("m-red-1", "red", "bot", Vec2 { x: 0.625, y: 0.735 });

    let mut entities = HashMap::new();
    entities.insert(
        "dragon".to_string(),
        test_neutral_timer("dragon", Vec2 { x: 0.67, y: 0.70 }, true),
    );
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };

    let runtime = test_runtime(
        vec![adc, jungler, enemy],
        vec![minion],
        vec![],
        neutral.clone(),
    );

    let target = pick_combat_target(&runtime, 0, runtime.time_sec, &neutral);
    assert!(matches!(target, Some(CombatTarget::Neutral(ref key)) if key == "dragon"));
}

#[test]
fn objective_fight_guardrail_allows_local_fight_but_blocks_far_chase() {
    let mut adc = test_champion("adc-blue", "blue", "ADC", "bot", Vec2 { x: 0.66, y: 0.71 });
    adc.state = "objective".to_string();
    let jungler = test_champion(
        "jgl-blue",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.66, y: 0.70 },
    );
    let enemy_far = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.05, y: 0.05 });
    let enemy_near = test_champion("mid-red", "red", "MID", "mid", Vec2 { x: 0.70, y: 0.70 });

    let mut entities = HashMap::new();
    let mut dragon = test_neutral_timer("dragon", Vec2 { x: 0.67, y: 0.70 }, true);
    dragon.hp = dragon.max_hp * 0.80;
    entities.insert("dragon".to_string(), dragon);
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };
    let runtime = test_runtime(
        vec![adc, jungler, enemy_near, enemy_far],
        vec![],
        vec![],
        neutral.clone(),
    );

    assert!(decision_intent_objective_chase_guardrail_allows(
        &runtime, 0, 2, &neutral
    ));
    assert!(!decision_intent_objective_chase_guardrail_allows(
        &runtime, 0, 3, &neutral
    ));
}

#[test]
fn structure_pressure_is_blocked_with_two_enemy_minions_near_tower() {
    let laner = test_champion("top-blue", "blue", "TOP", "top", Vec2 { x: 0.28, y: 0.09 });
    let tower = test_structure(
        "red-top-outer",
        "red",
        "top",
        Vec2 {
            x: 0.275390625,
            y: 0.07161458333333333,
        },
    );

    let allied_wave = test_minion("m-blue-1", "blue", "top", Vec2 { x: 0.29, y: 0.08 });
    let enemy_wave_1 = test_minion("m-red-1", "red", "top", Vec2 { x: 0.27, y: 0.074 });
    let enemy_wave_2 = test_minion("m-red-2", "red", "top", Vec2 { x: 0.271, y: 0.073 });

    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities: HashMap::new(),
        extra: HashMap::new(),
    };

    let runtime = test_runtime(
        vec![laner],
        vec![allied_wave, enemy_wave_1, enemy_wave_2],
        vec![tower],
        neutral.clone(),
    );

    let target = pick_combat_target(&runtime, 0, runtime.time_sec, &neutral);
    assert!(!matches!(target, Some(CombatTarget::Structure(_))));
}

#[test]
fn chase_under_enemy_tower_without_allied_wave_is_blocked() {
    let blue = test_champion(
        "top-blue",
        "blue",
        "TOP",
        "top",
        Vec2 { x: 0.285, y: 0.082 },
    );
    let red = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.276, y: 0.075 });
    let tower = test_structure("red-top-outer", "red", "top", Vec2 { x: 0.275, y: 0.072 });
    let neutral = empty_neutral();
    let runtime = test_runtime(vec![blue, red], Vec::new(), vec![tower], neutral.clone());

    let target = pick_combat_target(&runtime, 0, runtime.time_sec, &neutral);

    assert!(!matches!(target, Some(CombatTarget::Champion(1))));
}

#[test]
fn chase_under_enemy_tower_with_wave_and_numbers_is_allowed() {
    let blue = test_champion(
        "top-blue",
        "blue",
        "TOP",
        "top",
        Vec2 { x: 0.285, y: 0.082 },
    );
    let ally = test_champion(
        "jgl-blue",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.286, y: 0.083 },
    );
    let red = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.276, y: 0.075 });
    let tower = test_structure("red-top-outer", "red", "top", Vec2 { x: 0.275, y: 0.072 });
    let wave = vec![
        test_minion("m-blue-1", "blue", "top", Vec2 { x: 0.277, y: 0.073 }),
        test_minion("m-blue-2", "blue", "top", Vec2 { x: 0.278, y: 0.074 }),
    ];
    let neutral = empty_neutral();
    let runtime = test_runtime(vec![blue, ally, red], wave, vec![tower], neutral.clone());

    let target = pick_combat_target(&runtime, 0, runtime.time_sec, &neutral);

    assert!(matches!(target, Some(CombatTarget::Champion(2))));
}

#[test]
fn one_vs_three_engage_is_blocked() {
    let blue = test_champion("mid-blue", "blue", "MID", "mid", Vec2 { x: 0.50, y: 0.50 });
    let red_1 = test_champion("red-1", "red", "MID", "mid", Vec2 { x: 0.505, y: 0.50 });
    let red_2 = test_champion("red-2", "red", "JGL", "jungle", Vec2 { x: 0.51, y: 0.505 });
    let red_3 = test_champion("red-3", "red", "SUP", "bot", Vec2 { x: 0.495, y: 0.505 });
    let neutral = empty_neutral();
    let runtime = test_runtime(
        vec![blue, red_1, red_2, red_3],
        Vec::new(),
        Vec::new(),
        neutral.clone(),
    );

    let target = pick_combat_target(&runtime, 0, runtime.time_sec, &neutral);

    assert!(!matches!(target, Some(CombatTarget::Champion(_))));
}

#[test]
fn low_hp_dive_without_wave_is_blocked() {
    let mut blue = test_champion(
        "top-blue",
        "blue",
        "TOP",
        "top",
        Vec2 { x: 0.285, y: 0.082 },
    );
    blue.hp = 42.0;
    let red = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.276, y: 0.075 });
    let tower = test_structure("red-top-outer", "red", "top", Vec2 { x: 0.275, y: 0.072 });
    let neutral = empty_neutral();
    let runtime = test_runtime(vec![blue, red], Vec::new(), vec![tower], neutral.clone());

    let target = pick_combat_target(&runtime, 0, runtime.time_sec, &neutral);

    assert!(!matches!(target, Some(CombatTarget::Champion(1))));
}

#[test]
fn trading_poke_allowed_with_safe_advantage() {
    let blue = test_champion("top-blue", "blue", "TOP", "top", Vec2 { x: 0.25, y: 0.085 });
    let mut red = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.266, y: 0.08 });
    red.hp = 68.0;
    let mut minions = vec![
        test_minion("m-blue-1", "blue", "top", Vec2 { x: 0.264, y: 0.08 }),
        test_minion("m-blue-2", "blue", "top", Vec2 { x: 0.265, y: 0.081 }),
        test_minion("m-red-1", "red", "top", Vec2 { x: 0.267, y: 0.08 }),
        test_minion("m-red-2", "red", "top", Vec2 { x: 0.268, y: 0.081 }),
    ];
    for minion in &mut minions {
        minion.path_index = 9;
    }
    let runtime = test_runtime(vec![blue, red], minions, Vec::new(), empty_neutral());

    assert!(can_open_trade_window(
        &runtime.champions[0],
        &runtime.champions[1],
        runtime.time_sec,
        &runtime.champions,
        &runtime.minions,
        &runtime.structures,
        &runtime.lane_combat_state_by_champion,
        runtime.ai_mode,
        &runtime.policy,
    ));
}

#[test]
fn trading_all_in_blocked_under_tower_without_wave_or_numbers() {
    let blue = test_champion(
        "top-blue",
        "blue",
        "TOP",
        "top",
        Vec2 { x: 0.285, y: 0.082 },
    );
    let mut red = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.276, y: 0.075 });
    red.hp = 15.0;
    let tower = test_structure("red-top-outer", "red", "top", Vec2 { x: 0.275, y: 0.072 });

    assert!(!should_commit_all_in_trade(
        &blue,
        &red,
        &[blue.clone(), red.clone()],
        &[],
        &[tower]
    ));
}

#[test]
fn trading_disengage_low_hp_with_nearby_enemies() {
    let mut blue = test_champion("mid-blue", "blue", "MID", "mid", Vec2 { x: 0.50, y: 0.50 });
    blue.hp = 28.0;
    let red_1 = test_champion("mid-red", "red", "MID", "mid", Vec2 { x: 0.53, y: 0.50 });
    let red_2 = test_champion(
        "jgl-red",
        "red",
        "JGL",
        "jungle",
        Vec2 { x: 0.535, y: 0.505 },
    );
    let minions = vec![
        test_minion("m-red-1", "red", "mid", Vec2 { x: 0.525, y: 0.50 }),
        test_minion("m-red-2", "red", "mid", Vec2 { x: 0.526, y: 0.501 }),
    ];
    let runtime = test_runtime(
        vec![blue, red_1, red_2],
        minions,
        Vec::new(),
        empty_neutral(),
    );

    assert!(should_disengage_champion_trade(
        &runtime.champions[0],
        &runtime.champions[1],
        runtime.time_sec,
        &runtime.champions,
        &runtime.minions,
        &runtime.structures,
        runtime.ai_mode,
        &runtime.policy,
    ));
}

#[test]
fn combat_bait_kite_tag_when_low_hp_has_allies_and_enemy_extended() {
    let mut blue = test_champion("top-blue", "blue", "TOP", "top", Vec2 { x: 0.10, y: 0.66 });
    blue.hp = 30.0;
    let ally_1 = test_champion(
        "jgl-blue",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.092, y: 0.67 },
    );
    let ally_2 = test_champion(
        "mid-blue",
        "blue",
        "MID",
        "mid",
        Vec2 { x: 0.094, y: 0.668 },
    );
    let red = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.091, y: 0.67 });
    let neutral = empty_neutral();
    let runtime = test_runtime(
        vec![blue, ally_1, ally_2, red],
        Vec::new(),
        Vec::new(),
        neutral.clone(),
    );

    let (tag, reason) = fight_debug_for_trade(
        &runtime,
        &runtime.champions[0],
        &runtime.champions[3],
        &neutral,
    );

    assert_eq!(tag, "fight:bait");
    assert!(reason.contains("kite_with_allies"));
}

#[test]
fn combat_objective_fight_tag_with_allies_near_objective() {
    let mut adc = test_champion("adc-blue", "blue", "ADC", "bot", Vec2 { x: 0.66, y: 0.71 });
    adc.state = "objective".to_string();
    let jungler = test_champion(
        "jgl-blue",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.66, y: 0.70 },
    );
    let enemy = test_champion("mid-red", "red", "MID", "mid", Vec2 { x: 0.70, y: 0.70 });
    let mut entities = HashMap::new();
    let mut dragon = test_neutral_timer("dragon", Vec2 { x: 0.67, y: 0.70 }, true);
    dragon.hp = dragon.max_hp * 0.80;
    entities.insert("dragon".to_string(), dragon);
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };
    let runtime = test_runtime(
        vec![adc, jungler, enemy],
        Vec::new(),
        Vec::new(),
        neutral.clone(),
    );

    let (tag, reason) = fight_debug_for_trade(
        &runtime,
        &runtime.champions[0],
        &runtime.champions[2],
        &neutral,
    );

    assert_eq!(tag, "fight:objective");
    assert!(reason.contains("objective_context"));
}

#[test]
fn combat_objective_chase_outside_context_is_blocked() {
    let mut adc = test_champion("adc-blue", "blue", "ADC", "bot", Vec2 { x: 0.66, y: 0.71 });
    adc.state = "objective".to_string();
    let jungler = test_champion(
        "jgl-blue",
        "blue",
        "JGL",
        "jungle",
        Vec2 { x: 0.66, y: 0.70 },
    );
    let enemy_far = test_champion("top-red", "red", "TOP", "top", Vec2 { x: 0.05, y: 0.05 });
    let enemy_near = test_champion("mid-red", "red", "MID", "mid", Vec2 { x: 0.70, y: 0.70 });
    let mut entities = HashMap::new();
    entities.insert(
        "dragon".to_string(),
        test_neutral_timer("dragon", Vec2 { x: 0.67, y: 0.70 }, true),
    );
    let neutral = NeutralTimersRuntime {
        dragon_soul_unlocked: false,
        elder_unlocked: false,
        entities,
        extra: HashMap::new(),
    };
    let runtime = test_runtime(
        vec![adc, jungler, enemy_near, enemy_far],
        Vec::new(),
        Vec::new(),
        neutral.clone(),
    );

    assert!(!decision_intent_objective_chase_guardrail_allows(
        &runtime, 0, 3, &neutral
    ));
}
