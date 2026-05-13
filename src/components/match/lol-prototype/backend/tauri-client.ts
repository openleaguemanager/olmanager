import { invoke } from "@tauri-apps/api/core";

import type {
  LolSimV1DisposeRequest,
  LolSimV1DisposeResponse,
  LolSimV1DebugForceUltimateRequest,
  LolSimV1DebugForceUltimateResponse,
  LolSimV1InitRequest,
  LolSimV1RunToCompletionRequest,
  LolSimV1RunToCompletionResponse,
  LolSimV1ResetRequest,
  LolSimV1SkipToEndRequest,
  LolSimV1SkipToEndResponse,
  LolSimV1StateResponse,
  LolSimV1TickRequest,
} from "./contract-v1";
import type {
  LolSimV3DisposeRequest,
  LolSimV3DisposeResponse,
  LolSimV3InitRequest,
  LolSimV3RunToCompletionRequest,
  LolSimV3RunToCompletionResponse,
  LolSimV3ResetRequest,
  LolSimV3TickRequest,
  LolSimV3TickResponse,
} from "./contract-v3";

export type LolSimBackendMode = "v2" | "v3";

export function getLolSimBackendMode(): LolSimBackendMode {
  // Default back to V2 until V3 reaches full functional parity.
  return "v2";
}

export async function lolSimV2Init(request: LolSimV1InitRequest): Promise<LolSimV1StateResponse> {
  return invoke<LolSimV1StateResponse>("lol_sim_v2_init", { request });
}

export async function lolSimV2Tick(request: LolSimV1TickRequest): Promise<LolSimV1StateResponse> {
  return invoke<LolSimV1StateResponse>("lol_sim_v2_tick", { request });
}

export async function lolSimV2Reset(request: LolSimV1ResetRequest): Promise<LolSimV1StateResponse> {
  return invoke<LolSimV1StateResponse>("lol_sim_v2_reset", { request });
}

export async function lolSimV2Dispose(request: LolSimV1DisposeRequest): Promise<LolSimV1DisposeResponse> {
  return invoke<LolSimV1DisposeResponse>("lol_sim_v2_dispose", { request });
}

export async function lolSimV2RunToCompletion(
  request: LolSimV1RunToCompletionRequest,
): Promise<LolSimV1RunToCompletionResponse> {
  return invoke<LolSimV1RunToCompletionResponse>("lol_sim_v2_run_to_completion", { request });
}

export async function lolSimV2SkipToEnd(
  request: LolSimV1SkipToEndRequest,
): Promise<LolSimV1SkipToEndResponse> {
  return invoke<LolSimV1SkipToEndResponse>("lol_sim_v2_skip_to_end", { request });
}

export async function lolSimV2DebugForceUltimate(
  request: LolSimV1DebugForceUltimateRequest,
): Promise<LolSimV1DebugForceUltimateResponse> {
  return invoke<LolSimV1DebugForceUltimateResponse>("lol_sim_v2_debug_force_ultimate", { request });
}

export async function lolSimV3Init(request: LolSimV3InitRequest): Promise<LolSimV3TickResponse> {
  return invoke<LolSimV3TickResponse>("lol_sim_v3_init", { request });
}

export async function lolSimV3Tick(request: LolSimV3TickRequest): Promise<LolSimV3TickResponse> {
  return invoke<LolSimV3TickResponse>("lol_sim_v3_tick", { request });
}

export async function lolSimV3Reset(request: LolSimV3ResetRequest): Promise<LolSimV3TickResponse> {
  return invoke<LolSimV3TickResponse>("lol_sim_v3_reset", { request });
}

export async function lolSimV3Dispose(request: LolSimV3DisposeRequest): Promise<LolSimV3DisposeResponse> {
  return invoke<LolSimV3DisposeResponse>("lol_sim_v3_dispose", { request });
}

export async function lolSimV3RunToCompletion(
  request: LolSimV3RunToCompletionRequest,
): Promise<LolSimV3RunToCompletionResponse> {
  return invoke<LolSimV3RunToCompletionResponse>("lol_sim_v3_run_to_completion", { request });
}

function createSessionId() {
  return `lol-sim-v2-${Date.now()}-${Math.floor(Math.random() * 1_000_000)}`;
}

function createSessionIdV3() {
  return `lol-sim-v3-${Date.now()}-${Math.floor(Math.random() * 1_000_000)}`;
}

export class LolSimV2Client {
  readonly sessionId: string;

  constructor(sessionId?: string) {
    this.sessionId = sessionId ?? createSessionId();
  }

  async init(request: Omit<LolSimV1InitRequest, "sessionId">): Promise<LolSimV1StateResponse> {
    return lolSimV2Init({ ...request, sessionId: this.sessionId });
  }

  async tick(request: Omit<LolSimV1TickRequest, "sessionId">): Promise<LolSimV1StateResponse> {
    return lolSimV2Tick({ ...request, sessionId: this.sessionId });
  }

  async reset(request: Omit<LolSimV1ResetRequest, "sessionId">): Promise<LolSimV1StateResponse> {
    return lolSimV2Reset({ ...request, sessionId: this.sessionId });
  }

  async dispose(): Promise<LolSimV1DisposeResponse> {
    return lolSimV2Dispose({ sessionId: this.sessionId });
  }

  async skipToEnd(request?: Omit<LolSimV1SkipToEndRequest, "sessionId">): Promise<LolSimV1SkipToEndResponse> {
    return lolSimV2SkipToEnd({ sessionId: this.sessionId, ...(request ?? {}) });
  }

  async debugForceUltimate(
    request: Omit<LolSimV1DebugForceUltimateRequest, "sessionId">,
  ): Promise<LolSimV1DebugForceUltimateResponse> {
    return lolSimV2DebugForceUltimate({ ...request, sessionId: this.sessionId });
  }
}

export class LolSimV3Client {
  readonly sessionId: string;

  constructor(sessionId?: string) {
    this.sessionId = sessionId ?? createSessionIdV3();
  }

  async init(request: Omit<LolSimV3InitRequest, "sessionId">): Promise<LolSimV3TickResponse> {
    return lolSimV3Init({ ...request, sessionId: this.sessionId });
  }

  async tick(request: Omit<LolSimV3TickRequest, "sessionId">): Promise<LolSimV3TickResponse> {
    return lolSimV3Tick({ ...request, sessionId: this.sessionId });
  }

  async reset(request: Omit<LolSimV3ResetRequest, "sessionId">): Promise<LolSimV3TickResponse> {
    return lolSimV3Reset({ ...request, sessionId: this.sessionId });
  }

  async dispose(): Promise<LolSimV3DisposeResponse> {
    return lolSimV3Dispose({ sessionId: this.sessionId });
  }
}
