use super::{
    lane_path_anchor, lane_path_position, ratio, LolSimV3Event, LolSimV3EventKind,
    LolSimV3EventQueue, LolSimV3IntentKind, LolSimV3Intention, LolSimV3Team, LolSimV3Vec2,
    LolSimV3WorldState,
};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
struct RoleBaselineProfile {
    combat: f64,
    durability: f64,
    push: f64,
    roam_objective: f64,
}

#[derive(Debug, Clone, Copy)]
struct ChampionInfluenceProfile {
    combat: f64,
    durability: f64,
    push: f64,
    roam_objective: f64,
}

#[derive(Debug, Clone, Copy)]
struct ChampionProfile {
    baseline: RoleBaselineProfile,
    influence: ChampionInfluenceProfile,
    combat_mult: f64,
    durability_mult: f64,
    push_mult: f64,
    roam_objective_mult: f64,
}

#[derive(Debug, Clone, Default)]
pub struct LolSimV3SystemsReport {
    pub lane_actions: usize,
    pub combat_actions: usize,
    pub objective_actions: usize,
    pub economy_actions: usize,
    pub movement_actions: usize,
    pub events: Vec<LolSimV3Event>,
}

/// Step 5: resolve intentions by systems.
///
/// This stage mutates world state from intention proposals.
pub fn resolve_intentions_by_systems(
    world: &mut LolSimV3WorldState,
    intentions: &[LolSimV3Intention],
) -> LolSimV3SystemsReport {
    let mut report = LolSimV3SystemsReport::default();
    let mut queue = LolSimV3EventQueue::new();

    let mut sorted = intentions.to_vec();
    sorted.sort_by(|a, b| b.priority.cmp(&a.priority));

    for intention in &sorted {
        match intention.kind {
            LolSimV3IntentKind::FarmLane | LolSimV3IntentKind::PushTower => {
                if apply_lane_system(world, intention, &mut queue) {
                    report.lane_actions += 1;
                }
                if apply_economy_system(world, intention, &mut queue) {
                    report.economy_actions += 1;
                }
                if apply_movement_system(world, intention, &mut queue) {
                    report.movement_actions += 1;
                }
            }
            LolSimV3IntentKind::TradeWithEnemy => {
                if apply_combat_system(world, intention, &mut queue) {
                    report.combat_actions += 1;
                }
                if apply_economy_system(world, intention, &mut queue) {
                    report.economy_actions += 1;
                }
                if apply_movement_system(world, intention, &mut queue) {
                    report.movement_actions += 1;
                }
            }
            LolSimV3IntentKind::RoamLane => {
                if apply_roam_system(world, intention, &mut queue) {
                    report.combat_actions += 1;
                }
                if apply_movement_system(world, intention, &mut queue) {
                    report.movement_actions += 1;
                }
            }
            LolSimV3IntentKind::RotateToObjective
            | LolSimV3IntentKind::TakeDragon
            | LolSimV3IntentKind::TakeBaron
            | LolSimV3IntentKind::DefendBase => {
                if apply_objective_system(world, intention, &mut queue) {
                    report.objective_actions += 1;
                }
                if apply_movement_system(world, intention, &mut queue) {
                    report.movement_actions += 1;
                }
            }
            LolSimV3IntentKind::Recall => {
                if apply_recall_system(world, intention, &mut queue) {
                    report.movement_actions += 1;
                }
            }
            LolSimV3IntentKind::WaitRespawn => {
                if apply_movement_system(world, intention, &mut queue) {
                    report.movement_actions += 1;
                }
            }
        }
    }

    process_respawns(world, &mut queue);
    process_neutral_spawns(world, &mut queue);
    process_neutral_camp_takes(world, &mut queue);
    process_minion_waves(world, &mut queue);
    process_minion_lane_combat(world, &mut queue);
    process_minion_structure_pressure(world, &mut queue);
    process_tower_attacks(world, &mut queue);

    world.tick = world.tick.saturating_add(1);
    world.time_sec += world.tick_dt_sec;
    recompute_winner(world);
    if world.winner.is_some() {
        world.running = false;
    }

    report.events = queue.into_events();

    report
}

pub fn recompute_winner(world: &mut LolSimV3WorldState) {
    if world.winner.is_some() {
        return;
    }

    let blue_nexus_alive = world
        .structures
        .iter()
        .find(|structure| structure.id == "blue-nexus")
        .map(|structure| structure.alive)
        .unwrap_or(true);
    let red_nexus_alive = world
        .structures
        .iter()
        .find(|structure| structure.id == "red-nexus")
        .map(|structure| structure.alive)
        .unwrap_or(true);

    if !blue_nexus_alive && red_nexus_alive {
        world.winner = Some(LolSimV3Team::Red);
        return;
    }
    if !red_nexus_alive && blue_nexus_alive {
        world.winner = Some(LolSimV3Team::Blue);
        return;
    }

    // Safety close condition to guarantee bounded simulations in this scaffold.
    if world.tick >= 1200 {
        let blue_score = world.scoreboard.blue.gold
            + world.scoreboard.blue.kills * 120
            + world.scoreboard.blue.dragons * 250;
        let red_score = world.scoreboard.red.gold
            + world.scoreboard.red.kills * 120
            + world.scoreboard.red.dragons * 250;
        world.winner = if blue_score >= red_score {
            Some(LolSimV3Team::Blue)
        } else {
            Some(LolSimV3Team::Red)
        };
    }
}

fn apply_lane_system(
    world: &mut LolSimV3WorldState,
    intention: &LolSimV3Intention,
    queue: &mut LolSimV3EventQueue,
) -> bool {
    let Some(champion) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == intention.champion_id)
    else {
        return false;
    };

    if !champion.alive {
        return false;
    }

    match intention.kind {
        LolSimV3IntentKind::FarmLane => {
            let tick = world.tick;
            let time_sec = world.time_sec;
            let champion_id = champion.id.clone();
            let profile = champion_profile(champion.role.as_str(), champion.id.as_str());
            let before = champion.hp;
            champion.hp = (champion.hp + (0.8 * profile.push_mult)).min(champion.max_hp);
            queue.push(
                tick,
                time_sec,
                LolSimV3EventKind::DamageApplied,
                Some(champion_id.clone()),
                Some(champion_id),
                Some(champion.hp - before),
                None,
                None,
            );
            true
        }
        LolSimV3IntentKind::PushTower => {
            let (tick, time_sec, champion_id) = (world.tick, world.time_sec, champion.id.clone());
            let profile = champion_profile(champion.role.as_str(), champion.id.as_str());
            {
                let before = champion.hp;
                champion.hp = (champion.hp - (0.4 / profile.push_mult.max(0.70))).max(1.0);
                queue.push(
                    tick,
                    time_sec,
                    LolSimV3EventKind::DamageApplied,
                    Some(champion_id.clone()),
                    Some(champion_id.clone()),
                    Some(champion.hp - before),
                    None,
                    None,
                );
            }
            let _ = apply_tower_and_nexus_pressure(world, intention, queue, &champion_id, profile);
            true
        }
        _ => false,
    }
}

