import { describe, expect, it } from "vitest";
import rustUltimateCatalog from "../../../../../src-tauri/src/application/lol_sim_v2/ultimate_identity.rs?raw";

import {
  BESPOKE_SIGNATURE_KINDS,
  FALLBACK_SIGNATURE_ID,
  getUltimateSilhouetteKind,
  hasExplicitUltimateSignatureVisual,
  resolveUltimateRenderPhase,
  resolveUltimateRenderTiming,
  resolveUltimateSpatialRenderContext,
  resolveUltimateVisualConfig,
  SIGNATURE_COMPOSITE_RATIONALES,
  SIGNATURE_SILHOUETTE_OVERRIDES,
  type UltimateIdentityEventMetadata,
  type UltimateSpatialShape,
} from "./ultimateIdentityVfx";

interface CatalogSignature {
  championKey: string;
  championName: string;
  primitive: string;
  signatureId: string;
  palette: string[];
  shapeLanguage: string[];
  motionHints: string[];
}

const catalogSignatures = (): CatalogSignature[] => {
  const matches = rustUltimateCatalog.matchAll(
    /ident!\(\s*"([^"]+)",\s*"([^"]+)",\s*(\w+),\s*"([^"]+)",\s*\[[^\]]*\],\s*\[[^\]]*\],\s*\[([^\]]*)\],\s*\[([^\]]*)\],\s*\[([^\]]*)\]/g,
  );

  const quoted = (raw: string) => Array.from(raw.matchAll(/"([^"]+)"/g), ([, value]) => value);

  return Array.from(matches, ([, championKey, championName, primitive, signatureId, palette, shapeLanguage, motionHints]) => ({
    championKey,
    championName,
    primitive,
    signatureId,
    palette: quoted(palette),
    shapeLanguage: quoted(shapeLanguage),
    motionHints: quoted(motionHints),
  }));
};

const toSnakePrimitive = (primitive: string) =>
  primitive.replace(/[A-Z]/g, (char, index) => `${index === 0 ? "" : "_"}${char.toLowerCase()}`);

const catalogMetadata = (signature: CatalogSignature) => ({
  event: "champion_ultimate_cast",
  ultimateIdentity: {
    championKey: signature.championKey,
    championName: signature.championName,
    technicalPrimitive: toSnakePrimitive(signature.primitive),
    signatureId: signature.signatureId,
      visual: {
        visualEventId: `ultimate.${signature.signatureId}`,
        palette: signature.palette,
        shapeLanguage: signature.shapeLanguage,
        motionHints: signature.motionHints,
      },
  },
});

