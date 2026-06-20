import { describe, expect, it } from "vitest";

import { mapRuntimeWinnerToCanonicalScores } from "@/ui-v2/_legacy/pages/MatchSimulation.resultMapping";
import type { MatchSnapshot } from "@/ui-v2/_legacy/components/match/types";

function makeSnapshot(overrides: Partial<MatchSnapshot> = {}): MatchSnapshot {
  return {
    phase: "FullTime",
    current_minute: 35,
    home_score: 0,
    away_score: 0,
    possession: "Home",
    ball_zone: "Midfield",
    home_team: {
      id: "home-team",
      name: "Home Team",
      draft_strategy: "Objective control",
      players: [],
    },
    away_team: {
      id: "away-team",
      name: "Away Team",
      draft_strategy: "Skirmish",
      players: [],
    },
    home_bench: [],
    away_bench: [],
    home_possession_pct: 55,
    away_possession_pct: 45,
    events: [],
    home_subs_made: 0,
    away_subs_made: 0,
    max_subs: 0,
    home_roles: { captain: null, shotcaller: null },
    away_roles: { captain: null, shotcaller: null },
    substitutions: [],
    allows_extra_time: false,
    home_yellows: {},
    away_yellows: {},
    sent_off: [],
    ...overrides,
  };
}

describe("mapRuntimeWinnerToCanonicalScores", () => {
  it("returns existing scores when winnerSide is null", () => {
    const snapshot = makeSnapshot({ home_score: 0, away_score: 0 });
    const scores = mapRuntimeWinnerToCanonicalScores({
      canonicalSnapshot: snapshot,
      snapshotForResult: snapshot,
      winnerSide: null,
    });

    expect(scores).toEqual({ home_score: 0, away_score: 0 });
  });

  it("maps blue winner to the home team in the result snapshot", () => {
    const canonicalSnapshot = makeSnapshot({ home_score: 0, away_score: 0 });
    const snapshotForResult = makeSnapshot({ home_score: 0, away_score: 0 });

    const scores = mapRuntimeWinnerToCanonicalScores({
      canonicalSnapshot,
      snapshotForResult,
      winnerSide: "blue",
    });

    expect(scores).toEqual({ home_score: 1, away_score: 0 });
  });

  it("maps red winner to the away team in the result snapshot", () => {
    const canonicalSnapshot = makeSnapshot({ home_score: 0, away_score: 0 });
    const snapshotForResult = makeSnapshot({ home_score: 0, away_score: 0 });

    const scores = mapRuntimeWinnerToCanonicalScores({
      canonicalSnapshot,
      snapshotForResult,
      winnerSide: "red",
    });

    expect(scores).toEqual({ home_score: 0, away_score: 1 });
  });

  it("maps winners back to canonical sides when the result snapshot is side-swapped", () => {
    const canonicalSnapshot = makeSnapshot({ home_score: 0, away_score: 0 });
    // Side-swapped result snapshot: home_team is the original away team.
    const snapshotForResult = makeSnapshot({
      home_team: { id: "away-team", name: "Away Team", draft_strategy: "Skirmish", players: [] },
      away_team: { id: "home-team", name: "Home Team", draft_strategy: "Objective control", players: [] },
    });

    const scores = mapRuntimeWinnerToCanonicalScores({
      canonicalSnapshot,
      snapshotForResult,
      winnerSide: "blue",
    });

    // Blue side in the swapped snapshot is the original away team, so canonical away_score wins.
    expect(scores).toEqual({ home_score: 0, away_score: 1 });
  });
});
