use super::{
    ratio, LolSimV3LanePressureView, LolSimV3MinionView, LolSimV3NeutralCampView,
    LolSimV3NeutralTimerSummaryView, LolSimV3ObjectivePressureSummaryView, LolSimV3ObjectiveView,
    LolSimV3PhaseContributionView, LolSimV3RoleLaneContributionView, LolSimV3Snapshot,
    LolSimV3StructureView, LolSimV3TowerTargetView, LolSimV3UnitView, LolSimV3WorldState,
};

#[derive(Debug, Clone)]
pub struct LolSimV3SnapshotOptions {
    pub include_dead_units: bool,
    pub max_units: usize,
}

impl Default for LolSimV3SnapshotOptions {
    fn default() -> Self {
        Self {
            include_dead_units: false,
            max_units: 20,
        }
    }
}

/// Step 7: lightweight public snapshot.
///
/// This function intentionally projects only render-facing data.
/// Internal engine-only details stay in `LolSimV3WorldState`.
pub fn build_lightweight_snapshot(
    world: &LolSimV3WorldState,
    options: &LolSimV3SnapshotOptions,
) -> LolSimV3Snapshot {
    let units = world
        .champions
        .iter()
        .filter(|champion| options.include_dead_units || champion.alive)
        .take(options.max_units)
        .map(|champion| LolSimV3UnitView {
            id: champion.id.clone(),
            name: champion.name.clone(),
            champion_id: champion.champion_id.clone(),
            team: champion.team.as_str().to_string(),
            role: champion.role.clone(),
            lane: champion.lane.clone(),
            alive: champion.alive,
            pos: champion.pos,
            hp_ratio: ratio(champion.hp, champion.max_hp),
            state: champion.state.clone(),
        })
        .collect();

    let structures = world
        .structures
        .iter()
        .map(|structure| LolSimV3StructureView {
            id: structure.id.clone(),
            team: structure.team.as_str().to_string(),
            lane: structure.lane.clone(),
            kind: structure.kind.clone(),
            alive: structure.alive,
            hp_ratio: ratio(structure.hp, structure.max_hp),
            pos: structure.pos,
        })
        .collect();

    let minions = world
        .minions
        .iter()
        .filter(|minion| options.include_dead_units || minion.alive)
        .map(|minion| LolSimV3MinionView {
            id: minion.id.clone(),
            team: minion.team.as_str().to_string(),
            lane: minion.lane.clone(),
            kind: minion.kind.clone(),
            alive: minion.alive,
            hp_ratio: ratio(minion.hp, minion.max_hp),
            pos: minion.pos,
        })
        .collect();

    let objectives = world
        .objectives
        .iter()
        .map(|objective| LolSimV3ObjectiveView {
            key: objective.key.clone(),
            alive: objective.alive,
            next_spawn_at_sec: objective.next_spawn_at_sec,
            pos: objective.pos,
        })
        .collect();

    let neutral_camps = world
        .neutral_camps
        .iter()
        .map(|camp| LolSimV3NeutralCampView {
            key: camp.key.clone(),
            team: camp.team.as_str().to_string(),
            alive: camp.alive,
            next_spawn_at_sec: camp.next_spawn_at_sec,
            pos: camp.pos,
        })
        .collect();

    let lane_pressure = ["top", "mid", "bot"]
        .iter()
        .map(|lane| {
            let blue = lane_pressure_score(world, "blue", lane);
            let red = lane_pressure_score(world, "red", lane);
            LolSimV3LanePressureView {
                lane: (*lane).to_string(),
                blue,
                red,
            }
        })
        .collect();

    let mut tower_targets = world
        .tower_threat
        .iter()
        .filter(|entry| entry.target_id.is_some())
        .map(|entry| LolSimV3TowerTargetView {
            tower_id: entry.tower_id.clone(),
            target_id: entry.target_id.clone(),
            target_kind: entry.target_kind.clone(),
            lock_until_sec: entry.lock_until_sec,
        })
        .collect::<Vec<_>>();
    tower_targets.sort_by(|a, b| a.tower_id.cmp(&b.tower_id));

    let neutral_timers = LolSimV3NeutralTimerSummaryView {
        next_dragon_at_sec: world
            .objectives
            .iter()
            .find(|objective| objective.key == "dragon")
            .and_then(|objective| objective.next_spawn_at_sec),
        next_baron_at_sec: world
            .objectives
            .iter()
            .find(|objective| objective.key == "baron")
            .and_then(|objective| objective.next_spawn_at_sec),
        camps_alive: world.neutral_camps.iter().filter(|camp| camp.alive).count() as i64,
        camps_respawning: world
            .neutral_camps
            .iter()
            .filter(|camp| !camp.alive && camp.next_spawn_at_sec.is_some())
            .count() as i64,
    };

    let mut phase_contributions = build_phase_contributions(world);
    phase_contributions.sort_by(|a, b| a.team.cmp(&b.team).then(a.phase.cmp(&b.phase)));

    let mut role_lane_contributions = build_role_lane_contributions(world);
    role_lane_contributions.sort_by(|a, b| {
        a.team
            .cmp(&b.team)
            .then(a.lane.cmp(&b.lane))
            .then(a.role.cmp(&b.role))
    });

    let objective_pressure_summary = build_objective_pressure_summary(world);

    LolSimV3Snapshot {
        tick: world.tick,
        time_sec: world.time_sec,
        running: world.running,
        winner: world.winner.map(|team| team.as_str().to_string()),
        units,
        minions,
        structures,
        objectives,
        neutral_camps,
        scoreboard: world.scoreboard.clone(),
        lane_pressure,
        tower_targets,
        neutral_timers,
        phase_contributions,
        role_lane_contributions,
        objective_pressure_summary,
    }
}

