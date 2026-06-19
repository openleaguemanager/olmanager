import { describe, expect, it } from "vitest";

import { mapRuntimeWinnerToCanonicalScores } from "@/ui-v2/_legacy/pages/MatchSimulation.resultMapping";
import type { MatchSnapshot } from "@/ui-v2/_legacy/components/match/types";

function makePlayer(id: string, name: string, position = "ADC") {
  return {
    id,
    name,
    position,
    condition: 90,
    fitness: 90,
    mechanics: 70,
    laning: 70,
    teamfighting: 70,
    macro_play: 70,
    consistency: 70,
    shotcalling: 60,
    champion_pool: 70,
    discipline: 70,
    mental_resilience: 70,
    pace: 70,
    stamina: 70,
    strength: 60,
    agility: 70,
    passing: 70,
    shooting: 70,
    tackling: 40,
    dribbling: 70,
    defending: 40,
    positioning: 70,
    vision: 70,
    decisions: 70,
    composure: 70,
    aggression: 50,
    teamwork: 70,
    leadership: 60,
    handling: 20,
    reflexes: 20,
    aerial: 50,
    traits: [],
  };
}

function makeSnapshot(overrides: Partial<MatchSnapshot> = {}): MatchSnapshot {
  return {
    phase: "FullTime",
    current_minute: 35,
    home_score: 0,
    away_score: 0,
    possession: "Home",
    ball_zone: "Midfield",
    home_team: {
      id: "fnc",
      name: "Fnatic",
      draft_strategy: "Objective control",
      players: [makePlayer("fnc-adc", "FNC ADC")],
    },
    away_team: {
      id: "g2",
      name: "G2 Esports",
      draft_strategy: "Skirmish",
      players: [makePlayer("g2-adc", "G2 ADC")],
    },
    home_bench: [],
    away_bench: [],
    home_possession_pct: 50,
    away_possession_pct: 50,
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
  it("maps a red runtime win back to the canonical home team when the user controls red", () => {
    const canonicalSnapshot = makeSnapshot();
    const snapshotForResult = makeSnapshot({
      home_team: canonicalSnapshot.away_team,
      away_team: canonicalSnapshot.home_team,
      home_bench: canonicalSnapshot.away_bench,
      away_bench: canonicalSnapshot.home_bench,
    });

    const scores = mapRuntimeWinnerToCanonicalScores({
      canonicalSnapshot,
      snapshotForResult,
      winner: "red",
    });

    expect(scores).toEqual({ home_score: 1, away_score: 0 });
  });

  it("preserves canonical fallback scores when runtime has no winner", () => {
    const canonicalSnapshot = makeSnapshot({ home_score: 2, away_score: 1 });

    const scores = mapRuntimeWinnerToCanonicalScores({
      canonicalSnapshot,
      snapshotForResult: canonicalSnapshot,
      winner: null,
    });

    expect(scores).toEqual({ home_score: 2, away_score: 1 });
  });
});
