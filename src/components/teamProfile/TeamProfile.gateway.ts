import { getApiClientSync } from "../../api/client";
import type { TeamRecentMatchEntry, TeamStatsOverview } from "./TeamProfile.types";

export const TEAM_PROFILE_RECENT_MATCH_LIMIT = 5;

export async function fetchTeamStatsOverview(
  teamId: string,
): Promise<TeamStatsOverview | null> {
  return getApiClientSync().teams.getStatsOverview({ teamId });
}

export async function fetchTeamRecentMatches(
  teamId: string,
  limit = TEAM_PROFILE_RECENT_MATCH_LIMIT,
): Promise<TeamRecentMatchEntry[]> {
  const result = await getApiClientSync().teams.getMatchHistory({ teamId, limit });
  return Array.isArray(result) ? result : [];
}
