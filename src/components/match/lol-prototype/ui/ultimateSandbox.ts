import rustUltimateCatalog from "../../../../../src-tauri/src/application/lol_sim_v2/ultimate_identity.rs?raw";

import type { SimEvent, Vec2 } from "../engine/types";
import type {
  UltimateIdentityEventMetadata,
  UltimatePrimitive,
  UltimateSpatialShape,
} from "./ultimateIdentityVfx";

export interface UltimateSandboxIdentity {
  championKey: string;
  championName: string;
  primitive: UltimatePrimitive;
  signatureId: string;
  palette: string[];
  shapeLanguage: string[];
  motionHints: string[];
}

export const ULTIMATE_SANDBOX_CASTER_ID = "ultimate-sandbox-caster";
export const ULTIMATE_SANDBOX_DUMMY_ID = "ultimate-sandbox-dummy";
export const ULTIMATE_SANDBOX_SECOND_DUMMY_ID = "ultimate-sandbox-dummy-2";
export const ULTIMATE_SANDBOX_CASTER_POS: Vec2 = { x: 0.22, y: 0.72 };
export const ULTIMATE_SANDBOX_DUMMY_POS: Vec2 = { x: 0.75, y: 0.35 };

const toSnakeCase = (value: string): string =>
  value.replace(/[A-Z]/g, (char, index) => `${index === 0 ? "" : "_"}${char.toLowerCase()}`);

const quotedValues = (raw: string): string[] =>
  Array.from(raw.matchAll(/"([^"]+)"/g), ([, value]) => value);

const primitiveToShape = (primitive: UltimatePrimitive): UltimateSpatialShape => {
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

const normalizeDirection = (origin: Vec2, target: Vec2): Vec2 => {
  const dx = target.x - origin.x;
  const dy = target.y - origin.y;
  const length = Math.hypot(dx, dy) || 1;
  return { x: dx / length, y: dy / length };
};

const batchOneRealRouteMetadata = (
  identity: UltimateSandboxIdentity,
  originPos: Vec2,
  targetPos: Vec2,
): Partial<UltimateIdentityEventMetadata> => {
  const direction = normalizeDirection(originPos, targetPos);
  switch (identity.championKey) {
    case "aatrox":
      return { shape: "aura", targetPos: originPos, radius: 0.12, durationMs: 12000, persistent: true, pulseCount: 8, bespokeKind: "darkin_self_buff" };
    case "ahri":
      return { shape: "lock", destinationPos: { x: originPos.x + direction.x * 0.075, y: originPos.y + direction.y * 0.075 }, stage: 1, stageCount: 3, sequenceKind: "recast_dash_charges", recastWindowMs: 10000, bespokeKind: "triple_spirit_dash" };
    case "akali":
      return { shape: "lock", lockedTargetId: ULTIMATE_SANDBOX_DUMMY_ID, destinationPos: targetPos, stage: 2, stageCount: 2, sequenceKind: "execute_recast_dash", recastWindowMs: 2500, requiresCondition: "stage_2_or_low_hp_execute", bespokeKind: "two_stage_neon_execution" };
    case "akshan":
      return { shape: "lock", lockedTargetId: ULTIMATE_SANDBOX_DUMMY_ID, followTarget: true, durationMs: 2400, pulseCount: 5, stageCount: 5, sequenceKind: "lock_on_multi_shot_channel", tetherKind: "target_reticle_channel", bespokeKind: "comeuppance_lock_on_shots" };
    case "alistar":
      return { shape: "aura", targetPos: originPos, radius: 0.11, durationMs: 7000, persistent: true, pulseCount: 5, bespokeKind: "cleanse_damage_reduction" };
    case "ambessa":
      return { shape: "projectile", lockedTargetId: ULTIMATE_SANDBOX_DUMMY_ID, destinationPos: targetPos, followTarget: true, tetherKind: "brief_suppression", bespokeKind: "noxian_execution_dash" };
    case "amumu":
      return { shape: "circle", targetPos: originPos, radius: 0.14, targetIds: [ULTIMATE_SANDBOX_DUMMY_ID, ULTIMATE_SANDBOX_SECOND_DUMMY_ID], affectedTargetIds: [ULTIMATE_SANDBOX_DUMMY_ID, ULTIMATE_SANDBOX_SECOND_DUMMY_ID], tetherKind: "bandage_root", bespokeKind: "aoe_bandage_lockdown" };
    case "anivia":
      return { shape: "zone", radius: 0.12, durationMs: 5200, persistent: true, pulseCount: 8, affectedTargetIds: [ULTIMATE_SANDBOX_DUMMY_ID, ULTIMATE_SANDBOX_SECOND_DUMMY_ID], bespokeKind: "persistent_slow_damage_storm" };
    case "annie":
      return { shape: "circle", radius: 0.105, destinationPos: targetPos, durationMs: 45000, persistent: true, pulseCount: 6, bespokeKind: "tibbers_drop_burst_pet" };
    case "aphelios":
      return { shape: "projectile", radius: 0.09, impactAt: 650, affectedTargetIds: [ULTIMATE_SANDBOX_DUMMY_ID, ULTIMATE_SANDBOX_SECOND_DUMMY_ID], requiresCondition: "weapon_specific_followup_pending", bespokeKind: "moonlight_bloom_weapon_pending" };
    default:
      return {};
  }
};

export const ULTIMATE_SANDBOX_IDENTITIES: UltimateSandboxIdentity[] = Array.from(
  rustUltimateCatalog.matchAll(
    /ident!\(\s*"([^"]+)",\s*"([^"]+)",\s*(\w+),\s*"([^"]+)",\s*\[[^\]]*\],\s*\[[^\]]*\],\s*\[([^\]]*)\],\s*\[([^\]]*)\],\s*\[([^\]]*)\]/g,
  ),
  ([, championKey, championName, primitive, signatureId, palette, shapeLanguage, motionHints]) => ({
    championKey,
    championName,
    primitive: toSnakeCase(primitive) as UltimatePrimitive,
    signatureId,
    palette: quotedValues(palette),
    shapeLanguage: quotedValues(shapeLanguage),
    motionHints: quotedValues(motionHints),
  }),
).sort((a, b) => a.championName.localeCompare(b.championName));

export function createUltimateSandboxEvent(
  identity: UltimateSandboxIdentity,
  castTimeSec: number,
): SimEvent {
  const originPos = ULTIMATE_SANDBOX_CASTER_POS;
  const targetPos = ULTIMATE_SANDBOX_DUMMY_POS;
  const metadata: UltimateIdentityEventMetadata = {
    event: "champion_ultimate_cast",
    actorId: ULTIMATE_SANDBOX_CASTER_ID,
    championId: identity.championKey,
    ultimateIdentity: {
      championKey: identity.championKey,
      championName: identity.championName,
      technicalPrimitive: identity.primitive,
      signatureId: identity.signatureId,
      visual: {
        visualEventId: `ultimate.${identity.signatureId}`,
        palette: identity.palette,
        shapeLanguage: identity.shapeLanguage,
        motionHints: identity.motionHints,
      },
      status: "active",
    },
    originPos,
    targetPos,
    targetId: ULTIMATE_SANDBOX_DUMMY_ID,
    direction: normalizeDirection(originPos, targetPos),
    shape: primitiveToShape(identity.primitive),
  };
  Object.assign(metadata, batchOneRealRouteMetadata(identity, originPos, targetPos));

  return {
    t: castTimeSec,
    text: `${identity.championName} casts ${identity.signatureId}`,
    type: "info",
    metadata: metadata as unknown as Record<string, unknown>,
  };
}