fn apply_combat_system(
    world: &mut LolSimV3WorldState,
    intention: &LolSimV3Intention,
    queue: &mut LolSimV3EventQueue,
) -> bool {
    let attacker_idx = world
        .champions
        .iter()
        .position(|champion| champion.id == intention.champion_id);
    let Some(attacker_idx) = attacker_idx else {
        return false;
    };

    if !world.champions[attacker_idx].alive {
        return false;
    }

    let attacker_team = world.champions[attacker_idx].team;
    let attacker_id = world.champions[attacker_idx].id.clone();
    let attacker_role = world.champions[attacker_idx].role.clone();
    let target_idx = world
        .champions
        .iter()
        .position(|champion| champion.alive && champion.team != attacker_team);
    let Some(target_idx) = target_idx else {
        return false;
    };

    let attacker_profile = champion_profile(attacker_role.as_str(), attacker_id.as_str());
    let target_profile = champion_profile(
        world.champions[target_idx].role.as_str(),
        world.champions[target_idx].id.as_str(),
    );
    let base_damage = (8.0 + (intention.priority as f64 * 0.04)) * attacker_profile.combat_mult;
    let damage = base_damage / target_profile.durability_mult.max(0.85);
    let target_lane = world.champions[target_idx].lane.clone();
    let victim_team = world.champions[target_idx].team;
    register_aggression(world, &attacker_id, victim_team, target_lane.as_str());
    let target_id;
    {
        let target = &mut world.champions[target_idx];
        target_id = target.id.clone();
        target.hp -= damage;
    }
    queue.push(
        world.tick,
        world.time_sec,
        LolSimV3EventKind::TradeStarted,
        Some(attacker_id.clone()),
        Some(target_id.clone()),
        None,
        None,
        None,
    );
    queue.push(
        world.tick,
        world.time_sec,
        LolSimV3EventKind::DamageApplied,
        Some(attacker_id.clone()),
        Some(target_id.clone()),
        Some(-damage),
        None,
        None,
    );
    let target_dead = world.champions[target_idx].hp <= 0.0;
    if target_dead {
        let target = &mut world.champions[target_idx];
        target.hp = 0.0;
        target.alive = false;
        target.respawn_at_sec = world.time_sec + 35.0;
        target.state = super::LolSimV3AgentState::Dead;
        queue.push(
            world.tick,
            world.time_sec,
            LolSimV3EventKind::ChampionKilled,
            Some(attacker_id),
            Some(target_id),
            None,
            None,
            None,
        );
        match attacker_team {
            LolSimV3Team::Blue => world.scoreboard.blue.kills += 1,
            LolSimV3Team::Red => world.scoreboard.red.kills += 1,
        }
    }

    true
}

fn apply_objective_system(
    world: &mut LolSimV3WorldState,
    intention: &LolSimV3Intention,
    queue: &mut LolSimV3EventQueue,
) -> bool {
    match intention.kind {
        LolSimV3IntentKind::TakeDragon => {
            let profile = world
                .champions
                .iter()
                .find(|champion| champion.id == intention.champion_id)
                .map(|champion| champion_profile(champion.role.as_str(), champion.id.as_str()))
                .unwrap_or(default_profile());

            if !team_has_objective_vision(world, intention.team, "dragon") {
                return false;
            }

            let Some(dragon) = world
                .objectives
                .iter_mut()
                .find(|objective| objective.key == "dragon")
            else {
                return false;
            };

            if world.time_sec < 5.0 * 60.0 {
                return false;
            }

            if !dragon.alive {
                return false;
            }

            dragon.alive = false;
            dragon.next_spawn_at_sec = Some(world.time_sec + 5.0 * 60.0);

            match intention.team {
                LolSimV3Team::Blue => {
                    world.scoreboard.blue.dragons += 1;
                    world.scoreboard.blue.gold +=
                        (45.0 * profile.roam_objective_mult).round() as i64;
                }
                LolSimV3Team::Red => {
                    world.scoreboard.red.dragons += 1;
                    world.scoreboard.red.gold +=
                        (45.0 * profile.roam_objective_mult).round() as i64;
                }
            }
            queue.push(
                world.tick,
                world.time_sec,
                LolSimV3EventKind::DragonTaken,
                Some(intention.champion_id.clone()),
                Some("dragon".to_string()),
                None,
                None,
                Some(LolSimV3Vec2 { x: 0.67, y: 0.70 }),
            );
            true
        }
        LolSimV3IntentKind::TakeBaron => {
            let profile = world
                .champions
                .iter()
                .find(|champion| champion.id == intention.champion_id)
                .map(|champion| champion_profile(champion.role.as_str(), champion.id.as_str()))
                .unwrap_or(default_profile());

            if !team_has_objective_vision(world, intention.team, "baron") {
                return false;
            }

            let Some(baron) = world
                .objectives
                .iter_mut()
                .find(|objective| objective.key == "baron")
            else {
                return false;
            };

            if world.time_sec < 20.0 * 60.0 {
                return false;
            }

            if !baron.alive {
                return false;
            }

            baron.alive = false;
            baron.next_spawn_at_sec = Some(world.time_sec + 6.0 * 60.0);

            match intention.team {
                LolSimV3Team::Blue => {
                    world.scoreboard.blue.gold +=
                        (300.0 * profile.roam_objective_mult).round() as i64
                }
                LolSimV3Team::Red => {
                    world.scoreboard.red.gold +=
                        (300.0 * profile.roam_objective_mult).round() as i64
                }
            }
            queue.push(
                world.tick,
                world.time_sec,
                LolSimV3EventKind::BaronTaken,
                Some(intention.champion_id.clone()),
                Some("baron".to_string()),
                Some(300.0),
                None,
                Some(LolSimV3Vec2 { x: 0.33, y: 0.30 }),
            );
            true
        }
        LolSimV3IntentKind::DefendBase => true,
        LolSimV3IntentKind::RotateToObjective => true,
        _ => false,
    }
}

fn process_neutral_spawns(world: &mut LolSimV3WorldState, queue: &mut LolSimV3EventQueue) {
    for objective in &mut world.objectives {
        if objective.alive {
            continue;
        }
        if let Some(next_spawn_at) = objective.next_spawn_at_sec {
            if world.time_sec >= next_spawn_at {
                objective.alive = true;
                objective.next_spawn_at_sec = None;
            }
        }
    }

    for camp in &mut world.neutral_camps {
        if camp.alive {
            continue;
        }
        if let Some(next_spawn_at) = camp.next_spawn_at_sec {
            if world.time_sec >= next_spawn_at {
                camp.alive = true;
                camp.next_spawn_at_sec = None;
                let mut metadata = HashMap::new();
                metadata.insert("v".to_string(), serde_json::json!(1));
                metadata.insert(
                    "overlayType".to_string(),
                    serde_json::json!("neutral-spawn"),
                );
                metadata.insert("key".to_string(), serde_json::json!(camp.key));
                metadata.insert("source".to_string(), serde_json::json!("timer"));
                metadata.insert("importance".to_string(), serde_json::json!("low"));
                queue.push_with_context(
                    world.tick,
                    world.time_sec,
                    LolSimV3EventKind::NeutralCampSpawned,
                    None,
                    Some(camp.key.clone()),
                    Some(camp.team.as_str().to_string()),
                    None,
                    None,
                    None,
                    Some(camp.pos),
                    metadata,
                );
            }
        }
    }
}

fn process_neutral_camp_takes(world: &mut LolSimV3WorldState, queue: &mut LolSimV3EventQueue) {
    for camp in &mut world.neutral_camps {
        if !camp.alive {
            continue;
        }

        let taker = world
            .champions
            .iter()
            .filter(|champion| champion.alive && champion.team == camp.team)
            .find(|champion| champion.role == "JGL" && distance(champion.pos, camp.pos) <= 0.08)
            .or_else(|| {
                world.champions.iter().find(|champion| {
                    champion.alive
                        && champion.team == camp.team
                        && distance(champion.pos, camp.pos) <= 0.06
                })
            });

        let Some(taker) = taker else {
            continue;
        };

        camp.alive = false;
        camp.next_spawn_at_sec = Some(world.time_sec + camp_respawn_sec(camp.key.as_str()));

        match camp.team {
            LolSimV3Team::Blue => world.scoreboard.blue.gold += 28,
            LolSimV3Team::Red => world.scoreboard.red.gold += 28,
        }

        let mut metadata = HashMap::new();
        metadata.insert("v".to_string(), serde_json::json!(1));
        metadata.insert(
            "overlayType".to_string(),
            serde_json::json!("neutral-taken"),
        );
        metadata.insert("key".to_string(), serde_json::json!(camp.key));
        metadata.insert("source".to_string(), serde_json::json!("jungle-camp"));
        metadata.insert("gold".to_string(), serde_json::json!(28));
        metadata.insert("importance".to_string(), serde_json::json!("low"));
        queue.push_with_context(
            world.tick,
            world.time_sec,
            LolSimV3EventKind::NeutralCampTaken,
            Some(taker.id.clone()),
            Some(camp.key.clone()),
            Some(camp.team.as_str().to_string()),
            None,
            Some(28.0),
            None,
            Some(camp.pos),
            metadata,
        );
        queue.push(
            world.tick,
            world.time_sec,
            LolSimV3EventKind::GoldChanged,
            Some(taker.id.clone()),
            Some(camp.key.clone()),
            Some(28.0),
            None,
            None,
        );
    }
}

