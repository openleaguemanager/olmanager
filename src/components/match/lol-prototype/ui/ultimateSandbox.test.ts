import { describe, expect, it } from "vitest";

import {
  createUltimateSandboxEvent,
  ULTIMATE_SANDBOX_CASTER_ID,
  ULTIMATE_SANDBOX_CASTER_POS,
  ULTIMATE_SANDBOX_DUMMY_ID,
  ULTIMATE_SANDBOX_DUMMY_POS,
  ULTIMATE_SANDBOX_IDENTITIES,
} from "./ultimateSandbox";
import type { UltimateIdentityEventMetadata } from "./ultimateIdentityVfx";

describe("ultimate sandbox", () => {
  it("exposes the full ultimate identity catalog for visual testing", () => {
    expect(ULTIMATE_SANDBOX_IDENTITIES).toHaveLength(171);
    expect(ULTIMATE_SANDBOX_IDENTITIES[0]).toEqual(
      expect.objectContaining({
        championName: expect.any(String),
        primitive: expect.any(String),
        signatureId: expect.any(String),
      }),
    );
  });

  it("creates renderer-compatible cast metadata with origin, target and normalized direction", () => {
    const identity = ULTIMATE_SANDBOX_IDENTITIES.find((item) => item.championKey === "ashe") ?? ULTIMATE_SANDBOX_IDENTITIES[0];
    const event = createUltimateSandboxEvent(identity, 12.5);
    const metadata = event.metadata as UltimateIdentityEventMetadata;

    expect(event.t).toBe(12.5);
    expect(metadata.event).toBe("champion_ultimate_cast");
    expect(metadata.actorId).toBe(ULTIMATE_SANDBOX_CASTER_ID);
    expect(metadata.targetId).toBe(ULTIMATE_SANDBOX_DUMMY_ID);
    expect(metadata.originPos).toEqual(ULTIMATE_SANDBOX_CASTER_POS);
    expect(metadata.targetPos).toEqual(ULTIMATE_SANDBOX_DUMMY_POS);
    expect(metadata.ultimateIdentity?.signatureId).toBe(
      identity.signatureId,
    );
    expect(Math.hypot(metadata.direction?.x ?? 0, metadata.direction?.y ?? 0)).toBeCloseTo(1);
    expect(metadata.shape).toBeTruthy();
  });

  it("forces Bache 1 champions through real metadata routes instead of generic burst", () => {
    const expected: Record<string, string> = {
      aatrox: "darkin_self_buff",
      ahri: "triple_spirit_dash",
      akali: "two_stage_neon_execution",
      akshan: "comeuppance_lock_on_shots",
      alistar: "cleanse_damage_reduction",
      ambessa: "noxian_execution_dash",
      amumu: "aoe_bandage_lockdown",
      anivia: "persistent_slow_damage_storm",
      annie: "tibbers_drop_burst_pet",
      aphelios: "moonlight_bloom_weapon_pending",
    };

    for (const identity of ULTIMATE_SANDBOX_IDENTITIES.filter((item) => item.championKey in expected)) {
      const event = createUltimateSandboxEvent(identity, 1);
      const metadata = event.metadata as UltimateIdentityEventMetadata;

      expect(metadata.bespokeKind, identity.championKey).toBe(expected[identity.championKey]);
      expect(metadata.shape, identity.championKey).toBeTruthy();
      expect(metadata.ultimateIdentity?.signatureId, identity.championKey).toBe(identity.signatureId);
      if (["amumu", "anivia", "aphelios"].includes(identity.championKey)) {
        expect(metadata.affectedTargetIds?.length, identity.championKey).toBeGreaterThan(1);
      }
      if (["ahri", "akali", "ambessa"].includes(identity.championKey)) {
        expect(metadata.destinationPos, identity.championKey).toBeDefined();
      }
      if (identity.championKey === "akshan") {
        expect(metadata.lockedTargetId).toBe(ULTIMATE_SANDBOX_DUMMY_ID);
        expect(metadata.sequenceKind).toBe("lock_on_multi_shot_channel");
      }
    }
  });
});
