import type { SimEvent, Vec2 } from "../engine/types";

export type UltimatePrimitive =
  | "self_aura"
  | "ally_aura"
  | "aoe_pulse"
  | "zone_summon"
  | "basic_summon"
  | "linear_projectile"
  | "beam_line"
  | "artillery"
  | "global_presence"
  | "targeted_dash"
  | "execute_marker"
  | "suppression_lock"
  | "duel_realm"
  | "assassin_mark"
  | "unstoppable_charge"
  | "blink_burst";

export type UltimateSpatialShape =
  | "aura"
  | "circle"
  | "line"
  | "projectile"
  | "beam"
  | "global_overlay"
  | "global"
  | "cone"
  | "lock"
  | "zone";

export interface UltimateIdentityEventMetadata {
  event?: string;
  actorId?: string;
  championId?: string;
  ultimateIdentity?: {
    championKey?: string;
    championName?: string;
    technicalPrimitive?: UltimatePrimitive | string;
    signatureId?: string;
    gameplayTags?: string[];
    semanticEffects?: string[];
    visual?: {
      visualEventId?: string;
      palette?: string[];
      shapeLanguage?: string[];
      motionHints?: string[];
    };
    status?: string;
  };
  bespokeKind?: string;
  secondaryPos?: Vec2;
  destinationPos?: Vec2;
  zoneOrientation?: Vec2;
  requiresCondition?: string;
  proxyOriginKind?: string;
  originPos?: Vec2;
  targetId?: string;
  lockedTargetId?: string;
  targetIds?: string[];
  affectedTargetIds?: string[];
  targetPos?: Vec2;
  direction?: Vec2;
  shape?: UltimateSpatialShape;
  radius?: number;
  width?: number;
  range?: number;
  delayMs?: number;
  durationMs?: number;
  impactAt?: number;
  persistent?: boolean;
  pulseCount?: number;
  travelSpeed?: number;
  followTarget?: boolean;
  stage?: number;
  stageCount?: number;
  sequenceKind?: string;
  returnPath?: Vec2[];
  returnToOrigin?: boolean;
  bounceTargets?: Array<string | Vec2>;
  bounceCount?: number;
  recastWindowMs?: number;
  tetherKind?: string;
  global?: boolean;
}

export interface UltimateSpatialRenderContext {
  origin: Vec2;
  target?: Vec2;
  direction?: Vec2;
  angle: number;
  shape: UltimateSpatialShape;
  usedFallbackAngle: boolean;
  lockedTarget?: Vec2;
  targetPoints: Vec2[];
  bouncePoints: Vec2[];
  returnPathPoints: Vec2[];
  followTarget: boolean;
  stage?: number;
  stageCount?: number;
  sequenceKind?: string;
  recastWindowMs?: number;
  tetherKind?: string;
  bespokeKind?: string;
  secondary?: Vec2;
  destination?: Vec2;
  zoneOrientation?: Vec2;
  requiresCondition?: string;
  proxyOriginKind?: string;
}

export interface UltimateVisualConfig {
  signatureId: string;
  primitive: UltimatePrimitive | "fallback";
  palette: string[];
  shapeLanguage: string[];
  motion: string[];
  glow: number;
  delayMs: number;
  durationMs: number;
  impactAt: number;
  persistent: boolean;
  pulseCount: number;
}

export type UltimateRenderPhase = "windup" | "active" | "impact" | "fade";

export interface UltimateRenderTiming {
  delayMs: number;
  durationMs: number;
  impactAt: number;
  persistent: boolean;
  pulseCount: number;
  totalMs: number;
}

export type UltimateSilhouetteKind =
  | "darkin_wings"
  | "crystal_arrow"
  | "chaos_rocket"
  | "arcane_barrage"
  | "moonlight_vigil"
  | "bouncing_fire"
  | "glacial_fissure"
  | "spinning_axes"
  | "precision_snipe"
  | "prismatic_laser"
  | "bullet_cone"
  | "void_ray"
  | "soldier_wall"
  | "stasis_hourglass"
  | "death_realm"
  | "hextech_cage"
  | "lamb_wolf_sanctuary"
  | "chrono_rewind"
  | "protective_shield"
  | "global_darkness"
  | "card_destiny"
  | "cannon_barrage"
  | "requiem_omen"
  | "meteor_fall"
  | "tibbers_bear"
  | "turret_construct"
  | "trap_mushroom"
  | "tentacle_slam"
  | "thorn_roots"
  | "feather_storm"
  | "blade_execution"
  | "two_stage_execution"
  | "drowned_x"
  | "guillotine_axe"
  | "sky_sword"
  | "divine_swords"
  | "mountain_impact"
  | "gauntlet_lockon"
  | "shadow_mark"
  | "crowstorm"
  | "spirit_dash"
  | "wind_fate_slash"
  | "beast_charge"
  | "petal_charm"
  | "storm_field"
  | "static_field"
  | "equalizer_line"
  | "demonic_wings"
  | "divine_wings"
  | "dragon_descent"
  | "colossus_landing"
  | "blade_curtain"
  | "nature_vines"
  | "music_wave"
  | "cosmic_stars"
  | "void_bloom"
  | "hextech_construct"
  | "water_wave"
  | "ice_prison"
  | "fire_explosion"
  | "shockwave_ring"
  | "mask_orbit"
  | "signature_composite";

export const BESPOKE_SIGNATURE_KINDS: Record<string, string> = {
  aatrox_world_ender: "darkin_self_buff",
  ahri_spirit_rush: "triple_spirit_dash",
  akali_perfect_execution: "two_stage_neon_execution",
  akshan_comeuppance_lockon: "comeuppance_lock_on_shots",
  alistar_unbreakable_will: "cleanse_damage_reduction",
  ambessa_public_execution: "noxian_execution_dash",
  amumu_curse_sad_mummy: "aoe_bandage_lockdown",
  anivia_glacial_storm: "persistent_slow_damage_storm",
  annie_summon_tibbers: "tibbers_drop_burst_pet",
  aphelios_moonlight_vigil: "moonlight_bloom_weapon_pending",
  sylas_hijack: "stolen_ultimate_pending",
  mordekaiser_realm_death: "death_realm",
  ryze_realm_warp: "portal",
  twistedfate_destiny: "global_reveal_gate",
  shen_stand_united: "ally_shield_arrival",
  kindred_lambs_respite: "sanctuary_heal",
  taliyah_weavers_wall: "terrain_wall",
  azir_emperors_divide: "soldier_wall",
  yasuo_last_breath: "airborne_slash",
  orianna_command_shockwave: "proxy_shockwave",
  ornn_call_forge_god: "two_stage_ram",
  nocturne_paranoia: "blackout_dash",
  galio_heroes_entrance: "global_landing",
  pantheon_grand_starfall: "global_landing",
  ekko_chronobreak: "rewind_ghost",
  xayah_featherstorm: "feather_fan_recall",
  yuumi_final_chapter: "host_waves",
};

const FALLBACK_VISUAL: UltimateVisualConfig = {
  signatureId: "fallback.unknown_ultimate",
  primitive: "fallback",
  palette: ["#e5e7eb", "#94a3b8", "#38bdf8"],
  shapeLanguage: ["soft_ring", "neutral_spark"],
  motion: ["single_pulse", "fade"],
  glow: 0.45,
  delayMs: 0,
  durationMs: 1500,
  impactAt: 0,
  persistent: false,
  pulseCount: 1,
};

export const FALLBACK_SIGNATURE_ID = FALLBACK_VISUAL.signatureId;

const SIGNATURE_VISUAL_OVERRIDES: Record<
  string,
  Partial<UltimateVisualConfig>
> = {
  aatrox_world_ender: { glow: 0.78, durationMs: 12000 },
  alistar_unbreakable_will: { glow: 0.62, durationMs: 7000 },
  amumu_curse_sad_mummy: { glow: 0.78, durationMs: 1600 },
  anivia_glacial_storm: { glow: 0.58, durationMs: 5200 },
  annie_summon_tibbers: { glow: 0.9, durationMs: 45000 },
  azir_emperors_divide: { glow: 0.78, durationMs: 1900 },
  bard_tempered_fate: { glow: 0.84, durationMs: 2600 },
  diana_moonfall: { glow: 0.8, durationMs: 1500 },
  ekko_chronobreak: { glow: 0.86, durationMs: 1800 },
  janna_monsoon: { glow: 0.58, durationMs: 2100 },
  katarina_death_lotus: { glow: 0.82, durationMs: 1600 },
  kindred_lambs_respite: { glow: 0.72, durationMs: 2400 },
  orianna_command_shockwave: { glow: 0.68, durationMs: 1350 },
  swain_demonic_ascension: { glow: 0.76, durationMs: 2300 },
  taric_cosmic_radiance: { glow: 0.7, durationMs: 2400 },
  zilean_chrono_shift: { glow: 0.8, durationMs: 2500 },
  ashe_enchanted_crystal_arrow: { glow: 0.7, durationMs: 2100 },
  draven_whirling_death: { glow: 0.74, durationMs: 1900 },
  ezreal_trueshot_barrage: { glow: 0.82, durationMs: 1800 },
  jinx_super_mega_death_rocket: { glow: 0.86, durationMs: 2200 },
  lux_final_spark: { glow: 0.88, durationMs: 1250 },
  malzahar_nether_grasp: { glow: 0.76, durationMs: 2300 },
  missfortune_bullet_time: { glow: 0.8, durationMs: 2400 },
  karthus_requiem: { glow: 0.9, durationMs: 2600 },
  nocturne_paranoia: { glow: 0.92, durationMs: 2200 },
  twistedfate_destiny: { glow: 0.78, durationMs: 2400 },
  akshan_comeuppance_lockon: { glow: 0.82, durationMs: 2400 },
  aphelios_moonlight_vigil: { glow: 0.8, durationMs: 1900 },
  ahri_spirit_rush: { glow: 0.82, durationMs: 1600 },
  akali_perfect_execution: { glow: 0.88, durationMs: 1350 },
  ambessa_public_execution: { glow: 0.84, durationMs: 1350 },
  darius_noxian_guillotine: { glow: 0.84, durationMs: 1250 },
  malphite_unstoppable_force: { glow: 0.9, durationMs: 1500 },
  mordekaiser_realm_death: { glow: 0.82, durationMs: 2400 },
  pyke_death_from_below: { glow: 0.88, durationMs: 1300 },
  vi_cease_desist: { glow: 0.8, durationMs: 1500 },
  zed_death_mark: { glow: 0.86, durationMs: 1800 },
};

export const SIGNATURE_SILHOUETTE_OVERRIDES: Record<
  string,
  UltimateSilhouetteKind
