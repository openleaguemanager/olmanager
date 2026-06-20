import type { MatchSnapshot } from "@/ui-v2/_legacy/components/match/types";

interface MapRuntimeWinnerToCanonicalScoresParams {
  canonicalSnapshot: MatchSnapshot;
  snapshotForResult: MatchSnapshot;
  winnerSide: "blue" | "red" | null;
}

export function mapRuntimeWinnerToCanonicalScores({
  canonicalSnapshot,
  snapshotForResult,
  winnerSide,
}: MapRuntimeWinnerToCanonicalScoresParams): Pick<MatchSnapshot, "home_score" | "away_score"> {
  if (!winnerSide) {
    return {
      home_score: canonicalSnapshot.home_score ?? 0,
      away_score: canonicalSnapshot.away_score ?? 0,
    };
  }

  const winnerTeamId =
    winnerSide === "blue" ? snapshotForResult.home_team.id : snapshotForResult.away_team.id;

  return {
    home_score: canonicalSnapshot.home_team.id === winnerTeamId ? 1 : 0,
    away_score: canonicalSnapshot.away_team.id === winnerTeamId ? 1 : 0,
  };
}