fn build_phase_contributions(world: &LolSimV3WorldState) -> Vec<LolSimV3PhaseContributionView> {
    let phases = [
        "laning",
        "pushing",
        "roaming",
        "objective_setup",
        "fighting",
        "recalling",
        "dead",
    ];
    let teams = ["blue", "red"];
    let mut rows = Vec::with_capacity(phases.len() * teams.len());

    for team in teams {
        let team_units = world
            .champions
            .iter()
            .filter(|champion| champion.team.as_str() == team)
            .count()
            .max(1) as f64;
        for phase in phases {
            let count = world
                .champions
                .iter()
                .filter(|champion| champion.team.as_str() == team)
                .filter(|champion| {
                    serde_json::to_string(&champion.state)
                        .ok()
                        .map(|s| s.trim_matches('"').eq_ignore_ascii_case(phase))
                        .unwrap_or(false)
                })
                .count() as f64;
            rows.push(LolSimV3PhaseContributionView {
                team: team.to_string(),
                phase: phase.to_string(),
                value: (count / team_units).clamp(0.0, 1.0),
            });
        }
    }

    rows
}

fn build_role_lane_contributions(
    world: &LolSimV3WorldState,
) -> Vec<LolSimV3RoleLaneContributionView> {
    let objective_summary = build_objective_pressure_summary(world);
    world
        .champions
        .iter()
        .map(|champion| {
            let lane_score =
                lane_pressure_score(world, champion.team.as_str(), champion.lane.as_str());
            let role_weight = if champion.role.eq_ignore_ascii_case("JGL") {
                1.15
            } else if champion.role.eq_ignore_ascii_case("SUP") {
                0.95
            } else {
                1.0
            };
            let objective_base = if champion.team.as_str() == "blue" {
                objective_summary.blue
            } else {
                objective_summary.red
            };
            LolSimV3RoleLaneContributionView {
                team: champion.team.as_str().to_string(),
                role: champion.role.clone(),
                lane: champion.lane.clone(),
                pressure: (lane_score * role_weight).clamp(-1.0, 1.0),
                objective_pressure: (objective_base * role_weight).clamp(0.0, 1.0),
            }
        })
        .collect()
}

fn build_objective_pressure_summary(
    world: &LolSimV3WorldState,
) -> LolSimV3ObjectivePressureSummaryView {
    let mut blue = 0.0;
    let mut red = 0.0;

    for objective in &world.objectives {
        let alive_bonus = if objective.alive { 0.35 } else { 0.15 };
        for champion in world.champions.iter().filter(|champion| champion.alive) {
            let dx = champion.pos.x - objective.pos.x;
            let dy = champion.pos.y - objective.pos.y;
            let dist = (dx * dx + dy * dy).sqrt();
            let proximity = (1.0 - (dist / 0.5)).clamp(0.0, 1.0);
            let role_weight = if champion.role.eq_ignore_ascii_case("JGL") {
                1.20
            } else if champion.role.eq_ignore_ascii_case("SUP") {
                1.05
            } else {
                1.0
            };
            let pressure = proximity * role_weight * alive_bonus;
            if champion.team.as_str() == "blue" {
                blue += pressure;
            } else {
                red += pressure;
            }
        }
    }

    let normalize = |value: f64| (value / 4.0).clamp(0.0, 1.0);
    let blue_n = normalize(blue);
    let red_n = normalize(red);
    LolSimV3ObjectivePressureSummaryView {
        blue: blue_n,
        red: red_n,
        contested: (blue_n - red_n).abs() <= 0.12,
        delta: (blue_n - red_n).clamp(-1.0, 1.0),
    }
}

fn lane_pressure_score(world: &LolSimV3WorldState, team: &str, lane: &str) -> f64 {
    let team_alive = world
        .minions
        .iter()
        .filter(|minion| minion.alive && minion.team.as_str() == team && minion.lane == lane)
        .count() as f64;
    let enemy_alive = world
        .minions
        .iter()
        .filter(|minion| minion.alive && minion.team.as_str() != team && minion.lane == lane)
        .count() as f64;

    if team_alive <= 0.0 && enemy_alive <= 0.0 {
        return 0.0;
    }

    ((team_alive - enemy_alive) / (team_alive + enemy_alive)).clamp(-1.0, 1.0)
}

pub fn world_snapshot(world: &LolSimV3WorldState) -> LolSimV3Snapshot {
    build_lightweight_snapshot(world, &LolSimV3SnapshotOptions::default())
}