fn camp_respawn_sec(camp_key: &str) -> f64 {
    if camp_key.ends_with("-red") || camp_key.ends_with("-blue") {
        300.0
    } else {
        150.0
    }
}

fn apply_economy_system(
    world: &mut LolSimV3WorldState,
    intention: &LolSimV3Intention,
    queue: &mut LolSimV3EventQueue,
) -> bool {
    let gain = match intention.kind {
        LolSimV3IntentKind::FarmLane => 22,
        LolSimV3IntentKind::PushTower => 16,
        LolSimV3IntentKind::TradeWithEnemy => 12,
        LolSimV3IntentKind::TakeBaron => 10,
        _ => 0,
    };

    if gain == 0 {
        return false;
    }

    match intention.team {
        LolSimV3Team::Blue => world.scoreboard.blue.gold += gain,
        LolSimV3Team::Red => world.scoreboard.red.gold += gain,
    }
    queue.push(
        world.tick,
        world.time_sec,
        LolSimV3EventKind::GoldChanged,
        Some(intention.champion_id.clone()),
        None,
        Some(gain as f64),
        None,
        None,
    );
    true
}

fn apply_movement_system(
    world: &mut LolSimV3WorldState,
    intention: &LolSimV3Intention,
    queue: &mut LolSimV3EventQueue,
) -> bool {
    let Some(champion_idx) = world
        .champions
        .iter_mut()
        .position(|champion| champion.id == intention.champion_id)
    else {
        return false;
    };

    if !world.champions[champion_idx].alive {
        return false;
    }

    let champion_team = world.champions[champion_idx].team;
    let champion_role = world.champions[champion_idx].role.clone();
    let champion_lane = world.champions[champion_idx].lane.clone();

    let target = match intention.kind {
        LolSimV3IntentKind::FarmLane => lane_anchor(champion_team, champion_lane.as_str()),
        LolSimV3IntentKind::PushTower => {
            next_attackable_structure_position(world, champion_team, champion_lane.as_str())
                .unwrap_or_else(|| lane_path_anchor(champion_team, champion_lane.as_str(), 5))
        }
        LolSimV3IntentKind::RoamLane => roam_anchor(champion_team, champion_role.as_str()),
        LolSimV3IntentKind::RotateToObjective | LolSimV3IntentKind::TakeDragon => {
            LolSimV3Vec2 { x: 0.67, y: 0.70 }
        }
        LolSimV3IntentKind::TakeBaron => LolSimV3Vec2 { x: 0.33, y: 0.30 },
        LolSimV3IntentKind::Recall | LolSimV3IntentKind::DefendBase => base_anchor(champion_team),
        LolSimV3IntentKind::TradeWithEnemy => {
            lane_path_anchor(champion_team, champion_lane.as_str(), 4)
        }
        LolSimV3IntentKind::WaitRespawn => world.champions[champion_idx].pos,
    };

    let champion = &mut world.champions[champion_idx];
    let from = champion.pos;
    move_toward(&mut champion.pos, target, 0.0085);
    let to = champion.pos;
    queue.push(
        world.tick,
        world.time_sec,
        LolSimV3EventKind::UnitMoved,
        Some(champion.id.clone()),
        None,
        None,
        Some(from),
        Some(to),
    );
    true
}

fn apply_recall_system(
    world: &mut LolSimV3WorldState,
    intention: &LolSimV3Intention,
    queue: &mut LolSimV3EventQueue,
) -> bool {
    let Some(champion) = world
        .champions
        .iter_mut()
        .find(|champion| champion.id == intention.champion_id)
    else {
        return false;
    };
    if !champion.alive {
        return false;
    }

    champion.pos = base_anchor(champion.team);
    champion.hp = champion.max_hp;
    champion.state = super::LolSimV3AgentState::Laning;
    queue.push(
        world.tick,
        world.time_sec,
        LolSimV3EventKind::AgentStateChanged,
        Some(champion.id.clone()),
        None,
        None,
        None,
        Some(champion.pos),
    );
    true
}

fn apply_roam_system(
    world: &mut LolSimV3WorldState,
    intention: &LolSimV3Intention,
    queue: &mut LolSimV3EventQueue,
) -> bool {
    let roamer_idx = world
        .champions
        .iter()
        .position(|champion| champion.id == intention.champion_id && champion.alive);
    let Some(roamer_idx) = roamer_idx else {
        return false;
    };

    let roamer_team = world.champions[roamer_idx].team;
    let roamer_id = world.champions[roamer_idx].id.clone();
    let roamer_role = world.champions[roamer_idx].role.clone();
    let roamer_profile = champion_profile(roamer_role.as_str(), roamer_id.as_str());
    let target_idx = world
        .champions
        .iter()
        .position(|champion| champion.alive && champion.team != roamer_team);
    let Some(target_idx) = target_idx else {
        return false;
    };

    let target_profile = champion_profile(
        world.champions[target_idx].role.as_str(),
        world.champions[target_idx].id.as_str(),
    );
    let dmg = (6.0 * roamer_profile.roam_objective_mult) / target_profile.durability_mult.max(0.85);
    let target = &mut world.champions[target_idx];
    target.hp -= dmg;
    queue.push(
        world.tick,
        world.time_sec,
        LolSimV3EventKind::TradeStarted,
        Some(roamer_id.clone()),
        Some(target.id.clone()),
        None,
        None,
        None,
    );
    if target.hp <= 0.0 {
        target.hp = 0.0;
        target.alive = false;
        target.respawn_at_sec = world.time_sec + 30.0;
        target.state = super::LolSimV3AgentState::Dead;
        match roamer_team {
            LolSimV3Team::Blue => world.scoreboard.blue.kills += 1,
            LolSimV3Team::Red => world.scoreboard.red.kills += 1,
        }
        queue.push(
            world.tick,
            world.time_sec,
            LolSimV3EventKind::ChampionKilled,
            Some(roamer_id),
            Some(target.id.clone()),
            None,
            None,
            None,
        );
    }
    true
}

fn base_anchor(team: LolSimV3Team) -> LolSimV3Vec2 {
    match team {
        LolSimV3Team::Blue => LolSimV3Vec2 { x: 0.13, y: 0.87 },
        LolSimV3Team::Red => LolSimV3Vec2 { x: 0.87, y: 0.13 },
    }
}

fn lane_anchor(team: LolSimV3Team, lane: &str) -> LolSimV3Vec2 {
    let progress = match lane {
        "top" => 4,
        "bot" => 6,
        _ => 3,
    };
    lane_path_anchor(team, lane, progress)
}

fn roam_anchor(team: LolSimV3Team, role: &str) -> LolSimV3Vec2 {
    match (team, role) {
        (LolSimV3Team::Blue, "JGL") => LolSimV3Vec2 { x: 0.56, y: 0.46 },
        (LolSimV3Team::Blue, "SUP") => LolSimV3Vec2 { x: 0.54, y: 0.50 },
        (LolSimV3Team::Red, "JGL") => LolSimV3Vec2 { x: 0.44, y: 0.54 },
        (LolSimV3Team::Red, "SUP") => LolSimV3Vec2 { x: 0.46, y: 0.50 },
        (LolSimV3Team::Blue, _) => LolSimV3Vec2 { x: 0.58, y: 0.42 },
        (LolSimV3Team::Red, _) => LolSimV3Vec2 { x: 0.42, y: 0.58 },
    }
}

fn team_has_objective_vision(
    world: &LolSimV3WorldState,
    team: LolSimV3Team,
    objective_key: &str,
) -> bool {
    let Some(objective) = world
        .objectives
        .iter()
        .find(|objective| objective.key == objective_key)
    else {
        return false;
    };

    let nearby_allies = world
        .champions
        .iter()
        .filter(|champion| champion.alive && champion.team == team)
        .filter(|champion| distance(champion.pos, objective.pos) <= 0.16)
        .count();

    let support_or_jgl_nearby = world.champions.iter().any(|champion| {
        champion.alive
            && champion.team == team
            && (champion.role == "SUP" || champion.role == "JGL")
            && distance(champion.pos, objective.pos) <= 0.22
    });

    nearby_allies >= 2 || support_or_jgl_nearby
}

