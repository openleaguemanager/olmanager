use super::{LolSimV3StructureState, LolSimV3Team, LolSimV3Vec2};

#[derive(Debug, Clone, Copy)]
struct StructureDef {
    id: &'static str,
    team: LolSimV3Team,
    lane: &'static str,
    kind: &'static str,
    x: f64,
    y: f64,
}

pub fn full_structure_layout() -> Vec<LolSimV3StructureState> {
    STRUCTURES
        .iter()
        .map(|def| create_structure(*def))
        .collect()
}

pub fn role_start_position(team: LolSimV3Team, role: &str) -> LolSimV3Vec2 {
    let (lane, progress, offset) = match role {
        "TOP" => (
            "top",
            1usize,
            LolSimV3Vec2 {
                x: -0.006,
                y: 0.004,
            },
        ),
        "JGL" => (
            "mid",
            1usize,
            LolSimV3Vec2 {
                x: 0.028,
                y: -0.012,
            },
        ),
        "MID" => ("mid", 1usize, LolSimV3Vec2 { x: -0.004, y: 0.0 }),
        "ADC" => (
            "bot",
            1usize,
            LolSimV3Vec2 {
                x: -0.009,
                y: 0.003,
            },
        ),
        "SUP" => (
            "bot",
            2usize,
            LolSimV3Vec2 {
                x: 0.006,
                y: -0.004,
            },
        ),
        _ => ("mid", 1usize, LolSimV3Vec2 { x: 0.0, y: 0.0 }),
    };

    let anchor = lane_path_anchor(team, lane, progress);
    LolSimV3Vec2 {
        x: (anchor.x + offset.x).clamp(0.01, 0.99),
        y: (anchor.y + offset.y).clamp(0.01, 0.99),
    }
}

pub fn lane_path_anchor(team: LolSimV3Team, lane: &str, progress: usize) -> LolSimV3Vec2 {
    let path = match lane {
        "top" => &LANE_PATH_TOP_BLUE[..],
        "bot" => &LANE_PATH_BOT_BLUE[..],
        _ => &LANE_PATH_MID_BLUE[..],
    };
    let idx = progress.min(path.len().saturating_sub(1));
    let point = path[idx];
    match team {
        LolSimV3Team::Blue => point,
        LolSimV3Team::Red => LolSimV3Vec2 {
            x: 1.0 - point.x,
            y: 1.0 - point.y,
        },
    }
}

pub fn lane_path_position(team: LolSimV3Team, lane: &str, progress: f64) -> LolSimV3Vec2 {
    let path = match lane {
        "top" => &LANE_PATH_TOP_BLUE[..],
        "bot" => &LANE_PATH_BOT_BLUE[..],
        _ => &LANE_PATH_MID_BLUE[..],
    };
    if path.len() <= 1 {
        return lane_path_anchor(team, lane, 0);
    }

    let clamped = progress.clamp(0.0, 1.0);
    let scaled = clamped * (path.len().saturating_sub(1) as f64);
    let from_idx = scaled.floor() as usize;
    let to_idx = (from_idx + 1).min(path.len().saturating_sub(1));
    let t = scaled - (from_idx as f64);
    let from = path[from_idx];
    let to = path[to_idx];
    let blue_point = LolSimV3Vec2 {
        x: from.x + (to.x - from.x) * t,
        y: from.y + (to.y - from.y) * t,
    };
    match team {
        LolSimV3Team::Blue => blue_point,
        LolSimV3Team::Red => LolSimV3Vec2 {
            x: 1.0 - blue_point.x,
            y: 1.0 - blue_point.y,
        },
    }
}

fn create_structure(def: StructureDef) -> LolSimV3StructureState {
    let max_hp = match def.kind {
        "nexus" => 6500.0,
        "inhib" => 4000.0,
        _ => 3500.0,
    };

    LolSimV3StructureState {
        id: def.id.to_string(),
        team: def.team,
        lane: def.lane.to_string(),
        kind: def.kind.to_string(),
        alive: true,
        hp: max_hp,
        max_hp,
        pos: LolSimV3Vec2 { x: def.x, y: def.y },
    }
}

