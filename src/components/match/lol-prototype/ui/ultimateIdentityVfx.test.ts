import { describe, expect, it } from "vitest";
import rustUltimateCatalog from "../../../../../src-tauri/src/application/lol_sim_v2/ultimate_identity.rs?raw";

import {
  getUltimateSilhouetteKind,
  resolveUltimateVisualConfig,
  SIGNATURE_COMPOSITE_RATIONALES,
  SIGNATURE_SILHOUETTE_OVERRIDES,
} from "./ultimateIdentityVfx";

interface CatalogSignature {
  championKey: string;
  championName: string;
  primitive: string;
  signatureId: string;
}

const catalogSignatures = (): CatalogSignature[] => {
  const matches = rustUltimateCatalog.matchAll(
    /ident!\(\s*"([^"]+)",\s*"([^"]+)",\s*(\w+),\s*"([^"]+)"/g,
  );

  return Array.from(matches, ([, championKey, championName, primitive, signatureId]) => ({
    championKey,
    championName,
    primitive,
    signatureId,
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
      palette: ["#f8fafc", "#60a5fa", "#111827"],
      shapeLanguage: [signature.signatureId],
      motionHints: [signature.signatureId],
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
});