fn distance(a: LolSimV3Vec2, b: LolSimV3Vec2) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

fn move_toward(current: &mut LolSimV3Vec2, target: LolSimV3Vec2, step: f64) {
    let dx = target.x - current.x;
    let dy = target.y - current.y;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist <= f64::EPSILON {
        return;
    }

    let scale = (step / dist).min(1.0);
    current.x = (current.x + dx * scale).clamp(0.01, 0.99);
    current.y = (current.y + dy * scale).clamp(0.01, 0.99);
}

fn process_respawns(world: &mut LolSimV3WorldState, queue: &mut LolSimV3EventQueue) {
    for champion in &mut world.champions {
        if champion.alive {
            continue;
        }
        if world.time_sec >= champion.respawn_at_sec {
            champion.alive = true;
            champion.hp = champion.max_hp;
            champion.state = super::LolSimV3AgentState::Laning;
            champion.pos = base_anchor(champion.team);
            queue.push(
                world.tick,
                world.time_sec,
                LolSimV3EventKind::AgentStateChanged,
                Some(champion.id.clone()),
                None,
                None,
                None,
                Some(champion.pos),
            );
        }
    }
}

fn process_minion_waves(world: &mut LolSimV3WorldState, queue: &mut LolSimV3EventQueue) {
    while world.time_sec >= world.next_wave_spawn_at_sec {
        spawn_lane_wave(world, queue, "top");
        spawn_lane_wave(world, queue, "mid");
        spawn_lane_wave(world, queue, "bot");
        world.next_wave_spawn_at_sec += 30.0;
    }

    for minion in &mut world.minions {
        if !minion.alive {
            continue;
        }
        let speed = match minion.kind.as_str() {
            "ranged" => 0.0100,
            "siege" => 0.0085,
            _ => 0.0105,
        };
        minion.lane_progress = (minion.lane_progress + (speed * world.tick_dt_sec)).clamp(0.0, 1.0);
        minion.pos = lane_path_position(minion.team, minion.lane.as_str(), minion.lane_progress);
        if minion.lane_progress >= 0.995 {
            minion.alive = false;
        }
    }

    world.minions.retain(|minion| minion.alive);
}

fn process_minion_lane_combat(world: &mut LolSimV3WorldState, queue: &mut LolSimV3EventQueue) {
    let mut planned_attacks: Vec<(String, usize, f64)> = Vec::new();

    for attacker_idx in 0..world.minions.len() {
        let attacker = &world.minions[attacker_idx];
        if !attacker.alive || !minion_ready_to_attack(world, attacker) {
            continue;
        }

        let Some(target_idx) = select_minion_lane_target(world, attacker_idx) else {
            continue;
        };

        let damage = minion_attack_damage(attacker.kind.as_str());
        planned_attacks.push((attacker.id.clone(), target_idx, damage));
    }

    for (attacker_id, target_idx, damage) in planned_attacks {
        if target_idx >= world.minions.len() || !world.minions[target_idx].alive {
            continue;
        }

        let target = &mut world.minions[target_idx];
        target.hp = (target.hp - damage).max(0.0);
        if target.hp <= 0.0 {
            target.alive = false;
        }

        queue.push(
            world.tick,
            world.time_sec,
            LolSimV3EventKind::DamageApplied,
            Some(attacker_id),
            Some(target.id.clone()),
            Some(-damage),
            None,
            None,
        );
    }

    world.minions.retain(|minion| minion.alive);
}

fn process_minion_structure_pressure(
    world: &mut LolSimV3WorldState,
    queue: &mut LolSimV3EventQueue,
) {
    const MINION_STRUCTURE_RANGE: f64 = 0.11;
    const MINION_STRUCTURE_ADVANCE_LEEWAY: f64 = 0.03;
    const MINION_STRUCTURE_BASE_DAMAGE: f64 = 11.0;

    let mut planned_attacks: Vec<(String, LolSimV3Team, usize, f64)> = Vec::new();

    for minion in world.minions.iter().filter(|minion| minion.alive) {
        if !minion_ready_for_structure_attack(world, minion) {
            continue;
        }

        let enemy_team = match minion.team {
            LolSimV3Team::Blue => LolSimV3Team::Red,
            LolSimV3Team::Red => LolSimV3Team::Blue,
        };
        let Some(target_idx) =
            next_attackable_structure_idx(world, enemy_team, minion.lane.as_str())
        else {
            continue;
        };

        let Some(target) = world.structures.get(target_idx) else {
            continue;
        };
        if target.team == minion.team {
            continue;
        }

        let target_progress = estimate_lane_progress(minion.team, minion.lane.as_str(), target.pos);
        if minion.lane_progress + MINION_STRUCTURE_ADVANCE_LEEWAY < target_progress {
            continue;
        }
        if distance(minion.pos, target.pos) > MINION_STRUCTURE_RANGE {
            continue;
        }

        let kind_scalar = match minion.kind.as_str() {
            "siege" => 1.8,
            "ranged" => 0.85,
            _ => 1.0,
        };
        let wave_index = wave_index_from_time(world.time_sec);
        let pressure_scalar = minion_structure_pressure_scalar(
            world,
            minion.team,
            minion.lane.as_str(),
            target_idx,
            wave_index,
        );
        if pressure_scalar <= 0.0 {
            continue;
        }

        planned_attacks.push((
            minion.id.clone(),
            minion.team,
            target_idx,
            MINION_STRUCTURE_BASE_DAMAGE * kind_scalar * pressure_scalar,
        ));
    }

    for (attacker_id, attacker_team, target_idx, damage) in planned_attacks {
        if target_idx >= world.structures.len() || !world.structures[target_idx].alive {
            continue;
        }
        let target_team = world.structures[target_idx].team;
        if attacker_team == target_team {
            continue;
        }

        let destroyed = {
            let target = &mut world.structures[target_idx];
            target.hp = (target.hp - damage).max(0.0);
            if target.hp <= 0.0 {
                target.alive = false;
                Some((target.kind.clone(), target.id.clone(), target.pos))
            } else {
                None
            }
        };

        queue.push(
            world.tick,
            world.time_sec,
            LolSimV3EventKind::DamageApplied,
            Some(attacker_id.clone()),
            Some(world.structures[target_idx].id.clone()),
            Some(-damage),
            None,
            None,
        );
        maybe_emit_tower_damage_event(
            world,
            queue,
            attacker_id.as_str(),
            target_idx,
            damage,
            "minion_pressure",
        );

        if let Some((kind, structure_id, pos)) = destroyed {
            if kind == "tower" {
                match target_team {
                    LolSimV3Team::Blue => world.scoreboard.red.towers += 1,
                    LolSimV3Team::Red => world.scoreboard.blue.towers += 1,
                }
                let lane = world
                    .structures
                    .iter()
                    .find(|structure| structure.id == structure_id)
                    .map(structure_corridor_lane)
                    .unwrap_or("mid")
                    .to_string();
                let mut metadata = HashMap::new();
                metadata.insert("v".to_string(), serde_json::json!(1));
                metadata.insert(
                    "overlayType".to_string(),
                    serde_json::json!("structure-destroyed"),
                );
                metadata.insert("key".to_string(), serde_json::json!(structure_id.clone()));
                metadata.insert("source".to_string(), serde_json::json!("minion_pressure"));
                metadata.insert("importance".to_string(), serde_json::json!("high"));
                queue.push_with_context(
                    world.tick,
                    world.time_sec,
                    LolSimV3EventKind::TowerDestroyed,
                    Some(attacker_id.clone()),
                    Some(structure_id),
                    Some(target_team.as_str().to_string()),
                    Some(lane),
                    None,
                    None,
                    Some(pos),
                    metadata,
                );
            } else if kind == "nexus" {
                world.winner = Some(match target_team {
                    LolSimV3Team::Blue => LolSimV3Team::Red,
                    LolSimV3Team::Red => LolSimV3Team::Blue,
                });
                queue.push(
                    world.tick,
                    world.time_sec,
                    LolSimV3EventKind::NexusDestroyed,
                    Some(attacker_id),
                    Some(structure_id),
                    None,
                    None,
                    Some(pos),
                );
            }
        }
    }
}

