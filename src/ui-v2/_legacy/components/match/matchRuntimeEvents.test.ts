import { describe, expect, it } from "vitest";

import {
  mapRuntimeEventsToMatchEvents,
  mergeRuntimeEventsIntoSnapshot,
} from "@/ui-v2/_legacy/components/match/matchRuntimeEvents";
import type { MatchSnapshot } from "@/ui-v2/_legacy/components/match/types";

function makeSnapshot(overrides: Partial<MatchSnapshot> = {}): MatchSnapshot {
  return {
    phase: "FullTime",
    current_minute: 35,
    home_score: 1,
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

describe("mapRuntimeEventsToMatchEvents", () => {
  it("maps BLUE/RED to Home/Away by default", () => {
    const events = mapRuntimeEventsToMatchEvents([
      { t: 180, type: "kill", text: "BLUE bot lane killed RED ADC" },
      { t: 360, type: "dragon", text: "RED secured Infernal Dragon" },
    ]);

    expect(events.map((event) => [event.event_type, event.side])).toEqual([
      ["Kill", "Home"],
      ["Dragon", "Away"],
    ]);
  });

  it("maps BLUE to Away and RED to Home when the blue team is the canonical away team", () => {
    const blueTeamId = "away-team";
    const homeTeamId = "home-team";
    const events = mapRuntimeEventsToMatchEvents(
      [
        { t: 180, type: "kill", text: "BLUE bot lane killed RED ADC" },
        { t: 360, type: "dragon", text: "RED secured Infernal Dragon" },
      ],
      blueTeamId,
      homeTeamId,
    );

    expect(events.map((event) => [event.event_type, event.side])).toEqual([
      ["Kill", "Away"],
      ["Dragon", "Home"],
    ]);
  });

  it("keeps HOME/AWAY mapping even when blueTeamId is provided", () => {
    const events = mapRuntimeEventsToMatchEvents(
      [
        { t: 180, type: "kill", text: "HOME team scored" },
        { t: 360, type: "dragon", text: "AWAY team secured dragon" },
      ],
      "away-team",
      "home-team",
    );

    expect(events.map((event) => [event.event_type, event.side])).toEqual([
      ["Kill", "Home"],
      ["Dragon", "Away"],
    ]);
  });
});

describe("mergeRuntimeEventsIntoSnapshot", () => {
  it("merges runtime events using the snapshot's blue side mapping", () => {
    const snapshot = makeSnapshot({
      events: [{ minute: 1, event_type: "Kill", side: "Away", zone: "Top", player_id: null, secondary_player_id: null }],
    });

    const merged = mergeRuntimeEventsIntoSnapshot(
      snapshot,
      [
        { t: 600, type: "dragon", text: "BLUE secured Infernal Dragon" },
        { t: 1500, type: "baron", text: "RED secured Baron Nashor" },
      ],
      snapshot.home_team.id,
    );

    expect(merged.events).toHaveLength(3);
    expect(merged.events.slice(1).map((event) => [event.event_type, event.side])).toEqual([
      ["Dragon", "Home"],
      ["Baron", "Away"],
    ]);
  });

  it("maps BLUE events to the canonical away team when the blue team is away", () => {
    const snapshot = makeSnapshot();

    const merged = mergeRuntimeEventsIntoSnapshot(
      snapshot,
      [{ t: 600, type: "dragon", text: "BLUE secured Infernal Dragon" }],
      snapshot.away_team.id,
    );

    expect(merged.events).toEqual([
      { minute: 10, event_type: "Dragon", side: "Away", zone: "mid", player_id: null, secondary_player_id: null },
    ]);
  });
});
