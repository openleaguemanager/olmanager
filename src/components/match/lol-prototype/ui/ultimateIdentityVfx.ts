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
}

export interface UltimateVisualConfig {
  signatureId: string;
  primitive: UltimatePrimitive | "fallback";
  palette: string[];
  shapeLanguage: string[];
  motion: string[];
  glow: number;
  durationMs: number;
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
  | "shockwave_ring"
  | "mask_orbit"
  | "signature_composite";

const FALLBACK_VISUAL: UltimateVisualConfig = {
  signatureId: "fallback.unknown_ultimate",
  primitive: "fallback",
  palette: ["#e5e7eb", "#94a3b8", "#38bdf8"],
  shapeLanguage: ["soft_ring", "neutral_spark"],
  motion: ["single_pulse", "fade"],
  glow: 0.45,
  durationMs: 1500,
};

const SIGNATURE_VISUAL_OVERRIDES: Record<
  string,
  Partial<UltimateVisualConfig>
> = {
  aatrox_world_ender: { glow: 0.72, durationMs: 1900 },
  alistar_unbreakable_will: { glow: 0.55, durationMs: 1700 },
  amumu_curse_sad_mummy: { glow: 0.75, durationMs: 1400 },
  anivia_glacial_storm: { glow: 0.5, durationMs: 2200 },
  annie_summon_tibbers: { glow: 0.86, durationMs: 1800 },
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
  ahri_spirit_rush: { glow: 0.78, durationMs: 1600 },
  akali_perfect_execution: { glow: 0.86, durationMs: 1350 },
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
  jayce_transform_mercury: "gauntlet_lockon",
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
  anivia_glacial_storm: "storm_field",
  aurora_between_worlds: "stasis_hourglass",
  belveth_endless_banquet: "void_ray",
  blitzcrank_static_field: "static_field",
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
  swain_demonic_ascension: "darkin_wings",
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
  heimerdinger_apex_turret: "turret_construct",
  illaoi_leap_faith: "tentacle_slam",
  ivern_summon_daisy: "turret_construct",
  shaco_hallucinate: "shadow_mark",
  teemo_noxious_trap: "trap_mushroom",
  viktor_chaos_storm: "storm_field",
  yorick_eulogy_isles: "turret_construct",
  akshan_comeuppance_lockon: "precision_snipe",
  aphelios_moonlight_vigil: "moonlight_vigil",
  ashe_enchanted_crystal_arrow: "crystal_arrow",
  brand_pyroclasm_bounce: "bouncing_fire",
  braum_glacial_fissure: "glacial_fissure",
  corki_missile_barrage: "chaos_rocket",
  draven_whirling_death: "spinning_axes",
  ezreal_trueshot_barrage: "arcane_barrage",
  irelia_vanguard_edge: "blade_execution",
  jhin_curtain_call: "precision_snipe",
  jinx_super_mega_death_rocket: "chaos_rocket",
  kled_chaaaaaaaarge: "beast_charge",
  kogmaw_living_artillery: "meteor_fall",
  maokai_natures_grasp: "thorn_roots",
  nami_tidal_wave: "storm_field",
  nautilus_depth_charge: "glacial_fissure",
  ornn_call_forge_god: "beast_charge",
  poppy_keepers_verdict: "gauntlet_lockon",
  qiyana_supreme_display_talent: "mask_orbit",
  renata_hostile_takeover: "bullet_cone",
  sejuani_glacial_prison: "crystal_arrow",
  seraphine_encore: "petal_charm",
  smolder_mmmmom: "darkin_wings",
  sona_crescendo: "prismatic_laser",
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
  malzahar_nether_grasp: "void_ray",
  missfortune_bullet_time: "bullet_cone",
  rumble_equalizer: "equalizer_line",
  senna_dawning_shadow: "prismatic_laser",
  taliyah_weavers_wall: "glacial_fissure",
  velkoz_life_form_disintegration_ray: "void_ray",
  gangplank_cannon_barrage: "cannon_barrage",
  karthus_requiem: "requiem_omen",
  lillia_lilting_lullaby: "petal_charm",
  nocturne_paranoia: "global_darkness",
  pantheon_grand_starfall: "meteor_fall",
  ryze_realm_warp: "card_destiny",
  shen_stand_united: "protective_shield",
  soraka_wish: "meteor_fall",
  twistedfate_destiny: "card_destiny",
  xerath_rite_arcane: "arcane_barrage",
  ahri_spirit_rush: "spirit_dash",
  akali_perfect_execution: "two_stage_execution",
  ambessa_public_execution: "gauntlet_lockon",
  briar_certain_death: "beast_charge",
  camille_hextech_ultimatum: "hextech_cage",
  chogath_feast: "beast_charge",
  darius_noxian_guillotine: "guillotine_axe",
  evelynn_last_caress: "petal_charm",
  fiddlesticks_crowstorm: "crowstorm",
  jarvaniv_cataclysm: "mountain_impact",
  kaisa_killer_instinct: "void_ray",
  kassadin_riftwalk: "void_ray",
  kayn_umbral_trespass: "shadow_mark",
  khazix_void_assault: "beast_charge",
  leesin_dragons_rage: "beast_charge",
  lissandra_frozen_tomb: "stasis_hourglass",
  malphite_unstoppable_force: "mountain_impact",
  mordekaiser_realm_death: "death_realm",
  pyke_death_from_below: "drowned_x",
  reksai_void_rush: "beast_charge",
  sett_show_stopper: "mountain_impact",
  shyvana_dragons_descent: "darkin_wings",
  sion_unstoppable_onslaught: "beast_charge",
  sylas_hijack: "signature_composite",
  trundle_subjugate: "hextech_cage",
  veigar_primordial_burst: "meteor_fall",
  vi_cease_desist: "gauntlet_lockon",
  viego_heartbreaker: "shadow_mark",
  yasuo_last_breath: "wind_fate_slash",
  zed_death_mark: "shadow_mark",
  aurelionsol_falling_star: "meteor_fall",
  azir_emperors_divide: "soldier_wall",
  bard_tempered_fate: "stasis_hourglass",
  ekko_chronobreak: "chrono_rewind",
  fizz_chum_waters: "beast_charge",
  galio_heroes_entrance: "mountain_impact",
  hecarim_onslaught_shadows: "beast_charge",
  kalista_fates_call: "crystal_arrow",
  karma_mantra: "mask_orbit",
  kindred_lambs_respite: "lamb_wolf_sanctuary",
  ksante_all_out: "death_realm",
  leblanc_mimic: "shadow_mark",
  leona_solar_flare: "meteor_fall",
  lulu_wild_growth: "petal_charm",
  nidalee_aspect_cougar: "beast_charge",
  nunu_absolute_zero: "storm_field",
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
  return {
    signatureId,
    primitive:
      (identity.technicalPrimitive as UltimatePrimitive | undefined) ??
      "fallback",
    palette: [...visual.palette],
    shapeLanguage: [...(visual.shapeLanguage ?? [])],
    motion: [...(visual.motionHints ?? [])],
    glow: override.glow ?? 0.62,
    durationMs: override.durationMs ?? 1700,
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
): boolean {
  const kind = getUltimateSilhouetteKind(config);
  const primary = config.palette[0] ?? FALLBACK_VISUAL.palette[0];
  const secondary = config.palette[1] ?? primary;
  const accent = config.palette[2] ?? secondary;
  const angle = signatureAngle(config.signatureId);
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
    case "two_stage_execution":
    case "mountain_impact":
    case "gauntlet_lockon":
    case "shadow_mark":
    case "darkin_wings":
    case "spirit_dash":
    case "beast_charge": {
      return false;
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
  charge = false,
) {
  const angle = signatureAngle(config.signatureId);
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
  blink = false,
) {
  const angle = signatureAngle(config.signatureId);
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
  drawDashTrail(ctx, x, y, config, progress, alpha * 0.9);
}

function signatureAngle(signatureId: string): number {
  const seed = Array.from(signatureId).reduce(
    (acc, char) => acc + char.charCodeAt(0),
    0,
  );
  return (seed % 360) * (Math.PI / 180);
}

function drawLinearProjectileSignature(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  config: UltimateVisualConfig,
  progress: number,
  alpha: number,
) {
  const angle = signatureAngle(config.signatureId);
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
) {
  const angle = config.motion.some(
    (motion) => motion.includes("heaven") || motion.includes("sword_rain"),
  )
    ? -Math.PI / 2
    : signatureAngle(config.signatureId);
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
    const elapsedMs = nowMs - event.t * 1000;
    if (elapsedMs < 0 || elapsedMs > config.durationMs) return;

    const pos = positions.get(event.metadata.actorId ?? "");
    if (!pos) return;

    const progress = Math.max(0, Math.min(1, elapsedMs / config.durationMs));
    const alpha = (1 - progress) * (0.45 + config.glow * 0.45);
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
    const x = pos.x * width;
    const y = pos.y * height;

    ctx.save();
    ctx.globalCompositeOperation = "lighter";
    ctx.shadowColor = config.palette[0] ?? FALLBACK_VISUAL.palette[0];
    ctx.shadowBlur = 12 + config.glow * 18;

    if (drawChampionSilhouette(ctx, x, y, config, progress, alpha, width, height)) {
      ctx.restore();
      return;
    }

    if (config.primitive === "linear_projectile") {
      drawLinearProjectileSignature(ctx, x, y, config, progress, alpha);
      ctx.restore();
      return;
    }
    if (config.primitive === "beam_line") {
      drawBeamSignature(ctx, x, y, config, progress, alpha, width, height);
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
      drawDashTrail(ctx, x, y, config, progress, alpha);
      ctx.restore();
      return;
    }
    if (config.primitive === "unstoppable_charge") {
      drawDashTrail(ctx, x, y, config, progress, alpha, true);
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
      drawAssassinOrBlink(ctx, x, y, config, progress, alpha);
      ctx.restore();
      return;
    }
    if (config.primitive === "blink_burst") {
      drawAssassinOrBlink(ctx, x, y, config, progress, alpha, true);
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