fn minion_ready_for_structure_attack(
    world: &LolSimV3WorldState,
    minion: &super::LolSimV3MinionState,
) -> bool {
    const MINION_STRUCTURE_ATTACK_COOLDOWN_SEC: f64 = 1.5;

    let mut hasher = DefaultHasher::new();
    minion.id.hash(&mut hasher);
    "structure".hash(&mut hasher);
    let phase_bucket = (hasher.finish() % 1000) as f64 / 1000.0;
    let phase = phase_bucket * MINION_STRUCTURE_ATTACK_COOLDOWN_SEC;

    let current_bucket =
        ((world.time_sec + phase) / MINION_STRUCTURE_ATTACK_COOLDOWN_SEC).floor() as i64;
    let previous_time = (world.time_sec - world.tick_dt_sec).max(0.0);
    let previous_bucket =
        ((previous_time + phase) / MINION_STRUCTURE_ATTACK_COOLDOWN_SEC).floor() as i64;
    current_bucket > previous_bucket
}

fn minion_ready_to_attack(world: &LolSimV3WorldState, minion: &super::LolSimV3MinionState) -> bool {
    const MINION_ATTACK_COOLDOWN_SEC: f64 = 1.25;

    let mut hasher = DefaultHasher::new();
    minion.id.hash(&mut hasher);
    let phase_bucket = (hasher.finish() % 1000) as f64 / 1000.0;
    let phase = phase_bucket * MINION_ATTACK_COOLDOWN_SEC;

    let current_bucket = ((world.time_sec + phase) / MINION_ATTACK_COOLDOWN_SEC).floor() as i64;
    let previous_time = (world.time_sec - world.tick_dt_sec).max(0.0);
    let previous_bucket = ((previous_time + phase) / MINION_ATTACK_COOLDOWN_SEC).floor() as i64;
    current_bucket > previous_bucket
}

fn select_minion_lane_target(world: &LolSimV3WorldState, attacker_idx: usize) -> Option<usize> {
    const LANE_COMBAT_PROGRESS_WINDOW: f64 = 0.05;
    const LANE_COMBAT_DISTANCE: f64 = 0.08;

    let attacker = world.minions.get(attacker_idx)?;
    let enemy_team = match attacker.team {
        LolSimV3Team::Blue => LolSimV3Team::Red,
        LolSimV3Team::Red => LolSimV3Team::Blue,
    };

    world
        .minions
        .iter()
        .enumerate()
        .filter(|(idx, target)| {
            *idx != attacker_idx
                && target.alive
                && target.team == enemy_team
                && target.lane == attacker.lane
                && (target.lane_progress - attacker.lane_progress).abs()
                    <= LANE_COMBAT_PROGRESS_WINDOW
                && distance(target.pos, attacker.pos) <= LANE_COMBAT_DISTANCE
        })
        .min_by(|(_, a), (_, b)| {
            let ad = distance(a.pos, attacker.pos);
            let bd = distance(b.pos, attacker.pos);
            ad.partial_cmp(&bd)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.id.cmp(&b.id))
        })
        .map(|(idx, _)| idx)
}

fn minion_attack_damage(kind: &str) -> f64 {
    match kind {
        "ranged" => 22.0,
        "siege" => 38.0,
        _ => 26.0,
    }
}

fn process_tower_attacks(world: &mut LolSimV3WorldState, queue: &mut LolSimV3EventQueue) {
    prune_recent_aggressions(world);

    let tower_indices: Vec<usize> = world
        .structures
        .iter()
        .enumerate()
        .filter_map(|(idx, structure)| {
            if structure.alive && structure.kind == "tower" {
                Some(idx)
            } else {
                None
            }
        })
        .collect();

    for tower_idx in tower_indices {
        if !tower_ready_to_fire(world, tower_idx) {
            continue;
        }

        let Some(target) = select_tower_target(world, tower_idx) else {
            clear_tower_threat_lock(world, tower_idx);
            continue;
        };

        let tower_id = world.structures[tower_idx].id.clone();
        match target {
            TowerTarget::Minion(minion_idx) => {
                let damage = 145.0;
                let minion_id;
                {
                    let minion = &mut world.minions[minion_idx];
                    minion_id = minion.id.clone();
                    minion.hp = (minion.hp - damage).max(0.0);
                    if minion.hp <= 0.0 {
                        minion.alive = false;
                    }
                }
                set_tower_threat_lock(world, tower_idx, &minion_id, "minion");

                queue.push(
                    world.tick,
                    world.time_sec,
                    LolSimV3EventKind::DamageApplied,
                    Some(tower_id.clone()),
                    Some(minion_id),
                    Some(-damage),
                    None,
                    None,
                );
            }
            TowerTarget::Champion(champion_idx) => {
                let damage = 92.0;
                let target_id = world.champions[champion_idx].id.clone();
                {
                    let target = &mut world.champions[champion_idx];
                    target.hp = (target.hp - damage).max(0.0);
                }
                set_tower_threat_lock(world, tower_idx, &target_id, "champion");

                queue.push(
                    world.tick,
                    world.time_sec,
                    LolSimV3EventKind::DamageApplied,
                    Some(tower_id.clone()),
                    Some(target_id.clone()),
                    Some(-damage),
                    None,
                    None,
                );

                let champion_dead = {
                    let target = &world.champions[champion_idx];
                    target.hp <= 0.0 && target.alive
                };
                if champion_dead {
                    let target = &mut world.champions[champion_idx];
                    target.alive = false;
                    target.respawn_at_sec = world.time_sec + 35.0;
                    target.state = super::LolSimV3AgentState::Dead;
                    queue.push(
                        world.tick,
                        world.time_sec,
                        LolSimV3EventKind::ChampionKilled,
                        Some(tower_id),
                        Some(target_id),
                        None,
                        None,
                        None,
                    );
                }
            }
        }
    }

    world.minions.retain(|minion| minion.alive);
}

fn tower_ready_to_fire(world: &LolSimV3WorldState, tower_idx: usize) -> bool {
    const TOWER_SHOT_COOLDOWN_SEC: f64 = 1.0;

    let Some(structure) = world.structures.get(tower_idx) else {
        return false;
    };

    let mut hasher = DefaultHasher::new();
    structure.id.hash(&mut hasher);
    let phase_bucket = (hasher.finish() % 1000) as f64 / 1000.0;
    let phase = phase_bucket * TOWER_SHOT_COOLDOWN_SEC;

    let current_bucket = ((world.time_sec + phase) / TOWER_SHOT_COOLDOWN_SEC).floor() as i64;
    let previous_time = (world.time_sec - world.tick_dt_sec).max(0.0);
    let previous_bucket = ((previous_time + phase) / TOWER_SHOT_COOLDOWN_SEC).floor() as i64;
    current_bucket > previous_bucket
}

enum TowerTarget {
    Minion(usize),
    Champion(usize),
}

impl TowerTarget {
    fn id<'a>(&self, world: &'a LolSimV3WorldState) -> Option<&'a str> {
        match self {
            Self::Minion(idx) => world.minions.get(*idx).map(|minion| minion.id.as_str()),
            Self::Champion(idx) => world
                .champions
                .get(*idx)
                .map(|champion| champion.id.as_str()),
        }
    }
}

