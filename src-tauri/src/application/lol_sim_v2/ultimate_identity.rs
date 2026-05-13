use serde::Serialize;
use serde_json::{json, Value};

use super::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum UltimatePrimitive {
    SelfAura,
    AllyAura,
    AoePulse,
    ZoneSummon,
    BasicSummon,
    LinearProjectile,
    BeamLine,
    Artillery,
    GlobalPresence,
    TargetedDash,
    ExecuteMarker,
    SuppressionLock,
    DuelRealm,
    AssassinMark,
    UnstoppableCharge,
    BlinkBurst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum UltimateSpatialShape {
    Aura,
    Line,
    Projectile,
    Beam,
    Cone,
    Circle,
    Lock,
    Global,
    GlobalOverlay,
    Zone,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UltimateCastSpatialMetadata {
    pub origin_pos: Vec2,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bespoke_kind: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_pos: Option<Vec2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_pos: Option<Vec2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_orientation: Option<Vec2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_condition: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_origin_kind: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_target_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_target_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_pos: Option<Vec2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Vec2>,
    pub shape: UltimateSpatialShape,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_ms: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_at: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulse_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_kind: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_path: Option<Vec<Vec2>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_to_origin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce_targets: Option<Vec<Vec2>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recast_window_ms: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tether_kind: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum UltimateImplementationStatus {
    Active,
    Partial,
    BespokePending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UltimateVisualHint {
    pub visual_event_id: &'static str,
    pub palette: &'static [&'static str],
    pub shape_language: &'static [&'static str],
    pub motion_hints: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UltimateIdentity {
    pub champion_key: &'static str,
    pub champion_name: &'static str,
    pub technical_primitive: UltimatePrimitive,
    pub signature_id: &'static str,
    pub gameplay_tags: &'static [&'static str],
    pub semantic_effects: &'static [&'static str],
    pub visual: UltimateVisualHint,
    pub status: UltimateImplementationStatus,
}

macro_rules! ident {
    ($key:literal, $name:literal, $primitive:ident, $signature:literal, [$($tag:literal),*], [$($effect:literal),*], [$($color:literal),+], [$($shape:literal),+], [$($motion:literal),+], $status:ident) => {
        UltimateIdentity {
            champion_key: $key,
            champion_name: $name,
            technical_primitive: UltimatePrimitive::$primitive,
            signature_id: $signature,
            gameplay_tags: &[$($tag),*],
            semantic_effects: &[$($effect),*],
            visual: UltimateVisualHint {
                visual_event_id: concat!("ultimate.", $signature),
                palette: &[$($color),+],
                shape_language: &[$($shape),+],
                motion_hints: &[$($motion),+],
            },
            status: UltimateImplementationStatus::$status,
        }
    };
}

#[cfg(test)]
pub(super) const BATCH_1_CHAMPIONS: &[&str] = &[
    "aatrox",
    "alistar",
    "drmundo",
    "fiora",
    "jax",
    "jayce",
    "masteryi",
    "mel",
    "naafiri",
    "nasus",
    "olaf",
    "quinn",
    "rakan",
    "renekton",
    "rengar",
    "riven",
    "singed",
    "sivir",
    "tahmkench",
    "tryndamere",
    "twitch",
    "udyr",
    "vayne",
    "amumu",
    "anivia",
    "aurora",
    "belveth",
    "blitzcrank",
    "cassiopeia",
    "diana",
    "gnar",
    "gragas",
    "hwei",
    "janna",
    "kennen",
    "katarina",
    "milio",
    "morgana",
    "neeko",
    "nilah",
    "orianna",
    "rammus",
    "rell",
    "samira",
    "skarner",
    "swain",
    "talon",
    "taric",
    "thresh",
    "vladimir",
    "volibear",
    "wukong",
    "zac",
    "zeri",
    "ziggs",
    "zyra",
    "annie",
    "elise",
    "heimerdinger",
    "illaoi",
    "ivern",
    "shaco",
    "teemo",
    "viktor",
    "yorick",
];

#[cfg(test)]
pub(super) const BATCH_2_CHAMPIONS: &[&str] = &[
    "akshan",
    "aphelios",
    "ashe",
    "brand",
    "braum",
    "corki",
    "draven",
    "ezreal",
    "irelia",
    "jhin",
    "jinx",
    "kled",
    "kogmaw",
    "maokai",
    "nami",
    "nautilus",
    "ornn",
    "poppy",
    "qiyana",
    "renataglasc",
    "sejuani",
    "seraphine",
    "smolder",
    "sona",
    "tristana",
    "urgot",
    "varus",
    "vex",
    "warwick",
    "yone",
    "yuumi",
    "zoe",
    "caitlyn",
    "garen",
    "gwen",
    "kayle",
    "lucian",
    "lux",
    "malzahar",
    "missfortune",
    "rumble",
    "senna",
    "taliyah",
    "velkoz",
    "gangplank",
    "karthus",
    "lillia",
    "nocturne",
    "pantheon",
    "ryze",
    "shen",
    "soraka",
    "twistedfate",
    "xerath",
];

#[cfg(test)]
pub(super) const BATCH_3_CHAMPIONS: &[&str] = &[
    "ahri",
    "akali",
    "ambessa",
    "briar",
    "camille",
    "chogath",
    "darius",
    "evelynn",
    "fiddlesticks",
    "jarvaniv",
    "kaisa",
    "kassadin",
    "kayn",
    "khazix",
    "leesin",
    "lissandra",
    "malphite",
    "mordekaiser",
    "pyke",
    "reksai",
    "sett",
    "shyvana",
    "sion",
    "sylas",
    "trundle",
    "veigar",
    "vi",
    "viego",
    "yasuo",
    "zed",
];

#[cfg(test)]
pub(super) const BATCH_4_CHAMPIONS: &[&str] = &[
    "aurelionsol",
    "azir",
    "bardo",
    "ekko",
    "fizz",
    "galio",
    "hecarim",
    "kalista",
    "karma",
    "kindred",
    "ksante",
    "leblanc",
    "leona",
    "lulu",
    "nidalee",
    "nunuywillump",
    "syndra",
    "xayah",
    "xinzhao",
    "yunara",
    "zaahen",
    "zilean",
];

#[cfg(test)]
pub(super) const EXPECTED_ULTIMATES_TXT_CHAMPIONS: &[&str] = &[
    "aatrox",
    "ahri",
    "akali",
    "akshan",
    "alistar",
    "ambessa",
    "amumu",
    "anivia",
    "annie",
    "aphelios",
    "ashe",
    "aurelionsol",
    "aurora",
    "azir",
    "bardo",
    "belveth",
    "blitzcrank",
    "brand",
    "braum",
    "briar",
    "caitlyn",
    "camille",
    "cassiopeia",
    "chogath",
    "corki",
    "darius",
    "diana",
    "drmundo",
    "draven",
    "ekko",
    "elise",
    "evelynn",
    "ezreal",
    "fiddlesticks",
    "fiora",
    "fizz",
    "galio",
    "gangplank",
    "garen",
    "gnar",
    "gragas",
    "gwen",
    "hecarim",
    "heimerdinger",
    "hwei",
    "illaoi",
    "irelia",
    "ivern",
    "janna",
    "jarvaniv",
    "jax",
    "jayce",
    "jhin",
    "jinx",
    "ksante",
    "kaisa",
    "kalista",
    "karma",
    "karthus",
    "kassadin",
    "katarina",
    "kayle",
    "kayn",
    "kennen",
    "khazix",
    "kindred",
    "kled",
    "kogmaw",
    "leblanc",
    "leesin",
    "leona",
    "lillia",
    "lissandra",
    "lucian",
    "lulu",
    "lux",
    "malphite",
    "malzahar",
    "maokai",
    "masteryi",
    "mel",
    "milio",
    "missfortune",
    "mordekaiser",
    "morgana",
    "naafiri",
    "nami",
    "nasus",
    "nautilus",
    "neeko",
    "nidalee",
    "nilah",
    "nocturne",
    "nunuywillump",
    "olaf",
    "orianna",
    "ornn",
    "pantheon",
    "poppy",
    "pyke",
    "qiyana",
    "quinn",
    "rakan",
    "rammus",
    "reksai",
    "rell",
    "renataglasc",
    "renekton",
    "rengar",
    "riven",
    "rumble",
    "ryze",
    "samira",
    "sejuani",
    "senna",
    "seraphine",
    "sett",
    "shaco",
    "shen",
    "shyvana",
    "singed",
    "sion",
    "sivir",
    "skarner",
    "smolder",
    "sona",
    "soraka",
    "swain",
    "sylas",
    "syndra",
    "tahmkench",
    "taliyah",
    "talon",
    "taric",
    "teemo",
    "thresh",
    "tristana",
    "trundle",
    "tryndamere",
    "twistedfate",
    "twitch",
    "udyr",
    "urgot",
    "varus",
    "vayne",
    "veigar",
    "velkoz",
    "vex",
    "vi",
    "viego",
    "viktor",
    "vladimir",
    "volibear",
    "warwick",
    "wukong",
    "xayah",
    "xerath",
    "xinzhao",
    "yasuo",
    "yone",
    "yorick",
    "yunara",
    "yuumi",
    "zaahen",
    "zac",
    "zed",
    "zeri",
    "ziggs",
    "zilean",
    "zoe",
    "zyra",
];

pub(super) const ULTIMATE_IDENTITIES: &[UltimateIdentity] = &[
    ident!(
        "aatrox",
        "Aatrox",
        SelfAura,
        "aatrox_world_ender",
        ["self_buff", "drain", "reset_pressure"],
        ["heal", "damage_amp"],
        ["#7f1d1d", "#ef4444", "#111827"],
        ["torn_wings", "blood_runes"],
        ["expanding_omen", "heavy_pulse"],
        Active
    ),
    ident!(
        "alistar",
        "Alistar",
        SelfAura,
        "alistar_unbreakable_will",
        ["self_buff", "cleanse", "tank"],
        ["shield", "damage_reduction"],
        ["#9ca3af", "#f8fafc", "#4b5563"],
        ["horns", "stone_plate"],
        ["stomp_shock", "steady_glow"],
        Active
    ),
    ident!(
        "drmundo",
        "Dr. Mundo",
        SelfAura,
        "mundo_maximum_dosage",
        ["self_buff", "regen", "tank"],
        ["heal", "speed"],
        ["#84cc16", "#a855f7", "#22c55e"],
        ["chemical_bubbles", "jagged_heart"],
        ["sloshing_surge", "elastic_pulse"],
        Partial
    ),
    ident!(
        "fiora",
        "Fiora",
        SelfAura,
        "fiora_grand_challenge",
        ["duel", "single_target", "heal_zone"],
        ["mark", "heal"],
        ["#f472b6", "#fef3c7", "#be185d"],
        ["vitals", "rapier_cross"],
        ["precise_ticks", "duel_ring"],
        BespokePending
    ),
    ident!(
        "jax",
        "Jax",
        SelfAura,
        "jax_grandmasters_might",
        ["self_buff", "hybrid_resist", "duelist"],
        ["shield", "damage"],
        ["#7c3aed", "#f59e0b", "#312e81"],
        ["lamp_flare", "three_strikes"],
        ["measured_orbits", "spark_pop"],
        Partial
    ),
    ident!(
        "jayce",
        "Jayce",
        SelfAura,
        "jayce_transform_mercury",
        ["stance", "poke", "tempo"],
        ["form_swap", "damage"],
        ["#2563eb", "#fbbf24", "#93c5fd"],
        ["hextech_panels", "hammer_gate"],
        ["snap_transform", "electric_arc"],
        Partial
    ),
    ident!(
        "masteryi",
        "Master Yi",
        SelfAura,
        "masteryi_highlander",
        ["self_buff", "reset", "speed"],
        ["speed", "damage_amp"],
        ["#facc15", "#eab308", "#22c55e"],
        ["wuju_blades", "focus_lines"],
        ["fast_afterimages", "sharp_pulse"],
        Active
    ),
    ident!(
        "mel",
        "Mel",
        AllyAura,
        "mel_solar_snare",
        ["aura", "shield", "radiant"],
        ["shield", "team_buff"],
        ["#fbbf24", "#fef3c7", "#fb7185"],
        ["sun_disc", "gold_leaf"],
        ["royal_bloom", "delayed_flash"],
        Partial
    ),
    ident!(
        "naafiri",
        "Naafiri",
        SelfAura,
        "naafiri_hounds_pursuit",
        ["self_buff", "pack", "shield"],
        ["shield", "summon_synergy"],
        ["#991b1b", "#ef4444", "#f97316"],
        ["hound_marks", "claw_chevrons"],
        ["pack_surge", "predator_breathe"],
        Active
    ),
    ident!(
        "nasus",
        "Nasus",
        SelfAura,
        "nasus_fury_sands",
        ["self_buff", "zone_pressure", "scaling"],
        ["heal", "aoe_damage"],
        ["#57534e", "#d6d3d1", "#f59e0b"],
        ["sandstorm", "jackal_crown"],
        ["slow_growth", "grain_spiral"],
        Active
    ),
    ident!(
        "olaf",
        "Olaf",
        SelfAura,
        "olaf_ragnarok",
        ["self_buff", "unstoppable", "berserker"],
        ["shield", "cleanse"],
        ["#f97316", "#ef4444", "#facc15"],
        ["axes", "war_paint"],
        ["forward_roar", "flame_sparks"],
        Active
    ),
    ident!(
        "quinn",
        "Quinn",
        SelfAura,
        "quinn_behind_enemy_lines",
        ["self_buff", "roam", "speed"],
        ["speed", "reposition"],
        ["#facc15", "#78350f", "#fde68a"],
        ["falcon_wings", "wind_cut"],
        ["takeoff_sweep", "feather_trails"],
        Partial
    ),
    ident!(
        "rakan",
        "Rakan",
        SelfAura,
        "rakan_quickness",
        ["self_buff", "charm", "engage"],
        ["cc", "speed"],
        ["#facc15", "#fb7185", "#fef3c7"],
        ["feathers", "heart_arc"],
        ["dance_dash", "spiral_glitter"],
        Active
    ),
    ident!(
        "renekton",
        "Renekton",
        SelfAura,
        "renekton_dominus",
        ["self_buff", "rage", "drain"],
        ["heal", "aoe_damage"],
        ["#7f1d1d", "#ef4444", "#f59e0b"],
        ["crocodile_crest", "rage_runes"],
        ["heavy_breath", "burning_aura"],
        Active
    ),
    ident!(
        "rengar",
        "Rengar",
        SelfAura,
        "rengar_thrill_hunt",
        ["self_buff", "vision", "stealth_hunter"],
        ["reveal", "speed"],
        ["#dc2626", "#fef2f2", "#111827"],
        ["predator_eye", "claw_scan"],
        ["heartbeat_ping", "hunting_sweep"],
        Partial
    ),
    ident!(
        "riven",
        "Riven",
        SelfAura,
        "riven_blade_exile",
        ["self_buff", "execute", "blade"],
        ["damage_amp", "execute"],
        ["#86efac", "#22c55e", "#f8fafc"],
        ["broken_blade", "rune_shards"],
        ["blade_reforge", "wind_slash_hint"],
        Partial
    ),
    ident!(
        "singed",
        "Singed",
        SelfAura,
        "singed_insanity_potion",
        ["self_buff", "chemist", "speed"],
        ["heal", "speed", "resist"],
        ["#22c55e", "#a3e635", "#4ade80"],
        ["bubbles", "poison_vial"],
        ["fizzing_loop", "unstable_jitter"],
        Active
    ),
    ident!(
        "sivir",
        "Sivir",
        AllyAura,
        "sivir_on_the_hunt",
        ["team_aura", "speed", "engage"],
        ["team_speed"],
        ["#0ea5e9", "#facc15", "#67e8f9"],
        ["boomerang_ring", "banner_chevrons"],
        ["team_wave", "forward_stream"],
        Active
    ),
    ident!(
        "tahmkench",
        "Tahm Kench",
        AllyAura,
        "tahmkench_devour",
        ["save", "shield", "single_target"],
        ["shield", "reposition"],
        ["#15803d", "#86efac", "#f59e0b"],
        ["river_maw", "bubble_shell"],
        ["gulp_snap", "protective_wobble"],
        BespokePending
    ),
    ident!(
        "tryndamere",
        "Tryndamere",
        SelfAura,
        "tryndamere_undying_rage",
        ["self_buff", "invulnerability", "berserker"],
        ["shield", "death_prevention"],
        ["#dc2626", "#facc15", "#7f1d1d"],
        ["rage_crown", "blade_notches"],
        ["refuse_death_flash", "redline_pulse"],
        Active
    ),
    ident!(
        "twitch",
        "Twitch",
        SelfAura,
        "twitch_spray_and_pray",
        ["self_buff", "piercing", "marksman"],
        ["range", "damage_amp"],
        ["#84cc16", "#c084fc", "#4ade80"],
        ["toxic_bolts", "crosshair_spray"],
        ["ratatat_lines", "venom_trails"],
        Partial
    ),
    ident!(
        "udyr",
        "Udyr",
        SelfAura,
        "udyr_awakened_spirit",
        ["stance", "self_buff", "spirit"],
        ["stance_swap", "damage"],
        ["#38bdf8", "#f97316", "#a3e635"],
        ["spirit_quad", "paw_flame"],
        ["stance_cycle", "primal_burst"],
        Partial
    ),
    ident!(
        "vayne",
        "Vayne",
        SelfAura,
        "vayne_final_hour",
        ["self_buff", "stealth", "duelist"],
        ["damage_amp", "stealth"],
        ["#4f46e5", "#f8fafc", "#111827"],
        ["silver_bolts", "bat_wings"],
        ["nocturne_flicker", "sharp_roll"],
        Partial
    ),
    ident!(
        "amumu",
        "Amumu",
        AoePulse,
        "amumu_curse_sad_mummy",
        ["aoe", "cc", "lockdown"],
        ["stun", "damage"],
        ["#d97706", "#facc15", "#78350f"],
        ["bandage_ring", "tear_drops"],
        ["instant_snap", "sad_echo"],
        Active
    ),
    ident!(
        "anivia",
        "Anivia",
        AoePulse,
        "anivia_glacial_storm",
        ["zone", "slow", "control"],
        ["slow", "damage_over_time"],
        ["#7dd3fc", "#e0f2fe", "#38bdf8"],
        ["snow_spiral", "crystal_shards"],
        ["persistent_swirl", "cold_breathe"],
        Active
    ),
    ident!(
        "aurora",
        "Aurora",
        AoePulse,
        "aurora_between_worlds",
        ["aoe", "trap", "spirit"],
        ["cc", "zone"],
        ["#a78bfa", "#f0abfc", "#22d3ee"],
        ["spirit_dome", "rabbit_steps"],
        ["phase_ripple", "soft_blink"],
        BespokePending
    ),
    ident!(
        "belveth",
        "Bel'Veth",
        AoePulse,
        "belveth_endless_banquet",
        ["aoe", "void", "summon_synergy"],
        ["damage", "spawn"],
        ["#7e22ce", "#c084fc", "#111827"],
        ["void_manta", "lavender裂"],
        ["consume_implosion", "void_bloom"],
        Active
    ),
    ident!(
        "blitzcrank",
        "Blitzcrank",
        AoePulse,
        "blitzcrank_static_field",
        ["aoe", "silence", "electric"],
        ["damage", "silence"],
        ["#2563eb", "#93c5fd", "#facc15"],
        ["tesla_coils", "gear_ring"],
        ["electric_pop", "radial_zap"],
        Active
    ),
    ident!(
        "cassiopeia",
        "Cassiopeia",
        AoePulse,
        "cassiopeia_petrifying_gaze",
        ["cone", "cc", "poison"],
        ["stun", "damage"],
        ["#22c55e", "#84cc16", "#166534"],
        ["serpent_eye", "stone_cracks"],
        ["gaze_flash", "venom_wave"],
        Partial
    ),
    ident!(
        "diana",
        "Diana",
        AoePulse,
        "diana_moonfall",
        ["aoe", "pull", "moon"],
        ["knockup", "damage"],
        ["#f8fafc", "#c7d2fe", "#64748b"],
        ["crescent_moon", "gravity_threads"],
        ["inward_pull", "lunar_pop"],
        Active
    ),
    ident!(
        "gnar",
        "Gnar",
        AoePulse,
        "gnar_gnar",
        ["aoe", "wall_stun", "rage"],
        ["stun", "damage"],
        ["#d97706", "#f97316", "#facc15"],
        ["paw_slam", "boomerang_teeth"],
        ["tantrum_wave", "impact_bounce"],
        Active
    ),
    ident!(
        "gragas",
        "Gragas",
        AoePulse,
        "gragas_explosive_cask",
        ["aoe", "displace", "barrel"],
        ["knockback", "damage"],
        ["#d6a35f", "#f59e0b", "#78350f"],
        ["barrel_rings", "foam_splash"],
        ["rolling_pop", "drunken_wobble"],
        Active
    ),
    ident!(
        "hwei",
        "Hwei",
        AoePulse,
        "hwei_spiraling_despair",
        ["aoe", "paint", "despair"],
        ["damage", "slow"],
        ["#312e81", "#818cf8", "#f472b6"],
        ["ink_spiral", "brush_marks"],
        ["paint_bleed", "delayed_bloom"],
        Active
    ),
    ident!(
        "janna",
        "Janna",
        AoePulse,
        "janna_monsoon",
        ["aoe", "heal", "disengage"],
        ["heal", "knockback"],
        ["#bbf7d0", "#e0f2fe", "#86efac"],
        ["wind_eye", "leaf_spokes"],
        ["outward_gust", "soft_channel"],
        Active
    ),
    ident!(
        "kennen",
        "Kennen",
        AoePulse,
        "kennen_slicing_maelstrom",
        ["aoe", "stun", "lightning"],
        ["damage", "stun"],
        ["#7dd3fc", "#facc15", "#2563eb"],
        ["shuriken_storm", "lightning_orbs"],
        ["rapid_strikes", "storm_orbit"],
        Active
    ),
    ident!(
        "katarina",
        "Katarina",
        AoePulse,
        "katarina_death_lotus",
        ["aoe", "channel", "reset"],
        ["damage", "anti_heal"],
        ["#dc2626", "#f87171", "#111827"],
        ["dagger_flower", "red_petals"],
        ["spin_flurry", "blade_rain"],
        Active
    ),
    ident!(
        "milio",
        "Milio",
        AoePulse,
        "milio_breath_life",
        ["aoe", "cleanse", "heal"],
        ["heal", "cleanse"],
        ["#fb923c", "#fef3c7", "#fdba74"],
        ["fuemigo_flames", "warm_circles"],
        ["campfire_bloom", "gentle_pop"],
        Active
    ),
    ident!(
        "morgana",
        "Morgana",
        AoePulse,
        "morgana_soul_shackles",
        ["aoe", "delayed_cc", "dark"],
        ["damage", "stun"],
        ["#7c3aed", "#c084fc", "#111827"],
        ["chains", "fallen_wings"],
        ["tether_snap", "dark_pulse"],
        Active
    ),
    ident!(
        "neeko",
        "Neeko",
        AoePulse,
        "neeko_pop_blossom",
        ["aoe", "disguise", "root"],
        ["stun", "damage"],
        ["#a78bfa", "#f472b6", "#22c55e"],
        ["flower_burst", "chameleon_specks"],
        ["jump_bloom", "playful_pop"],
        Active
    ),
    ident!(
        "nilah",
        "Nilah",
        AoePulse,
        "nilah_apotheosis",
        ["aoe", "pull", "heal"],
        ["damage", "heal", "knockup"],
        ["#06b6d4", "#a5f3fc", "#0891b2"],
        ["water_whip", "joy_arc"],
        ["inward_tide", "whip_splash"],
        Active
    ),
    ident!(
        "orianna",
        "Orianna",
        AoePulse,
        "orianna_command_shockwave",
        ["aoe", "pull", "ball"],
        ["knockup", "damage"],
        ["#cbd5e1", "#94a3b8", "#facc15"],
        ["clockwork_ball", "concentric_gears"],
        ["vacuum_snap", "mechanical_ring"],
        Active
    ),
    ident!(
        "rammus",
        "Rammus",
        AoePulse,
        "rammus_soaring_slam",
        ["aoe", "slow", "tremor"],
        ["slow", "damage"],
        ["#92400e", "#f59e0b", "#78350f"],
        ["shell_quake", "ground_cracks"],
        ["tremor_steps", "earth_ripple"],
        Active
    ),
    ident!(
        "rell",
        "Rell",
        AoePulse,
        "rell_magnet_storm",
        ["aoe", "pull", "metal"],
        ["pull", "damage"],
        ["#ca8a04", "#facc15", "#71717a"],
        ["magnet_ring", "metal_shards"],
        ["dragging_orbit", "iron_surge"],
        Active
    ),
    ident!(
        "samira",
        "Samira",
        AoePulse,
        "samira_inferno_trigger",
        ["aoe", "style", "channel"],
        ["damage", "lifesteal"],
        ["#f97316", "#dc2626", "#facc15"],
        ["bullet_flower", "coin_sparks"],
        ["stylish_spin", "rapid_barrage"],
        Active
    ),
    ident!(
        "skarner",
        "Skarner",
        AoePulse,
        "skarner_impale",
        ["aoe", "suppression", "crystal"],
        ["stun", "drag"],
        ["#7c3aed", "#c084fc", "#f8fafc"],
        ["crystal_claws", "burrow_lines"],
        ["pinch_snap", "drag_tether"],
        BespokePending
    ),
    ident!(
        "swain",
        "Swain",
        AoePulse,
        "swain_demonic_ascension",
        ["aoe", "drain", "demon"],
        ["damage", "heal"],
        ["#dc2626", "#111827", "#f87171"],
        ["raven_wings", "demon_eye"],
        ["draining_orbit", "dark_flap"],
        Active
    ),
    ident!(
        "talon",
        "Talon",
        AoePulse,
        "talon_shadow_assault",
        ["aoe", "stealth", "blades"],
        ["damage", "stealth"],
        ["#6b7280", "#d1d5db", "#111827"],
        ["blade_ring", "smoke_cut"],
        ["out_and_back", "razor_swirl"],
        Partial
    ),
    ident!(
        "taric",
        "Taric",
        AllyAura,
        "taric_cosmic_radiance",
        ["team_aura", "invulnerability", "delay"],
        ["shield", "death_prevention"],
        ["#7dd3fc", "#f8fafc", "#c084fc"],
        ["gem_facets", "star_columns"],
        ["delayed_descent", "cosmic_shimmer"],
        Active
    ),
    ident!(
        "thresh",
        "Thresh",
        AoePulse,
        "thresh_the_box",
        ["zone", "slow", "walls"],
        ["slow", "damage"],
        ["#2dd4bf", "#99f6e4", "#064e3b"],
        ["spectral_box", "soul_lanterns"],
        ["wall_snap", "ghostly_fade"],
        Active
    ),
    ident!(
        "vladimir",
        "Vladimir",
        AoePulse,
        "vladimir_hemoplague",
        ["aoe", "delayed_burst", "drain"],
        ["damage", "heal"],
        ["#991b1b", "#ef4444", "#fecaca"],
        ["blood_droplets", "plague_ring"],
        ["delayed_burst", "sanguine_bloom"],
        Active
    ),
    ident!(
        "volibear",
        "Volibear",
        AoePulse,
        "volibear_stormbringer",
        ["aoe", "tower_disable", "storm"],
        ["damage", "slow"],
        ["#facc15", "#38bdf8", "#1e3a8a"],
        ["storm_claws", "bear_rune"],
        ["thunder_drop", "screen_shake_hint"],
        Partial
    ),
    ident!(
        "wukong",
        "Wukong",
        AoePulse,
        "wukong_cyclone",
        ["aoe", "knockup", "clone"],
        ["knockup", "damage"],
        ["#f59e0b", "#d97706", "#fde68a"],
        ["staff_circle", "monkey_clouds"],
        ["spinning_staff", "double_sweep"],
        Active
    ),
    ident!(
        "zac",
        "Zac",
        AoePulse,
        "zac_lets_bounce",
        ["aoe", "knockup", "elastic"],
        ["knockup", "damage"],
        ["#84cc16", "#22c55e", "#bef264"],
        ["slime_splash", "elastic_ring"],
        ["squash_stretch", "bouncy_pulse"],
        Partial
    ),
    ident!(
        "zeri",
        "Zeri",
        AoePulse,
        "zeri_lightning_crash",
        ["aoe", "speed", "electric"],
        ["damage", "speed"],
        ["#a3e635", "#22d3ee", "#facc15"],
        ["lightning_graph", "skate_trails"],
        ["overcharge_snap", "electric_orbit"],
        Active
    ),
    ident!(
        "ziggs",
        "Ziggs",
        AoePulse,
        "ziggs_mega_inferno_bomb",
        ["aoe", "bomb", "zone"],
        ["damage"],
        ["#f97316", "#facc15", "#dc2626"],
        ["bomb_target", "explosive_smile"],
        ["falling_whistle", "huge_pop"],
        Active
    ),
    ident!(
        "zyra",
        "Zyra",
        AoePulse,
        "zyra_stranglethorns",
        ["zone", "plants", "knockup"],
        ["damage", "knockup"],
        ["#22c55e", "#a3e635", "#be123c"],
        ["thorn_vines", "petal_spikes"],
        ["vine_growth", "thorn_snap"],
        Active
    ),
    ident!(
        "annie",
        "Annie",
        BasicSummon,
        "annie_summon_tibbers",
        ["summon", "burst", "pet"],
        ["spawn", "damage"],
        ["#f97316", "#dc2626", "#111827"],
        ["teddy_bear", "fire_claws"],
        ["bear_drop", "flame_roar"],
        Active
    ),
    ident!(
        "elise",
        "Elise",
        BasicSummon,
        "elise_spider_form",
        ["stance", "summon", "spiders"],
        ["spawn", "form_swap"],
        ["#7f1d1d", "#111827", "#ef4444"],
        ["spider_web", "fangs"],
        ["web_snap", "skitter_pulse"],
        BespokePending
    ),
    ident!(
        "heimerdinger",
        "Heimerdinger",
        BasicSummon,
        "heimerdinger_apex_turret",
        ["summon", "turret", "zone"],
        ["spawn", "damage"],
        ["#facc15", "#60a5fa", "#f97316"],
        ["hex_turret", "wrench_sparks"],
        ["deploy_pop", "gear_spin"],
        BespokePending
    ),
    ident!(
        "illaoi",
        "Illaoi",
        ZoneSummon,
        "illaoi_leap_faith",
        ["zone", "tentacles", "slam"],
        ["spawn", "damage"],
        ["#166534", "#22c55e", "#facc15"],
        ["tentacle_spokes", "idol_eye"],
        ["sea_slam", "tentacle_rise"],
        Partial
    ),
    ident!(
        "ivern",
        "Ivern",
        BasicSummon,
        "ivern_summon_daisy",
        ["summon", "pet", "disrupt"],
        ["spawn", "knockup"],
        ["#22c55e", "#86efac", "#a16207"],
        ["tree_golem", "flower_steps"],
        ["gentle_arrival", "earthy_pulse"],
        Active
    ),
    ident!(
        "shaco",
        "Shaco",
        BasicSummon,
        "shaco_hallucinate",
        ["summon", "clone", "trickster"],
        ["spawn", "deceive"],
        ["#ec4899", "#111827", "#f9a8d4"],
        ["jester_mask", "split_mirror"],
        ["blink_split", "laugh_jitter"],
        Active
    ),
    ident!(
        "teemo",
        "Teemo",
        ZoneSummon,
        "teemo_noxious_trap",
        ["zone", "trap", "poison"],
        ["spawn", "slow", "damage_over_time"],
        ["#22c55e", "#84cc16", "#fef3c7"],
        ["mushroom_cap", "toxic_ring"],
        ["pop_trap", "poison_puff"],
        BespokePending
    ),
    ident!(
        "viktor",
        "Viktor",
        ZoneSummon,
        "viktor_chaos_storm",
        ["zone", "storm", "control"],
        ["spawn", "damage_over_time"],
        ["#7c3aed", "#facc15", "#60a5fa"],
        ["hex_storm", "gravity_core"],
        ["machine_orbit", "storm_chase"],
        Partial
    ),
    ident!(
        "yorick",
        "Yorick",
        BasicSummon,
        "yorick_eulogy_isles",
        ["summon", "maiden", "splitpush"],
        ["spawn", "damage"],
        ["#0f766e", "#99f6e4", "#111827"],
        ["maiden_veil", "grave_mist"],
        ["spectral_rise", "mist_drift"],
        Active
    ),
    ident!(
        "akshan",
        "Akshan",
        LinearProjectile,
        "akshan_comeuppance_lockon",
        ["projectile", "execute", "channel_lock"],
        ["damage", "execute"],
        ["#facc15", "#22d3ee", "#78350f"],
        ["grappling_crosshair", "gold_bullets"],
        ["target_lock", "serial_shots", "coin_glint"],
        Partial
    ),
    ident!(
        "aphelios",
        "Aphelios",
        LinearProjectile,
        "aphelios_moonlight_vigil",
        ["projectile", "moonlight", "weapon_followup"],
        ["damage", "slow"],
        ["#c7d2fe", "#818cf8", "#111827"],
        ["lunar_disc", "five_weapon_orbit"],
        ["silent_moon_arc", "delayed_weapon_bloom"],
        Partial
    ),
    ident!(
        "ashe",
        "Ashe",
        LinearProjectile,
        "ashe_enchanted_crystal_arrow",
        ["global_missile", "stun", "reveal"],
        ["damage", "stun", "reveal"],
        ["#67e8f9", "#e0f2fe", "#2563eb"],
        ["crystal_arrowhead", "frost_wings"],
        ["long_glide", "ice_shatter_impact"],
        Active
    ),
    ident!(
        "brand",
        "Brand",
        LinearProjectile,
        "brand_pyroclasm_bounce",
        ["bouncing_projectile", "fire", "spread"],
        ["damage", "bounce"],
        ["#dc2626", "#f97316", "#facc15"],
        ["living_flame", "bounce_embers"],
        ["ricochet_hops", "combustion_pop"],
        Partial
    ),
    ident!(
        "braum",
        "Braum",
        LinearProjectile,
        "braum_glacial_fissure",
        ["line", "knockup", "terrain_crack"],
        ["knockup", "slow", "damage"],
        ["#bfdbfe", "#38bdf8", "#64748b"],
        ["ice_fissure", "shield_sigil"],
        ["ground_split", "frost_aftershock"],
        Active
    ),
    ident!(
        "corki",
        "Corki",
        LinearProjectile,
        "corki_missile_barrage",
        ["projectile", "ammo", "poke"],
        ["damage"],
        ["#fb923c", "#facc15", "#475569"],
        ["rocket_fin", "yordle_smoke"],
        ["ammo_pop", "wobbly_trail"],
        Active
    ),
    ident!(
        "draven",
        "Draven",
        LinearProjectile,
        "draven_whirling_death",
        ["global_missile", "returning", "execute_pressure"],
        ["damage", "return"],
        ["#f97316", "#facc15", "#7f1d1d"],
        ["twin_axes", "showman_stars"],
        ["out_and_back_spin", "axe_spark_impact"],
        Active
    ),
    ident!(
        "ezreal",
        "Ezreal",
        LinearProjectile,
        "ezreal_trueshot_barrage",
        ["global_missile", "wide_beam_projectile", "poke"],
        ["damage"],
        ["#facc15", "#60a5fa", "#fef3c7"],
        ["gauntlet_arc", "wide_energy_bow"],
        ["charging_flash", "map_long_sweep"],
        Active
    ),
    ident!(
        "irelia",
        "Irelia",
        LinearProjectile,
        "irelia_vanguard_edge",
        ["line", "wall_edges", "slow"],
        ["damage", "slow", "zone"],
        ["#f59e0b", "#fef3c7", "#7c2d12"],
        ["floating_blades", "blade_curtain"],
        ["fan_throw", "edge_wall_snap"],
        Active
    ),
    ident!(
        "jhin",
        "Jhin",
        LinearProjectile,
        "jhin_curtain_call",
        ["long_range", "four_shots", "slow"],
        ["damage", "slow", "execute"],
        ["#b91c1c", "#f9a8d4", "#111827"],
        ["theater_mask", "fourth_shot_rose"],
        ["curtain_open", "measured_shots"],
        Active
    ),
    ident!(
        "jinx",
        "Jinx",
        LinearProjectile,
        "jinx_super_mega_death_rocket",
        ["global_missile", "execute", "explosion"],
        ["damage", "aoe_damage"],
        ["#fb7185", "#60a5fa", "#f97316"],
        ["shark_rocket", "graffiti_burst"],
        ["accelerating_rocket", "chaotic_explosion"],
        Active
    ),
    ident!(
        "kled",
        "Kled",
        LinearProjectile,
        "kled_chaaaaaaaarge",
        ["charge", "team_follow", "engage"],
        ["dash", "shield", "damage"],
        ["#d97706", "#ef4444", "#facc15"],
        ["skaarl_trail", "cowardly_banner"],
        ["mount_charge", "dust_plume"],
        Partial
    ),
    ident!(
        "kogmaw",
        "Kog'Maw",
        Artillery,
        "kogmaw_living_artillery",
        ["artillery", "delayed_impact", "void"],
        ["damage", "reveal"],
        ["#a3e635", "#84cc16", "#7e22ce"],
        ["acid_marker", "void_spit"],
        ["high_arc_drop", "splat_impact"],
        Active
    ),
    ident!(
        "maokai",
        "Maokai",
        LinearProjectile,
        "maokai_natures_grasp",
        ["wide_line", "root", "terrain"],
        ["damage", "root"],
        ["#166534", "#22c55e", "#854d0e"],
        ["advancing_roots", "tree_knuckles"],
        ["slow_crawling_wall", "vine_clamp"],
        Active
    ),
    ident!(
        "nami",
        "Nami",
        LinearProjectile,
        "nami_tidal_wave",
        ["wide_line", "knockup", "water"],
        ["damage", "knockup", "slow"],
        ["#2dd4bf", "#a5f3fc", "#0f766e"],
        ["cresting_wave", "foam_pearls"],
        ["rolling_tide", "splash_lift"],
        Active
    ),
    ident!(
        "nautilus",
        "Nautilus",
        LinearProjectile,
        "nautilus_depth_charge",
        ["tracking_line", "knockup", "depth"],
        ["damage", "knockup"],
        ["#0f766e", "#60a5fa", "#334155"],
        ["anchor_ping", "seismic_bubbles"],
        ["underground_chase", "depth_breach"],
        Active
    ),
    ident!(
        "ornn",
        "Ornn",
        LinearProjectile,
        "ornn_call_forge_god",
        ["two_stage_line", "knockup", "forge"],
        ["damage", "knockup", "slow"],
        ["#ef4444", "#f97316", "#57534e"],
        ["lava_ram", "anvil_runes"],
        ["distant_stampede", "headbutt_recast"],
        Partial
    ),
    ident!(
        "poppy",
        "Poppy",
        LinearProjectile,
        "poppy_keepers_verdict",
        ["charged_line", "knockback", "hammer"],
        ["damage", "knockup", "displace"],
        ["#facc15", "#60a5fa", "#7c2d12"],
        ["giant_hammer", "heroic_star"],
        ["charge_windup", "home_run_launch"],
        Partial
    ),
    ident!(
        "qiyana",
        "Qiyana",
        LinearProjectile,
        "qiyana_supreme_display_talent",
        ["line", "terrain_combo", "stun"],
        ["damage", "stun"],
        ["#ec4899", "#facc15", "#06b6d4"],
        ["elemental_ringblade", "terrain_shards"],
        ["wall_ripple", "element_snap"],
        BespokePending
    ),
    ident!(
        "renataglasc",
        "Renata Glasc",
        LinearProjectile,
        "renata_hostile_takeover",
        ["wide_line", "berserk", "chemtech"],
        ["cc", "damage_amp"],
        ["#ec4899", "#a3e635", "#111827"],
        ["perfume_cloud", "chemtech_masks"],
        ["slow_rolling_gas", "mindbreak_flash"],
        Active
    ),
    ident!(
        "sejuani",
        "Sejuani",
        LinearProjectile,
        "sejuani_glacial_prison",
        ["projectile", "stun", "ice_aoe"],
        ["damage", "stun", "slow"],
        ["#93c5fd", "#e0f2fe", "#1e3a8a"],
        ["ice_bola", "boar_tusk"],
        ["bola_throw", "prison_burst"],
        Active
    ),
    ident!(
        "seraphine",
        "Seraphine",
        LinearProjectile,
        "seraphine_encore",
        ["line", "charm", "extend_on_hit"],
        ["damage", "cc"],
        ["#fb7185", "#f0abfc", "#67e8f9"],
        ["soundwave_staff", "heart_notes"],
        ["musical_wave", "encore_extension"],
        Active
    ),
    ident!(
        "smolder",
        "Smolder",
        LinearProjectile,
        "smolder_mmmmom",
        ["wide_line", "dragon_fire", "heal"],
        ["damage", "heal"],
        ["#fb923c", "#facc15", "#dc2626"],
        ["mother_dragon_shadow", "flame_lane"],
        ["protective_flyover", "molten_sweep"],
        Active
    ),
    ident!(
        "sona",
        "Sona",
        LinearProjectile,
        "sona_crescendo",
        ["line", "stun", "music"],
        ["damage", "stun"],
        ["#facc15", "#c084fc", "#60a5fa"],
        ["music_staff", "golden_bars"],
        ["crescendo_wave", "chord_sparkle"],
        Active
    ),
    ident!(
        "tristana",
        "Tristana",
        LinearProjectile,
        "tristana_buster_shot",
        ["single_projectile", "knockback", "yordle_cannon"],
        ["damage", "knockback"],
        ["#f97316", "#fde68a", "#64748b"],
        ["cannon_muzzle", "bomb_smoke"],
        ["point_blank_blast", "recoil_pop"],
        Partial
    ),
    ident!(
        "urgot",
        "Urgot",
        LinearProjectile,
        "urgot_fear_beyond_death",
        ["projectile", "execute", "reel"],
        ["damage", "execute", "fear"],
        ["#6b7280", "#22c55e", "#111827"],
        ["chem_drill", "chain_hook"],
        ["harpoon_shot", "reel_grinder"],
        Partial
    ),
    ident!(
        "varus",
        "Varus",
        LinearProjectile,
        "varus_chain_corruption",
        ["projectile", "root_spread", "darkin"],
        ["damage", "root"],
        ["#7c3aed", "#c084fc", "#111827"],
        ["corruption_chain", "darkin_bow"],
        ["chain_latch", "spreading_tendrils"],
        Active
    ),
    ident!(
        "vex",
        "Vex",
        LinearProjectile,
        "vex_shadow_surge",
        ["projectile", "reset_dash", "gloom"],
        ["damage", "dash"],
        ["#334155", "#a78bfa", "#111827"],
        ["shadow_hand", "gloom_eye"],
        ["shadow_launch", "snap_recast"],
        Partial
    ),
    ident!(
        "warwick",
        "Warwick",
        LinearProjectile,
        "warwick_infinite_duress",
        ["leap_line", "suppression", "lifesteal"],
        ["damage", "heal", "stun"],
        ["#7f1d1d", "#ef4444", "#111827"],
        ["blood_claws", "wolf_lunge"],
        ["predator_leap", "bite_lock"],
        Partial
    ),
    ident!(
        "yone",
        "Yone",
        LinearProjectile,
        "yone_fate_sealed",
        ["dash_line", "knockup", "spirit"],
        ["damage", "knockup", "reposition"],
        ["#dc2626", "#38bdf8", "#111827"],
        ["dual_masks", "spirit_slash"],
        ["blink_cut", "soul_pullback"],
        Partial
    ),
    ident!(
        "yuumi",
        "Yuumi",
        LinearProjectile,
        "yuumi_final_chapter",
        ["waves", "root", "attached_cast"],
        ["damage", "slow", "root"],
        ["#facc15", "#f0abfc", "#60a5fa"],
        ["book_pages", "cat_paw_notes"],
        ["page_flips", "successive_waves"],
        Active
    ),
    ident!(
        "zoe",
        "Zoe",
        LinearProjectile,
        "zoe_portal_jump",
        ["blink_projectile", "trickster", "return"],
        ["reposition"],
        ["#fb7185", "#facc15", "#60a5fa"],
        ["portal_star", "sleepy_sparkles"],
        ["pop_out", "snap_back"],
        Partial
    ),
    ident!(
        "caitlyn",
        "Caitlyn",
        BeamLine,
        "caitlyn_ace_in_the_hole",
        ["beam_line", "lockon", "snipe"],
        ["damage"],
        ["#dc2626", "#f8fafc", "#334155"],
        ["rifle_scope", "red_laser_dot"],
        ["scope_lock", "single_clean_shot"],
        Active
    ),
    ident!(
        "garen",
        "Garen",
        BeamLine,
        "garen_demacian_justice",
        ["vertical_beam", "execute", "demacia"],
        ["damage", "execute"],
        ["#facc15", "#fef3c7", "#2563eb"],
        ["sky_sword", "demacian_wings"],
        ["heaven_drop", "justice_flash"],
        Active
    ),
    ident!(
        "gwen",
        "Gwen",
        BeamLine,
        "gwen_needlework",
        ["needle_volley", "line", "heal"],
        ["damage", "heal"],
        ["#67e8f9", "#f0abfc", "#e0f2fe"],
        ["sewing_needles", "scissor_thread"],
        ["triple_needle_fan", "snip_sparkles"],
        Active
    ),
    ident!(
        "kayle",
        "Kayle",
        BeamLine,
        "kayle_divine_judgment",
        ["delayed_beam", "invulnerability", "swords"],
        ["shield", "damage"],
        ["#facc15", "#fef3c7", "#f97316"],
        ["falling_swords", "angel_wings"],
        ["radiant_delay", "sword_rain"],
        Active
    ),
    ident!(
        "lucian",
        "Lucian",
        BeamLine,
        "lucian_the_culling",
        ["channeled_beam", "bullets", "dash_marksman"],
        ["damage"],
        ["#fde68a", "#facc15", "#111827"],
        ["dual_pistols", "bullet_lanes"],
        ["rapid_barrage", "recoil_stutter"],
        Active
    ),
    ident!(
        "lux",
        "Lux",
        BeamLine,
        "lux_final_spark",
        ["beam_line", "long_range", "light"],
        ["damage"],
        ["#fef3c7", "#fb7185", "#60a5fa"],
        ["prismatic_beam", "wand_star"],
        ["charge_glint", "instant_laser"],
        Active
    ),
    ident!(
        "malzahar",
        "Malzahar",
        BeamLine,
        "malzahar_nether_grasp",
        ["channel_beam", "suppression", "void"],
        ["stun", "damage_over_time"],
        ["#7e22ce", "#c084fc", "#111827"],
        ["void_tether", "nether_eye"],
        ["sustained_channel", "void_drain"],
        Active
    ),
    ident!(
        "missfortune",
        "Miss Fortune",
        BeamLine,
        "missfortune_bullet_time",
        ["cone_beam", "channel", "bullets"],
        ["damage"],
        ["#dc2626", "#f97316", "#facc15"],
        ["bullet_fan", "pirate_crossfire"],
        ["sweeping_cone", "bullet_rain"],
        Active
    ),
    ident!(
        "rumble",
        "Rumble",
        BeamLine,
        "rumble_equalizer",
        ["line_zone", "burn", "slow"],
        ["damage_over_time", "slow"],
        ["#ef4444", "#f97316", "#475569"],
        ["flame_carpet", "mech_targeting"],
        ["dragged_fireline", "napalm_ticks"],
        Active
    ),
    ident!(
        "senna",
        "Senna",
        BeamLine,
        "senna_dawning_shadow",
        ["global_beam", "shield", "damage"],
        ["damage", "shield"],
        ["#111827", "#f8fafc", "#2dd4bf"],
        ["relic_cannon", "mist_cross"],
        ["global_shadow_beam", "ally_shield_wake"],
        Active
    ),
    ident!(
        "taliyah",
        "Taliyah",
        BeamLine,
        "taliyah_weavers_wall",
        ["global_wall_line", "terrain", "ride"],
        ["reposition", "zone"],
        ["#d97706", "#f59e0b", "#94a3b8"],
        ["stone_wall", "woven_thread"],
        ["wall_surge", "stone_ride"],
        BespokePending
    ),
    ident!(
        "velkoz",
        "Vel'Koz",
        BeamLine,
        "velkoz_life_form_disintegration_ray",
        ["channeled_beam", "true_damage", "void"],
        ["damage"],
        ["#f0abfc", "#ec4899", "#7e22ce"],
        ["void_eye_ray", "geometry_lattice"],
        ["tracking_laser", "disintegration_ticks"],
        Active
    ),
    ident!(
        "gangplank",
        "Gangplank",
        GlobalPresence,
        "gangplank_cannon_barrage",
        ["global_zone", "slow", "cannon"],
        ["damage", "slow"],
        ["#f97316", "#111827", "#facc15"],
        ["cannon_crosshair", "powder_smoke"],
        ["map_barrage", "shell_impacts"],
        Active
    ),
    ident!(
        "karthus",
        "Karthus",
        GlobalPresence,
        "karthus_requiem",
        ["global_delayed", "channel", "damage"],
        ["damage"],
        ["#ef4444", "#7c3aed", "#111827"],
        ["death_chorus", "red_omen"],
        ["long_channel", "mapwide_soul_flash"],
        Active
    ),
    ident!(
        "lillia",
        "Lillia",
        GlobalPresence,
        "lillia_lilting_lullaby",
        ["global_sleep", "dream", "delayed_cc"],
        ["stun", "damage_amp"],
        ["#d8b4fe", "#f0abfc", "#86efac"],
        ["dream_spores", "deer_hooves"],
        ["sleepy_drift", "soft_pop"],
        Partial
    ),
    ident!(
        "nocturne",
        "Nocturne",
        GlobalPresence,
        "nocturne_paranoia",
        ["global_blind", "dash_followup", "darkness"],
        ["blind", "damage", "dash"],
        ["#020617", "#7f1d1d", "#334155"],
        ["blackout_vignette", "nightmare_blades"],
        ["lights_out", "predator_dive"],
        BespokePending
    ),
    ident!(
        "pantheon",
        "Pantheon",
        GlobalPresence,
        "pantheon_grand_starfall",
        ["global_delayed", "landing_line", "spear"],
        ["damage", "reposition"],
        ["#f97316", "#facc15", "#7c2d12"],
        ["falling_spear", "starfall_lane"],
        ["sky_channel", "meteor_landing"],
        Partial
    ),
    ident!(
        "ryze",
        "Ryze",
        GlobalPresence,
        "ryze_realm_warp",
        ["global_portal", "team_reposition", "delay"],
        ["reposition"],
        ["#2563eb", "#60a5fa", "#facc15"],
        ["rune_portal", "arcane_circle"],
        ["portal_channel", "team_blink"],
        BespokePending
    ),
    ident!(
        "shen",
        "Shen",
        GlobalPresence,
        "shen_stand_united",
        ["global_shield", "ally_dash", "delay"],
        ["shield", "reposition"],
        ["#a855f7", "#e0f2fe", "#111827"],
        ["spirit_blade", "protective_eye"],
        ["ally_channel", "spirit_arrival"],
        BespokePending
    ),
    ident!(
        "soraka",
        "Soraka",
        GlobalPresence,
        "soraka_wish",
        ["global_heal", "celestial", "team_save"],
        ["heal"],
        ["#22c55e", "#fef3c7", "#c084fc"],
        ["falling_stars", "crescent_staff"],
        ["mapwide_twinkle", "gentle_heal_flash"],
        Active
    ),
    ident!(
        "twistedfate",
        "Twisted Fate",
        GlobalPresence,
        "twistedfate_destiny",
        ["global_vision", "teleport", "cards"],
        ["reveal", "reposition"],
        ["#facc15", "#ef4444", "#111827"],
        ["card_eye", "destiny_gate"],
        ["vision_flip", "gold_card_portal"],
        BespokePending
    ),
    ident!(
        "xerath",
        "Xerath",
        GlobalPresence,
        "xerath_rite_arcane",
        ["global_artillery", "recast_shots", "arcane"],
        ["damage"],
        ["#38bdf8", "#60a5fa", "#fef3c7"],
        ["arcane_sigil", "sky_bolts"],
        ["ascended_channel", "repeated_orbital_shots"],
        Active
    ),
    ident!(
        "ahri",
        "Ahri",
        BlinkBurst,
        "ahri_spirit_rush",
        ["mobility", "recast_dash", "foxfire"],
        ["dash", "damage"],
        ["#f9a8d4", "#fef3c7", "#60a5fa"],
        ["nine_tail_arc", "heart_wisp"],
        ["triple_blink", "charm_spark"],
        Partial
    ),
    ident!(
        "akali",
        "Akali",
        AssassinMark,
        "akali_perfect_execution",
        ["assassin", "execute", "recast_dash"],
        ["dash", "execute_damage"],
        ["#22d3ee", "#a3e635", "#111827"],
        ["kunai_cross", "smoke_slash"],
        ["two_stage_cut", "neon_afterimage"],
        Active
    ),
    ident!(
        "ambessa",
        "Ambessa",
        TargetedDash,
        "ambessa_public_execution",
        ["targeted_dash", "suppression", "duel_pressure"],
        ["dash", "lockdown", "damage"],
        ["#7f1d1d", "#f97316", "#292524"],
        ["noxian_hook_line", "twin_drake_blades"],
        ["marked_lunge", "drag_back_impact"],
        Partial
    ),
    ident!(
        "briar",
        "Briar",
        TargetedDash,
        "briar_certain_death",
        ["long_range_dash", "frenzy", "single_target"],
        ["dash", "fear", "damage"],
        ["#be123c", "#fca5a5", "#1f2937"],
        ["blood_fangs", "pillory_shard"],
        ["screaming_lock_on", "feeding_crash"],
        Partial
    ),
    ident!(
        "camille",
        "Camille",
        SuppressionLock,
        "camille_hextech_ultimatum",
        ["single_target", "lockdown", "duel_cage"],
        ["untargetable", "lockdown", "damage"],
        ["#38bdf8", "#f8fafc", "#1e3a8a"],
        ["hextech_hexagon", "leg_blade_cross"],
        ["precision_leap", "cage_snap"],
        Partial
    ),
    ident!(
        "chogath",
        "Cho'Gath",
        ExecuteMarker,
        "chogath_feast",
        ["execute", "single_target", "monster_growth"],
        ["true_damage", "stack_growth"],
        ["#a855f7", "#ef4444", "#312e81"],
        ["void_jaws", "growth_spikes"],
        ["bite_lunge", "devour_flash"],
        Active
    ),
    ident!(
        "darius",
        "Darius",
        ExecuteMarker,
        "darius_noxian_guillotine",
        ["execute", "reset", "single_target"],
        ["true_damage", "reset"],
        ["#991b1b", "#facc15", "#111827"],
        ["noxian_axe", "bleed_stacks"],
        ["vertical_guillotine", "reset_thunder"],
        Active
    ),
    ident!(
        "evelynn",
        "Evelynn",
        BlinkBurst,
        "evelynn_last_caress",
        ["execute", "blink_back", "assassin"],
        ["damage", "reposition"],
        ["#db2777", "#7c3aed", "#020617"],
        ["heart_sigil", "demon_lash"],
        ["seductive_flash", "backflip_fade"],
        Active
    ),
    ident!(
        "fiddlesticks",
        "Fiddlesticks",
        BlinkBurst,
        "fiddlesticks_crowstorm",
        ["blink_channel", "aoe_fear", "ambush"],
        ["reposition", "damage", "fear"],
        ["#111827", "#f97316", "#facc15"],
        ["murder_crows", "scarecrow_eye"],
        ["channel_portal", "crow_explosion"],
        Partial
    ),
    ident!(
        "jarvaniv",
        "Jarvan IV",
        TargetedDash,
        "jarvaniv_cataclysm",
        ["engage", "terrain_cage", "single_target"],
        ["dash", "damage", "terrain"],
        ["#facc15", "#92400e", "#e5e7eb"],
        ["demacian_standard", "stone_crater"],
        ["royal_leap", "wall_erupt"],
        Partial
    ),
    ident!(
        "kaisa",
        "Kai'Sa",
        TargetedDash,
        "kaisa_killer_instinct",
        ["targeted_dash", "shield", "plasma_mark"],
        ["dash", "shield"],
        ["#a855f7", "#38bdf8", "#f0abfc"],
        ["void_wing_dash", "plasma_ring"],
        ["marked_arrival", "living_suit_burst"],
        Active
    ),
    ident!(
        "kassadin",
        "Kassadin",
        BlinkBurst,
        "kassadin_riftwalk",
        ["blink", "stacking_damage", "void"],
        ["reposition", "damage"],
        ["#6d28d9", "#60a5fa", "#111827"],
        ["void_rift", "crescent_blade"],
        ["short_blink", "rift_echo"],
        Active
    ),
    ident!(
        "kayn",
        "Kayn",
        AssassinMark,
        "kayn_umbral_trespass",
        ["untargetable", "single_target", "form_identity"],
        ["stasis", "damage", "heal"],
        ["#1d4ed8", "#dc2626", "#020617"],
        ["shadow_scythe", "body_possession"],
        ["inside_target", "scythe_exit"],
        BespokePending
    ),
    ident!(
        "khazix",
        "Kha'Zix",
        AssassinMark,
        "khazix_void_assault",
        ["stealth", "recast_mobility", "isolation"],
        ["stealth", "speed"],
        ["#7c3aed", "#a3e635", "#111827"],
        ["evolved_claws", "void_carapace"],
        ["stealth_flicker", "predator_recast"],
        Partial
    ),
    ident!(
        "leesin",
        "Lee Sin",
        TargetedDash,
        "leesin_dragons_rage",
        ["single_target", "kick", "displacement"],
        ["knockback", "damage"],
        ["#f97316", "#fef3c7", "#7c2d12"],
        ["dragon_kick", "martial_ring"],
        ["snap_kick", "shockwave_line"],
        Active
    ),
    ident!(
        "lissandra",
        "Lissandra",
        SuppressionLock,
        "lissandra_frozen_tomb",
        ["single_target", "self_stasis", "ice_lock"],
        ["stasis", "slow", "damage"],
        ["#67e8f9", "#e0f2fe", "#1e3a8a"],
        ["ice_sarcophagus", "black_ice_ring"],
        ["frozen_snap", "glacial_spread"],
        Active
    ),
    ident!(
        "malphite",
        "Malphite",
        UnstoppableCharge,
        "malphite_unstoppable_force",
        ["unstoppable_charge", "aoe_knockup", "engage"],
        ["dash", "knockup", "damage"],
        ["#78716c", "#f97316", "#fde68a"],
        ["mountain_shard", "impact_crater"],
        ["unstoppable_arc", "seismic_impact"],
        Active
    ),
    ident!(
        "mordekaiser",
        "Mordekaiser",
        DuelRealm,
        "mordekaiser_realm_death",
        ["duel_realm", "single_target", "stat_steal"],
        ["isolate", "debuff"],
        ["#16a34a", "#111827", "#86efac"],
        ["death_realm_gate", "iron_crown"],
        ["realm_overlay", "duel_cage_close"],
        BespokePending
    ),
    ident!(
        "pyke",
        "Pyke",
        ExecuteMarker,
        "pyke_death_from_below",
        ["execute", "reset", "x_marker"],
        ["execute_damage", "blink", "reset"],
        ["#14b8a6", "#7dd3fc", "#022c22"],
        ["drowned_x", "harpoon_wake"],
        ["x_flash", "undertow_blink"],
        Active
    ),
    ident!(
        "reksai",
        "Rek'Sai",
        TargetedDash,
        "reksai_void_rush",
        ["targeted_dash", "burrow", "execute_pressure"],
        ["dash", "damage"],
        ["#7c3aed", "#f97316", "#1e1b4b"],
        ["tunnel_teeth", "tremor_target"],
        ["submerge_lock", "void_breach"],
        Partial
    ),
    ident!(
        "sett",
        "Sett",
        SuppressionLock,
        "sett_show_stopper",
        ["suppression", "slam", "frontline_displacement"],
        ["grab", "reposition", "damage"],
        ["#f97316", "#fca5a5", "#7f1d1d"],
        ["knuckle_grab", "arena_slam"],
        ["carry_arc", "groundbreaker"],
        Active
    ),
    ident!(
        "shyvana",
        "Shyvana",
        UnstoppableCharge,
        "shyvana_dragons_descent",
        ["transform", "unstoppable_charge", "dragon"],
        ["dash", "transform", "damage"],
        ["#f97316", "#dc2626", "#111827"],
        ["dragon_wings", "flame_landing"],
        ["wing_surge", "dragon_crash"],
        Partial
    ),
    ident!(
        "sion",
        "Sion",
        UnstoppableCharge,
        "sion_unstoppable_onslaught",
        ["unstoppable_charge", "steering", "collision"],
        ["dash", "knockup", "damage"],
        ["#7f1d1d", "#9ca3af", "#111827"],
        ["warpath_tracks", "undead_horns"],
        ["long_charge", "collision_burst"],
        Partial
    ),
    ident!(
        "sylas",
        "Sylas",
        TargetedDash,
        "sylas_hijack",
        ["hijack", "copied_ultimate", "single_target"],
        ["steal_spell"],
        ["#60a5fa", "#f8fafc", "#7c3aed"],
        ["broken_chains", "stolen_crown"],
        ["chain_latch", "identity_mirror"],
        BespokePending
    ),
    ident!(
        "trundle",
        "Trundle",
        SuppressionLock,
        "trundle_subjugate",
        ["single_target", "stat_drain", "duel_debuff"],
        ["drain", "debuff"],
        ["#38bdf8", "#64748b", "#1e3a8a"],
        ["ice_crown", "troll_club"],
        ["frost_drain", "kingdom_claim"],
        Active
    ),
    ident!(
        "veigar",
        "Veigar",
        ExecuteMarker,
        "veigar_primordial_burst",
        ["single_target", "missing_health_execute", "dark_magic"],
        ["damage", "execute_scaling"],
        ["#7c3aed", "#facc15", "#020617"],
        ["primordial_star", "tiny_master_crown"],
        ["dark_orb_drop", "villainous_pop"],
        Active
    ),
    ident!(
        "vi",
        "Vi",
        TargetedDash,
        "vi_cease_desist",
        ["targeted_dash", "unstoppable", "single_target_lock"],
        ["dash", "knockup", "damage"],
        ["#ec4899", "#60a5fa", "#1f2937"],
        ["hextech_gauntlet", "punch_lane"],
        ["locked_pursuit", "uppercut_impact"],
        Active
    ),
    ident!(
        "viego",
        "Viego",
        AssassinMark,
        "viego_heartbreaker",
        ["execute", "possession_reset", "assassin"],
        ["blink", "damage", "reset"],
        ["#86efac", "#14b8a6", "#111827"],
        ["ruined_blade", "mist_crown"],
        ["heartbreaker_blink", "possession_mist"],
        BespokePending
    ),
    ident!(
        "yasuo",
        "Yasuo",
        TargetedDash,
        "yasuo_last_breath",
        ["knockup_dependency", "blink_dash", "airborne_lock"],
        ["dash", "suspend", "damage"],
        ["#93c5fd", "#f8fafc", "#1e3a8a"],
        ["wind_slashes", "airborne_ring"],
        ["blink_to_knockup", "suspended_cuts"],
        BespokePending
    ),
    ident!(
        "zed",
        "Zed",
        AssassinMark,
        "zed_death_mark",
        ["assassin_mark", "delayed_pop", "shadow_swap"],
        ["mark", "reposition", "damage"],
        ["#111827", "#ef4444", "#64748b"],
        ["shadow_clone", "death_mark_shuriken"],
        ["shadow_arrival", "delayed_pop"],
        Active
    ),
    ident!(
        "aurelionsol",
        "Aurelion Sol",
        Artillery,
        "aurelionsol_falling_star",
        ["artillery", "delayed_impact", "cosmic"],
        ["damage", "stun"],
        ["#f97316", "#facc15", "#312e81"],
        ["star_dragon", "meteor_disc"],
        ["orbital_descent", "cosmic_aftershock"],
        Active
    ),
    ident!(
        "azir",
        "Azir",
        ZoneSummon,
        "azir_emperors_divide",
        ["wall", "soldiers", "displacement"],
        ["knockback", "damage", "zone"],
        ["#d97706", "#facc15", "#fef3c7"],
        ["sand_soldier_wall", "shuriman_sun_disc"],
        ["phalanx_surge", "sand_wall_rise"],
        BespokePending
    ),
    ident!(
        "bardo",
        "Bardo",
        GlobalPresence,
        "bard_tempered_fate",
        ["global_stasis", "delayed_impact", "cosmic"],
        ["stasis", "zone"],
        ["#facc15", "#7c3aed", "#60a5fa"],
        ["cosmic_chime", "stasis_hourglass"],
        ["slow_arc", "golden_freeze"],
        BespokePending
    ),
    ident!(
        "ekko",
        "Ekko",
        BlinkBurst,
        "ekko_chronobreak",
        ["rewind", "self_heal", "delayed_burst"],
        ["heal", "damage", "reposition"],
        ["#22d3ee", "#a3e635", "#111827"],
        ["time_ghost", "clock_hand"],
        ["rewind_snap", "afterimage_pop"],
        Partial
    ),
    ident!(
        "fizz",
        "Fizz",
        ExecuteMarker,
        "fizz_chum_waters",
        ["delayed_marker", "shark", "knockup"],
        ["damage", "knockup", "slow"],
        ["#2563eb", "#38bdf8", "#f97316"],
        ["shark_fin", "water_bait_mark"],
        ["bait_latch", "shark_breach"],
        Partial
    ),
    ident!(
        "galio",
        "Galio",
        GlobalPresence,
        "galio_heroes_entrance",
        ["global_ally", "shield", "landing_knockup"],
        ["shield", "dash", "knockup"],
        ["#f8fafc", "#60a5fa", "#facc15"],
        ["colossus_wings", "demacian_impact"],
        ["sky_channel", "heroic_crash"],
        Partial
    ),
    ident!(
        "hecarim",
        "Hecarim",
        UnstoppableCharge,
        "hecarim_onslaught_shadows",
        ["charge", "fear", "spectral_cavalry"],
        ["dash", "fear", "damage"],
        ["#0f766e", "#14b8a6", "#111827"],
        ["ghost_horse", "shadow_riders"],
        ["cavalry_surge", "terror_wake"],
        Active
    ),
    ident!(
        "kalista",
        "Kalista",
        TargetedDash,
        "kalista_fates_call",
        ["ally_reposition", "knockup", "oathsworn"],
        ["dash", "knockup", "save"],
        ["#22c55e", "#67e8f9", "#0f172a"],
        ["soul_spear", "oathsworn_tether"],
        ["ally_pull", "spear_launch"],
        BespokePending
    ),
    ident!(
        "karma",
        "Karma",
        SelfAura,
        "karma_mantra",
        ["empower_next_spell", "stance", "spirit"],
        ["damage_amp", "shield_amp", "heal_amp"],
        ["#22c55e", "#facc15", "#f8fafc"],
        ["twin_dragons", "ionian_mandala"],
        ["mantra_bloom", "focused_breath"],
        BespokePending
    ),
    ident!(
        "kindred",
        "Kindred",
        AllyAura,
        "kindred_lambs_respite",
        ["zone", "death_prevention", "heal"],
        ["shield", "heal", "death_prevention"],
        ["#e0f2fe", "#111827", "#f8fafc"],
        ["lamb_mask", "wolf_orbit"],
        ["sanctuary_pulse", "threshold_heal"],
        BespokePending
    ),
    ident!(
        "ksante",
        "K'Sante",
        DuelRealm,
        "ksante_all_out",
        ["duel_displacement", "form_shift", "single_target"],
        ["dash", "damage", "form_swap"],
        ["#d97706", "#facc15", "#38bdf8"],
        ["ntofo_blades", "wall_break"],
        ["drag_through_wall", "all_out_flash"],
        BespokePending
    ),
    ident!(
        "leblanc",
        "LeBlanc",
        BlinkBurst,
        "leblanc_mimic",
        ["copied_spell", "deception", "mirror"],
        ["damage", "spell_replay"],
        ["#ec4899", "#7c3aed", "#f8fafc"],
        ["mirror_sigils", "rose_clone"],
        ["spell_echo", "deceptive_flash"],
        BespokePending
    ),
    ident!(
        "leona",
        "Leona",
        Artillery,
        "leona_solar_flare",
        ["delayed_impact", "stun", "solar"],
        ["stun", "slow", "damage"],
        ["#facc15", "#fef3c7", "#fb923c"],
        ["sunburst_cross", "zenith_disc"],
        ["solar_descent", "radiant_snap"],
        Active
    ),
    ident!(
        "lulu",
        "Lulu",
        AllyAura,
        "lulu_wild_growth",
        ["ally_save", "knockup", "growth"],
        ["heal", "knockup", "slow"],
        ["#c084fc", "#86efac", "#f0abfc"],
        ["pix_sparkle", "giant_leaf"],
        ["sudden_growth", "fae_pop"],
        Active
    ),
    ident!(
        "nidalee",
        "Nidalee",
        SelfAura,
        "nidalee_aspect_cougar",
        ["stance", "transform", "huntress"],
        ["form_swap", "damage"],
        ["#f97316", "#facc15", "#166534"],
        ["cougar_paws", "spear_moon"],
        ["feral_shift", "pounce_ready"],
        Partial
    ),
    ident!(
        "nunuywillump",
        "Nunu y Willump",
        AoePulse,
        "nunu_absolute_zero",
        ["channel", "huge_aoe", "slow"],
        ["damage", "slow"],
        ["#a5f3fc", "#e0f2fe", "#60a5fa"],
        ["snowflake_ring", "yeti_roar"],
        ["channel_grow", "frozen_blast"],
        Partial
    ),
    ident!(
        "syndra",
        "Syndra",
        ExecuteMarker,
        "syndra_unleashed_power",
        ["single_target", "sphere_count", "burst"],
        ["damage"],
        ["#7c3aed", "#111827", "#c084fc"],
        ["dark_spheres", "sovereign_crown"],
        ["sphere_barrage", "telekinetic_crush"],
        Active
    ),
    ident!(
        "xayah",
        "Xayah",
        BlinkBurst,
        "xayah_featherstorm",
        ["untargetable", "feathers", "reposition"],
        ["shield", "damage", "feather_recall"],
        ["#ec4899", "#f9a8d4", "#111827"],
        ["feather_fan", "rebel_wings"],
        ["airborne_flare", "feather_fall"],
        Partial
    ),
    ident!(
        "xinzhao",
        "Xin Zhao",
        AllyAura,
        "xinzhao_crescent_guard",
        ["self_guard", "knockback", "duel_zone"],
        ["shield", "knockback", "damage"],
        ["#facc15", "#e0f2fe", "#2563eb"],
        ["crescent_spear", "guard_ring"],
        ["sweep_circle", "protective_dome"],
        Partial
    ),
    ident!(
        "yunara",
        "Yunara",
        AoePulse,
        "yunara_pack_surge",
        ["aoe", "slow", "pack_identity"],
        ["damage", "slow"],
        ["#cd5c5c", "#fca5a5", "#7f1d1d"],
        ["pack_sigil", "scarlet_orbit"],
        ["pack_collapse", "red_wave"],
        BespokePending
    ),
    ident!(
        "zaahen",
        "Zaahen",
        AoePulse,
        "zaahen_annihilation",
        ["aoe", "invulnerability", "annihilation"],
        ["shield", "damage"],
        ["#020617", "#f8fafc", "#7c3aed"],
        ["black_sun", "void_crown"],
        ["annihilation_charge", "dark_flash"],
        BespokePending
    ),
    ident!(
        "zilean",
        "Zilean",
        AllyAura,
        "zilean_chrono_shift",
        ["revive", "single_target_save", "time"],
        ["shield", "heal", "death_prevention"],
        ["#d97706", "#facc15", "#60a5fa"],
        ["clock_runes", "rewind_hourglass"],
        ["time_anchor", "revive_rewind"],
        BespokePending
    ),
];

pub(super) fn normalize_champion_key(value: &str) -> String {
    value
        .to_lowercase()
        .replace(|ch: char| !ch.is_ascii_alphanumeric(), "")
}

pub(super) fn ultimate_identity_for(champion_id: &str) -> Option<&'static UltimateIdentity> {
    let normalized = normalize_champion_key(champion_id);
    ULTIMATE_IDENTITIES
        .iter()
        .find(|identity| identity.champion_key == normalized)
}