const STRUCTURES: [StructureDef; 30] = [
    StructureDef {
        id: "blue-top-outer",
        team: LolSimV3Team::Blue,
        lane: "top",
        kind: "tower",
        x: 0.072265625,
        y: 0.2838541666666667,
    },
    StructureDef {
        id: "blue-top-inner",
        team: LolSimV3Team::Blue,
        lane: "top",
        kind: "tower",
        x: 0.099609375,
        y: 0.5533854166666666,
    },
    StructureDef {
        id: "blue-top-inhib-tower",
        team: LolSimV3Team::Blue,
        lane: "top",
        kind: "tower",
        x: 0.09049479166666667,
        y: 0.69921875,
    },
    StructureDef {
        id: "blue-mid-outer",
        team: LolSimV3Team::Blue,
        lane: "mid",
        kind: "tower",
        x: 0.4016927083333333,
        y: 0.5755208333333334,
    },
    StructureDef {
        id: "blue-mid-inner",
        team: LolSimV3Team::Blue,
        lane: "mid",
        kind: "tower",
        x: 0.3470052083333333,
        y: 0.6705729166666666,
    },
    StructureDef {
        id: "blue-mid-inhib-tower",
        team: LolSimV3Team::Blue,
        lane: "mid",
        kind: "tower",
        x: 0.2623697916666667,
        y: 0.7408854166666666,
    },
    StructureDef {
        id: "blue-bot-inner",
        team: LolSimV3Team::Blue,
        lane: "bot",
        kind: "tower",
        x: 0.4720052083333333,
        y: 0.8958333333333334,
    },
    StructureDef {
        id: "blue-bot-outer",
        team: LolSimV3Team::Blue,
        lane: "bot",
        kind: "tower",
        x: 0.720703125,
        y: 0.9231770833333334,
    },
    StructureDef {
        id: "blue-bot-inhib-tower",
        team: LolSimV3Team::Blue,
        lane: "bot",
        kind: "tower",
        x: 0.298828125,
        y: 0.9127604166666666,
    },
    StructureDef {
        id: "blue-inhib-top",
        team: LolSimV3Team::Blue,
        lane: "base",
        kind: "inhib",
        x: 0.08658854166666667,
        y: 0.7591145833333334,
    },
    StructureDef {
        id: "blue-inhib-mid",
        team: LolSimV3Team::Blue,
        lane: "base",
        kind: "inhib",
        x: 0.224609375,
        y: 0.7864583333333334,
    },
    StructureDef {
        id: "blue-inhib-bot",
        team: LolSimV3Team::Blue,
        lane: "base",
        kind: "inhib",
        x: 0.24544270833333334,
        y: 0.9114583333333334,
    },
    StructureDef {
        id: "blue-nexus-top-tower",
        team: LolSimV3Team::Blue,
        lane: "base",
        kind: "tower",
        x: 0.126953125,
        y: 0.8372395833333334,
    },
    StructureDef {
        id: "blue-nexus-bot-tower",
        team: LolSimV3Team::Blue,
        lane: "base",
        kind: "tower",
        x: 0.15950520833333334,
        y: 0.875,
    },
    StructureDef {
        id: "blue-nexus",
        team: LolSimV3Team::Blue,
        lane: "base",
        kind: "nexus",
        x: 0.115234375,
        y: 0.8815104166666666,
    },
    StructureDef {
        id: "red-top-outer",
        team: LolSimV3Team::Red,
        lane: "top",
        kind: "tower",
        x: 0.275390625,
        y: 0.07161458333333333,
    },
    StructureDef {
        id: "red-top-inner",
        team: LolSimV3Team::Red,
        lane: "top",
        kind: "tower",
        x: 0.533203125,
        y: 0.08203125,
    },
    StructureDef {
        id: "red-top-inhib-tower",
        team: LolSimV3Team::Red,
        lane: "top",
        kind: "tower",
        x: 0.7024739583333334,
        y: 0.09375,
    },
    StructureDef {
        id: "red-mid-outer",
        team: LolSimV3Team::Red,
        lane: "mid",
        kind: "tower",
        x: 0.595703125,
        y: 0.44140625,
    },
    StructureDef {
        id: "red-mid-inner",
        team: LolSimV3Team::Red,
        lane: "mid",
        kind: "tower",
        x: 0.6569010416666666,
        y: 0.33203125,
    },
    StructureDef {
        id: "red-mid-inhib-tower",
        team: LolSimV3Team::Red,
        lane: "mid",
        kind: "tower",
        x: 0.740234375,
        y: 0.26171875,
    },
    StructureDef {
        id: "red-bot-inner",
        team: LolSimV3Team::Red,
        lane: "bot",
        kind: "tower",
        x: 0.9016927083333334,
        y: 0.44921875,
    },
    StructureDef {
        id: "red-bot-outer",
        team: LolSimV3Team::Red,
        lane: "bot",
        kind: "tower",
        x: 0.9303385416666666,
        y: 0.7057291666666666,
    },
    StructureDef {
        id: "red-bot-inhib-tower",
        team: LolSimV3Team::Red,
        lane: "bot",
        kind: "tower",
        x: 0.912109375,
        y: 0.3125,
    },
    StructureDef {
        id: "red-inhib-top",
        team: LolSimV3Team::Red,
        lane: "base",
        kind: "inhib",
        x: 0.7545572916666666,
        y: 0.09114583333333333,
    },
    StructureDef {
        id: "red-inhib-mid",
        team: LolSimV3Team::Red,
        lane: "base",
        kind: "inhib",
        x: 0.783203125,
        y: 0.22395833333333334,
    },
    StructureDef {
        id: "red-inhib-bot",
        team: LolSimV3Team::Red,
        lane: "base",
        kind: "inhib",
        x: 0.9108072916666666,
        y: 0.24869791666666666,
    },
    StructureDef {
        id: "red-nexus-top-tower",
        team: LolSimV3Team::Red,
        lane: "base",
        kind: "tower",
        x: 0.845703125,
        y: 0.1328125,
    },
    StructureDef {
        id: "red-nexus-bot-tower",
        team: LolSimV3Team::Red,
        lane: "base",
        kind: "tower",
        x: 0.8717447916666666,
        y: 0.1640625,
    },
    StructureDef {
        id: "red-nexus",
        team: LolSimV3Team::Red,
        lane: "base",
        kind: "nexus",
        x: 0.8912760416666666,
        y: 0.1171875,
    },
];

