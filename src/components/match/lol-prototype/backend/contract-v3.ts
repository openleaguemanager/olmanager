/**
 * LoL Sim Contract V3
 *
 * Contrato público para el nuevo runtime semántico.
 * Esta capa existe para desacoplar frontend de detalles internos del engine.
 */

export type LolSimV3Team = "blue" | "red";

export type LolSimV3AgentState =
  | "laning"
  | "pushing"
  | "roaming"
  | "objective_setup"
  | "fighting"
  | "recalling"
  | "dead";

export type LolSimV3EventKind =
  | "unit_moved"
  | "agent_state_changed"
  | "trade_started"
  | "damage_applied"
  | "champion_killed"
  | "tower_destroyed"
  | "dragon_taken"
  | "baron_taken"
  | "wave_spawned"
  | "neutral_camp_spawned"
  | "neutral_camp_taken"
  | "tower_damaged"
  | "nexus_destroyed"
  | "gold_changed";

export interface LolSimV3Vec2 {
  x: number;
  y: number;
}

export interface LolSimV3UnitView {
  id: string;
  name: string;
  championId?: string;
  team: LolSimV3Team | string;
  role: string;
  lane: string;
  alive: boolean;
  pos: LolSimV3Vec2;
  hpRatio: number;
  state: LolSimV3AgentState;
}

export interface LolSimV3StructureView {
  id: string;
  team: LolSimV3Team | string;
  lane: string;
  kind: string;
  alive: boolean;
  hpRatio: number;
  pos: LolSimV3Vec2;
}

export interface LolSimV3MinionView {
  id: string;
  team: LolSimV3Team | string;
  lane: string;
  kind: "melee" | "ranged" | "siege" | string;
  alive: boolean;
  hpRatio: number;
  pos: LolSimV3Vec2;
}

export interface LolSimV3ObjectiveView {
  key: string;
  alive: boolean;
  nextSpawnAtSec: number | null;
  pos: LolSimV3Vec2;
}

export interface LolSimV3NeutralCampView {
  key: string;
  team: LolSimV3Team | string;
  alive: boolean;
  nextSpawnAtSec: number | null;
  pos: LolSimV3Vec2;
}

export interface LolSimV3ScoreboardTeam {
  kills: number;
  towers: number;
  dragons: number;
  gold: number;
}

export interface LolSimV3Scoreboard {
  blue: LolSimV3ScoreboardTeam;
  red: LolSimV3ScoreboardTeam;
}

export interface LolSimV3Snapshot {
  tick: number;
  timeSec: number;
  running: boolean;
  winner: LolSimV3Team | string | null;
  units: LolSimV3UnitView[];
  minions: LolSimV3MinionView[];
  structures: LolSimV3StructureView[];
  objectives: LolSimV3ObjectiveView[];
  neutralCamps: LolSimV3NeutralCampView[];
  scoreboard: LolSimV3Scoreboard;
  lanePressure?: Array<{ lane: string; blue: number; red: number }>;
  towerTargets?: Array<{ towerId: string; targetId?: string; targetKind?: string; lockUntilSec: number }>;
  neutralTimers?: {
    nextDragonAtSec?: number | null;
    nextBaronAtSec?: number | null;
    campsAlive: number;
    campsRespawning: number;
  };
  phaseContributions?: Array<{ team: string; phase: string; value: number }>;
  roleLaneContributions?: Array<{
    team: string;
    role: string;
    lane: string;
    pressure: number;
    objectivePressure: number;
  }>;
  objectivePressureSummary?: {
    blue: number;
    red: number;
    contested: boolean;
    delta: number;
  };
}

export interface LolSimV3Event {
  id: string;
  t: number;
  kind: LolSimV3EventKind;
  actorId?: string;
  targetId?: string;
  team?: LolSimV3Team | string;
  lane?: string;
  amount?: number;
  fromState?: LolSimV3AgentState;
  toState?: LolSimV3AgentState;
  fromPos?: LolSimV3Vec2;
  toPos?: LolSimV3Vec2;
  metadata?: Record<string, unknown>;
}

export interface LolSimV3InitRequest {
  sessionId: string;
  seed: string;
  snapshot: unknown;
  championByPlayerId?: Record<string, string>;
  tickDtSec?: number;
}

export interface LolSimV3TickRequest {
  sessionId: string;
  running: boolean;
  steps?: number;
}

export interface LolSimV3TickResponse {
  sessionId: string;
  snapshot: LolSimV3Snapshot;
  events: LolSimV3Event[];
}

export interface LolSimV3ResetRequest {
  sessionId: string;
  seed: string;
  tickDtSec?: number;
}

export interface LolSimV3DisposeRequest {
  sessionId: string;
}

export interface LolSimV3DisposeResponse {
  sessionId: string;
  disposed: boolean;
}

export interface LolSimV3RunToCompletionRequest {
  seed: string;
  snapshot: unknown;
  championByPlayerId?: Record<string, string>;
  tickDtSec?: number;
  maxSteps?: number;
}

export interface LolSimV3RunToCompletionResponse {
  winner: LolSimV3Team | string | null;
  steps: number;
  elapsedSimulatedSec: number;
  snapshot: LolSimV3Snapshot;
  events: LolSimV3Event[];
}