pub(super) fn ultimate_identity_value(identity: &UltimateIdentity) -> Value {
    json!(identity)
}

fn ultimate_timing_defaults(
    identity: &UltimateIdentity,
    spatial: &UltimateCastSpatialMetadata,
) -> (u32, u32, u32, bool, u32) {
    let signature = identity.signature_id;
    let tags = identity.gameplay_tags;
    let delayed_ground_aoe = matches!(
        signature,
        "leona_solar_flare"
            | "ziggs_mega_inferno_bomb"
            | "aurelionsol_falling_star"
            | "bard_tempered_fate"
            | "karthus_requiem"
            | "kayle_divine_judgment"
            | "taric_cosmic_radiance"
            | "zyra_stranglethorns"
            | "nunu_absolute_zero"
            | "fizz_chum_waters"
            | "xerath_rite_arcane"
            | "gangplank_cannon_barrage"
    ) || tags.iter().any(|tag| tag.contains("delayed"));
    let persistent_zone = matches!(
        signature,
        "anivia_glacial_storm"
            | "rumble_equalizer"
            | "viktor_chaos_storm"
            | "kindred_lambs_respite"
            | "morgana_soul_shackles"
            | "fiddlesticks_crowstorm"
            | "kennen_slicing_maelstrom"
            | "swain_demonic_ascension"
            | "janna_monsoon"
            | "nunu_absolute_zero"
            | "gangplank_cannon_barrage"
            | "zyra_stranglethorns"
    ) || matches!(spatial.shape, UltimateSpatialShape::Zone)
        || tags.iter().any(|tag| tag == &"zone" || tag.contains("global_zone") || tag.contains("line_zone"));
    let channel = matches!(
        signature,
        "lucian_the_culling"
            | "missfortune_bullet_time"
            | "velkoz_life_form_disintegration_ray"
            | "katarina_death_lotus"
            | "samira_inferno_trigger"
            | "xerath_rite_arcane"
    ) || tags.iter().any(|tag| tag.contains("channel"));

    let delay_ms = if delayed_ground_aoe {
        match signature {
            "karthus_requiem" | "taric_cosmic_radiance" => 2400,
            "nunu_absolute_zero" => 1800,
            "bard_tempered_fate" => 900,
            "gangplank_cannon_barrage" | "xerath_rite_arcane" => 650,
            _ => 750,
        }
    } else {
        spatial.delay_ms.unwrap_or(250)
    };
    let duration_ms = if persistent_zone {
        match signature {
            "anivia_glacial_storm" | "rumble_equalizer" | "swain_demonic_ascension" => 5200,
            "gangplank_cannon_barrage" | "kindred_lambs_respite" => 4400,
            "nunu_absolute_zero" | "janna_monsoon" => 3600,
            _ => 3200,
        }
    } else if channel {
        match signature {
            "lucian_the_culling" | "missfortune_bullet_time" | "velkoz_life_form_disintegration_ray" => 3000,
            "xerath_rite_arcane" => 4200,
            _ => 2400,
        }
    } else {
        spatial.duration_ms.unwrap_or(1700)
    };
    let impact_at = spatial.impact_at.unwrap_or(delay_ms);
    let persistent = spatial.persistent.unwrap_or(persistent_zone);
    let pulse_count = spatial.pulse_count.unwrap_or_else(|| {
        if channel || persistent {
            (duration_ms / 450).clamp(3, 12)
        } else if delayed_ground_aoe {
            2
        } else {
            1
        }
    });

    (delay_ms, duration_ms, impact_at, persistent, pulse_count)
}