const LANE_PATH_TOP_BLUE: [LolSimV3Vec2; 11] = [
    LolSimV3Vec2 { x: 0.108, y: 0.88 },
    LolSimV3Vec2 { x: 0.096, y: 0.76 },
    LolSimV3Vec2 { x: 0.091, y: 0.67 },
    LolSimV3Vec2 { x: 0.087, y: 0.56 },
    LolSimV3Vec2 { x: 0.084, y: 0.43 },
    LolSimV3Vec2 { x: 0.082, y: 0.31 },
    LolSimV3Vec2 { x: 0.104, y: 0.20 },
    LolSimV3Vec2 { x: 0.182, y: 0.11 },
    LolSimV3Vec2 { x: 0.266, y: 0.08 },
    LolSimV3Vec2 { x: 0.516, y: 0.08 },
    LolSimV3Vec2 { x: 0.89, y: 0.12 },
];

const LANE_PATH_MID_BLUE: [LolSimV3Vec2; 7] = [
    LolSimV3Vec2 { x: 0.12, y: 0.88 },
    LolSimV3Vec2 { x: 0.22, y: 0.78 },
    LolSimV3Vec2 { x: 0.34, y: 0.67 },
    LolSimV3Vec2 { x: 0.46, y: 0.54 },
    LolSimV3Vec2 { x: 0.58, y: 0.42 },
    LolSimV3Vec2 { x: 0.70, y: 0.30 },
    LolSimV3Vec2 { x: 0.89, y: 0.12 },
];

const LANE_PATH_BOT_BLUE: [LolSimV3Vec2; 14] = [
    LolSimV3Vec2 { x: 0.12, y: 0.88 },
    LolSimV3Vec2 { x: 0.24, y: 0.89 },
    LolSimV3Vec2 { x: 0.36, y: 0.90 },
    LolSimV3Vec2 { x: 0.49, y: 0.907 },
    LolSimV3Vec2 { x: 0.62, y: 0.909 },
    LolSimV3Vec2 { x: 0.72, y: 0.912 },
    LolSimV3Vec2 { x: 0.84, y: 0.86 },
    LolSimV3Vec2 { x: 0.872, y: 0.80 },
    LolSimV3Vec2 { x: 0.894, y: 0.73 },
    LolSimV3Vec2 { x: 0.908, y: 0.65 },
    LolSimV3Vec2 { x: 0.916, y: 0.57 },
    LolSimV3Vec2 { x: 0.918, y: 0.48 },
    LolSimV3Vec2 { x: 0.912, y: 0.35 },
    LolSimV3Vec2 { x: 0.89, y: 0.12 },
];
