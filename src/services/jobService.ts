import { getApiClientSync } from "../api/client";
import type { GameStateData } from "../store/gameStore";

export interface JobOpportunity {
  team_id: string;
  team_name: string;
  city: string;
  reputation: number;
  last_league_position: number | null;
}

export interface JobApplicationResponse {
  result: "hired" | "rejected" | "invalid_team" | "already_employed";
  game: GameStateData;
}

export async function getAvailableJobs(): Promise<JobOpportunity[]> {
  return getApiClientSync().jobs.getAvailable();
}

export async function applyForJob(
  teamId: string,
): Promise<JobApplicationResponse> {
  return getApiClientSync().jobs.apply({ teamId });
}