fn ultimate_target_ids(spatial: &UltimateCastSpatialMetadata) -> Option<Vec<String>> {
    spatial.target_ids.clone().or_else(|| {
        spatial
            .target_id
            .as_ref()
            .map(|target_id| vec![target_id.clone()])
    })
}

fn is_target_lock_signature(identity: &UltimateIdentity, spatial: &UltimateCastSpatialMetadata) -> bool {
    matches!(
        identity.technical_primitive,
        UltimatePrimitive::TargetedDash
            | UltimatePrimitive::ExecuteMarker
            | UltimatePrimitive::SuppressionLock
            | UltimatePrimitive::DuelRealm
            | UltimatePrimitive::AssassinMark
    ) || matches!(spatial.shape, UltimateSpatialShape::Lock)
        || identity
            .gameplay_tags
            .iter()
            .any(|tag| tag.contains("target") || tag.contains("lock") || tag.contains("execute"))
}

fn inferred_sequence_kind(identity: &UltimateIdentity) -> Option<&'static str> {
    match identity.signature_id {
        "ahri_spirit_rush" | "akali_perfect_execution" | "wukong_cyclone" | "zoe_portal_jump" => Some("recast"),
        "jhin_curtain_call" | "xerath_rite_arcane" | "lucian_the_culling" | "missfortune_bullet_time" => Some("multi_shot_channel"),
        "draven_whirling_death" | "talon_shadow_assault" | "xayah_featherstorm" => Some("return_path"),
        "brand_pyroclasm_bounce" | "seraphine_encore" | "varus_chain_corruption" | "morgana_soul_shackles" => Some("chain"),
        _ => None,
    }
}