> = {
  aatrox_world_ender: "darkin_wings",
  alistar_unbreakable_will: "beast_charge",
  mundo_maximum_dosage: "petal_charm",
  fiora_grand_challenge: "blade_execution",
  jax_grandmasters_might: "storm_field",
  jayce_transform_mercury: "hextech_construct",
  masteryi_highlander: "blade_execution",
  mel_solar_snare: "protective_shield",
  naafiri_hounds_pursuit: "beast_charge",
  nasus_fury_sands: "meteor_fall",
  olaf_ragnarok: "spinning_axes",
  quinn_behind_enemy_lines: "feather_storm",
  rakan_quickness: "petal_charm",
  renekton_dominus: "beast_charge",
  rengar_thrill_hunt: "beast_charge",
  riven_blade_exile: "blade_execution",
  singed_insanity_potion: "petal_charm",
  sivir_on_the_hunt: "spinning_axes",
  tahmkench_devour: "protective_shield",
  tryndamere_undying_rage: "guillotine_axe",
  twitch_spray_and_pray: "bullet_cone",
  udyr_awakened_spirit: "mask_orbit",
  vayne_final_hour: "global_darkness",
  amumu_curse_sad_mummy: "shockwave_ring",
  anivia_glacial_storm: "ice_prison",
  aurora_between_worlds: "stasis_hourglass",
  belveth_endless_banquet: "void_bloom",
  blitzcrank_static_field: "hextech_construct",
  cassiopeia_petrifying_gaze: "precision_snipe",
  diana_moonfall: "mask_orbit",
  gnar_gnar: "mountain_impact",
  gragas_explosive_cask: "chaos_rocket",
  hwei_spiraling_despair: "petal_charm",
  janna_monsoon: "storm_field",
  kennen_slicing_maelstrom: "static_field",
  katarina_death_lotus: "blade_execution",
  milio_breath_life: "protective_shield",
  morgana_soul_shackles: "hextech_cage",
  neeko_pop_blossom: "petal_charm",
  nilah_apotheosis: "shockwave_ring",
  orianna_command_shockwave: "shockwave_ring",
  rammus_soaring_slam: "mountain_impact",
  rell_magnet_storm: "storm_field",
  samira_inferno_trigger: "bullet_cone",
  skarner_impale: "hextech_cage",
  swain_demonic_ascension: "demonic_wings",
  talon_shadow_assault: "blade_execution",
  taric_cosmic_radiance: "protective_shield",
  thresh_the_box: "hextech_cage",
  vladimir_hemoplague: "shockwave_ring",
  volibear_stormbringer: "static_field",
  wukong_cyclone: "storm_field",
  zac_lets_bounce: "petal_charm",
  zeri_lightning_crash: "static_field",
  ziggs_mega_inferno_bomb: "meteor_fall",
  zyra_stranglethorns: "thorn_roots",
  annie_summon_tibbers: "tibbers_bear",
  elise_spider_form: "tentacle_slam",
  heimerdinger_apex_turret: "hextech_construct",
  illaoi_leap_faith: "tentacle_slam",
  ivern_summon_daisy: "turret_construct",
  shaco_hallucinate: "shadow_mark",
  teemo_noxious_trap: "trap_mushroom",
  viktor_chaos_storm: "hextech_construct",
  yorick_eulogy_isles: "turret_construct",
  akshan_comeuppance_lockon: "precision_snipe",
  aphelios_moonlight_vigil: "moonlight_vigil",
  ashe_enchanted_crystal_arrow: "crystal_arrow",
  brand_pyroclasm_bounce: "fire_explosion",
  braum_glacial_fissure: "glacial_fissure",
  corki_missile_barrage: "chaos_rocket",
  draven_whirling_death: "spinning_axes",
  ezreal_trueshot_barrage: "arcane_barrage",
  irelia_vanguard_edge: "blade_curtain",
  jhin_curtain_call: "precision_snipe",
  jinx_super_mega_death_rocket: "chaos_rocket",
  kled_chaaaaaaaarge: "beast_charge",
  kogmaw_living_artillery: "meteor_fall",
  maokai_natures_grasp: "nature_vines",
  nami_tidal_wave: "water_wave",
  nautilus_depth_charge: "glacial_fissure",
  ornn_call_forge_god: "beast_charge",
  poppy_keepers_verdict: "gauntlet_lockon",
  qiyana_supreme_display_talent: "mask_orbit",
  renata_hostile_takeover: "bullet_cone",
  sejuani_glacial_prison: "ice_prison",
  seraphine_encore: "music_wave",
  smolder_mmmmom: "dragon_descent",
  sona_crescendo: "music_wave",
  tristana_buster_shot: "chaos_rocket",
  urgot_fear_beyond_death: "drowned_x",
  varus_chain_corruption: "thorn_roots",
  vex_shadow_surge: "shadow_mark",
  warwick_infinite_duress: "beast_charge",
  yone_fate_sealed: "wind_fate_slash",
  yuumi_final_chapter: "arcane_barrage",
  zoe_portal_jump: "stasis_hourglass",
  caitlyn_ace_in_the_hole: "precision_snipe",
  garen_demacian_justice: "sky_sword",
  gwen_needlework: "blade_execution",
  kayle_divine_judgment: "divine_swords",
  lucian_the_culling: "bullet_cone",
  lux_final_spark: "prismatic_laser",
  malzahar_nether_grasp: "void_bloom",
  missfortune_bullet_time: "bullet_cone",
  rumble_equalizer: "equalizer_line",
  senna_dawning_shadow: "prismatic_laser",
  taliyah_weavers_wall: "glacial_fissure",
  velkoz_life_form_disintegration_ray: "void_ray",
  gangplank_cannon_barrage: "fire_explosion",
  karthus_requiem: "requiem_omen",
  lillia_lilting_lullaby: "nature_vines",
  nocturne_paranoia: "global_darkness",
  pantheon_grand_starfall: "meteor_fall",
  ryze_realm_warp: "card_destiny",
  shen_stand_united: "protective_shield",
  soraka_wish: "cosmic_stars",
  twistedfate_destiny: "card_destiny",
  xerath_rite_arcane: "cosmic_stars",
  ahri_spirit_rush: "spirit_dash",
  akali_perfect_execution: "two_stage_execution",
  ambessa_public_execution: "gauntlet_lockon",
  briar_certain_death: "beast_charge",
  camille_hextech_ultimatum: "hextech_construct",
  chogath_feast: "beast_charge",
  darius_noxian_guillotine: "guillotine_axe",
  evelynn_last_caress: "petal_charm",
  fiddlesticks_crowstorm: "crowstorm",
  jarvaniv_cataclysm: "mountain_impact",
  kaisa_killer_instinct: "void_bloom",
  kassadin_riftwalk: "void_bloom",
  kayn_umbral_trespass: "shadow_mark",
  khazix_void_assault: "beast_charge",
  leesin_dragons_rage: "beast_charge",
  lissandra_frozen_tomb: "ice_prison",
  malphite_unstoppable_force: "mountain_impact",
  mordekaiser_realm_death: "death_realm",
  pyke_death_from_below: "drowned_x",
  reksai_void_rush: "beast_charge",
  sett_show_stopper: "mountain_impact",
  shyvana_dragons_descent: "dragon_descent",
  sion_unstoppable_onslaught: "beast_charge",
  sylas_hijack: "signature_composite",
  trundle_subjugate: "hextech_cage",
  veigar_primordial_burst: "meteor_fall",
  vi_cease_desist: "gauntlet_lockon",
  viego_heartbreaker: "void_bloom",
  yasuo_last_breath: "wind_fate_slash",
  zed_death_mark: "shadow_mark",
  aurelionsol_falling_star: "cosmic_stars",
  azir_emperors_divide: "soldier_wall",
  bard_tempered_fate: "cosmic_stars",
  ekko_chronobreak: "chrono_rewind",
  fizz_chum_waters: "water_wave",
  galio_heroes_entrance: "colossus_landing",
  hecarim_onslaught_shadows: "beast_charge",
  kalista_fates_call: "crystal_arrow",
  karma_mantra: "mask_orbit",
  kindred_lambs_respite: "lamb_wolf_sanctuary",
  ksante_all_out: "death_realm",
  leblanc_mimic: "shadow_mark",
  leona_solar_flare: "meteor_fall",
  lulu_wild_growth: "petal_charm",
  nidalee_aspect_cougar: "beast_charge",
  nunu_absolute_zero: "ice_prison",
  syndra_unleashed_power: "meteor_fall",
  xayah_featherstorm: "feather_storm",
  xinzhao_crescent_guard: "protective_shield",
  yunara_pack_surge: "shockwave_ring",
  zaahen_annihilation: "mask_orbit",
  zilean_chrono_shift: "chrono_rewind",
};

export const SIGNATURE_COMPOSITE_RATIONALES: Record<string, string> = {
  sylas_hijack:
    "Sylas explicitly mirrors the stolen champion ultimate, so its silhouette is a champion-specific composite rather than an implicit fallback.",
};

export const hasExplicitUltimateSignatureVisual = (signatureId: string) =>
  Object.prototype.hasOwnProperty.call(SIGNATURE_SILHOUETTE_OVERRIDES, signatureId);

const DELAYED_GROUND_AOE_SIGNATURES = new Set([
  "leona_solar_flare",
  "ziggs_mega_inferno_bomb",
  "aurelionsol_falling_star",
  "bard_tempered_fate",
  "karthus_requiem",
  "kayle_divine_judgment",
  "taric_cosmic_radiance",
  "zyra_stranglethorns",
  "nunu_absolute_zero",
  "fizz_chum_waters",
  "xerath_rite_arcane",
  "gangplank_cannon_barrage",
]);

const PERSISTENT_ZONE_SIGNATURES = new Set([
  "anivia_glacial_storm",
  "rumble_equalizer",
  "viktor_chaos_storm",
  "kindred_lambs_respite",
  "morgana_soul_shackles",
  "fiddlesticks_crowstorm",
  "kennen_slicing_maelstrom",
  "swain_demonic_ascension",
  "janna_monsoon",
  "nunu_absolute_zero",
  "gangplank_cannon_barrage",
  "zyra_stranglethorns",
]);

const CHANNEL_SIGNATURES = new Set([
  "lucian_the_culling",
  "missfortune_bullet_time",
  "velkoz_life_form_disintegration_ray",
  "katarina_death_lotus",
  "samira_inferno_trigger",
  "xerath_rite_arcane",
]);

const clampMs = (value: unknown, fallback: number) =>
  typeof value === "number" && Number.isFinite(value) && value >= 0
    ? value
    : fallback;