fn select_tower_target(world: &LolSimV3WorldState, tower_idx: usize) -> Option<TowerTarget> {
    const TOWER_ATTACK_RANGE: f64 = 0.11;
    let tower = world.structures.get(tower_idx)?;
    let enemy_team = match tower.team {
        LolSimV3Team::Blue => LolSimV3Team::Red,
        LolSimV3Team::Red => LolSimV3Team::Blue,
    };
    let lane = structure_corridor_lane(tower);

    let minion_target = world
        .minions
        .iter()
        .enumerate()
        .filter(|(_, minion)| {
            minion.alive
                && minion.team == enemy_team
                && minion.lane == lane
                && distance(minion.pos, tower.pos) <= TOWER_ATTACK_RANGE
        })
        .min_by(|(_, a), (_, b)| {
            let ad = distance(a.pos, tower.pos);
            let bd = distance(b.pos, tower.pos);
            ad.partial_cmp(&bd)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.id.cmp(&b.id))
        })
        .map(|(idx, _)| idx);

    let minion_target = minion_target.map(TowerTarget::Minion);

    let champion_target = world
        .champions
        .iter()
        .enumerate()
        .filter(|(_, champion)| {
            champion.alive
                && champion.team == enemy_team
                && champion.lane == lane
                && distance(champion.pos, tower.pos) <= TOWER_ATTACK_RANGE
        })
        .min_by(|(_, a), (_, b)| {
            let ad = distance(a.pos, tower.pos);
            let bd = distance(b.pos, tower.pos);
            ad.partial_cmp(&bd)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.id.cmp(&b.id))
        })
        .map(|(idx, _)| TowerTarget::Champion(idx));

    let dive_target = if minion_target.is_none() {
        world
            .champions
            .iter()
            .enumerate()
            .filter(|(_, champion)| {
                champion.alive
                    && champion.team == enemy_team
                    && champion.lane == lane
                    && distance(champion.pos, tower.pos) <= TOWER_ATTACK_RANGE
                    && has_recent_dive_aggression(world, champion.id.as_str(), tower.team, lane)
            })
            .min_by(|(_, a), (_, b)| {
                let ad = distance(a.pos, tower.pos);
                let bd = distance(b.pos, tower.pos);
                ad.partial_cmp(&bd)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.id.cmp(&b.id))
            })
            .map(|(idx, _)| TowerTarget::Champion(idx))
    } else {
        None
    };

    let selected = dive_target.or(minion_target).or(champion_target);
    if let Some(current) = select_locked_tower_target(world, tower_idx, lane, enemy_team) {
        return Some(current);
    }

    selected
}

fn select_locked_tower_target(
    world: &LolSimV3WorldState,
    tower_idx: usize,
    lane: &str,
    enemy_team: LolSimV3Team,
) -> Option<TowerTarget> {
    const TOWER_ATTACK_RANGE: f64 = 0.11;

    let tower = world.structures.get(tower_idx)?;
    let threat = world
        .tower_threat
        .iter()
        .find(|state| state.tower_id == tower.id && state.lock_until_sec >= world.time_sec)?;
    let target_id = threat.target_id.as_deref()?;
    let target_kind = threat.target_kind.as_deref()?;

    match target_kind {
        "minion" => world
            .minions
            .iter()
            .enumerate()
            .find(|(_, minion)| {
                minion.id == target_id
                    && minion.alive
                    && minion.team == enemy_team
                    && minion.lane == lane
                    && distance(minion.pos, tower.pos) <= TOWER_ATTACK_RANGE
            })
            .map(|(idx, _)| TowerTarget::Minion(idx)),
        "champion" => world
            .champions
            .iter()
            .enumerate()
            .find(|(_, champion)| {
                champion.id == target_id
                    && champion.alive
                    && champion.team == enemy_team
                    && champion.lane == lane
                    && distance(champion.pos, tower.pos) <= TOWER_ATTACK_RANGE
            })
            .map(|(idx, _)| TowerTarget::Champion(idx)),
        _ => None,
    }
}

fn set_tower_threat_lock(
    world: &mut LolSimV3WorldState,
    tower_idx: usize,
    target_id: &str,
    target_kind: &str,
) {
    const TOWER_TARGET_LOCK_SEC: f64 = 1.5;

    let Some(tower_id) = world
        .structures
        .get(tower_idx)
        .map(|structure| structure.id.clone())
    else {
        return;
    };

    if let Some(state) = world
        .tower_threat
        .iter_mut()
        .find(|state| state.tower_id == tower_id)
    {
        state.target_id = Some(target_id.to_string());
        state.target_kind = Some(target_kind.to_string());
        state.lock_until_sec = world.time_sec + TOWER_TARGET_LOCK_SEC;
        return;
    }

    world.tower_threat.push(super::LolSimV3TowerThreatState {
        tower_id,
        target_id: Some(target_id.to_string()),
        target_kind: Some(target_kind.to_string()),
        lock_until_sec: world.time_sec + TOWER_TARGET_LOCK_SEC,
    });
}

fn clear_tower_threat_lock(world: &mut LolSimV3WorldState, tower_idx: usize) {
    let Some(tower_id) = world
        .structures
        .get(tower_idx)
        .map(|structure| structure.id.clone())
    else {
        return;
    };
    if let Some(state) = world
        .tower_threat
        .iter_mut()
        .find(|state| state.tower_id == tower_id)
    {
        state.target_id = None;
        state.target_kind = None;
        state.lock_until_sec = world.time_sec;
    }
}

fn has_recent_dive_aggression(
    world: &LolSimV3WorldState,
    attacker_id: &str,
    victim_team: LolSimV3Team,
    lane: &str,
) -> bool {
    world.recent_aggressions.iter().any(|entry| {
        entry.attacker_id == attacker_id
            && entry.victim_team == victim_team
            && entry.lane == lane
            && (world.time_sec - entry.at_sec) <= 2.0
    })
}

fn prune_recent_aggressions(world: &mut LolSimV3WorldState) {
    const AGGRESSION_MEMORY_SEC: f64 = 2.0;
    world
        .recent_aggressions
        .retain(|entry| (world.time_sec - entry.at_sec) <= AGGRESSION_MEMORY_SEC);
}

fn register_aggression(
    world: &mut LolSimV3WorldState,
    attacker_id: &str,
    victim_team: LolSimV3Team,
    lane: &str,
) {
    world
        .recent_aggressions
        .push(super::LolSimV3AggressionRecord {
            attacker_id: attacker_id.to_string(),
            victim_team,
            lane: lane.to_string(),
            at_sec: world.time_sec,
        });
}

fn structure_corridor_lane(structure: &super::LolSimV3StructureState) -> &str {
    match structure.lane.as_str() {
        "top" | "mid" | "bot" => structure.lane.as_str(),
        _ => {
            if structure.id.contains("nexus-top") {
                "top"
            } else if structure.id.contains("nexus-bot") {
                "bot"
            } else {
                "mid"
            }
        }
    }
}

fn spawn_lane_wave(world: &mut LolSimV3WorldState, queue: &mut LolSimV3EventQueue, lane: &str) {
    for team in [LolSimV3Team::Blue, LolSimV3Team::Red] {
        let mut melee_count = 0;
        for _ in 0..3 {
            spawn_minion(world, team, lane, "melee");
            melee_count += 1;
        }
        let mut ranged_count = 0;
        for _ in 0..3 {
            spawn_minion(world, team, lane, "ranged");
            ranged_count += 1;
        }
        let mut siege_count = 0;
        let wave_number = ((world.next_wave_spawn_at_sec - 65.0) / 30.0).max(0.0) as u64 + 1;
        if wave_number % 3 == 0 {
            spawn_minion(world, team, lane, "siege");
            siege_count += 1;
        }

        let mut metadata = HashMap::new();
        metadata.insert("v".to_string(), serde_json::json!(1));
        metadata.insert("overlayType".to_string(), serde_json::json!("wave-spawn"));
        metadata.insert("source".to_string(), serde_json::json!("wave"));
        metadata.insert("wave".to_string(), serde_json::json!(wave_number));
        metadata.insert("meleeCount".to_string(), serde_json::json!(melee_count));
        metadata.insert("rangedCount".to_string(), serde_json::json!(ranged_count));
        metadata.insert("siegeCount".to_string(), serde_json::json!(siege_count));
        metadata.insert(
            "totalCount".to_string(),
            serde_json::json!(melee_count + ranged_count + siege_count),
        );
        metadata.insert(
            "importance".to_string(),
            serde_json::json!(if siege_count > 0 { "medium" } else { "low" }),
        );
        queue.push_with_context(
            world.tick,
            world.time_sec,
            LolSimV3EventKind::WaveSpawned,
            None,
            None,
            Some(team.as_str().to_string()),
            Some(lane.to_string()),
            Some((melee_count + ranged_count + siege_count) as f64),
            None,
            Some(lane_path_position(team, lane, 0.0)),
            metadata,
        );
    }
}

fn spawn_minion(world: &mut LolSimV3WorldState, team: LolSimV3Team, lane: &str, kind: &str) {
    let id = format!(
        "{}-{}-{}-{}",
        team.as_str(),
        lane,
        kind,
        world.next_minion_serial
    );
    world.next_minion_serial = world.next_minion_serial.saturating_add(1);
    let max_hp = match kind {
        "ranged" => 300.0,
        "siege" => 600.0,
        _ => 420.0,
    };
    let lane_progress = 0.0;
    world.minions.push(super::LolSimV3MinionState {
        id,
        team,
        lane: lane.to_string(),
        kind: kind.to_string(),
        alive: true,
        hp: max_hp,
        max_hp,
        pos: lane_path_position(team, lane, lane_progress),
        lane_progress,
    });
}