fn inferred_stage_count(identity: &UltimateIdentity) -> Option<u32> {
    match identity.signature_id {
        "jhin_curtain_call" => Some(4),
        "xerath_rite_arcane" => Some(4),
        "ahri_spirit_rush" => Some(3),
        "akali_perfect_execution" | "wukong_cyclone" => Some(2),
        "lucian_the_culling" => Some(8),
        "missfortune_bullet_time" => Some(6),
        _ => None,
    }
}

fn inferred_recast_window_ms(identity: &UltimateIdentity) -> Option<u32> {
    match identity.signature_id {
        "ahri_spirit_rush" => Some(10000),
        "akali_perfect_execution" => Some(2500),
        "wukong_cyclone" => Some(8000),
        "zoe_portal_jump" => Some(900),
        _ => None,
    }
}

fn inferred_tether_kind(identity: &UltimateIdentity) -> Option<&'static str> {
    match identity.signature_id {
        "malzahar_nether_grasp" => Some("suppression_channel"),
        "morgana_soul_shackles" => Some("soul_chain"),
        "camille_hextech_ultimatum" => Some("duel_lock"),
        "warwick_infinite_duress" | "skarner_impale" => Some("suppress_tether"),
        "lulu_wild_growth" | "zilean_chrono_shift" | "tahmkench_devour" => Some("ally_tether"),
        _ => None,
    }
}