export function resolveUltimateRenderTiming(
  metadata?: UltimateIdentityEventMetadata | null,
  signatureId = metadata?.ultimateIdentity?.signatureId ?? FALLBACK_VISUAL.signatureId,
): UltimateRenderTiming {
  const tags = metadata?.ultimateIdentity?.gameplayTags ?? [];
  const delayed =
    DELAYED_GROUND_AOE_SIGNATURES.has(signatureId) ||
    tags.some((tag) => tag.includes("delayed"));
  const persistentDefault =
    PERSISTENT_ZONE_SIGNATURES.has(signatureId) ||
    metadata?.shape === "zone" ||
    tags.some((tag) => tag === "zone" || tag.includes("global_zone") || tag.includes("line_zone"));
  const channel =
    CHANNEL_SIGNATURES.has(signatureId) ||
    tags.some((tag) => tag.includes("channel"));
  const defaultDelay = delayed
    ? signatureId === "karthus_requiem" || signatureId === "taric_cosmic_radiance"
      ? 2400
      : signatureId === "nunu_absolute_zero"
        ? 1800
        : signatureId === "bard_tempered_fate"
          ? 900
          : signatureId === "gangplank_cannon_barrage" || signatureId === "xerath_rite_arcane"
            ? 650
            : 750
    : 0;
  const defaultDuration = persistentDefault
    ? signatureId === "anivia_glacial_storm" || signatureId === "rumble_equalizer" || signatureId === "swain_demonic_ascension"
      ? 5200
      : signatureId === "gangplank_cannon_barrage" || signatureId === "kindred_lambs_respite"
        ? 4400
        : signatureId === "nunu_absolute_zero" || signatureId === "janna_monsoon"
          ? 3600
          : 3200
    : channel
      ? signatureId === "xerath_rite_arcane"
        ? 4200
        : signatureId === "lucian_the_culling" || signatureId === "missfortune_bullet_time" || signatureId === "velkoz_life_form_disintegration_ray"
          ? 3000
          : 2400
      : 1700;
  const delayMs = clampMs(metadata?.delayMs, defaultDelay);
  const durationMs = clampMs(metadata?.durationMs, defaultDuration);
  const impactAt = clampMs(metadata?.impactAt, delayMs);
  const persistent = metadata?.persistent ?? persistentDefault;
  const pulseCount = Math.max(
    1,
    Math.round(clampMs(metadata?.pulseCount, channel || persistent ? Math.min(12, Math.max(3, Math.round(durationMs / 450))) : delayed ? 2 : 1)),
  );

  return {
    delayMs,
    durationMs,
    impactAt,
    persistent,
    pulseCount,
    totalMs: Math.max(durationMs, impactAt + (persistent || channel ? durationMs : 650)),
  };
}

export function resolveUltimateRenderPhase(elapsedMs: number, timing: UltimateRenderTiming): UltimateRenderPhase {
  if (elapsedMs < timing.impactAt) return "windup";
  if (elapsedMs < timing.impactAt + 180) return "impact";
  if (elapsedMs <= timing.totalMs - 300) return "active";
  return "fade";
}

export function resolveUltimateVisualConfig(
  metadata?: UltimateIdentityEventMetadata | null,
): UltimateVisualConfig {
  const identity = metadata?.ultimateIdentity;
  const signatureId = identity?.signatureId;
  const visual = identity?.visual;

  if (!signatureId || !visual?.palette?.length) {
    return {
      ...FALLBACK_VISUAL,
      palette: [...FALLBACK_VISUAL.palette],
      shapeLanguage: [...FALLBACK_VISUAL.shapeLanguage],
      motion: [...FALLBACK_VISUAL.motion],
    };
  }

  const override = SIGNATURE_VISUAL_OVERRIDES[signatureId] ?? {};
  const timing = resolveUltimateRenderTiming(metadata, signatureId);
  return {
    signatureId,
    primitive:
      (identity.technicalPrimitive as UltimatePrimitive | undefined) ??
      "fallback",
    palette: [...visual.palette],
    shapeLanguage: [...(visual.shapeLanguage ?? [])],
    motion: [...(visual.motionHints ?? [])],
    glow: override.glow ?? 0.62,
    delayMs: timing.delayMs,
    durationMs: metadata?.durationMs ?? (timing.persistent ? timing.durationMs : override.durationMs ?? timing.durationMs),
    impactAt: timing.impactAt,
    persistent: timing.persistent,
    pulseCount: timing.pulseCount,
  };
}

function isUltimateIdentityMetadata(
  value: unknown,
): value is UltimateIdentityEventMetadata {
  const maybe = value as UltimateIdentityEventMetadata | undefined;
  return maybe?.event === "champion_ultimate_cast";
}

function unitPositionByActor(
  units: Array<{ id: string; pos: Vec2 }>,
): Map<string, Vec2> {
  return new Map(units.map((unit) => [unit.id, unit.pos]));
}

const isVec2 = (value: unknown): value is Vec2 => {
  const maybe = value as Vec2 | undefined;
  return typeof maybe?.x === "number" && typeof maybe.y === "number";
};

const uniqueIds = (...groups: Array<Array<string | undefined> | undefined>) => {
  const seen = new Set<string>();
  const result: string[] = [];
  groups.flatMap((group) => group ?? []).forEach((id) => {
    if (!id || seen.has(id)) return;
    seen.add(id);
    result.push(id);
  });
  return result;
};

const resolveLiveTarget = (
  metadata: UltimateIdentityEventMetadata,
  positions?: Map<string, Vec2>,
): Vec2 | undefined => {
  const targetId = metadata.lockedTargetId ?? metadata.targetId;
  return metadata.followTarget && targetId ? positions?.get(targetId) ?? metadata.targetPos : metadata.targetPos;
};

const pointsForIds = (ids: string[], positions?: Map<string, Vec2>) =>
  ids.flatMap((id) => {
    const point = positions?.get(id);
    return point ? [point] : [];
  });

const resolveBouncePoints = (
  metadata: UltimateIdentityEventMetadata,
  targetPoints: Vec2[],
  positions?: Map<string, Vec2>,
) => {
  const explicit = (metadata.bounceTargets ?? []).flatMap((target) => {
    if (typeof target === "string") {
      const point = positions?.get(target);
      return point ? [point] : [];
    }
    return isVec2(target) ? [target] : [];
  });
  const chain = explicit.length > 0 ? explicit : targetPoints;
  const limit = metadata.bounceCount && metadata.bounceCount > 0 ? metadata.bounceCount : chain.length;
  return chain.slice(0, Math.max(0, limit));
};