fn apply_tower_and_nexus_pressure(
    world: &mut LolSimV3WorldState,
    intention: &LolSimV3Intention,
    queue: &mut LolSimV3EventQueue,
    actor_id: &str,
    profile: ChampionProfile,
) -> bool {
    let enemy = match intention.team {
        LolSimV3Team::Blue => LolSimV3Team::Red,
        LolSimV3Team::Red => LolSimV3Team::Blue,
    };

    let Some(target_idx) =
        next_attackable_structure_idx(world, enemy, intention_lane(world, actor_id))
    else {
        return false;
    };

    if !can_apply_structure_pressure(world, actor_id, target_idx) {
        return false;
    }

    let lane = intention_lane(world, actor_id);
    let pressure_scalar = structure_pressure_scalar(world, intention.team, lane, target_idx);
    if pressure_scalar <= 0.0 {
        return false;
    }

    let (victim_team, victim_lane) = {
        let target = &world.structures[target_idx];
        (target.team, structure_corridor_lane(target).to_string())
    };
    register_aggression(world, actor_id, victim_team, victim_lane.as_str());

    let mut applied_damage = 0.0;
    let destroyed = {
        let target = &mut world.structures[target_idx];
        let damage = if target.kind == "nexus" {
            90.0 * profile.roam_objective_mult * pressure_scalar
        } else {
            45.0 * profile.push_mult * pressure_scalar
        };
        applied_damage = damage;
        target.hp -= damage;

        if target.hp <= 0.0 {
            target.hp = 0.0;
            target.alive = false;
            Some((target.kind.clone(), target.id.clone(), target.pos))
        } else {
            None
        }
    };

    if let Some((kind, id, pos)) = destroyed {
        if kind == "tower" {
            match intention.team {
                LolSimV3Team::Blue => world.scoreboard.blue.towers += 1,
                LolSimV3Team::Red => world.scoreboard.red.towers += 1,
            }
            let mut metadata = HashMap::new();
            metadata.insert("v".to_string(), serde_json::json!(1));
            metadata.insert(
                "overlayType".to_string(),
                serde_json::json!("structure-destroyed"),
            );
            metadata.insert("key".to_string(), serde_json::json!(id.clone()));
            metadata.insert("source".to_string(), serde_json::json!("champion_push"));
            metadata.insert("importance".to_string(), serde_json::json!("high"));
            queue.push_with_context(
                world.tick,
                world.time_sec,
                LolSimV3EventKind::TowerDestroyed,
                Some(actor_id.to_string()),
                Some(id),
                Some(victim_team.as_str().to_string()),
                Some(victim_lane.clone()),
                None,
                None,
                Some(pos),
                metadata,
            );
        } else if kind == "nexus" {
            world.winner = Some(intention.team);
            queue.push(
                world.tick,
                world.time_sec,
                LolSimV3EventKind::NexusDestroyed,
                Some(actor_id.to_string()),
                Some(id),
                None,
                None,
                Some(pos),
            );
        }
    }

    maybe_emit_tower_damage_event(
        world,
        queue,
        actor_id,
        target_idx,
        applied_damage,
        "champion_push",
    );

    true
}

fn maybe_emit_tower_damage_event(
    world: &mut LolSimV3WorldState,
    queue: &mut LolSimV3EventQueue,
    actor_id: &str,
    target_idx: usize,
    damage: f64,
    source: &str,
) {
    const TOWER_DAMAGE_EVENT_COOLDOWN_SEC: f64 = 3.0;

    let Some(structure) = world.structures.get(target_idx) else {
        return;
    };
    if structure.kind != "tower" || !structure.alive {
        return;
    }

    let hp_bucket = (ratio(structure.hp, structure.max_hp) * 10.0).floor() as i64;
    let lane = structure_corridor_lane(structure).to_string();
    let tower_id = structure.id.clone();
    let team = structure.team.as_str().to_string();
    let pos = structure.pos;

    let mut should_emit = true;
    if let Some(telemetry) = world
        .tower_damage_telemetry
        .iter_mut()
        .find(|entry| entry.tower_id == tower_id)
    {
        should_emit = (world.time_sec - telemetry.last_emit_at_sec)
            >= TOWER_DAMAGE_EVENT_COOLDOWN_SEC
            || hp_bucket < telemetry.last_hp_bucket;
        if should_emit {
            telemetry.last_emit_at_sec = world.time_sec;
            telemetry.last_hp_bucket = hp_bucket;
        }
    } else {
        world
            .tower_damage_telemetry
            .push(super::LolSimV3TowerDamageTelemetryState {
                tower_id: tower_id.clone(),
                last_emit_at_sec: world.time_sec,
                last_hp_bucket: hp_bucket,
            });
    }

    if !should_emit {
        return;
    }

    let mut metadata = HashMap::new();
    metadata.insert("v".to_string(), serde_json::json!(1));
    metadata.insert(
        "overlayType".to_string(),
        serde_json::json!("structure-pressure"),
    );
    metadata.insert("key".to_string(), serde_json::json!(tower_id));
    metadata.insert("source".to_string(), serde_json::json!(source));
    metadata.insert("hpBucket".to_string(), serde_json::json!(hp_bucket));
    metadata.insert(
        "importance".to_string(),
        serde_json::json!(if hp_bucket <= 2 { "high" } else { "normal" }),
    );
    queue.push_with_context(
        world.tick,
        world.time_sec,
        LolSimV3EventKind::TowerDamaged,
        Some(actor_id.to_string()),
        Some(structure.id.clone()),
        Some(team),
        Some(lane),
        Some(-damage),
        None,
        Some(pos),
        metadata,
    );
}

fn wave_index_from_time(time_sec: f64) -> u64 {
    if time_sec < 65.0 {
        return 0;
    }
    (((time_sec - 65.0) / 30.0).floor() as u64).saturating_add(1)
}

fn minion_structure_pressure_scalar(
    world: &LolSimV3WorldState,
    attacking_team: LolSimV3Team,
    lane: &str,
    target_idx: usize,
    wave_index: u64,
) -> f64 {
    const DEFENDER_CHAMPION_RANGE: f64 = 0.16;
    const DEFENDER_MINION_RANGE: f64 = 0.14;

    let Some(target) = world.structures.get(target_idx) else {
        return 0.0;
    };
    let defending_team = target.team;

    let time_factor = (world.time_sec / (18.0 * 60.0)).clamp(0.0, 1.0);
    let wave_factor = ((wave_index as f64) / 12.0).clamp(0.0, 1.0);
    let ramp = (0.58 + (0.30 * time_factor) + (0.12 * wave_factor)).clamp(0.55, 1.0);

    let defender_champions = world
        .champions
        .iter()
        .filter(|champion| {
            champion.alive
                && champion.team == defending_team
                && champion.lane == lane
                && distance(champion.pos, target.pos) <= DEFENDER_CHAMPION_RANGE
        })
        .count() as f64;
    let defender_minions = world
        .minions
        .iter()
        .filter(|minion| {
            minion.alive
                && minion.team == defending_team
                && minion.lane == lane
                && distance(minion.pos, target.pos) <= DEFENDER_MINION_RANGE
        })
        .count() as f64;

    let defender_scalar =
        (1.0 - (defender_champions * 0.08) - (defender_minions * 0.015)).clamp(0.62, 1.0);

    let front_scalar = if attacking_team == defending_team {
        0.0
    } else {
        1.0
    };
    (ramp * defender_scalar * front_scalar).clamp(0.0, 1.0)
}

fn can_apply_structure_pressure(
    world: &LolSimV3WorldState,
    actor_id: &str,
    target_idx: usize,
) -> bool {
    const STRUCTURE_PRESSURE_RANGE: f64 = 0.10;

    let Some(actor) = world
        .champions
        .iter()
        .find(|champion| champion.id == actor_id && champion.alive)
    else {
        return false;
    };
    let Some(target) = world.structures.get(target_idx) else {
        return false;
    };

    target.team != actor.team && distance(actor.pos, target.pos) <= STRUCTURE_PRESSURE_RANGE
}