fn inferred_bespoke_kind(identity: &UltimateIdentity) -> Option<&'static str> {
    match identity.signature_id {
        "sylas_hijack" => Some("stolen_ultimate_pending"),
        "mordekaiser_realm_death" => Some("death_realm"),
        "ryze_realm_warp" => Some("portal"),
        "twistedfate_destiny" => Some("global_reveal_gate"),
        "shen_stand_united" => Some("ally_shield_arrival"),
        "kindred_lambs_respite" => Some("sanctuary_heal"),
        "taliyah_weavers_wall" => Some("terrain_wall"),
        "azir_emperors_divide" => Some("soldier_wall"),
        "yasuo_last_breath" => Some("airborne_slash"),
        "orianna_command_shockwave" => Some("proxy_shockwave"),
        "ornn_call_forge_god" => Some("two_stage_ram"),
        "nocturne_paranoia" => Some("blackout_dash"),
        "galio_heroes_entrance" | "pantheon_grand_starfall" => Some("global_landing"),
        "ekko_chronobreak" => Some("rewind_ghost"),
        "xayah_featherstorm" => Some("feather_fan_recall"),
        "yuumi_final_chapter" => Some("host_waves"),
        _ => None,
    }
}

fn inferred_requires_condition(identity: &UltimateIdentity) -> Option<&'static str> {
    match identity.signature_id {
        "yasuo_last_breath" => Some("target_airborne"),
        "sylas_hijack" => Some("copyable_enemy_ultimate"),
        _ => None,
    }
}