describe("ultimate identity VFX", () => {
  function resolveSample(
    primitive: string,
    signatureId: string,
    palette: string[],
    shapeLanguage: string[],
    motionHints: string[],
  ) {
    return resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: primitive,
        signatureId,
        visual: {
          visualEventId: `ultimate.${signatureId}`,
          palette,
          shapeLanguage,
          motionHints,
        },
      },
    });
  }

  it("falls back safely when signature metadata is missing", () => {
    const config = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
    });

    expect(config.signatureId).toBe("fallback.unknown_ultimate");
    expect(config.primitive).toBe("fallback");
    expect(config.palette.length).toBeGreaterThan(0);
  });

  it("keeps visual signature distinct even when primitive is shared", () => {
    const aatrox = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "self_aura",
        signatureId: "aatrox_world_ender",
        visual: {
          visualEventId: "ultimate.aatrox_world_ender",
          palette: ["#7f1d1d", "#ef4444"],
          shapeLanguage: ["torn_wings", "blood_runes"],
          motionHints: ["expanding_omen"],
        },
      },
    });
    const alistar = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "self_aura",
        signatureId: "alistar_unbreakable_will",
        visual: {
          visualEventId: "ultimate.alistar_unbreakable_will",
          palette: ["#9ca3af", "#f8fafc"],
          shapeLanguage: ["horns", "stone_plate"],
          motionHints: ["stomp_shock"],
        },
      },
    });

    expect(aatrox.primitive).toBe(alistar.primitive);
    expect(aatrox.signatureId).not.toBe(alistar.signatureId);
    expect(aatrox.palette).not.toEqual(alistar.palette);
    expect(aatrox.shapeLanguage).not.toEqual(alistar.shapeLanguage);
  });

  it("resolves new projectile signatures without collapsing iconic missiles", () => {
    const ashe = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "linear_projectile",
        signatureId: "ashe_enchanted_crystal_arrow",
        visual: {
          visualEventId: "ultimate.ashe_enchanted_crystal_arrow",
          palette: ["#67e8f9", "#e0f2fe", "#2563eb"],
          shapeLanguage: ["crystal_arrowhead", "frost_wings"],
          motionHints: ["long_glide", "ice_shatter_impact"],
        },
      },
    });
    const jinx = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "linear_projectile",
        signatureId: "jinx_super_mega_death_rocket",
        visual: {
          visualEventId: "ultimate.jinx_super_mega_death_rocket",
          palette: ["#fb7185", "#60a5fa", "#f97316"],
          shapeLanguage: ["shark_rocket", "graffiti_burst"],
          motionHints: ["accelerating_rocket", "chaotic_explosion"],
        },
      },
    });

    expect(ashe.primitive).toBe("linear_projectile");
    expect(jinx.primitive).toBe("linear_projectile");
    expect(ashe.signatureId).not.toBe(jinx.signatureId);
    expect(ashe.shapeLanguage).not.toEqual(jinx.shapeLanguage);
    expect(ashe.motion).not.toEqual(jinx.motion);
  });

  it("keeps beam and global visual identities distinct from projectile fallback", () => {
    const lux = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "beam_line",
        signatureId: "lux_final_spark",
        visual: {
          visualEventId: "ultimate.lux_final_spark",
          palette: ["#fef3c7", "#fb7185", "#60a5fa"],
          shapeLanguage: ["prismatic_beam", "wand_star"],
          motionHints: ["charge_glint", "instant_laser"],
        },
      },
    });
    const karthus = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "global_presence",
        signatureId: "karthus_requiem",
        visual: {
          visualEventId: "ultimate.karthus_requiem",
          palette: ["#ef4444", "#7c3aed", "#111827"],
          shapeLanguage: ["death_chorus", "red_omen"],
          motionHints: ["long_channel", "mapwide_soul_flash"],
        },
      },
    });

    expect(lux.primitive).toBe("beam_line");
    expect(karthus.primitive).toBe("global_presence");
    expect(lux.durationMs).not.toBe(karthus.durationMs);
    expect(resolveUltimateVisualConfig(null).primitive).toBe("fallback");
  });

  it("keeps execute-like dash champions visually distinct", () => {
    const akali = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "assassin_mark",
        signatureId: "akali_perfect_execution",
        visual: {
          visualEventId: "ultimate.akali_perfect_execution",
          palette: ["#22d3ee", "#a3e635", "#111827"],
          shapeLanguage: ["kunai_cross", "smoke_slash"],
          motionHints: ["two_stage_cut", "neon_afterimage"],
        },
      },
    });
    const pyke = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "execute_marker",
        signatureId: "pyke_death_from_below",
        visual: {
          visualEventId: "ultimate.pyke_death_from_below",
          palette: ["#14b8a6", "#7dd3fc", "#022c22"],
          shapeLanguage: ["drowned_x", "harpoon_wake"],
          motionHints: ["x_flash", "undertow_blink"],
        },
      },
    });
    const darius = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "execute_marker",
        signatureId: "darius_noxian_guillotine",
        visual: {
          visualEventId: "ultimate.darius_noxian_guillotine",
          palette: ["#991b1b", "#facc15", "#111827"],
          shapeLanguage: ["noxian_axe", "bleed_stacks"],
          motionHints: ["vertical_guillotine", "reset_thunder"],
        },
      },
    });

    expect(akali.primitive).toBe("assassin_mark");
    expect(pyke.primitive).toBe("execute_marker");
    expect(darius.primitive).toBe("execute_marker");
    expect(
      new Set([akali.signatureId, pyke.signatureId, darius.signatureId]).size,
    ).toBe(3);
    expect(pyke.shapeLanguage).not.toEqual(darius.shapeLanguage);
    expect(akali.motion).not.toEqual(pyke.motion);
  });

  it("resolves new mobility and realm primitives with usable visual signatures", () => {
    const malphite = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "unstoppable_charge",
        signatureId: "malphite_unstoppable_force",
        visual: {
          visualEventId: "ultimate.malphite_unstoppable_force",
          palette: ["#78716c", "#f97316", "#fde68a"],
          shapeLanguage: ["mountain_shard", "impact_crater"],
          motionHints: ["unstoppable_arc", "seismic_impact"],
        },
      },
    });
    const vi = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "targeted_dash",
        signatureId: "vi_cease_desist",
        visual: {
          visualEventId: "ultimate.vi_cease_desist",
          palette: ["#ec4899", "#60a5fa", "#1f2937"],
          shapeLanguage: ["hextech_gauntlet", "punch_lane"],
          motionHints: ["locked_pursuit", "uppercut_impact"],
        },
      },
    });
    const mordekaiser = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "duel_realm",
        signatureId: "mordekaiser_realm_death",
        visual: {
          visualEventId: "ultimate.mordekaiser_realm_death",
          palette: ["#16a34a", "#111827", "#86efac"],
          shapeLanguage: ["death_realm_gate", "iron_crown"],
          motionHints: ["realm_overlay", "duel_cage_close"],
        },
      },
    });

    expect(malphite.primitive).toBe("unstoppable_charge");
    expect(vi.primitive).toBe("targeted_dash");
    expect(mordekaiser.primitive).toBe("duel_realm");
    expect(mordekaiser.durationMs).toBeGreaterThan(malphite.durationMs);
    expect(malphite.shapeLanguage).not.toEqual(vi.shapeLanguage);
  });

  it("resolves representative batch 4 bespoke identities without generic collapse", () => {
    const bard = resolveSample(
      "global_presence",
      "bard_tempered_fate",
      ["#facc15", "#7c3aed", "#60a5fa"],
      ["cosmic_chime", "stasis_hourglass"],
      ["slow_arc", "golden_freeze"],
    );
    const azir = resolveSample(
      "zone_summon",
      "azir_emperors_divide",
      ["#d97706", "#facc15", "#fef3c7"],
      ["sand_soldier_wall", "shuriman_sun_disc"],
      ["phalanx_surge", "sand_wall_rise"],
    );
    const kindred = resolveSample(
      "ally_aura",
      "kindred_lambs_respite",
      ["#e0f2fe", "#111827", "#f8fafc"],
      ["lamb_mask", "wolf_orbit"],
      ["sanctuary_pulse", "threshold_heal"],
    );
    const zilean = resolveSample(
      "ally_aura",
      "zilean_chrono_shift",
      ["#d97706", "#facc15", "#60a5fa"],
      ["clock_runes", "rewind_hourglass"],
      ["time_anchor", "revive_rewind"],
    );

    expect(bard.primitive).toBe("global_presence");
    expect(azir.primitive).toBe("zone_summon");
    expect(kindred.primitive).toBe("ally_aura");
    expect(zilean.primitive).toBe("ally_aura");
    expect(bard.durationMs).toBeGreaterThan(azir.durationMs);
    expect(kindred.signatureId).not.toBe(zilean.signatureId);
    expect(kindred.shapeLanguage).not.toEqual(zilean.shapeLanguage);
    expect(new Set([bard.signatureId, azir.signatureId, kindred.signatureId, zilean.signatureId]).size).toBe(4);
  });

  it("maps A-H ultimates to recognizable real-game silhouettes", () => {
    const aatrox = resolveSample(
      "self_aura",
      "aatrox_world_ender",
      ["#7f1d1d", "#ef4444", "#111827"],
      ["torn_wings", "blood_runes"],
      ["expanding_omen", "heavy_pulse"],
    );
    const ashe = resolveSample(
      "linear_projectile",
      "ashe_enchanted_crystal_arrow",
      ["#67e8f9", "#e0f2fe", "#2563eb"],
      ["crystal_arrowhead", "frost_wings"],
      ["long_glide", "ice_shatter_impact"],
    );
    const azir = resolveSample(
      "zone_summon",
      "azir_emperors_divide",
      ["#d97706", "#facc15", "#fef3c7"],
      ["sand_soldier_wall", "shuriman_sun_disc"],
      ["phalanx_surge", "sand_wall_rise"],
    );
    const draven = resolveSample(
      "linear_projectile",
      "draven_whirling_death",
      ["#f97316", "#facc15", "#7f1d1d"],
      ["spinning_axes", "blood_trail"],
      ["boomerang_return", "whirling_cut"],
    );

    expect(getUltimateSilhouetteKind(aatrox)).toBe("darkin_wings");
    expect(getUltimateSilhouetteKind(ashe)).toBe("crystal_arrow");
    expect(getUltimateSilhouetteKind(azir)).toBe("soldier_wall");
    expect(getUltimateSilhouetteKind(draven)).toBe("spinning_axes");
  });

  it("maps I-R ultimates to champion-specific silhouettes, not primitive buckets", () => {
    const jinx = resolveSample(
      "linear_projectile",
      "jinx_super_mega_death_rocket",
      ["#fb7185", "#60a5fa", "#f97316"],
      ["shark_rocket", "graffiti_burst"],
      ["accelerating_rocket", "chaotic_explosion"],
    );
    const karthus = resolveSample(
      "global_presence",
      "karthus_requiem",
      ["#ef4444", "#7c3aed", "#111827"],
      ["death_chorus", "red_omen"],
      ["long_channel", "mapwide_soul_flash"],
    );
    const kindred = resolveSample(
      "ally_aura",
      "kindred_lambs_respite",
      ["#e0f2fe", "#111827", "#f8fafc"],
      ["lamb_mask", "wolf_orbit"],
      ["sanctuary_pulse", "threshold_heal"],
    );
    const mordekaiser = resolveSample(
      "duel_realm",
      "mordekaiser_realm_death",
      ["#16a34a", "#111827", "#86efac"],
      ["death_realm_gate", "iron_crown"],
      ["realm_overlay", "duel_cage_close"],
    );

    expect(getUltimateSilhouetteKind(jinx)).toBe("chaos_rocket");
    expect(getUltimateSilhouetteKind(karthus)).toBe("requiem_omen");
    expect(getUltimateSilhouetteKind(kindred)).toBe("lamb_wolf_sanctuary");
    expect(getUltimateSilhouetteKind(mordekaiser)).toBe("death_realm");
  });

  it("maps S-Z ultimates to recognizable silhouettes for final roster pass", () => {
    const twistedFate = resolveSample(
      "global_presence",
      "twistedfate_destiny",
      ["#facc15", "#60a5fa", "#111827"],
      ["destiny_cards", "eye_gate"],
      ["map_reveal", "card_gate"],
    );
    const xayah = resolveSample(
      "ally_aura",
      "xayah_featherstorm",
      ["#f472b6", "#111827", "#fb7185"],
      ["feather_fan", "blade_plumes"],
      ["untargetable_lift", "feather_recall_hint"],
    );
    const zed = resolveSample(
      "assassin_mark",
      "zed_death_mark",
      ["#111827", "#991b1b", "#ef4444"],
      ["shadow_mark", "triple_shuriken"],
      ["shadow_swap", "delayed_pop"],
    );
    const zilean = resolveSample(
      "ally_aura",
      "zilean_chrono_shift",
      ["#d97706", "#facc15", "#60a5fa"],
      ["clock_runes", "rewind_hourglass"],
      ["time_anchor", "revive_rewind"],
    );

    expect(getUltimateSilhouetteKind(twistedFate)).toBe("card_destiny");
    expect(getUltimateSilhouetteKind(xayah)).toBe("feather_storm");
    expect(getUltimateSilhouetteKind(zed)).toBe("shadow_mark");
    expect(getUltimateSilhouetteKind(zilean)).toBe("chrono_rewind");
  });

  it("has explicit frontend silhouette overrides for every Rust catalog signature", () => {
    const catalog = catalogSignatures();

    expect(catalog).toHaveLength(171);
    expect(Object.keys(SIGNATURE_SILHOUETTE_OVERRIDES)).toHaveLength(171);
    for (const signature of catalog) {
      expect(
        SIGNATURE_SILHOUETTE_OVERRIDES[signature.signatureId],
        signature.signatureId,
      ).toBeDefined();
    }
  });

  it("does not implicitly collapse catalog signatures to generic composites", () => {
    const catalog = catalogSignatures();

    for (const signature of catalog) {
      const kind = getUltimateSilhouetteKind(
        resolveUltimateVisualConfig(catalogMetadata(signature)),
      );

      if (kind === "signature_composite") {
        expect(
          SIGNATURE_COMPOSITE_RATIONALES[signature.signatureId],
          signature.signatureId,
        ).toBeTruthy();
      }
      expect(SIGNATURE_SILHOUETTE_OVERRIDES[signature.signatureId]).toBe(kind);
    }
  });

  it("Bache 5: every catalog signature resolves through explicit non-fallback visual identity", () => {
    const catalog = catalogSignatures();

    expect(catalog).toHaveLength(171);
    for (const signature of catalog) {
      const config = resolveUltimateVisualConfig(catalogMetadata(signature));
      expect(config.signatureId, signature.championName).toBe(signature.signatureId);
      expect(config.signatureId, signature.championName).not.toBe(FALLBACK_SIGNATURE_ID);
      expect(config.primitive, signature.championName).not.toBe("fallback");
      expect(hasExplicitUltimateSignatureVisual(signature.signatureId), signature.signatureId).toBe(true);
      expect(config.palette.length, signature.signatureId).toBeGreaterThanOrEqual(2);
      expect(config.shapeLanguage.length, signature.signatureId).toBeGreaterThan(0);
      expect(config.motion.length, signature.signatureId).toBeGreaterThan(0);
      expect(getUltimateSilhouetteKind(config), signature.signatureId).toBe(SIGNATURE_SILHOUETTE_OVERRIDES[signature.signatureId]);
    }
  });

  it("Bache 5: visual groups have concrete champion-specific silhouette language", () => {
    expect(SIGNATURE_SILHOUETTE_OVERRIDES).toMatchObject({
      aatrox_world_ender: "darkin_wings",
      swain_demonic_ascension: "demonic_wings",
      shyvana_dragons_descent: "dragon_descent",
      galio_heroes_entrance: "colossus_landing",
      irelia_vanguard_edge: "blade_curtain",
      maokai_natures_grasp: "nature_vines",
      sona_crescendo: "music_wave",
      bard_tempered_fate: "cosmic_stars",
      belveth_endless_banquet: "void_bloom",
      viktor_chaos_storm: "hextech_construct",
      nami_tidal_wave: "water_wave",
      lissandra_frozen_tomb: "ice_prison",
      brand_pyroclasm_bounce: "fire_explosion",
    });
  });

  it("Bache 5: critical champions preserve target, direction, timing, and spatial shape assertions", () => {
    const critical: Array<[string, UltimateSpatialShape, Partial<UltimateIdentityEventMetadata>]> = [
      ["ezreal", "projectile", { direction: { x: 1, y: 0 }, targetPos: { x: 0.82, y: 0.4 } }],
      ["ashe", "projectile", { direction: { x: 1, y: 0 }, targetPos: { x: 0.85, y: 0.35 } }],
      ["jinx", "projectile", { direction: { x: 1, y: 0 }, targetPos: { x: 0.86, y: 0.62 } }],
      ["draven", "projectile", { targetPos: { x: 0.82, y: 0.55 }, returnToOrigin: true, returnPath: [{ x: 0.2, y: 0.55 }, { x: 0.82, y: 0.55 }, { x: 0.2, y: 0.55 }] }],
      ["lux", "beam", { direction: { x: 1, y: 0 } }],
      ["caitlyn", "beam", { targetId: "red-carry", targetPos: { x: 0.75, y: 0.42 }, followTarget: true }],
      ["malphite", "projectile", { targetId: "red-carry", targetPos: { x: 0.72, y: 0.44 } }],
      ["orianna", "zone", { targetPos: { x: 0.58, y: 0.46 }, proxyOriginKind: "ball_or_target_point" }],
      ["karthus", "global", { global: true, delayMs: 2400 }],
      ["mordekaiser", "lock", { targetId: "red-top", lockedTargetId: "red-top", targetPos: { x: 0.62, y: 0.4 }, followTarget: true }],
      ["kindred", "zone", { targetPos: { x: 0.5, y: 0.5 }, persistent: true }],
      ["ryze", "global_overlay", { global: true, destinationPos: { x: 0.8, y: 0.3 } }],
      ["twistedfate", "global_overlay", { global: true, destinationPos: { x: 0.7, y: 0.25 } }],
      ["shen", "lock", { targetId: "blue-ally", targetPos: { x: 0.7, y: 0.25 }, destinationPos: { x: 0.7, y: 0.25 }, followTarget: true }],
      ["azir", "zone", { targetPos: { x: 0.55, y: 0.45 }, zoneOrientation: { x: 0, y: 1 } }],
      ["taliyah", "line", { targetPos: { x: 0.8, y: 0.5 }, direction: { x: 1, y: 0 } }],
      ["yasuo", "lock", { targetId: "red-mid", targetPos: { x: 0.66, y: 0.42 }, followTarget: true }],
      ["nocturne", "global_overlay", { global: true, targetId: "red-adc", targetPos: { x: 0.82, y: 0.7 }, destinationPos: { x: 0.82, y: 0.7 } }],
    ];

    const catalog = new Map(catalogSignatures().map((signature) => [signature.championKey, signature]));
    for (const [championKey, shape, metadata] of critical) {
      const signature = catalog.get(championKey);
      expect(signature, championKey).toBeDefined();
      const config = resolveUltimateVisualConfig(catalogMetadata(signature!));
      const timing = resolveUltimateRenderTiming({ ...catalogMetadata(signature!), ...metadata, shape }, signature!.signatureId);
      const spatial = resolveUltimateSpatialRenderContext(
        { ...catalogMetadata(signature!), ...metadata, originPos: { x: 0.2, y: 0.4 }, shape },
        { x: 0.2, y: 0.4 },
        config,
        new Map([["red-carry", { x: 0.77, y: 0.43 }], ["red-top", { x: 0.64, y: 0.41 }], ["blue-ally", { x: 0.72, y: 0.24 }], ["red-mid", { x: 0.67, y: 0.43 }], ["red-adc", { x: 0.83, y: 0.71 }]]),
      );

      expect(spatial.shape, championKey).toBe(shape);
      expect(timing.durationMs, championKey).toBeGreaterThan(0);
      if (shape === "projectile" || shape === "beam" || shape === "line" || shape === "lock") {
        expect(spatial.usedFallbackAngle, championKey).toBe(false);
      }
      if (metadata.targetId || metadata.targetPos) expect(spatial.target, championKey).toBeDefined();
      if (metadata.destinationPos) expect(spatial.destination, championKey).toEqual(metadata.destinationPos);
    }
  });

  it("covers A-H catalog signatures with explicit silhouette counts", () => {
    const batch = catalogSignatures().filter(({ championKey }) => /^[a-h]/.test(championKey));

    expect(batch).toHaveLength(45);
    expect(
      batch.filter(({ signatureId }) => SIGNATURE_SILHOUETTE_OVERRIDES[signatureId]).length,
    ).toBe(45);
  });

  it("covers I-R catalog signatures with explicit silhouette counts", () => {
    const batch = catalogSignatures().filter(({ championKey }) => /^[i-r]/.test(championKey));

    expect(batch).toHaveLength(67);
    expect(
      batch.filter(({ signatureId }) => SIGNATURE_SILHOUETTE_OVERRIDES[signatureId]).length,
    ).toBe(67);
  });

  it("covers S-Z catalog signatures with explicit silhouette counts", () => {
    const batch = catalogSignatures().filter(({ championKey }) => /^[s-z]/.test(championKey));

    expect(batch).toHaveLength(59);
    expect(
      batch.filter(({ signatureId }) => SIGNATURE_SILHOUETTE_OVERRIDES[signatureId]).length,
    ).toBe(59);
  });

  it("prioriza metadata espacial y evita actor-angle random en críticos", () => {
    const config = resolveSample(
      "linear_projectile",
      "ezreal_trueshot_barrage",
      ["#60a5fa", "#f8fafc"],
      ["arcane_wave"],
      ["map_long_sweep"],
    );
    const spatial = resolveUltimateSpatialRenderContext(
      {
        event: "champion_ultimate_cast",
        originPos: { x: 0.2, y: 0.3 },
        targetPos: { x: 0.8, y: 0.9 },
        shape: "projectile",
      },
      { x: 0.1, y: 0.1 },
      config,
    );

    expect(spatial.usedFallbackAngle).toBe(false);
    expect(spatial.angle).toBeCloseTo(Math.atan2(0.6, 0.6), 5);
  });

  it("clasifica shapes A-H, I-R y S-Z con uso coherente de target/dirección", () => {
    const asheShape = resolveUltimateSpatialRenderContext(
      { event: "champion_ultimate_cast", shape: "projectile", direction: { x: 1, y: 0 } },
      { x: 0.3, y: 0.5 },
      resolveSample("linear_projectile", "ashe_enchanted_crystal_arrow", ["#fff", "#00f"], ["arrow"], ["long_glide"]),
    );
    const oriannaShape = resolveUltimateSpatialRenderContext(
      { event: "champion_ultimate_cast", shape: "zone", targetPos: { x: 0.4, y: 0.4 } },
      { x: 0.2, y: 0.2 },
      resolveSample("aoe_pulse", "orianna_command_shockwave", ["#fff", "#00f"], ["ring"], ["pull"]),
    );
    const karthusShape = resolveUltimateSpatialRenderContext(
      { event: "champion_ultimate_cast", shape: "global", global: true },
      { x: 0.5, y: 0.5 },
      resolveSample("global_presence", "karthus_requiem", ["#fff", "#00f"], ["omen"], ["channel"]),
    );

    expect(asheShape.shape).toBe("projectile");
    expect(asheShape.usedFallbackAngle).toBe(false);
    expect(oriannaShape.shape).toBe("zone");
    expect(oriannaShape.target).toEqual({ x: 0.4, y: 0.4 });
    expect(karthusShape.shape).toBe("global");
  });

  it("mantiene fallback seguro cuando metadata espacial llega incompleta", () => {
    const config = resolveSample("beam_line", "lux_final_spark", ["#fff", "#00f"], ["beam"], ["instant_laser"]);
    const spatial = resolveUltimateSpatialRenderContext(
      { event: "champion_ultimate_cast", originPos: { x: 0.5, y: 0.5 }, shape: "line" },
      { x: 0.2, y: 0.2 },
      config,
    );

    expect(spatial.origin).toEqual({ x: 0.5, y: 0.5 });
    expect(spatial.usedFallbackAngle).toBe(true);
    expect(spatial.shape).toBe("line");
  });

  it("usa direction explícita para projectile/beam/line antes que signatureAngle", () => {
    const origin = { x: 0.2, y: 0.2 };
    const config = resolveSample("beam_line", "lux_final_spark", ["#fff", "#00f"], ["beam"], ["instant_laser"]);

    for (const shape of ["projectile", "beam", "line"] as const) {
      const spatial = resolveUltimateSpatialRenderContext(
        { event: "champion_ultimate_cast", originPos: origin, direction: { x: 0, y: 3 }, shape },
        { x: 0.9, y: 0.9 },
        config,
      );

      expect(spatial.shape).toBe(shape);
      expect(spatial.usedFallbackAngle).toBe(false);
      expect(spatial.direction).toEqual({ x: 0, y: 1 });
      expect(spatial.angle).toBeCloseTo(Math.PI / 2, 5);
    }
  });

  it("circle/zone usan targetPos como anchor espacial cuando viene en metadata", () => {
    const config = resolveSample("aoe_pulse", "orianna_command_shockwave", ["#fff", "#00f"], ["ring"], ["pull"]);
    const spatial = resolveUltimateSpatialRenderContext(
      {
        event: "champion_ultimate_cast",
        originPos: { x: 0.1, y: 0.1 },
        targetPos: { x: 0.65, y: 0.45 },
        shape: "circle",
      },
      { x: 0.2, y: 0.2 },
      config,
    );

    expect(spatial.origin).toEqual({ x: 0.1, y: 0.1 });
    expect(spatial.target).toEqual({ x: 0.65, y: 0.45 });
    expect(spatial.shape).toBe("circle");
    expect(spatial.usedFallbackAngle).toBe(false);
  });

  it("global overlay es una shape explícita y no necesita orientación del actor", () => {
    const config = resolveSample("global_presence", "karthus_requiem", ["#fff", "#00f"], ["omen"], ["channel"]);
    const spatial = resolveUltimateSpatialRenderContext(
      { event: "champion_ultimate_cast", originPos: { x: 0.05, y: 0.95 }, shape: "global_overlay", global: true },
      { x: 0.8, y: 0.1 },
      config,
    );

    expect(spatial.shape).toBe("global_overlay");
    expect(spatial.origin).toEqual({ x: 0.05, y: 0.95 });
    expect(spatial.target).toBeUndefined();
    expect(spatial.usedFallbackAngle).toBe(true);
  });

  it("delayed ground AoE expone telegraph antes del impacto", () => {
    const timing = resolveUltimateRenderTiming({
      event: "champion_ultimate_cast",
      shape: "circle",
      ultimateIdentity: {
        technicalPrimitive: "artillery",
        signatureId: "leona_solar_flare",
        gameplayTags: ["delayed_impact", "stun"],
        visual: { palette: ["#facc15"], shapeLanguage: [], motionHints: [] },
      },
    });

    expect(timing.delayMs).toBeGreaterThan(0);
    expect(timing.impactAt).toBe(timing.delayMs);
    expect(resolveUltimateRenderPhase(timing.impactAt - 1, timing)).toBe("windup");
    expect(resolveUltimateRenderPhase(timing.impactAt, timing)).toBe("impact");
  });

  it("persistent zones remain visible across duration with pulses", () => {
    const config = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      shape: "zone",
      ultimateIdentity: {
        technicalPrimitive: "aoe_pulse",
        signatureId: "anivia_glacial_storm",
        gameplayTags: ["zone", "slow"],
        visual: {
          visualEventId: "ultimate.anivia_glacial_storm",
          palette: ["#7dd3fc", "#e0f2fe"],
          shapeLanguage: ["snow_spiral"],
          motionHints: ["persistent_swirl"],
        },
      },
    });

    expect(config.persistent).toBe(true);
    expect(config.durationMs).toBeGreaterThan(3000);
    expect(config.pulseCount).toBeGreaterThan(3);
  });

  it("Bache 1 real ultimates resolve non-fallback VFX with honest timing and shape", () => {
    const batch = [
      ["aatrox", "aura", "darkin_wings", true],
      ["ahri", "lock", "spirit_dash", false],
      ["akali", "lock", "two_stage_execution", false],
      ["akshan", "lock", "precision_snipe", false],
      ["alistar", "aura", "beast_charge", true],
      ["ambessa", "projectile", "gauntlet_lockon", false],
      ["amumu", "circle", "shockwave_ring", false],
      ["anivia", "zone", "ice_prison", true],
      ["annie", "circle", "tibbers_bear", true],
      ["aphelios", "projectile", "moonlight_vigil", false],
    ] as const;
    const catalog = new Map(catalogSignatures().map((signature) => [signature.championKey, signature]));

    for (const [championKey, shape, silhouette, persistent] of batch) {
      const signature = catalog.get(championKey)!;
      const config = resolveUltimateVisualConfig({
        ...catalogMetadata(signature),
        shape,
        persistent,
        targetPos: { x: 0.7, y: 0.4 },
        direction: { x: 1, y: 0 },
      });
      const spatial = resolveUltimateSpatialRenderContext(
        { ...catalogMetadata(signature), shape, persistent, targetPos: { x: 0.7, y: 0.4 }, direction: { x: 1, y: 0 } },
        { x: 0.2, y: 0.4 },
        config,
      );

      expect(config.signatureId, championKey).not.toBe(FALLBACK_SIGNATURE_ID);
      expect(hasExplicitUltimateSignatureVisual(signature.signatureId), championKey).toBe(true);
      expect(getUltimateSilhouetteKind(config), championKey).toBe(silhouette);
      expect(spatial.shape, championKey).toBe(shape);
      expect(config.durationMs, championKey).toBeGreaterThan(0);
      if (persistent) expect(config.persistent, championKey).toBe(true);
    }
  });

  it("channel beam/cone uses duration and pulse stream instead of a single flash", () => {
    const lucian = resolveUltimateVisualConfig({
      event: "champion_ultimate_cast",
      ultimateIdentity: {
        technicalPrimitive: "beam_line",
        signatureId: "lucian_the_culling",
        gameplayTags: ["channeled_beam", "bullets"],
        visual: {
          visualEventId: "ultimate.lucian_the_culling",
          palette: ["#fde68a", "#facc15"],
          shapeLanguage: ["dual_pistols", "bullet_lanes"],
          motionHints: ["rapid_barrage"],
        },
      },
    });

    expect(lucian.durationMs).toBeGreaterThan(2500);
    expect(lucian.pulseCount).toBeGreaterThan(3);
  });

  it("Karthus/Taric/Kayle/Zyra delayed events expose timing metadata defaults", () => {
    for (const signatureId of [
      "karthus_requiem",
      "taric_cosmic_radiance",
      "kayle_divine_judgment",
      "zyra_stranglethorns",
    ]) {
      const timing = resolveUltimateRenderTiming({
        event: "champion_ultimate_cast",
        ultimateIdentity: {
          signatureId,
          gameplayTags: ["delayed_impact"],
          visual: { palette: ["#fff"], shapeLanguage: [], motionHints: [] },
        },
      });

      expect(timing.delayMs).toBeGreaterThan(0);
      expect(timing.durationMs).toBeGreaterThan(0);
      expect(timing.totalMs).toBeGreaterThan(timing.delayMs);
    }
  });

  it("fallback timing remains backward-compatible without timing metadata", () => {
    const config = resolveUltimateVisualConfig({ event: "champion_ultimate_cast" });
    const timing = resolveUltimateRenderTiming({ event: "champion_ultimate_cast" });

    expect(config.primitive).toBe("fallback");
    expect(timing.delayMs).toBe(0);
    expect(timing.persistent).toBe(false);
    expect(timing.pulseCount).toBe(1);
  });

  it("lock-on usa targetId/targetPos y sigue el target vivo cuando las unidades se mueven", () => {
    const config = resolveSample("suppression_lock", "malzahar_nether_grasp", ["#7c3aed", "#111827"], ["void_lock"], ["suppress"]);
    const movedUnits = new Map([["red-mid", { x: 0.72, y: 0.41 }]]);
    const spatial = resolveUltimateSpatialRenderContext(
      {
        event: "champion_ultimate_cast",
        originPos: { x: 0.25, y: 0.4 },
        targetId: "red-mid",
        lockedTargetId: "red-mid",
        targetPos: { x: 0.5, y: 0.5 },
        followTarget: true,
        shape: "lock",
      },
      { x: 0.25, y: 0.4 },
      config,
      movedUnits,
    );

    expect(spatial.target).toEqual({ x: 0.72, y: 0.41 });
    expect(spatial.lockedTarget).toEqual({ x: 0.72, y: 0.41 });
    expect(spatial.usedFallbackAngle).toBe(false);
  });

  it("tether expone línea actor-target cuando llega tetherKind", () => {
    const config = resolveSample("suppression_lock", "morgana_soul_shackles", ["#7f1d1d", "#a855f7"], ["soul_chain"], ["tether"]);
    const spatial = resolveUltimateSpatialRenderContext(
      {
        event: "champion_ultimate_cast",
        originPos: { x: 0.3, y: 0.3 },
        targetId: "red-jgl",
        targetPos: { x: 0.6, y: 0.5 },
        tetherKind: "soul_chain",
        followTarget: true,
        shape: "lock",
      },
      { x: 0.3, y: 0.3 },
      config,
      new Map([["red-jgl", { x: 0.62, y: 0.52 }]]),
    );

    expect(spatial.tetherKind).toBe("soul_chain");
    expect(spatial.target).toEqual({ x: 0.62, y: 0.52 });
  });

  it("bounce chain usa múltiples posiciones de target reales", () => {
    const config = resolveSample("linear_projectile", "brand_pyroclasm_bounce", ["#fb923c", "#facc15"], ["bouncing_fire"], ["ricochet"]);
    const spatial = resolveUltimateSpatialRenderContext(
      {
        event: "champion_ultimate_cast",
        targetIds: ["red-top", "red-mid", "red-bot"],
        bounceCount: 3,
        sequenceKind: "chain",
      },
      { x: 0.2, y: 0.2 },
      config,
      new Map([
        ["red-top", { x: 0.4, y: 0.2 }],
        ["red-mid", { x: 0.55, y: 0.45 }],
        ["red-bot", { x: 0.7, y: 0.7 }],
      ]),
    );

    expect(spatial.bouncePoints).toEqual([{ x: 0.4, y: 0.2 }, { x: 0.55, y: 0.45 }, { x: 0.7, y: 0.7 }]);
  });

  it("return path diferencia ida/vuelta en vez de proyectil random one-way", () => {
    const config = resolveSample("linear_projectile", "draven_whirling_death", ["#facc15", "#ef4444"], ["spinning_axes"], ["returning_axes"]);
    const spatial = resolveUltimateSpatialRenderContext(
      {
        event: "champion_ultimate_cast",
        originPos: { x: 0.2, y: 0.5 },
        targetPos: { x: 0.8, y: 0.5 },
        returnToOrigin: true,
        returnPath: [{ x: 0.2, y: 0.5 }, { x: 0.8, y: 0.5 }, { x: 0.2, y: 0.5 }],
        shape: "projectile",
      },
      { x: 0.2, y: 0.5 },
      config,
    );

    expect(spatial.returnPathPoints).toEqual([{ x: 0.2, y: 0.5 }, { x: 0.8, y: 0.5 }, { x: 0.2, y: 0.5 }]);
  });

  it("multi-stage expone stage metadata y el renderer puede diferenciar la etapa", () => {
    const config = resolveSample("artillery", "jhin_curtain_call", ["#f8fafc", "#dc2626"], ["curtain_scope"], ["fourth_shot"]);
    const spatial = resolveUltimateSpatialRenderContext(
      {
        event: "champion_ultimate_cast",
        targetPos: { x: 0.8, y: 0.35 },
        stage: 3,
        stageCount: 4,
        sequenceKind: "multi_shot_channel",
        recastWindowMs: 9000,
      },
      { x: 0.2, y: 0.35 },
      config,
    );

    expect(spatial.stage).toBe(3);
    expect(spatial.stageCount).toBe(4);
    expect(spatial.sequenceKind).toBe("multi_shot_channel");
    expect(spatial.recastWindowMs).toBe(9000);
  });

  it("Bache 4 asigna bespokeKind explícito a cada priority signature", () => {
    expect(BESPOKE_SIGNATURE_KINDS).toMatchObject({
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
    });
  });

  it("Orianna proxy shockwave usa ball/target point como origen renderizable, no actor origin", () => {
    const config = resolveSample("aoe_pulse", "orianna_command_shockwave", ["#fff", "#00f"], ["shockwave_ring"], ["pull"]);
    const spatial = resolveUltimateSpatialRenderContext(
      {
        event: "champion_ultimate_cast",
        originPos: { x: 0.1, y: 0.1 },
        targetPos: { x: 0.62, y: 0.44 },
        proxyOriginKind: "ball_or_target_point",
        shape: "zone",
      },
      { x: 0.2, y: 0.2 },
      config,
    );

    expect(spatial.bespokeKind).toBe("proxy_shockwave");
    expect(spatial.proxyOriginKind).toBe("ball_or_target_point");
    expect(spatial.origin).toEqual({ x: 0.62, y: 0.44 });
  });

  it("Ryze/TF/Shen preservan destination/target semantics", () => {
    for (const [signatureId, bespokeKind] of [
      ["ryze_realm_warp", "portal"],
      ["twistedfate_destiny", "global_reveal_gate"],
      ["shen_stand_united", "ally_shield_arrival"],
    ] as const) {
      const config = resolveSample("global_presence", signatureId, ["#fff", "#00f"], [signatureId], [signatureId]);
      const spatial = resolveUltimateSpatialRenderContext(
        {
          event: "champion_ultimate_cast",
          originPos: { x: 0.3, y: 0.35 },
          targetId: "blue-top",
          targetPos: { x: 0.74, y: 0.22 },
          destinationPos: { x: 0.8, y: 0.25 },
          global: true,
        },
        { x: 0.1, y: 0.1 },
        config,
      );

      expect(spatial.bespokeKind).toBe(bespokeKind);
      expect(spatial.destination).toEqual({ x: 0.8, y: 0.25 });
      expect(spatial.target).toEqual({ x: 0.74, y: 0.22 });
    }
  });

  it("Mordekaiser/Kindred quedan centrados en target/zone y Nocturne combina overlay global con dash target", () => {
    const morde = resolveUltimateSpatialRenderContext(
      {
        event: "champion_ultimate_cast",
        targetId: "red-top",
        lockedTargetId: "red-top",
        targetPos: { x: 0.7, y: 0.4 },
        followTarget: true,
        shape: "lock",
      },
      { x: 0.3, y: 0.4 },
      resolveSample("duel_realm", "mordekaiser_realm_death", ["#fff", "#000"], ["death_realm"], ["realm"]),
      new Map([["red-top", { x: 0.72, y: 0.42 }]]),
    );
    const kindred = resolveUltimateSpatialRenderContext(
      {
        event: "champion_ultimate_cast",
        targetPos: { x: 0.5, y: 0.5 },
        shape: "zone",
      },
      { x: 0.2, y: 0.2 },
      resolveSample("ally_aura", "kindred_lambs_respite", ["#fff", "#000"], ["lamb_mask"], ["heal"]),
    );
    const nocturne = resolveUltimateSpatialRenderContext(
      {
        event: "champion_ultimate_cast",
        originPos: { x: 0.2, y: 0.2 },
        targetId: "red-adc",
        targetPos: { x: 0.8, y: 0.7 },
        destinationPos: { x: 0.8, y: 0.7 },
        global: true,
        shape: "global_overlay",
      },
      { x: 0.2, y: 0.2 },
      resolveSample("global_presence", "nocturne_paranoia", ["#000", "#900"], ["blackout"], ["lights_out"]),
    );

    expect(morde.bespokeKind).toBe("death_realm");
    expect(morde.target).toEqual({ x: 0.72, y: 0.42 });
    expect(kindred.bespokeKind).toBe("sanctuary_heal");
    expect(kindred.target).toEqual({ x: 0.5, y: 0.5 });
    expect(nocturne.bespokeKind).toBe("blackout_dash");
    expect(nocturne.destination).toEqual({ x: 0.8, y: 0.7 });
    expect(nocturne.shape).toBe("global_overlay");
  });
});