fn structure_pressure_scalar(
    world: &LolSimV3WorldState,
    attacking_team: LolSimV3Team,
    lane: &str,
    target_idx: usize,
) -> f64 {
    const MINION_PRESSURE_RANGE: f64 = 0.18;
    const MINION_ADVANCE_LEEWAY: f64 = 0.03;
    const MINION_REQUIRED_SCORE: f64 = 0.40;
    const CHAMPION_SUPPORT_RANGE: f64 = 0.16;

    let Some(target) = world.structures.get(target_idx) else {
        return 0.0;
    };

    let lane = match lane {
        "top" | "mid" | "bot" => lane,
        _ => "mid",
    };
    let target_progress = estimate_lane_progress(attacking_team, lane, target.pos);

    let mut minion_score = 0.0;
    for minion in world.minions.iter().filter(|minion| {
        minion.alive
            && minion.team == attacking_team
            && minion.lane == lane
            && minion.lane_progress + MINION_ADVANCE_LEEWAY >= target_progress
    }) {
        let dist = distance(minion.pos, target.pos);
        if dist > MINION_PRESSURE_RANGE {
            continue;
        }
        let kind_weight = match minion.kind.as_str() {
            "siege" => 1.6,
            "ranged" => 0.8,
            _ => 1.0,
        };
        let proximity_weight = (1.0 - (dist / MINION_PRESSURE_RANGE)).clamp(0.0, 1.0);
        minion_score += kind_weight * proximity_weight;
    }

    if minion_score < MINION_REQUIRED_SCORE {
        return 0.0;
    }

    let champion_support = world
        .champions
        .iter()
        .filter(|champion| {
            champion.alive
                && champion.team == attacking_team
                && champion.lane == lane
                && distance(champion.pos, target.pos) <= CHAMPION_SUPPORT_RANGE
        })
        .count() as f64;

    let champion_bonus = (champion_support * 0.15).min(0.45);
    ((minion_score / 3.5) + champion_bonus).clamp(0.0, 1.35)
}

fn estimate_lane_progress(team: LolSimV3Team, lane: &str, pos: LolSimV3Vec2) -> f64 {
    let mut best_progress = 0.0;
    let mut best_dist = f64::MAX;
    for step in 0..=40 {
        let progress = step as f64 / 40.0;
        let lane_pos = lane_path_position(team, lane, progress);
        let dist = distance(lane_pos, pos);
        if dist < best_dist {
            best_dist = dist;
            best_progress = progress;
        }
    }
    best_progress
}

fn intention_lane<'a>(world: &'a LolSimV3WorldState, actor_id: &str) -> &'a str {
    world
        .champions
        .iter()
        .find(|champion| champion.id == actor_id)
        .map(|champion| champion.lane.as_str())
        .unwrap_or("mid")
}

fn next_attackable_structure_position(
    world: &LolSimV3WorldState,
    attacker_team: LolSimV3Team,
    lane: &str,
) -> Option<LolSimV3Vec2> {
    let enemy = match attacker_team {
        LolSimV3Team::Blue => LolSimV3Team::Red,
        LolSimV3Team::Red => LolSimV3Team::Blue,
    };
    next_attackable_structure_idx(world, enemy, lane).map(|idx| world.structures[idx].pos)
}

fn next_attackable_structure_idx(
    world: &LolSimV3WorldState,
    target_team: LolSimV3Team,
    lane: &str,
) -> Option<usize> {
    let team = target_team.as_str();
    let lane = match lane {
        "top" | "bot" | "mid" => lane,
        _ => "mid",
    };
    let ordered_ids = [
        format!("{team}-{lane}-outer"),
        format!("{team}-{lane}-inner"),
        format!("{team}-{lane}-inhib-tower"),
        format!("{team}-inhib-{lane}"),
        format!("{team}-nexus-top-tower"),
        format!("{team}-nexus-bot-tower"),
        format!("{team}-nexus"),
    ];

    ordered_ids.iter().find_map(|id| {
        world
            .structures
            .iter()
            .position(|structure| structure.id.as_str() == id.as_str() && structure.alive)
    })
}

fn champion_profile(role: &str, champion_id: &str) -> ChampionProfile {
    let baseline = role_baseline_profile(role);
    let influence = champion_influence_profile(champion_id);
    ChampionProfile {
        baseline,
        influence,
        combat_mult: bounded_multiplier(baseline.combat, influence.combat),
        durability_mult: bounded_multiplier(baseline.durability, influence.durability),
        push_mult: bounded_multiplier(baseline.push, influence.push),
        roam_objective_mult: bounded_multiplier(baseline.roam_objective, influence.roam_objective),
    }
}

fn role_baseline_profile(role: &str) -> RoleBaselineProfile {
    match role {
        "TOP" => RoleBaselineProfile {
            combat: 1.00,
            durability: 1.08,
            push: 1.06,
            roam_objective: 0.94,
        },
        "JGL" => RoleBaselineProfile {
            combat: 1.06,
            durability: 0.98,
            push: 0.94,
            roam_objective: 1.12,
        },
        "MID" => RoleBaselineProfile {
            combat: 1.08,
            durability: 0.96,
            push: 1.01,
            roam_objective: 1.04,
        },
        "ADC" => RoleBaselineProfile {
            combat: 1.05,
            durability: 0.92,
            push: 1.10,
            roam_objective: 0.97,
        },
        "SUP" => RoleBaselineProfile {
            combat: 0.94,
            durability: 1.04,
            push: 0.90,
            roam_objective: 1.08,
        },
        _ => RoleBaselineProfile {
            combat: 1.0,
            durability: 1.0,
            push: 1.0,
            roam_objective: 1.0,
        },
    }
}

fn champion_influence_profile(champion_id: &str) -> ChampionInfluenceProfile {
    let id = champion_id.to_lowercase();
    if id.contains("assassin") || id.contains("akali") || id.contains("zed") {
        return ChampionInfluenceProfile {
            combat: 1.08,
            durability: 0.94,
            push: 0.96,
            roam_objective: 1.10,
        };
    }
    if id.contains("tank") || id.contains("ornn") || id.contains("sion") || id.contains("sejuani") {
        return ChampionInfluenceProfile {
            combat: 0.94,
            durability: 1.10,
            push: 1.04,
            roam_objective: 1.02,
        };
    }
    if id.contains("marksman")
        || id.contains("adc")
        || id.contains("jinx")
        || id.contains("caitlyn")
    {
        return ChampionInfluenceProfile {
            combat: 1.06,
            durability: 0.92,
            push: 1.12,
            roam_objective: 0.96,
        };
    }

    let seed = stable_hash_u64(champion_id);
    let combat = hash_to_band(seed ^ 0x9E37_79B9_7F4A_7C15, 0.96, 1.04);
    let durability = hash_to_band(seed ^ 0xC2B2_AE3D_27D4_EB4F, 0.96, 1.04);
    let push = hash_to_band(seed ^ 0x1656_67B1_9E37_79F9, 0.96, 1.04);
    let roam_objective = hash_to_band(seed ^ 0x85EB_CA6B_D6E8_FD93, 0.96, 1.04);
    ChampionInfluenceProfile {
        combat,
        durability,
        push,
        roam_objective,
    }
}

fn default_profile() -> ChampionProfile {
    ChampionProfile {
        baseline: RoleBaselineProfile {
            combat: 1.0,
            durability: 1.0,
            push: 1.0,
            roam_objective: 1.0,
        },
        influence: ChampionInfluenceProfile {
            combat: 1.0,
            durability: 1.0,
            push: 1.0,
            roam_objective: 1.0,
        },
        combat_mult: 1.0,
        durability_mult: 1.0,
        push_mult: 1.0,
        roam_objective_mult: 1.0,
    }
}

fn bounded_multiplier(baseline: f64, influence: f64) -> f64 {
    (baseline * influence).clamp(0.85, 1.20)
}

fn stable_hash_u64(value: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn hash_to_band(seed: u64, min: f64, max: f64) -> f64 {
    let normalized = (seed as f64) / (u64::MAX as f64);
    min + (max - min) * normalized
}