fn inferred_proxy_origin_kind(identity: &UltimateIdentity) -> Option<&'static str> {
    match identity.signature_id {
        "orianna_command_shockwave" => Some("ball_or_target_point"),
        "yuumi_final_chapter" => Some("attached_ally_host"),
        "ekko_chronobreak" => Some("previous_position_ghost"),
        _ => None,
    }
}

fn inferred_global(identity: &UltimateIdentity) -> Option<bool> {
    match identity.signature_id {
        "nocturne_paranoia"
        | "twistedfate_destiny"
        | "ryze_realm_warp"
        | "shen_stand_united"
        | "galio_heroes_entrance"
        | "pantheon_grand_starfall" => Some(true),
        _ => None,
    }
}

fn inferred_zone_orientation(identity: &UltimateIdentity, spatial: &UltimateCastSpatialMetadata) -> Option<Vec2> {
    spatial.zone_orientation.or_else(|| match identity.signature_id {
        "taliyah_weavers_wall" | "azir_emperors_divide" => spatial.direction,
        _ => None,
    })
}

fn inferred_destination_pos(identity: &UltimateIdentity, spatial: &UltimateCastSpatialMetadata) -> Option<Vec2> {
    spatial.destination_pos.or_else(|| match identity.signature_id {
        "ryze_realm_warp"
        | "twistedfate_destiny"
        | "shen_stand_united"
        | "galio_heroes_entrance"
        | "pantheon_grand_starfall"
        | "nocturne_paranoia" => spatial.target_pos,
        _ => None,
    })
}