function hexToRgba(hex: string, alpha: number): string {
  const normalized = hex.replace("#", "");
  if (normalized.length !== 6) return `rgba(148, 163, 184, ${alpha})`;
  const r = Number.parseInt(normalized.slice(0, 2), 16);
  const g = Number.parseInt(normalized.slice(2, 4), 16);
  const b = Number.parseInt(normalized.slice(4, 6), 16);
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

function visualTokens(config: UltimateVisualConfig): string {
  return `${config.signatureId} ${config.shapeLanguage.join(" ")} ${config.motion.join(" ")}`.toLowerCase();
}

function hasVisualToken(config: UltimateVisualConfig, ...tokens: string[]): boolean {
  const visual = visualTokens(config);
  return tokens.some((token) => visual.includes(token));
}

export function getUltimateSilhouetteKind(
  config: UltimateVisualConfig,
): UltimateSilhouetteKind {
  const explicitKind = SIGNATURE_SILHOUETTE_OVERRIDES[config.signatureId];
  if (explicitKind) return explicitKind;

  if (hasVisualToken(config, "crystal_arrow", "frost_wings")) return "crystal_arrow";
  if (hasVisualToken(config, "rocket", "graffiti_burst")) return "chaos_rocket";
  if (hasVisualToken(config, "trueshot", "barrage", "arcane")) return "arcane_barrage";
  if (hasVisualToken(config, "axe", "whirling")) return "spinning_axes";
  if (hasVisualToken(config, "snipe", "curtain", "deadeye")) return "precision_snipe";
  if (hasVisualToken(config, "prismatic", "laser", "spark", "ray")) return "prismatic_laser";
  if (hasVisualToken(config, "bullet", "cone", "culling")) return "bullet_cone";
  if (hasVisualToken(config, "void_ray", "disintegration", "velkoz")) return "void_ray";
  if (hasVisualToken(config, "soldier_wall", "phalanx", "emperor")) return "soldier_wall";
  if (hasVisualToken(config, "hourglass", "stasis", "golden_freeze")) return "stasis_hourglass";
  if (hasVisualToken(config, "death_realm", "duel_cage", "iron_crown")) return "death_realm";
  if (hasVisualToken(config, "lamb", "wolf", "respite")) return "lamb_wolf_sanctuary";
  if (hasVisualToken(config, "clock", "chrono", "rewind", "time_anchor")) return "chrono_rewind";
  if (hasVisualToken(config, "lights_out", "paranoia", "darkness", "death_chorus", "requiem")) return "global_darkness";
  if (hasVisualToken(config, "card", "destiny", "gate")) return "card_destiny";
  if (hasVisualToken(config, "cannon", "barrage", "gangplank")) return "cannon_barrage";
  if (hasVisualToken(config, "meteor", "starfall", "comet")) return "meteor_fall";
  if (hasVisualToken(config, "tibbers", "bear")) return "tibbers_bear";
  if (hasVisualToken(config, "turret", "hextech_panels", "construct")) return "turret_construct";
  if (hasVisualToken(config, "mushroom", "trap")) return "trap_mushroom";
  if (hasVisualToken(config, "tentacle", "kraken")) return "tentacle_slam";
  if (hasVisualToken(config, "feather", "xayah")) return "feather_storm";
  if (hasVisualToken(config, "shadow", "mark", "death_mark")) return "shadow_mark";
  if (hasVisualToken(config, "drowned_x", "harpoon")) return "drowned_x";
  if (hasVisualToken(config, "guillotine", "noxian_axe")) return "guillotine_axe";
  if (hasVisualToken(config, "blade", "kunai", "dagger", "shuriken", "rapier")) return "blade_execution";
  if (hasVisualToken(config, "mountain", "crater", "seismic")) return "mountain_impact";
  if (hasVisualToken(config, "gauntlet", "punch", "lockon")) return "gauntlet_lockon";
  if (hasVisualToken(config, "spirit", "fox", "charm", "dash")) return "spirit_dash";
  if (hasVisualToken(config, "beast", "charge", "dragon", "unstoppable", "horse")) return "beast_charge";
  if (hasVisualToken(config, "heart", "petal", "flower", "charm")) return "petal_charm";
  if (hasVisualToken(config, "storm", "lightning", "electric", "monsoon")) return "storm_field";
  if (hasVisualToken(config, "mask", "orbit", "moon", "sun_disc")) return "mask_orbit";
  return "signature_composite";
}

function drawSignatureGlyph(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  radius: number,
  config: UltimateVisualConfig,
  alpha: number,
) {
  const primary = config.palette[0] ?? FALLBACK_VISUAL.palette[0];
  const secondary = config.palette[1] ?? primary;
  const shape = config.shapeLanguage.join(" ");

  ctx.save();
  ctx.globalAlpha *= alpha;
  ctx.strokeStyle = hexToRgba(primary, 0.95);
  ctx.fillStyle = hexToRgba(secondary, 0.12 + config.glow * 0.18);
  ctx.lineWidth = Math.max(1.4, radius * 0.045);

  if (
    shape.includes("blade") ||
    shape.includes("dagger") ||
    shape.includes("claw") ||
    shape.includes("kunai") ||
    shape.includes("shuriken")
  ) {
    for (let i = 0; i < 4; i += 1) {
      const angle = (Math.PI / 2) * i + radius * 0.01;
      ctx.beginPath();
      ctx.moveTo(
        x + Math.cos(angle) * radius * 0.25,
        y + Math.sin(angle) * radius * 0.25,
      );
      ctx.lineTo(x + Math.cos(angle) * radius, y + Math.sin(angle) * radius);
      ctx.stroke();
    }
  } else if (
    shape.includes("storm") ||
    shape.includes("lightning") ||
    shape.includes("electric")
  ) {
    for (let i = 0; i < 5; i += 1) {
      const angle = (Math.PI * 2 * i) / 5;
      ctx.beginPath();
      ctx.moveTo(
        x + Math.cos(angle) * radius * 0.35,
        y + Math.sin(angle) * radius * 0.35,
      );
      ctx.lineTo(
        x + Math.cos(angle + 0.22) * radius * 0.7,
        y + Math.sin(angle + 0.22) * radius * 0.7,
      );
      ctx.lineTo(
        x + Math.cos(angle - 0.1) * radius,
        y + Math.sin(angle - 0.1) * radius,
      );
      ctx.stroke();
    }
  } else if (
    shape.includes("flower") ||
    shape.includes("petal") ||
    shape.includes("sun") ||
    shape.includes("heart")
  ) {
    for (let i = 0; i < 6; i += 1) {
      const angle = (Math.PI * 2 * i) / 6;
      ctx.beginPath();
      ctx.ellipse(
        x + Math.cos(angle) * radius * 0.42,
        y + Math.sin(angle) * radius * 0.42,
        radius * 0.18,
        radius * 0.36,
        angle,
        0,
        Math.PI * 2,
      );
      ctx.fill();
      ctx.stroke();
    }
  } else {
    ctx.beginPath();
    ctx.arc(x, y, radius * 0.72, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();
  }

  ctx.restore();
}

function drawArrowhead(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  angle: number,
  size: number,
) {
  ctx.beginPath();
  ctx.moveTo(x + Math.cos(angle) * size, y + Math.sin(angle) * size);
  ctx.lineTo(
    x + Math.cos(angle + Math.PI * 0.78) * size * 0.72,
    y + Math.sin(angle + Math.PI * 0.78) * size * 0.72,
  );
  ctx.lineTo(x, y);
  ctx.lineTo(
    x + Math.cos(angle - Math.PI * 0.78) * size * 0.72,
    y + Math.sin(angle - Math.PI * 0.78) * size * 0.72,
  );
  ctx.closePath();
  ctx.fill();
  ctx.stroke();
}

function drawChampionSilhouette(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
  width: number,
  height: number,
  angle: number,
): boolean {
  const kind = getUltimateSilhouetteKind(config);
  const primary = config.palette[0] ?? FALLBACK_VISUAL.palette[0];
  const secondary = config.palette[1] ?? primary;
  const accent = config.palette[2] ?? secondary;
  const pulse = 0.75 + Math.sin(progress * Math.PI * 4) * 0.18;

  ctx.strokeStyle = hexToRgba(primary, alpha * pulse);
  ctx.fillStyle = hexToRgba(secondary, alpha * 0.18);
  ctx.lineWidth = 2 + config.glow * 2;
  ctx.lineCap = "round";
  ctx.lineJoin = "round";

  switch (kind) {
    case "crystal_arrow": {
      const length = 190;
      const headX = x + Math.cos(angle) * length * progress;
      const headY = y + Math.sin(angle) * length * progress;
      ctx.beginPath();
      ctx.moveTo(x - Math.cos(angle) * 28, y - Math.sin(angle) * 28);
      ctx.lineTo(headX, headY);
      ctx.stroke();
      ctx.strokeStyle = hexToRgba(accent, alpha * 0.75);
      for (const wing of [-1, 1]) {
        ctx.beginPath();
        ctx.moveTo(headX - Math.cos(angle) * 34, headY - Math.sin(angle) * 34);
        ctx.lineTo(
          headX - Math.cos(angle) * 64 + Math.cos(angle + wing * Math.PI / 2) * 18,
          headY - Math.sin(angle) * 64 + Math.sin(angle + wing * Math.PI / 2) * 18,
        );
        ctx.stroke();
      }
      drawArrowhead(ctx, headX, headY, angle, 24);
      return true;
    }
    case "chaos_rocket": {
      const length = 165 * progress;
      const headX = x + Math.cos(angle) * length;
      const headY = y + Math.sin(angle) * length;
      ctx.lineWidth = 8 + Math.sin(progress * Math.PI * 8) * 2;
      ctx.strokeStyle = hexToRgba(primary, alpha);
      ctx.beginPath();
      ctx.moveTo(x, y);
      ctx.bezierCurveTo(
        x + Math.cos(angle) * 45 - Math.sin(angle) * 20,
        y + Math.sin(angle) * 45 + Math.cos(angle) * 20,
        headX - Math.cos(angle) * 45 + Math.sin(angle) * 18,
        headY - Math.sin(angle) * 45 - Math.cos(angle) * 18,
        headX,
        headY,
      );
      ctx.stroke();
      ctx.fillStyle = hexToRgba(accent, alpha * 0.35);
      ctx.beginPath();
      ctx.arc(headX, headY, 18 + progress * 22, 0, Math.PI * 2);
      ctx.fill();
      drawSignatureGlyph(ctx, headX, headY, 18, config, alpha);
      return true;
    }
    case "bouncing_fire": {
      const bounceCount = 4;
      for (let i = 0; i < bounceCount; i += 1) {
        const localProgress = Math.min(1, progress + i * 0.12);
        const px = x + Math.cos(angle + i * 0.7) * 38 * i * localProgress;
        const py = y + Math.sin(angle + i * 0.7) * 28 * i * localProgress;
        ctx.fillStyle = hexToRgba(i % 2 === 0 ? primary : accent, alpha * 0.24);
        ctx.beginPath();
        ctx.arc(px, py, 16 + i * 3, 0, Math.PI * 2);
        ctx.fill();
        drawSignatureGlyph(ctx, px, py, 10 + i, config, alpha * 0.8);
      }
      return true;
    }
    case "arcane_barrage":
    case "bullet_cone": {
      const spread = kind === "bullet_cone" ? Math.PI / 5 : Math.PI / 11;
      const rays = kind === "bullet_cone" ? 7 : 4;
      for (let i = 0; i < rays; i += 1) {
        const t = i / (rays - 1) - 0.5;
        const local = angle + t * spread;
        ctx.lineWidth = kind === "bullet_cone" ? 4 : 7;
        ctx.strokeStyle = hexToRgba(i % 2 === 0 ? primary : accent, alpha * 0.78);
        ctx.beginPath();
        ctx.moveTo(x, y);
        ctx.lineTo(
          x + Math.cos(local) * Math.max(width, height) * 0.48,
          y + Math.sin(local) * Math.max(width, height) * 0.48,
        );
        ctx.stroke();
      }
      drawSignatureGlyph(ctx, x, y, 17, config, alpha);
      return true;
    }
    case "glacial_fissure":
    case "equalizer_line":
    case "wind_fate_slash": {
      const reach = Math.max(width, height) * 0.46;
      ctx.lineWidth = kind === "equalizer_line" ? 11 : 7;
      ctx.strokeStyle = hexToRgba(primary, alpha * 0.9);
      ctx.beginPath();
      ctx.moveTo(x - Math.cos(angle) * reach * 0.25, y - Math.sin(angle) * reach * 0.25);
      ctx.lineTo(x + Math.cos(angle) * reach, y + Math.sin(angle) * reach);
      ctx.stroke();
      for (let i = -2; i <= 2; i += 1) {
        const px = x + Math.cos(angle) * i * 24;
        const py = y + Math.sin(angle) * i * 24;
        drawSignatureGlyph(ctx, px, py, 9, config, alpha * 0.72);
      }
      return true;
    }
    case "spinning_axes": {
      for (let i = 0; i < 2; i += 1) {
        const local = angle + i * Math.PI + progress * Math.PI * 4;
        const cx = x + Math.cos(local) * 38;
        const cy = y + Math.sin(local) * 38;
        ctx.strokeStyle = hexToRgba(i === 0 ? primary : accent, alpha);
        ctx.beginPath();
        ctx.ellipse(cx, cy, 24, 8, local, 0, Math.PI * 2);
        ctx.stroke();
      }
      return true;
    }
    case "precision_snipe":
    case "sky_sword":
    case "divine_swords":
    case "prismatic_laser":
    case "void_ray": {
      const reach = Math.max(width, height) * 0.52;
      ctx.lineWidth = kind === "precision_snipe" ? 4 : kind === "void_ray" ? 14 : 10;
      ctx.strokeStyle = hexToRgba(primary, alpha);
      ctx.beginPath();
      const beamAngle = kind === "sky_sword" || kind === "divine_swords" ? -Math.PI / 2 : angle;
      ctx.moveTo(x, y);
      ctx.lineTo(x + Math.cos(beamAngle) * reach, y + Math.sin(beamAngle) * reach);
      ctx.stroke();
      if (kind === "precision_snipe") {
        ctx.beginPath();
        ctx.arc(x, y, 24 + progress * 12, 0, Math.PI * 2);
        ctx.moveTo(x - 36, y);
        ctx.lineTo(x + 36, y);
        ctx.moveTo(x, y - 36);
        ctx.lineTo(x, y + 36);
        ctx.stroke();
      }
      if (kind === "divine_swords") {
        for (let i = -1; i <= 1; i += 1) {
          drawArrowhead(ctx, x + i * 22, y - 46 - progress * 34, Math.PI / 2, 16);
        }
      }
      drawSignatureGlyph(ctx, x, y, 14, config, alpha);
      return true;
    }
    case "soldier_wall": {
      for (let i = -3; i <= 3; i += 1) {
        const px = x + i * 18;
        ctx.fillStyle = hexToRgba(i % 2 === 0 ? primary : accent, alpha * 0.28);
        ctx.fillRect(px - 5, y - 34 - progress * 16, 10, 68 + progress * 24);
        ctx.strokeRect(px - 5, y - 34 - progress * 16, 10, 68 + progress * 24);
      }
      return true;
    }
    case "stasis_hourglass":
    case "chrono_rewind": {
      const r = 34 + progress * 18;
      ctx.strokeStyle = hexToRgba(primary, alpha);
      ctx.beginPath();
      ctx.arc(x, y, r, 0, Math.PI * 2);
      ctx.stroke();
      ctx.beginPath();
      ctx.moveTo(x - r * 0.45, y - r * 0.6);
      ctx.lineTo(x + r * 0.45, y - r * 0.6);
      ctx.lineTo(x - r * 0.45, y + r * 0.6);
      ctx.lineTo(x + r * 0.45, y + r * 0.6);
      ctx.stroke();
      if (kind === "chrono_rewind") {
        ctx.beginPath();
        ctx.arc(x, y, r * 1.28, progress * Math.PI * 2, progress * Math.PI * 2 + Math.PI * 1.4);
        ctx.stroke();
      }
      return true;
    }
    case "death_realm":
    case "hextech_cage":
    case "lamb_wolf_sanctuary": {
      const r = kind === "death_realm" ? 58 + progress * 30 : 46 + progress * 20;
      ctx.fillStyle = hexToRgba(secondary, alpha * (kind === "death_realm" ? 0.24 : 0.12));
      ctx.beginPath();
      if (kind === "hextech_cage") {
        for (let i = 0; i < 6; i += 1) {
          const local = angle + (Math.PI * 2 * i) / 6;
          const px = x + Math.cos(local) * r;
          const py = y + Math.sin(local) * r;
          if (i === 0) ctx.moveTo(px, py);
          else ctx.lineTo(px, py);
        }
        ctx.closePath();
      } else {
        ctx.arc(x, y, r, 0, Math.PI * 2);
      }
      ctx.fill();
      ctx.setLineDash(kind === "death_realm" ? [12, 6] : [3, 9]);
      ctx.stroke();
      ctx.setLineDash([]);
      if (kind === "lamb_wolf_sanctuary") {
        drawSignatureGlyph(ctx, x - r * 0.35, y, 14, config, alpha);
        drawSignatureGlyph(ctx, x + r * 0.35, y, 14, config, alpha * 0.8);
      } else {
        drawSignatureGlyph(ctx, x, y, 22, config, alpha);
      }
      return true;
    }
    case "protective_shield":
    case "shockwave_ring": {
      const r = 36 + progress * 34;
      ctx.fillStyle = hexToRgba(secondary, alpha * 0.12);
      ctx.beginPath();
      ctx.arc(x, y, r, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();
      ctx.beginPath();
      ctx.arc(x, y, r * 0.58, 0, Math.PI * 2);
      ctx.stroke();
      drawSignatureGlyph(ctx, x, y, 16, config, alpha);
      return true;
    }
    case "global_darkness":
    case "card_destiny":
    case "cannon_barrage": {
      ctx.fillStyle = hexToRgba(primary, alpha * (kind === "global_darkness" ? 0.32 : 0.14));
      ctx.fillRect(0, 0, width, height);
      const marks = kind === "cannon_barrage" ? 8 : 5;
      for (let i = 0; i < marks; i += 1) {
        const px = ((i + 1) / (marks + 1)) * width;
        const py = height * (0.25 + ((i * 37) % 50) / 100);
        drawSignatureGlyph(ctx, px, py, kind === "card_destiny" ? 12 : 16, config, alpha * 0.85);
      }
      return true;
    }
    case "requiem_omen": {
      ctx.fillStyle = hexToRgba(primary, alpha * 0.28);
      ctx.fillRect(0, 0, width, height);
      for (let i = 0; i < 5; i += 1) {
        const px = ((i + 1) / 6) * width;
        const py = height * 0.36 + Math.sin(i) * 28;
        drawSignatureGlyph(ctx, px, py, 15 + progress * 6, config, alpha * 0.86);
      }
      return true;
    }
    case "darkin_wings":
    case "demonic_wings":
    case "divine_wings":
    case "dragon_descent":
    case "colossus_landing": {
      const wingSpan = kind === "colossus_landing" ? 64 : 86 + progress * 28;
      const body = 22 + progress * 18;
      ctx.fillStyle = hexToRgba(secondary, alpha * 0.18);
      ctx.beginPath();
      ctx.arc(x, y, body, 0, Math.PI * 2);
      ctx.fill();
      for (const side of [-1, 1]) {
        const sweep = kind === "dragon_descent" ? 0.92 : 0.72;
        ctx.beginPath();
        ctx.moveTo(x, y);
        ctx.quadraticCurveTo(x + side * wingSpan * 0.45, y - wingSpan * sweep, x + side * wingSpan, y - wingSpan * 0.16);
        ctx.quadraticCurveTo(x + side * wingSpan * 0.42, y + wingSpan * 0.34, x, y);
        ctx.fill();
        ctx.stroke();
      }
      if (kind === "colossus_landing") {
        ctx.beginPath();
        ctx.arc(x, y, 42 + progress * 35, 0, Math.PI * 2);
        ctx.stroke();
      }
      drawSignatureGlyph(ctx, x, y, 18, config, alpha);
      return true;
    }
    case "blade_curtain":
    case "nature_vines":
    case "music_wave":
    case "cosmic_stars":
    case "void_bloom":
    case "hextech_construct":
    case "water_wave":
    case "ice_prison":
    case "fire_explosion": {
      const count = kind === "blade_curtain" ? 9 : kind === "hextech_construct" ? 6 : 7;
      const r = 28 + progress * 42;
      if (kind === "water_wave" || kind === "music_wave") {
        for (let i = 0; i < 3; i += 1) {
          ctx.beginPath();
          ctx.arc(x, y, r * (0.55 + i * 0.22), angle - Math.PI * 0.82, angle + Math.PI * 0.82);
          ctx.stroke();
        }
      } else if (kind === "ice_prison" || kind === "hextech_construct") {
        const sides = kind === "hextech_construct" ? 6 : 5;
        ctx.beginPath();
        for (let i = 0; i <= sides; i += 1) {
          const local = angle + (Math.PI * 2 * i) / sides;
          const px = x + Math.cos(local) * r;
          const py = y + Math.sin(local) * r;
          if (i === 0) ctx.moveTo(px, py);
          else ctx.lineTo(px, py);
        }
        ctx.fill();
        ctx.stroke();
      } else {
        ctx.beginPath();
        ctx.arc(x, y, r, 0, Math.PI * 2);
        ctx.fill();
        ctx.stroke();
      }
      for (let i = 0; i < count; i += 1) {
        const local = angle + (Math.PI * 2 * i) / count + progress * Math.PI * (kind === "blade_curtain" ? 0.35 : 1);
        const distance = kind === "fire_explosion" || kind === "cosmic_stars" ? r * (0.35 + (i % 3) * 0.18) : r * 0.78;
        drawSignatureGlyph(ctx, x + Math.cos(local) * distance, y + Math.sin(local) * distance, kind === "blade_curtain" ? 8 : 11, config, alpha * 0.78);
      }
      return true;
    }
    case "meteor_fall":
    case "tibbers_bear":
    case "turret_construct":
    case "trap_mushroom":
    case "tentacle_slam":
    case "thorn_roots":
    case "feather_storm":
    case "storm_field":
    case "static_field":
    case "crowstorm":
    case "moonlight_vigil":
    case "petal_charm":
    case "mask_orbit": {
      const r = 30 + progress * 36;
      const count = kind === "feather_storm" ? 8 : kind === "storm_field" || kind === "static_field" || kind === "crowstorm" ? 10 : 5;
      ctx.beginPath();
      ctx.arc(x, y, r, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();
      for (let i = 0; i < count; i += 1) {
        const local = (Math.PI * 2 * i) / count + progress * Math.PI;
        drawSignatureGlyph(
          ctx,
          x + Math.cos(local) * r * 0.78,
          y + Math.sin(local) * r * 0.78,
          kind === "trap_mushroom" ? 8 : 11,
          config,
          alpha * 0.75,
        );
      }
      return true;
    }
    case "drowned_x":
    case "guillotine_axe":
    case "blade_execution":
    case "two_stage_execution": {
      drawExecuteMarker(ctx, x, y, config, progress, alpha);
      if (kind === "two_stage_execution") drawDashTrail(ctx, x, y, config, progress, alpha * 0.75, angle);
      return true;
    }
    case "mountain_impact":
    case "gauntlet_lockon": {
      drawDashTrail(ctx, x, y, config, progress, alpha, angle, kind === "mountain_impact");
      ctx.beginPath();
      ctx.arc(x, y, 22 + progress * 28, 0, Math.PI * 2);
      ctx.stroke();
      return true;
    }
    case "shadow_mark":
    case "spirit_dash":
    case "beast_charge": {
      drawAssassinOrBlink(ctx, x, y, config, progress, alpha, angle, kind === "spirit_dash");
      return true;
    }
    case "signature_composite":
    default:
      return false;
  }
}

function drawDashTrail(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
  angle: number,
  charge = false,
) {
  const length = charge ? 145 : 92;
  const startX = x - Math.cos(angle) * length * (0.25 + progress * 0.55);
  const startY = y - Math.sin(angle) * length * (0.25 + progress * 0.55);
  ctx.lineCap = charge ? "butt" : "round";
  ctx.strokeStyle = hexToRgba(
    config.palette[0] ?? FALLBACK_VISUAL.palette[0],
    alpha,
  );
  ctx.lineWidth = (charge ? 9 : 5) + config.glow * 4;
  ctx.beginPath();
  ctx.moveTo(startX, startY);
  ctx.lineTo(x + Math.cos(angle) * 22, y + Math.sin(angle) * 22);
  ctx.stroke();
  drawSignatureGlyph(ctx, x, y, charge ? 20 : 15, config, alpha);
}

function drawExecuteMarker(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
) {
  const radius = 24 + progress * 12;
  const shape = config.shapeLanguage.join(" ");
  ctx.strokeStyle = hexToRgba(
    config.palette[0] ?? FALLBACK_VISUAL.palette[0],
    alpha,
  );
  ctx.lineWidth = 3 + config.glow * 2;
  if (shape.includes("x_marker") || shape.includes("drowned_x")) {
    ctx.beginPath();
    ctx.moveTo(x - radius, y - radius);
    ctx.lineTo(x + radius, y + radius);
    ctx.moveTo(x + radius, y - radius);
    ctx.lineTo(x - radius, y + radius);
    ctx.stroke();
  } else {
    ctx.beginPath();
    ctx.moveTo(x, y - radius);
    ctx.lineTo(x - radius * 0.4, y + radius * 0.35);
    ctx.lineTo(x + radius * 0.4, y + radius * 0.35);
    ctx.closePath();
    ctx.stroke();
  }
  drawSignatureGlyph(ctx, x, y, 12 + progress * 8, config, alpha);
}

function drawLockOrRealm(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
  realm = false,
) {
  const radius = realm ? 48 + progress * 18 : 30 + progress * 10;
  ctx.strokeStyle = hexToRgba(
    config.palette[0] ?? FALLBACK_VISUAL.palette[0],
    alpha,
  );
  ctx.fillStyle = hexToRgba(
    config.palette[1] ?? config.palette[0] ?? FALLBACK_VISUAL.palette[1],
    alpha * (realm ? 0.22 : 0.12),
  );
  ctx.lineWidth = realm ? 4 : 3;
  if (realm) ctx.setLineDash([10, 5]);
  ctx.beginPath();
  ctx.arc(x, y, radius, 0, Math.PI * 2);
  ctx.fill();
  ctx.stroke();
  ctx.setLineDash([]);
  if (!realm) {
    ctx.beginPath();
    ctx.moveTo(x - radius, y);
    ctx.lineTo(x + radius, y);
    ctx.moveTo(x, y - radius);
    ctx.lineTo(x, y + radius);
    ctx.stroke();
  }
  drawSignatureGlyph(ctx, x, y, realm ? 22 : 16, config, alpha);
}

function drawAssassinOrBlink(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
  angle: number,
  blink = false,
) {
  const clones = blink ? 3 : 2;
  for (let i = 0; i < clones; i += 1) {
    const localAngle = angle + (Math.PI * 2 * i) / clones;
    const ghostX = x - Math.cos(localAngle) * (26 + progress * 38);
    const ghostY = y - Math.sin(localAngle) * (26 + progress * 38);
    drawSignatureGlyph(
      ctx,
      ghostX,
      ghostY,
      blink ? 10 : 13,
      config,
      alpha * (0.45 + i * 0.12),
    );
  }
  drawDashTrail(ctx, x, y, config, progress, alpha * 0.9, angle);
}

function signatureAngle(signatureId: string): number {
  const seed = Array.from(signatureId).reduce(
    (acc, char) => acc + char.charCodeAt(0),
    0,
  );
  return (seed % 360) * (Math.PI / 180);
}

const primitiveShapeFallback = (primitive: UltimatePrimitive | "fallback"): UltimateSpatialRenderContext["shape"] => {
  switch (primitive) {
    case "self_aura":
    case "ally_aura":
      return "aura";
    case "linear_projectile":
      return "projectile";
    case "beam_line":
      return "beam";
    case "global_presence":
      return "global_overlay";
    case "targeted_dash":
    case "unstoppable_charge":
      return "projectile";
    case "execute_marker":
    case "suppression_lock":
    case "duel_realm":
    case "assassin_mark":
    case "blink_burst":
      return "lock";
    case "zone_summon":
      return "zone";
    default:
      return "circle";
  }
};

const normalizeDirection = (direction?: Vec2): Vec2 | undefined => {
  if (!direction) return undefined;
  const len = Math.hypot(direction.x, direction.y);
  if (len <= 1e-6) return undefined;
  return { x: direction.x / len, y: direction.y / len };
};

export function resolveUltimateSpatialRenderContext(
  metadata: UltimateIdentityEventMetadata,
  actorPos: Vec2,
  config: UltimateVisualConfig,
  positions?: Map<string, Vec2>,
): UltimateSpatialRenderContext {
  const proxyOrigin =
    metadata.proxyOriginKind && metadata.targetPos
      ? metadata.targetPos
      : metadata.originPos ?? actorPos;
  const origin = proxyOrigin;
  const lockedTarget = resolveLiveTarget(metadata, positions);
  const target = lockedTarget ?? metadata.targetPos;
  const targetIds = uniqueIds(
    [metadata.lockedTargetId, metadata.targetId],
    metadata.targetIds,
    metadata.affectedTargetIds,
  );
  const targetPoints = [
    ...pointsForIds(targetIds, positions),
    ...(target && !targetIds.some((id) => positions?.get(id) === target) ? [target] : []),
  ];
  const bouncePoints = resolveBouncePoints(metadata, targetPoints, positions);
  const returnPathPoints = metadata.returnPath?.filter(isVec2) ?? [];
  const direction = normalizeDirection(
    metadata.direction ?? (target ? { x: target.x - origin.x, y: target.y - origin.y } : undefined),
  );
  return {
    origin,
    target,
    direction,
    angle: direction ? Math.atan2(direction.y, direction.x) : signatureAngle(config.signatureId),
    shape: metadata.shape ?? primitiveShapeFallback(config.primitive),
    usedFallbackAngle: !direction,
    lockedTarget,
    targetPoints,
    bouncePoints,
    returnPathPoints,
    followTarget: metadata.followTarget ?? false,
    stage: metadata.stage,
    stageCount: metadata.stageCount,
    sequenceKind: metadata.sequenceKind,
    recastWindowMs: metadata.recastWindowMs,
    tetherKind: metadata.tetherKind,
    bespokeKind:
      metadata.bespokeKind ??
      BESPOKE_SIGNATURE_KINDS[config.signatureId],
    secondary: metadata.secondaryPos,
    destination: metadata.destinationPos ?? (metadata.bespokeKind || BESPOKE_SIGNATURE_KINDS[config.signatureId] ? metadata.targetPos : undefined),
    zoneOrientation: normalizeDirection(metadata.zoneOrientation),
    requiresCondition: metadata.requiresCondition,
    proxyOriginKind: metadata.proxyOriginKind,
  };
}

function drawLinearProjectileSignature(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
  angle: number,
) {
  const length =
    config.motion.includes("long_glide") ||
    config.motion.includes("map_long_sweep")
      ? 150
      : 105;
  const wobble = config.motion.some(
    (motion) => motion.includes("wobbly") || motion.includes("ricochet"),
  )
    ? Math.sin(progress * Math.PI * 6) * 10
    : 0;
  const headX =
    x + Math.cos(angle) * length * progress - Math.sin(angle) * wobble;
  const headY =
    y + Math.sin(angle) * length * progress + Math.cos(angle) * wobble;

  ctx.lineCap = config.shapeLanguage.join(" ").includes("axe")
    ? "butt"
    : "round";
  ctx.strokeStyle = hexToRgba(
    config.palette[0] ?? FALLBACK_VISUAL.palette[0],
    alpha,
  );
  ctx.lineWidth = 3 + config.glow * 3;
  ctx.beginPath();
  ctx.moveTo(x, y);
  ctx.lineTo(headX, headY);
  ctx.stroke();
  drawSignatureGlyph(
    ctx,
    headX,
    headY,
    config.shapeLanguage.join(" ").includes("rocket") ? 16 : 11,
    config,
    alpha,
  );
}

function drawBeamSignature(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
  width: number,
  height: number,
  angleHint: number,
) {
  const angle = config.motion.some(
    (motion) => motion.includes("heaven") || motion.includes("sword_rain"),
  )
    ? -Math.PI / 2
    : angleHint;
  const reach = Math.max(width, height) * 0.45;
  const beamWidth = config.motion.some(
    (motion) => motion.includes("barrage") || motion.includes("cone"),
  )
    ? 22
    : 8;
  const pulse = 0.65 + Math.sin(progress * Math.PI * 4) * 0.25;

  ctx.lineCap = "round";
  ctx.strokeStyle = hexToRgba(
    config.palette[0] ?? FALLBACK_VISUAL.palette[0],
    alpha * pulse,
  );
  ctx.lineWidth = beamWidth * (0.7 + config.glow * 0.8);
  ctx.beginPath();
  ctx.moveTo(x, y);
  ctx.lineTo(x + Math.cos(angle) * reach, y + Math.sin(angle) * reach);
  ctx.stroke();
  drawSignatureGlyph(ctx, x, y, 14 + config.glow * 8, config, alpha);
}

function drawArtillerySignature(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
) {
  const radius = 22 + progress * 32;
  ctx.setLineDash([5, 7]);
  ctx.strokeStyle = hexToRgba(
    config.palette[0] ?? FALLBACK_VISUAL.palette[0],
    alpha,
  );
  ctx.lineWidth = 2 + config.glow * 2;
  ctx.beginPath();
  ctx.arc(x, y, radius, 0, Math.PI * 2);
  ctx.stroke();
  ctx.setLineDash([]);
  drawSignatureGlyph(
    ctx,
    x,
    y - 28 * (1 - progress),
    12 + progress * 10,
    config,
    alpha,
  );
}

function drawGlobalPresenceSignature(
  ctx: CanvasRenderingContext2D,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
  width: number,
  height: number,
) {
  const primary = config.palette[0] ?? FALLBACK_VISUAL.palette[0];
  ctx.fillStyle = hexToRgba(
    primary,
    alpha * (config.motion.includes("lights_out") ? 0.28 : 0.14),
  );
  ctx.fillRect(0, 0, width, height);
  const centerX = width / 2;
  const centerY = height / 2;
  const radius = Math.max(width, height) * (0.12 + progress * 0.28);
  ctx.strokeStyle = hexToRgba(config.palette[1] ?? primary, alpha * 0.75);
  ctx.lineWidth = 2 + config.glow * 2;
  ctx.beginPath();
  ctx.arc(centerX, centerY, radius, 0, Math.PI * 2);
  ctx.stroke();
  drawSignatureGlyph(
    ctx,
    centerX,
    centerY,
    Math.max(18, radius * 0.18),
    config,
    alpha * 0.9,
  );
}

function toCanvasPoint(point: Vec2, width: number, height: number): Vec2 {
  return { x: point.x * width, y: point.y * height };
}

function spatialEndPoint(
  spatial: UltimateSpatialRenderContext,
  width: number,
  height: number,
  range = 0.5,
): Vec2 {
  if (spatial.target) return toCanvasPoint(spatial.target, width, height);
  const origin = toCanvasPoint(spatial.origin, width, height);
  const reach = Math.max(width, height) * range;
  return {
    x: origin.x + Math.cos(spatial.angle) * reach,
    y: origin.y + Math.sin(spatial.angle) * reach,
  };
}

function drawSpatialAuraOrCircle(
  ctx: CanvasRenderingContext2D,
  center: Vec2,
  radius: number,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
  pulseCount = 1,
) {
  ctx.strokeStyle = hexToRgba(config.palette[0] ?? FALLBACK_VISUAL.palette[0], alpha);
  ctx.fillStyle = hexToRgba(
    config.palette[1] ?? config.palette[0] ?? FALLBACK_VISUAL.palette[1],
    alpha * 0.18,
  );
  ctx.lineWidth = 2 + config.glow * 2;
  ctx.beginPath();
  ctx.arc(center.x, center.y, radius, 0, Math.PI * 2);
  ctx.fill();
  ctx.stroke();
  drawSignatureGlyph(ctx, center.x, center.y, Math.max(14, radius * 0.48), config, alpha);
  if (progress > 0.35 || pulseCount > 1) {
    const tick = 0.65 + Math.sin(progress * Math.PI * 2 * pulseCount) * 0.25;
    ctx.globalAlpha *= tick;
    ctx.beginPath();
    ctx.arc(center.x, center.y, radius * 0.62, 0, Math.PI * 2);
    ctx.stroke();
  }
}

function drawSpatialLineLike(
  ctx: CanvasRenderingContext2D,
  start: Vec2,
  end: Vec2,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
  shape: "line" | "projectile" | "beam",
  pulseCount = 1,
) {
  const dx = end.x - start.x;
  const dy = end.y - start.y;
  const head = shape === "projectile" ? progress : 1;
  const headX = start.x + dx * head;
  const headY = start.y + dy * head;
  const angle = Math.atan2(dy, dx);

  ctx.lineCap = shape === "line" ? "butt" : "round";
  ctx.strokeStyle = hexToRgba(config.palette[0] ?? FALLBACK_VISUAL.palette[0], alpha);
  const channelPulse = shape === "beam" ? 0.78 + Math.sin(progress * Math.PI * 2 * pulseCount) * 0.22 : 1;
  ctx.lineWidth = (shape === "beam" ? 10 + config.glow * 8 : shape === "line" ? 6 + config.glow * 4 : 4 + config.glow * 3) * channelPulse;
  ctx.beginPath();
  ctx.moveTo(start.x, start.y);
  ctx.lineTo(headX, headY);
  ctx.stroke();

  if (shape === "beam") {
    ctx.strokeStyle = hexToRgba(config.palette[1] ?? config.palette[0] ?? FALLBACK_VISUAL.palette[1], alpha * 0.65);
    ctx.lineWidth *= 0.42;
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.lineTo(end.x, end.y);
    ctx.stroke();
  }

  drawSignatureGlyph(
    ctx,
    shape === "projectile" ? headX : start.x,
    shape === "projectile" ? headY : start.y,
    shape === "projectile" ? 14 : 16,
    config,
    alpha,
  );
  if (shape !== "beam") drawArrowhead(ctx, headX, headY, angle, shape === "line" ? 18 : 22);
}

function drawSpatialGlobalOverlay(
  ctx: CanvasRenderingContext2D,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
  width: number,
  height: number,
) {
  drawGlobalPresenceSignature(ctx, config, progress, alpha, width, height);
}

function drawUltimateTelegraph(
  ctx: CanvasRenderingContext2D,
  center: Vec2,
  radius: number,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
) {
  const primary = config.palette[0] ?? FALLBACK_VISUAL.palette[0];
  const secondary = config.palette[1] ?? primary;
  const reticleRadius = radius * (0.82 + progress * 0.18);
  ctx.setLineDash([8, 6]);
  ctx.strokeStyle = hexToRgba(primary, alpha * (0.55 + progress * 0.35));
  ctx.fillStyle = hexToRgba(secondary, alpha * 0.08);
  ctx.lineWidth = 2 + config.glow * 1.8;
  ctx.beginPath();
  ctx.arc(center.x, center.y, reticleRadius, 0, Math.PI * 2);
  ctx.fill();
  ctx.stroke();
  ctx.setLineDash([]);
  ctx.beginPath();
  ctx.moveTo(center.x - reticleRadius * 0.55, center.y);
  ctx.lineTo(center.x + reticleRadius * 0.55, center.y);
  ctx.moveTo(center.x, center.y - reticleRadius * 0.55);
  ctx.lineTo(center.x, center.y + reticleRadius * 0.55);
  ctx.stroke();
}

function drawLockReticleLine(
  ctx: CanvasRenderingContext2D,
  start: Vec2,
  target: Vec2,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
) {
  ctx.setLineDash([7, 6]);
  ctx.strokeStyle = hexToRgba(config.palette[0] ?? FALLBACK_VISUAL.palette[0], alpha * 0.8);
  ctx.lineWidth = 1.5 + config.glow * 1.5;
  ctx.beginPath();
  ctx.moveTo(start.x, start.y);
  ctx.lineTo(target.x, target.y);
  ctx.stroke();
  ctx.setLineDash([]);
  drawUltimateTelegraph(ctx, target, 16 + progress * 9, config, progress, alpha);
}

function drawTetherLine(
  ctx: CanvasRenderingContext2D,
  start: Vec2,
  target: Vec2,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
) {
  const segments = 4;
  ctx.strokeStyle = hexToRgba(config.palette[1] ?? config.palette[0] ?? FALLBACK_VISUAL.palette[1], alpha * 0.9);
  ctx.lineWidth = 2.2 + config.glow * 2.4;
  ctx.beginPath();
  ctx.moveTo(start.x, start.y);
  for (let i = 1; i <= segments; i += 1) {
    const t = i / segments;
    const wave = Math.sin((t + progress) * Math.PI * 4) * 5;
    const x = start.x + (target.x - start.x) * t;
    const y = start.y + (target.y - start.y) * t;
    const angle = Math.atan2(target.y - start.y, target.x - start.x) + Math.PI / 2;
    ctx.lineTo(x + Math.cos(angle) * wave, y + Math.sin(angle) * wave);
  }
  ctx.stroke();
}

function drawBounceChain(
  ctx: CanvasRenderingContext2D,
  points: Vec2[],
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
) {
  if (points.length < 2) return;
  ctx.strokeStyle = hexToRgba(config.palette[0] ?? FALLBACK_VISUAL.palette[0], alpha * 0.85);
  ctx.lineWidth = 2 + config.glow * 2;
  for (let i = 0; i < points.length - 1; i += 1) {
    const start = points[i];
    const end = points[i + 1];
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.quadraticCurveTo(
      (start.x + end.x) / 2,
      (start.y + end.y) / 2 - 18 * Math.sin(progress * Math.PI),
      end.x,
      end.y,
    );
    ctx.stroke();
    drawSignatureGlyph(ctx, end.x, end.y, 8 + i * 1.5, config, alpha * (0.85 - i * 0.08));
  }
}

function drawReturnPath(
  ctx: CanvasRenderingContext2D,
  points: Vec2[],
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
) {
  if (points.length < 2) return;
  const outwardEnd = Math.max(1, Math.floor(points.length / 2));
  ctx.strokeStyle = hexToRgba(config.palette[0] ?? FALLBACK_VISUAL.palette[0], alpha * 0.9);
  ctx.lineWidth = 3 + config.glow * 2;
  ctx.beginPath();
  ctx.moveTo(points[0].x, points[0].y);
  points.slice(1, outwardEnd + 1).forEach((point) => ctx.lineTo(point.x, point.y));
  ctx.stroke();
  ctx.setLineDash([5, 5]);
  ctx.strokeStyle = hexToRgba(config.palette[1] ?? config.palette[0] ?? FALLBACK_VISUAL.palette[1], alpha * 0.75);
  ctx.beginPath();
  ctx.moveTo(points[outwardEnd].x, points[outwardEnd].y);
  points.slice(outwardEnd + 1).forEach((point) => ctx.lineTo(point.x, point.y));
  ctx.stroke();
  ctx.setLineDash([]);
  const head = points[Math.min(points.length - 1, Math.round(progress * (points.length - 1)))];
  drawSignatureGlyph(ctx, head.x, head.y, 13, config, alpha);
}

function drawStageOverlay(
  ctx: CanvasRenderingContext2D,
  center: Vec2,
  config: UltimateVisualConfig,
  alpha: number,
  stage?: number,
  stageCount?: number,
) {
  if (!stageCount || stageCount <= 1) return;
  const radius = 24 + stageCount * 2;
  ctx.strokeStyle = hexToRgba(config.palette[1] ?? config.palette[0] ?? FALLBACK_VISUAL.palette[1], alpha * 0.75);
  ctx.lineWidth = 2;
  for (let i = 0; i < stageCount; i += 1) {
    const start = -Math.PI / 2 + (Math.PI * 2 * i) / stageCount;
    const end = start + (Math.PI * 2) / stageCount - 0.12;
    ctx.globalAlpha *= i < (stage ?? 1) ? 1 : 0.35;
    ctx.beginPath();
    ctx.arc(center.x, center.y, radius, start, end);
    ctx.stroke();
    ctx.globalAlpha /= i < (stage ?? 1) ? 1 : 0.35;
  }
}

function drawBespokeUltimate(
  ctx: CanvasRenderingContext2D,
  spatial: UltimateSpatialRenderContext,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
  width: number,
  height: number,
  radius: number,
): boolean {
  if (!spatial.bespokeKind) return false;
  const primary = config.palette[0] ?? FALLBACK_VISUAL.palette[0];
  const secondary = config.palette[1] ?? primary;
  const origin = toCanvasPoint(spatial.origin, width, height);
  const target = toCanvasPoint(spatial.target ?? spatial.destination ?? spatial.origin, width, height);
  const destination = spatial.destination ? toCanvasPoint(spatial.destination, width, height) : target;
  const angle = spatial.zoneOrientation
    ? Math.atan2(spatial.zoneOrientation.y, spatial.zoneOrientation.x)
    : spatial.angle;

  ctx.strokeStyle = hexToRgba(primary, alpha);
  ctx.fillStyle = hexToRgba(secondary, alpha * 0.18);
  ctx.lineWidth = 2.5 + config.glow * 2;
  ctx.lineCap = "round";
  ctx.lineJoin = "round";

  switch (spatial.bespokeKind) {
    case "stolen_ultimate_pending":
      drawTetherLine(ctx, origin, target, config, progress, alpha);
      drawSignatureGlyph(ctx, target.x, target.y, 20 + progress * 8, config, alpha);
      ctx.setLineDash([6, 5]);
      ctx.strokeRect(target.x - 34, target.y - 20, 68, 40);
      ctx.setLineDash([]);
      ctx.fillStyle = hexToRgba(primary, alpha * 0.22);
      ctx.fillRect(target.x - 32, target.y - 18, 64, 36);
      return true;
    case "death_realm":
      ctx.fillStyle = hexToRgba("#020617", alpha * 0.38);
      ctx.fillRect(0, 0, width, height);
      drawLockOrRealm(ctx, target.x, target.y, config, progress, alpha, true);
      return true;
    case "portal":
      for (const point of [origin, destination]) {
        ctx.beginPath();
        ctx.ellipse(point.x, point.y, radius * 0.82, radius * 0.42, progress * Math.PI, 0, Math.PI * 2);
        ctx.fill();
        ctx.stroke();
        drawSignatureGlyph(ctx, point.x, point.y, 14, config, alpha);
      }
      drawTetherLine(ctx, origin, destination, config, progress, alpha * 0.7);
      return true;
    case "global_reveal_gate":
      drawSpatialGlobalOverlay(ctx, config, progress, alpha, width, height);
      drawUltimateTelegraph(ctx, destination, radius, config, progress, alpha);
      drawSignatureGlyph(ctx, destination.x, destination.y, 18, config, alpha);
      return true;
    case "ally_shield_arrival":
      drawSpatialAuraOrCircle(ctx, target, radius, config, progress, alpha, 2);
      drawUltimateTelegraph(ctx, destination, radius * 0.72, config, progress, alpha);
      return true;
    case "sanctuary_heal":
      drawSpatialAuraOrCircle(ctx, target, radius * 1.25, config, progress, alpha, 6);
      if (progress > 0.68) drawSpatialAuraOrCircle(ctx, target, radius * 0.72, config, progress, alpha * 0.8, 1);
      return true;
    case "soldier_wall":
    case "terrain_wall": {
      const length = spatial.bespokeKind === "terrain_wall" ? Math.max(width, height) * 0.72 : radius * 3.6;
      ctx.lineWidth = spatial.bespokeKind === "terrain_wall" ? 12 : 8;
      ctx.beginPath();
      ctx.moveTo(target.x - Math.cos(angle) * length * 0.5, target.y - Math.sin(angle) * length * 0.5);
      ctx.lineTo(target.x + Math.cos(angle) * length * 0.5, target.y + Math.sin(angle) * length * 0.5);
      ctx.stroke();
      for (let i = -3; i <= 3; i += 1) {
        drawSignatureGlyph(ctx, target.x + Math.cos(angle) * i * 18, target.y + Math.sin(angle) * i * 18, 10, config, alpha);
      }
      return true;
    }
    case "airborne_slash":
      drawUltimateTelegraph(ctx, target, radius * 0.78, config, progress, alpha);
      drawChampionSilhouette(ctx, target.x, target.y - 18, config, progress, alpha, width, height, -Math.PI / 2);
      return true;
    case "proxy_shockwave":
      drawSpatialAuraOrCircle(ctx, origin, radius * 1.05, config, progress, alpha, 1);
      return true;
    case "two_stage_ram":
      drawSpatialLineLike(ctx, origin, destination, config, progress, alpha, "projectile", 1);
      drawSpatialLineLike(ctx, destination, origin, config, 1 - progress * 0.45, alpha * 0.75, "projectile", 1);
      return true;
    case "blackout_dash":
      ctx.fillStyle = hexToRgba("#020617", alpha * 0.44);
      ctx.fillRect(0, 0, width, height);
      drawLockReticleLine(ctx, origin, target, config, progress, alpha);
      drawDashTrail(ctx, target.x, target.y, config, progress, alpha, Math.atan2(target.y - origin.y, target.x - origin.x));
      return true;
    case "global_landing":
      drawSpatialGlobalOverlay(ctx, config, progress, alpha * 0.75, width, height);
      drawUltimateTelegraph(ctx, destination, radius * 1.25, config, progress, alpha);
      return true;
    case "rewind_ghost": {
      const ghost = spatial.secondary ? toCanvasPoint(spatial.secondary, width, height) : origin;
      drawReturnPath(ctx, [origin, ghost], config, progress, alpha);
      drawSignatureGlyph(ctx, ghost.x, ghost.y, 24, config, alpha * 0.72);
      drawSignatureGlyph(ctx, origin.x, origin.y, 16, config, alpha);
      return true;
    }
    case "feather_fan_recall":
      drawChampionSilhouette(ctx, origin.x, origin.y, config, progress, alpha, width, height, angle);
      drawReturnPath(ctx, [origin, target], config, progress, alpha * 0.8);
      return true;
    case "host_waves":
      drawTetherLine(ctx, origin, target, config, progress, alpha);
      drawBeamSignature(ctx, target.x, target.y, config, progress, alpha, width, height, angle);
      return true;
    default:
      return false;
  }
}

export function drawUltimateIdentityEvents(
  ctx: CanvasRenderingContext2D,
  events: SimEvent[],
  units: Array<{ id: string; pos: Vec2 }>,
  timeSec: number,
  width: number,
  height: number,
) {
  const positions = unitPositionByActor(units);
  const nowMs = timeSec * 1000;

  events.slice(-20).forEach((event) => {
    if (!isUltimateIdentityMetadata(event.metadata)) return;
    const config = resolveUltimateVisualConfig(event.metadata);
    const rawTiming = resolveUltimateRenderTiming(event.metadata, config.signatureId);
    const timing: UltimateRenderTiming = {
      ...rawTiming,
      durationMs: config.durationMs,
      totalMs: Math.max(config.durationMs, rawTiming.impactAt + (config.persistent || config.pulseCount > 2 ? config.durationMs : 650)),
    };
    const elapsedMs = nowMs - event.t * 1000;
    if (elapsedMs < 0 || elapsedMs > timing.totalMs) return;

    const isGlobalShape = event.metadata.shape === "global" || event.metadata.shape === "global_overlay" || event.metadata.global;
    const pos = event.metadata.originPos ?? positions.get(event.metadata.actorId ?? "") ?? (isGlobalShape ? { x: 0.5, y: 0.5 } : undefined);
    if (!pos) return;
    const spatial = resolveUltimateSpatialRenderContext(event.metadata, pos, config, positions);

    const phase = resolveUltimateRenderPhase(elapsedMs, timing);
    const windupProgress = timing.impactAt > 0 ? Math.max(0, Math.min(1, elapsedMs / timing.impactAt)) : 1;
    const activeElapsed = Math.max(0, elapsedMs - timing.impactAt);
    const progress = Math.max(0, Math.min(1, activeElapsed / Math.max(1, timing.durationMs)));
    const sustainAlpha = config.persistent || config.pulseCount > 2 ? 0.45 : 1 - progress;
    const phaseBoost = phase === "impact" ? 1 : phase === "windup" ? 0.72 : phase === "fade" ? 0.35 : 0.82;
    const alpha = Math.max(0.12, sustainAlpha * phaseBoost) * (0.45 + config.glow * 0.45);
    const baseRadius =
      config.primitive === "aoe_pulse"
        ? 44
        : config.primitive === "zone_summon"
          ? 36
          : 26;
    const radius =
      baseRadius +
      progress *
        (config.primitive === "self_aura" || config.primitive === "ally_aura"
          ? 26
          : 46);
    const anchor =
      spatial.shape === "circle" || spatial.shape === "zone" || spatial.shape === "lock"
        ? (spatial.target ?? spatial.origin)
        : spatial.origin;
    const x = anchor.x * width;
    const y = anchor.y * height;

    ctx.save();
    ctx.globalCompositeOperation = "lighter";
    ctx.shadowColor = config.palette[0] ?? FALLBACK_VISUAL.palette[0];
    ctx.shadowBlur = 12 + config.glow * 18;

    const originCanvas = toCanvasPoint(spatial.origin, width, height);
    const targetCanvas = spatial.lockedTarget ? toCanvasPoint(spatial.lockedTarget, width, height) : undefined;
    if (targetCanvas && (spatial.followTarget || spatial.shape === "lock")) {
      drawLockReticleLine(ctx, originCanvas, targetCanvas, config, progress, alpha);
    }
    if (targetCanvas && spatial.tetherKind) {
      drawTetherLine(ctx, originCanvas, targetCanvas, config, progress, alpha);
    }
    if (spatial.bouncePoints.length > 1) {
      drawBounceChain(ctx, spatial.bouncePoints.map((point) => toCanvasPoint(point, width, height)), config, progress, alpha);
    }
    const returnPath = spatial.returnPathPoints.length > 0
      ? spatial.returnPathPoints
      : event.metadata.returnToOrigin && spatial.target
        ? [spatial.origin, spatial.target, spatial.origin]
        : [];
    if (returnPath.length > 1) {
      drawReturnPath(ctx, returnPath.map((point) => toCanvasPoint(point, width, height)), config, progress, alpha);
    }
    drawStageOverlay(ctx, { x, y }, config, alpha, spatial.stage, spatial.stageCount);

    if (phase === "windup") {
      const telegraphCenter =
        spatial.shape === "global" || spatial.shape === "global_overlay"
          ? { x: width / 2, y: height / 2 }
          : toCanvasPoint(spatial.shape === "aura" ? spatial.origin : (spatial.target ?? spatial.origin), width, height);
      const telegraphRadius = event.metadata.radius
        ? event.metadata.radius * Math.min(width, height)
        : spatial.shape === "global" || spatial.shape === "global_overlay"
          ? Math.max(width, height) * (0.18 + windupProgress * 0.12)
          : baseRadius + 36;
      drawUltimateTelegraph(ctx, telegraphCenter, telegraphRadius, config, windupProgress, alpha);
      ctx.restore();
      return;
    }

    if (drawBespokeUltimate(ctx, spatial, config, progress, alpha, width, height, radius)) {
      ctx.restore();
      return;
    }

    if (
      hasExplicitUltimateSignatureVisual(config.signatureId) &&
      drawChampionSilhouette(ctx, x, y, config, progress, alpha, width, height, spatial.angle)
    ) {
      ctx.restore();
      return;
    }

    if (spatial.shape === "global" || spatial.shape === "global_overlay") {
      drawSpatialGlobalOverlay(ctx, config, progress, alpha, width, height);
      ctx.restore();
      return;
    }

    if (spatial.shape === "aura" || spatial.shape === "circle" || spatial.shape === "zone") {
      const spatialCenter = spatial.shape === "aura" ? spatial.origin : (spatial.target ?? spatial.origin);
      const center = toCanvasPoint(spatialCenter, width, height);
      const metadataRadius = event.metadata.radius ? event.metadata.radius * Math.min(width, height) : undefined;
      drawSpatialAuraOrCircle(
        ctx,
        center,
        metadataRadius ?? radius,
        config,
        progress,
        alpha,
        config.pulseCount,
      );
      ctx.restore();
      return;
    }

    if (spatial.shape === "line" || spatial.shape === "projectile" || spatial.shape === "beam") {
      const start = toCanvasPoint(spatial.origin, width, height);
      const end = spatialEndPoint(spatial, width, height, event.metadata.range ?? 0.5);
      drawSpatialLineLike(ctx, start, end, config, progress, alpha, spatial.shape, config.pulseCount);
      ctx.restore();
      return;
    }

    if (drawChampionSilhouette(ctx, x, y, config, progress, alpha, width, height, spatial.angle)) {
      ctx.restore();
      return;
    }

    if (config.primitive === "linear_projectile") {
      drawLinearProjectileSignature(ctx, x, y, config, progress, alpha, spatial.angle);
      ctx.restore();
      return;
    }
    if (config.primitive === "beam_line") {
      drawBeamSignature(ctx, x, y, config, progress, alpha, width, height, spatial.angle);
      ctx.restore();
      return;
    }
    if (config.primitive === "artillery") {
      drawArtillerySignature(ctx, x, y, config, progress, alpha);
      ctx.restore();
      return;
    }
    if (config.primitive === "global_presence") {
      drawGlobalPresenceSignature(ctx, config, progress, alpha, width, height);
      ctx.restore();
      return;
    }
    if (config.primitive === "targeted_dash") {
      drawDashTrail(ctx, x, y, config, progress, alpha, spatial.angle);
      ctx.restore();
      return;
    }
    if (config.primitive === "unstoppable_charge") {
      drawDashTrail(ctx, x, y, config, progress, alpha, spatial.angle, true);
      ctx.restore();
      return;
    }
    if (config.primitive === "execute_marker") {
      drawExecuteMarker(ctx, x, y, config, progress, alpha);
      ctx.restore();
      return;
    }
    if (config.primitive === "suppression_lock") {
      drawLockOrRealm(ctx, x, y, config, progress, alpha);
      ctx.restore();
      return;
    }
    if (config.primitive === "duel_realm") {
      drawLockOrRealm(ctx, x, y, config, progress, alpha, true);
      ctx.restore();
      return;
    }
    if (config.primitive === "assassin_mark") {
      drawAssassinOrBlink(ctx, x, y, config, progress, alpha, spatial.angle);
      ctx.restore();
      return;
    }
    if (config.primitive === "blink_burst") {
      drawAssassinOrBlink(ctx, x, y, config, progress, alpha, spatial.angle, true);
      ctx.restore();
      return;
    }
    ctx.strokeStyle = hexToRgba(
      config.palette[0] ?? FALLBACK_VISUAL.palette[0],
      alpha,
    );
    ctx.fillStyle = hexToRgba(
      config.palette[1] ?? config.palette[0] ?? FALLBACK_VISUAL.palette[1],
      alpha * 0.18,
    );
    ctx.lineWidth = 2 + config.glow * 2;
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();
    drawSignatureGlyph(ctx, x, y, Math.max(14, radius * 0.48), config, alpha);
    ctx.restore();
  });
}