pub(super) fn ultimate_cast_event_metadata(
    identity: &UltimateIdentity,
    actor_id: &str,
    spatial: &UltimateCastSpatialMetadata,
) -> Value {
    let (delay_ms, duration_ms, impact_at, persistent, pulse_count) =
        ultimate_timing_defaults(identity, spatial);
    let target_ids = ultimate_target_ids(spatial);
    let affected_target_ids = spatial.affected_target_ids.clone().or_else(|| target_ids.clone());
    let locked_target_id = spatial.locked_target_id.clone().or_else(|| {
        if is_target_lock_signature(identity, spatial) {
            spatial.target_id.clone()
        } else {
            None
        }
    });
    let follow_target = spatial.follow_target.unwrap_or_else(|| {
        spatial.target_id.is_some()
            && (matches!(spatial.shape, UltimateSpatialShape::Lock)
                || matches!(
                    identity.technical_primitive,
                    UltimatePrimitive::TargetedDash
                        | UltimatePrimitive::ExecuteMarker
                        | UltimatePrimitive::SuppressionLock
                        | UltimatePrimitive::AssassinMark
                ))
    });
    let stage_count = spatial.stage_count.or_else(|| inferred_stage_count(identity));
    let stage = spatial.stage.or_else(|| stage_count.map(|_| 1));
    let sequence_kind = spatial.sequence_kind.or_else(|| inferred_sequence_kind(identity));
    let return_to_origin = spatial.return_to_origin.or_else(|| {
        matches!(sequence_kind, Some("return_path")).then_some(true)
    });
    let bounce_count = spatial.bounce_count.or_else(|| {
        matches!(sequence_kind, Some("chain")).then_some(affected_target_ids.as_ref().map_or(3, |ids| ids.len().max(2) as u32))
    });
    let tether_kind = spatial.tether_kind.or_else(|| inferred_tether_kind(identity));
    let bespoke_kind = spatial.bespoke_kind.or_else(|| inferred_bespoke_kind(identity));
    let destination_pos = inferred_destination_pos(identity, spatial);
    let zone_orientation = inferred_zone_orientation(identity, spatial);
    let requires_condition = spatial
        .requires_condition
        .or_else(|| inferred_requires_condition(identity));
    let proxy_origin_kind = spatial
        .proxy_origin_kind
        .or_else(|| inferred_proxy_origin_kind(identity));
    let global = spatial.global.or_else(|| inferred_global(identity));
    json!({
        "event": "champion_ultimate_cast",
        "actorId": actor_id,
        "championId": identity.champion_key,
        "originPos": spatial.origin_pos,
        "bespokeKind": bespoke_kind,
        "secondaryPos": spatial.secondary_pos,
        "destinationPos": destination_pos,
        "zoneOrientation": zone_orientation,
        "requiresCondition": requires_condition,
        "proxyOriginKind": proxy_origin_kind,
        "targetId": spatial.target_id,
        "lockedTargetId": locked_target_id,
        "targetIds": target_ids,
        "affectedTargetIds": affected_target_ids,
        "targetPos": spatial.target_pos,
        "direction": spatial.direction,
        "shape": spatial.shape,
        "radius": spatial.radius,
        "width": spatial.width,
        "range": spatial.range,
        "delayMs": delay_ms,
        "durationMs": duration_ms,
        "impactAt": impact_at,
        "persistent": persistent,
        "pulseCount": pulse_count,
        "travelSpeed": spatial.travel_speed,
        "followTarget": follow_target,
        "stage": stage,
        "stageCount": stage_count,
        "sequenceKind": sequence_kind,
        "returnPath": spatial.return_path,
        "returnToOrigin": return_to_origin,
        "bounceTargets": spatial.bounce_targets,
        "bounceCount": bounce_count,
        "recastWindowMs": spatial.recast_window_ms.or_else(|| inferred_recast_window_ms(identity)),
        "tetherKind": tether_kind,
        "global": global,
        "ultimateIdentity": identity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn base_spatial() -> UltimateCastSpatialMetadata {
        UltimateCastSpatialMetadata {
            origin_pos: Vec2 { x: 0.25, y: 0.4 },
            bespoke_kind: None,
            secondary_pos: None,
            destination_pos: None,
            zone_orientation: None,
            requires_condition: None,
            proxy_origin_kind: None,
            target_id: Some("red-mid".to_string()),
            locked_target_id: None,
            target_ids: None,
            affected_target_ids: None,
            target_pos: Some(Vec2 { x: 0.75, y: 0.45 }),
            direction: Some(Vec2 { x: 1.0, y: 0.0 }),
            shape: UltimateSpatialShape::Circle,
            radius: Some(0.1),
            width: None,
            range: Some(0.7),
            delay_ms: None,
            duration_ms: None,
            impact_at: None,
            persistent: None,
            pulse_count: None,
            travel_speed: None,
            follow_target: None,
            stage: None,
            stage_count: None,
            sequence_kind: None,
            return_path: None,
            return_to_origin: None,
            bounce_targets: None,
            bounce_count: None,
            recast_window_ms: None,
            tether_kind: None,
            global: Some(false),
        }
    }

    #[test]
    fn batch_1_catalog_covers_requested_champions() {
        for champion in BATCH_1_CHAMPIONS {
            assert!(
                ultimate_identity_for(champion).is_some(),
                "missing {champion}"
            );
        }
    }

    #[test]
    fn batch_2_catalog_covers_requested_projectile_beam_and_global_champions() {
        for champion in BATCH_2_CHAMPIONS {
            assert!(
                ultimate_identity_for(champion).is_some(),
                "missing {champion}"
            );
        }
    }

    #[test]
    fn batch_3_catalog_covers_requested_dash_execute_and_lockdown_champions() {
        for champion in BATCH_3_CHAMPIONS {
            assert!(
                ultimate_identity_for(champion).is_some(),
                "missing {champion}"
            );
        }
    }

    #[test]
    fn batch_4_catalog_covers_remaining_high_bespoke_champions() {
        for champion in BATCH_4_CHAMPIONS {
            assert!(
                ultimate_identity_for(champion).is_some(),
                "missing {champion}"
            );
        }
    }

    #[test]
    fn catalog_covers_stable_ultimates_txt_roster() {
        let expected: HashSet<_> = EXPECTED_ULTIMATES_TXT_CHAMPIONS.iter().copied().collect();
        let actual: HashSet<_> = ULTIMATE_IDENTITIES
            .iter()
            .map(|identity| identity.champion_key)
            .collect();

        for champion in expected.difference(&actual) {
            panic!("missing expected champion from ultimates.txt: {champion}");
        }
        for champion in actual.difference(&expected) {
            panic!("unexpected champion outside ultimates.txt: {champion}");
        }
        assert_eq!(actual.len(), EXPECTED_ULTIMATES_TXT_CHAMPIONS.len());
    }

    #[test]
    fn all_catalog_identities_have_resolvable_visuals() {
        for identity in ULTIMATE_IDENTITIES {
            assert!(!identity.signature_id.is_empty(), "missing signature");
            assert!(
                identity.visual.visual_event_id.starts_with("ultimate."),
                "{} has non-ultimate visual id",
                identity.champion_name
            );
            assert_eq!(
                identity.visual.visual_event_id,
                format!("ultimate.{}", identity.signature_id)
            );
            assert!(
                identity.visual.palette.len() >= 2,
                "{} has weak palette",
                identity.champion_name
            );
            assert!(
                !identity.visual.shape_language.is_empty(),
                "{} has no shape language",
                identity.champion_name
            );
            assert!(
                !identity.visual.motion_hints.is_empty(),
                "{} has no motion hints",
                identity.champion_name
            );
        }
    }

    #[test]
    fn high_bespoke_batch_4_identities_are_marked_without_losing_visual_identity() {
        let bespoke = [
            "azir", "bardo", "kalista", "karma", "kindred", "ksante", "leblanc", "zaahen", "zilean",
        ];

        for champion in bespoke {
            let identity = ultimate_identity_for(champion).unwrap();
            assert!(
                matches!(
                    identity.status,
                    UltimateImplementationStatus::BespokePending
                        | UltimateImplementationStatus::Partial
                ),
                "{} should not pretend full mechanics are active",
                identity.champion_name
            );
            assert!(identity.visual.palette.len() >= 2);
            assert_ne!(identity.visual.shape_language, &["generic"]);
            assert!(identity.visual.visual_event_id.starts_with("ultimate."));
        }

        assert_eq!(
            ultimate_identity_for("Bardo").unwrap().technical_primitive,
            UltimatePrimitive::GlobalPresence
        );
        assert!(ultimate_identity_for("K'Sante")
            .unwrap()
            .gameplay_tags
            .contains(&"form_shift"));
        assert!(ultimate_identity_for("Zilean")
            .unwrap()
            .gameplay_tags
            .contains(&"revive"));
    }

    #[test]
    fn execute_like_champions_keep_distinct_identity() {
        let champions = ["akali", "pyke", "darius"];
        let identities: Vec<_> = champions
            .iter()
            .map(|champion| ultimate_identity_for(champion).unwrap())
            .collect();

        assert!(identities.iter().all(|identity| identity
            .gameplay_tags
            .iter()
            .any(|tag| tag.contains("execute") || *tag == "assassin")));

        let primitives: HashSet<_> = identities
            .iter()
            .map(|identity| identity.technical_primitive)
            .collect();
        let signatures: HashSet<_> = identities
            .iter()
            .map(|identity| identity.signature_id)
            .collect();
        let shape_sets: HashSet<_> = identities
            .iter()
            .map(|identity| identity.visual.shape_language)
            .collect();
        let motion_sets: HashSet<_> = identities
            .iter()
            .map(|identity| identity.visual.motion_hints)
            .collect();

        assert!(
            primitives.len() > 1,
            "execute-like champions should not collapse to one primitive"
        );
        assert_eq!(signatures.len(), champions.len());
        assert_eq!(shape_sets.len(), champions.len());
        assert_eq!(motion_sets.len(), champions.len());
    }

    #[test]
    fn engage_dash_champions_keep_distinct_identity() {
        let malphite = ultimate_identity_for("malphite").unwrap();
        let vi = ultimate_identity_for("vi").unwrap();
        let camille = ultimate_identity_for("camille").unwrap();

        assert_eq!(
            malphite.technical_primitive,
            UltimatePrimitive::UnstoppableCharge
        );
        assert_eq!(vi.technical_primitive, UltimatePrimitive::TargetedDash);
        assert_eq!(
            camille.technical_primitive,
            UltimatePrimitive::SuppressionLock
        );
        assert_ne!(malphite.signature_id, vi.signature_id);
        assert_ne!(vi.signature_id, camille.signature_id);
        assert_ne!(
            malphite.visual.shape_language,
            camille.visual.shape_language
        );
    }

    #[test]
    fn bespoke_or_partial_batch_3_identities_still_have_usable_visuals() {
        for champion in ["mordekaiser", "sylas", "viego"] {
            let identity = ultimate_identity_for(champion).unwrap();
            assert!(matches!(
                identity.status,
                UltimateImplementationStatus::BespokePending
                    | UltimateImplementationStatus::Partial
            ));
            assert!(!identity.signature_id.is_empty());
            assert!(identity.visual.visual_event_id.starts_with("ultimate."));
            assert!(identity.visual.palette.len() >= 2);
            assert!(!identity.visual.shape_language.is_empty());
            assert!(!identity.visual.motion_hints.is_empty());
        }

        assert_eq!(
            ultimate_identity_for("mordekaiser")
                .unwrap()
                .technical_primitive,
            UltimatePrimitive::DuelRealm
        );
        assert!(ultimate_identity_for("sylas")
            .unwrap()
            .gameplay_tags
            .contains(&"hijack"));
        assert!(ultimate_identity_for("viego")
            .unwrap()
            .gameplay_tags
            .contains(&"possession_reset"));
    }

    #[test]
    fn iconic_global_missiles_share_primitive_not_identity() {
        let champions = ["ashe", "jinx", "ezreal", "draven"];
        let identities: Vec<_> = champions
            .iter()
            .map(|champion| ultimate_identity_for(champion).unwrap())
            .collect();

        assert!(identities
            .iter()
            .all(|identity| identity.technical_primitive == UltimatePrimitive::LinearProjectile));

        let signatures: HashSet<_> = identities
            .iter()
            .map(|identity| identity.signature_id)
            .collect();
        let visual_ids: HashSet<_> = identities
            .iter()
            .map(|identity| identity.visual.visual_event_id)
            .collect();
        let shape_sets: HashSet<_> = identities
            .iter()
            .map(|identity| identity.visual.shape_language)
            .collect();

        assert_eq!(signatures.len(), champions.len());
        assert_eq!(visual_ids.len(), champions.len());
        assert_eq!(shape_sets.len(), champions.len());
    }

    #[test]
    fn beam_identity_is_semantically_distinct_from_projectile() {
        let lux = ultimate_identity_for("lux").unwrap();
        let ashe = ultimate_identity_for("ashe").unwrap();

        assert_eq!(lux.technical_primitive, UltimatePrimitive::BeamLine);
        assert_eq!(
            ashe.technical_primitive,
            UltimatePrimitive::LinearProjectile
        );
        assert_ne!(lux.visual.shape_language, ashe.visual.shape_language);
        assert!(lux.gameplay_tags.contains(&"beam_line"));
    }

    #[test]
    fn global_presence_can_represent_delayed_or_flash_identity() {
        let karthus = ultimate_identity_for("karthus").unwrap();
        let twisted_fate = ultimate_identity_for("Twisted Fate").unwrap();

        assert_eq!(
            karthus.technical_primitive,
            UltimatePrimitive::GlobalPresence
        );
        assert_eq!(
            twisted_fate.technical_primitive,
            UltimatePrimitive::GlobalPresence
        );
        assert!(karthus.gameplay_tags.contains(&"global_delayed"));
        assert!(twisted_fate.gameplay_tags.contains(&"global_vision"));
        assert_eq!(
            twisted_fate.status,
            UltimateImplementationStatus::BespokePending
        );
    }

    #[test]
    fn shared_primitive_still_has_unique_signature_and_visual() {
        let aatrox = ultimate_identity_for("Aatrox").unwrap();
        let alistar = ultimate_identity_for("Alistar").unwrap();
        assert_eq!(aatrox.technical_primitive, alistar.technical_primitive);
        assert_ne!(aatrox.signature_id, alistar.signature_id);
        assert_ne!(
            aatrox.visual.visual_event_id,
            alistar.visual.visual_event_id
        );
        assert_ne!(aatrox.visual.palette, alistar.visual.palette);
    }

    #[test]
    fn catalog_signatures_are_unique() {
        let mut seen = HashSet::new();
        for identity in ULTIMATE_IDENTITIES {
            assert!(
                seen.insert(identity.signature_id),
                "duplicate {}",
                identity.signature_id
            );
        }
    }

    #[test]
    fn semantic_event_serializes_expected_shape() {
        let identity = ultimate_identity_for("Amumu").unwrap();
        let event = ultimate_cast_event_metadata(
            identity,
            "blue-mid",
            &UltimateCastSpatialMetadata {
                origin_pos: Vec2 { x: 0.25, y: 0.4 },
                bespoke_kind: None,
                secondary_pos: None,
                destination_pos: None,
                zone_orientation: None,
                requires_condition: None,
                proxy_origin_kind: None,
                target_id: Some("red-mid".to_string()),
                locked_target_id: None,
                target_ids: None,
                affected_target_ids: None,
                target_pos: Some(Vec2 { x: 0.8, y: 0.4 }),
                direction: Some(Vec2 { x: 1.0, y: 0.0 }),
                shape: UltimateSpatialShape::Circle,
                radius: Some(0.1),
                width: None,
                range: Some(0.7),
                delay_ms: Some(250),
                duration_ms: Some(1250),
                impact_at: None,
                persistent: None,
                pulse_count: None,
                travel_speed: None,
                follow_target: Some(false),
                stage: None,
                stage_count: None,
                sequence_kind: None,
                return_path: None,
                return_to_origin: None,
                bounce_targets: None,
                bounce_count: None,
                recast_window_ms: None,
                tether_kind: None,
                global: Some(false),
            },
        );
        assert_eq!(event["event"], "champion_ultimate_cast");
        assert_eq!(event["actorId"], "blue-mid");
        assert_eq!(event["originPos"]["x"], 0.25);
        assert_eq!(event["shape"], "circle");
        assert_eq!(event["ultimateIdentity"]["technicalPrimitive"], "aoe_pulse");
        assert_eq!(
            event["ultimateIdentity"]["visual"]["visualEventId"],
            "ultimate.amumu_curse_sad_mummy"
        );
    }

    #[test]
    fn semantic_event_contract_serializes_origin_and_base_shapes() {
        let identity = ultimate_identity_for("Lux").unwrap();
        let event = ultimate_cast_event_metadata(
            identity,
            "blue-mid",
            &UltimateCastSpatialMetadata {
                origin_pos: Vec2 { x: 0.25, y: 0.4 },
                bespoke_kind: None,
                secondary_pos: None,
                destination_pos: None,
                zone_orientation: None,
                requires_condition: None,
                proxy_origin_kind: None,
                target_id: Some("red-mid".to_string()),
                locked_target_id: None,
                target_ids: None,
                affected_target_ids: None,
                target_pos: Some(Vec2 { x: 0.9, y: 0.42 }),
                direction: Some(Vec2 { x: 1.0, y: 0.0 }),
                shape: UltimateSpatialShape::Beam,
                radius: None,
                width: Some(0.055),
                range: Some(0.82),
                delay_ms: Some(250),
                duration_ms: Some(1250),
                impact_at: None,
                persistent: None,
                pulse_count: None,
                travel_speed: None,
                follow_target: Some(false),
                stage: None,
                stage_count: None,
                sequence_kind: None,
                return_path: None,
                return_to_origin: None,
                bounce_targets: None,
                bounce_count: None,
                recast_window_ms: None,
                tether_kind: None,
                global: Some(false),
            },
        );

        assert_eq!(event["originPos"]["x"], 0.25);
        assert_eq!(event["targetPos"]["x"], 0.9);
        assert_eq!(event["direction"]["x"], 1.0);
        assert_eq!(event["shape"], "beam");
        assert_eq!(event["width"], 0.055);
        assert_eq!(event["range"], 0.82);

        let serialized_shapes = [
            (UltimateSpatialShape::Aura, "aura"),
            (UltimateSpatialShape::Circle, "circle"),
            (UltimateSpatialShape::Line, "line"),
            (UltimateSpatialShape::Projectile, "projectile"),
            (UltimateSpatialShape::Beam, "beam"),
            (UltimateSpatialShape::GlobalOverlay, "global_overlay"),
        ];
        for (shape, expected) in serialized_shapes {
            assert_eq!(json!(shape), expected);
        }
    }

    #[test]
    fn delayed_and_persistent_ultimates_expose_timing_metadata() {
        let cases = [
            ("Karthus", 2400, false),
            ("Taric", 2400, false),
            ("Kayle", 750, false),
            ("Zyra", 750, true),
        ];

        for (champion, minimum_delay, expected_persistent) in cases {
            let identity = ultimate_identity_for(champion).unwrap();
            let event = ultimate_cast_event_metadata(
                identity,
                "blue-mid",
                &UltimateCastSpatialMetadata {
                    origin_pos: Vec2 { x: 0.25, y: 0.4 },
                    bespoke_kind: None,
                    secondary_pos: None,
                    destination_pos: None,
                    zone_orientation: None,
                    requires_condition: None,
                    proxy_origin_kind: None,
                    target_id: Some("red-mid".to_string()),
                    locked_target_id: None,
                    target_ids: None,
                    affected_target_ids: None,
                    target_pos: Some(Vec2 { x: 0.8, y: 0.4 }),
                    direction: Some(Vec2 { x: 1.0, y: 0.0 }),
                    shape: UltimateSpatialShape::Circle,
                    radius: Some(0.1),
                    width: None,
                    range: Some(0.7),
                    delay_ms: None,
                    duration_ms: None,
                    impact_at: None,
                    persistent: None,
                    pulse_count: None,
                    travel_speed: None,
                    follow_target: Some(false),
                    stage: None,
                    stage_count: None,
                    sequence_kind: None,
                    return_path: None,
                    return_to_origin: None,
                    bounce_targets: None,
                    bounce_count: None,
                    recast_window_ms: None,
                    tether_kind: None,
                    global: Some(false),
                },
            );

            assert!(event["delayMs"].as_u64().unwrap() >= minimum_delay);
            assert_eq!(event["impactAt"], event["delayMs"]);
            assert!(event["durationMs"].as_u64().unwrap() > 0);
            assert_eq!(event["persistent"], expected_persistent);
            assert!(event["pulseCount"].as_u64().unwrap() >= 1);
        }
    }

    #[test]
    fn target_follow_chain_return_and_stage_metadata_are_inferred_safely() {
        let malzahar = ultimate_identity_for("Malzahar").unwrap();
        let lock_event = ultimate_cast_event_metadata(
            malzahar,
            "blue-mid",
            &UltimateCastSpatialMetadata {
                origin_pos: Vec2 { x: 0.25, y: 0.4 },
                bespoke_kind: None,
                secondary_pos: None,
                destination_pos: None,
                zone_orientation: None,
                requires_condition: None,
                proxy_origin_kind: None,
                target_id: Some("red-mid".to_string()),
                locked_target_id: None,
                target_ids: None,
                affected_target_ids: None,
                target_pos: Some(Vec2 { x: 0.7, y: 0.4 }),
                direction: None,
                shape: UltimateSpatialShape::Lock,
                radius: None,
                width: None,
                range: None,
                delay_ms: None,
                duration_ms: None,
                impact_at: None,
                persistent: None,
                pulse_count: None,
                travel_speed: None,
                follow_target: None,
                stage: None,
                stage_count: None,
                sequence_kind: None,
                return_path: None,
                return_to_origin: None,
                bounce_targets: None,
                bounce_count: None,
                recast_window_ms: None,
                tether_kind: None,
                global: Some(false),
            },
        );
        assert_eq!(lock_event["lockedTargetId"], "red-mid");
        assert_eq!(lock_event["targetIds"][0], "red-mid");
        assert_eq!(lock_event["affectedTargetIds"][0], "red-mid");
        assert_eq!(lock_event["followTarget"], true);
        assert_eq!(lock_event["tetherKind"], "suppression_channel");

        let brand = ultimate_identity_for("Brand").unwrap();
        let chain_event = ultimate_cast_event_metadata(
            brand,
            "blue-mid",
            &UltimateCastSpatialMetadata {
                origin_pos: Vec2 { x: 0.25, y: 0.4 },
                bespoke_kind: None,
                secondary_pos: None,
                destination_pos: None,
                zone_orientation: None,
                requires_condition: None,
                proxy_origin_kind: None,
                target_id: Some("red-mid".to_string()),
                locked_target_id: None,
                target_ids: Some(vec!["red-mid".to_string(), "red-bot".to_string()]),
                affected_target_ids: None,
                target_pos: Some(Vec2 { x: 0.7, y: 0.4 }),
                direction: None,
                shape: UltimateSpatialShape::Projectile,
                radius: None,
                width: None,
                range: None,
                delay_ms: None,
                duration_ms: None,
                impact_at: None,
                persistent: None,
                pulse_count: None,
                travel_speed: None,
                follow_target: None,
                stage: None,
                stage_count: None,
                sequence_kind: None,
                return_path: None,
                return_to_origin: None,
                bounce_targets: None,
                bounce_count: None,
                recast_window_ms: None,
                tether_kind: None,
                global: Some(false),
            },
        );
        assert_eq!(chain_event["sequenceKind"], "chain");
        assert_eq!(chain_event["bounceCount"], 2);

        let draven = ultimate_identity_for("Draven").unwrap();
        let return_event = ultimate_cast_event_metadata(
            draven,
            "blue-adc",
            &UltimateCastSpatialMetadata {
                origin_pos: Vec2 { x: 0.2, y: 0.6 },
                bespoke_kind: None,
                secondary_pos: None,
                destination_pos: None,
                zone_orientation: None,
                requires_condition: None,
                proxy_origin_kind: None,
                target_id: Some("red-adc".to_string()),
                locked_target_id: None,
                target_ids: None,
                affected_target_ids: None,
                target_pos: Some(Vec2 { x: 0.8, y: 0.6 }),
                direction: None,
                shape: UltimateSpatialShape::Projectile,
                radius: None,
                width: None,
                range: None,
                delay_ms: None,
                duration_ms: None,
                impact_at: None,
                persistent: None,
                pulse_count: None,
                travel_speed: None,
                follow_target: None,
                stage: None,
                stage_count: None,
                sequence_kind: None,
                return_path: None,
                return_to_origin: None,
                bounce_targets: None,
                bounce_count: None,
                recast_window_ms: None,
                tether_kind: None,
                global: Some(false),
            },
        );
        assert_eq!(return_event["sequenceKind"], "return_path");
        assert_eq!(return_event["returnToOrigin"], true);

        let jhin = ultimate_identity_for("Jhin").unwrap();
        let stage_event = ultimate_cast_event_metadata(
            jhin,
            "blue-adc",
            &UltimateCastSpatialMetadata {
                origin_pos: Vec2 { x: 0.2, y: 0.6 },
                bespoke_kind: None,
                secondary_pos: None,
                destination_pos: None,
                zone_orientation: None,
                requires_condition: None,
                proxy_origin_kind: None,
                target_id: Some("red-adc".to_string()),
                locked_target_id: None,
                target_ids: None,
                affected_target_ids: None,
                target_pos: Some(Vec2 { x: 0.8, y: 0.6 }),
                direction: None,
                shape: UltimateSpatialShape::Line,
                radius: None,
                width: None,
                range: None,
                delay_ms: None,
                duration_ms: None,
                impact_at: None,
                persistent: None,
                pulse_count: None,
                travel_speed: None,
                follow_target: None,
                stage: Some(2),
                stage_count: None,
                sequence_kind: None,
                return_path: None,
                return_to_origin: None,
                bounce_targets: None,
                bounce_count: None,
                recast_window_ms: None,
                tether_kind: None,
                global: Some(false),
            },
        );
        assert_eq!(stage_event["stage"], 2);
        assert_eq!(stage_event["stageCount"], 4);
        assert_eq!(stage_event["sequenceKind"], "multi_shot_channel");
    }

    #[test]
    fn batch_4_bespoke_signatures_emit_specific_metadata() {
        let expected = [
            ("Sylas", "stolen_ultimate_pending"),
            ("Mordekaiser", "death_realm"),
            ("Ryze", "portal"),
            ("Twisted Fate", "global_reveal_gate"),
            ("Shen", "ally_shield_arrival"),
            ("Kindred", "sanctuary_heal"),
            ("Taliyah", "terrain_wall"),
            ("Azir", "soldier_wall"),
            ("Yasuo", "airborne_slash"),
            ("Orianna", "proxy_shockwave"),
            ("Ornn", "two_stage_ram"),
            ("Nocturne", "blackout_dash"),
            ("Galio", "global_landing"),
            ("Pantheon", "global_landing"),
            ("Ekko", "rewind_ghost"),
            ("Xayah", "feather_fan_recall"),
            ("Yuumi", "host_waves"),
        ];

        for (champion, bespoke_kind) in expected {
            let identity = ultimate_identity_for(champion).unwrap();
            let event = ultimate_cast_event_metadata(identity, "blue-mid", &base_spatial());

            assert_eq!(event["bespokeKind"], bespoke_kind, "{champion}");
        }
    }

    #[test]
    fn batch_4_bespoke_metadata_preserves_critical_semantics() {
        let spatial = base_spatial();

        let orianna = ultimate_cast_event_metadata(
            ultimate_identity_for("Orianna").unwrap(),
            "blue-mid",
            &spatial,
        );
        assert_eq!(orianna["proxyOriginKind"], "ball_or_target_point");
        assert_eq!(orianna["bespokeKind"], "proxy_shockwave");

        for champion in ["Ryze", "Twisted Fate", "Shen"] {
            let event = ultimate_cast_event_metadata(
                ultimate_identity_for(champion).unwrap(),
                "blue-mid",
                &spatial,
            );
            assert_eq!(event["destinationPos"]["x"], 0.75, "{champion}");
        }

        let morde = ultimate_cast_event_metadata(
            ultimate_identity_for("Mordekaiser").unwrap(),
            "blue-mid",
            &spatial,
        );
        assert_eq!(morde["lockedTargetId"], "red-mid");
        assert_eq!(morde["followTarget"], false);

        let kindred = ultimate_cast_event_metadata(
            ultimate_identity_for("Kindred").unwrap(),
            "blue-mid",
            &spatial,
        );
        assert_eq!(kindred["persistent"], true);
        assert_eq!(kindred["bespokeKind"], "sanctuary_heal");

        let mut global_spatial = base_spatial();
        global_spatial.global = None;
        let nocturne = ultimate_cast_event_metadata(
            ultimate_identity_for("Nocturne").unwrap(),
            "blue-jgl",
            &global_spatial,
        );
        assert_eq!(nocturne["global"], true);
        assert_eq!(nocturne["destinationPos"]["x"], 0.75);
        assert_eq!(nocturne["bespokeKind"], "blackout_dash");
    }
}
