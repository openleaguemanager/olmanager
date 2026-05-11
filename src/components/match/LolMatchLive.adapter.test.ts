import { describe, expect, it } from "vitest";
import { __testables, type ChampionSelectionByPlayer } from "./LolMatchLive";
import type { LolSimV3TickResponse } from "./lol-prototype/backend/contract-v3";

function baseResponse(): LolSimV3TickResponse {
  return {
    sessionId: "test",
    snapshot: {
      tick: 1,
      timeSec: 360,
      running: true,
      winner: null,
      units: [
        { id: "blue-top", name: "Blue Top", team: "blue", role: "TOP", lane: "top", alive: true, pos: { x: 0.1, y: 0.1 }, hpRatio: 1, state: "laning" },
        { id: "blue-jgl", name: "Blue Jgl", team: "blue", role: "JGL", lane: "mid", alive: true, pos: { x: 0.1, y: 0.2 }, hpRatio: 1, state: "laning" },
        { id: "blue-mid", name: "Blue Mid", team: "blue", role: "MID", lane: "mid", alive: true, pos: { x: 0.1, y: 0.3 }, hpRatio: 1, state: "laning" },
        { id: "blue-adc", name: "Blue Adc", team: "blue", role: "ADC", lane: "bot", alive: true, pos: { x: 0.1, y: 0.4 }, hpRatio: 1, state: "laning" },
        { id: "blue-sup", name: "Blue Sup", team: "blue", role: "SUP", lane: "bot", alive: true, pos: { x: 0.1, y: 0.5 }, hpRatio: 1, state: "laning" },
        { id: "red-top", name: "Red Top", team: "red", role: "TOP", lane: "top", alive: true, pos: { x: 0.9, y: 0.1 }, hpRatio: 1, state: "laning" },
        { id: "red-jgl", name: "Red Jgl", team: "red", role: "JGL", lane: "mid", alive: true, pos: { x: 0.9, y: 0.2 }, hpRatio: 1, state: "laning" },
        { id: "red-mid", name: "Red Mid", team: "red", role: "MID", lane: "mid", alive: true, pos: { x: 0.9, y: 0.3 }, hpRatio: 1, state: "laning" },
        { id: "red-adc", name: "Red Adc", team: "red", role: "ADC", lane: "bot", alive: true, pos: { x: 0.9, y: 0.4 }, hpRatio: 1, state: "laning" },
        { id: "red-sup", name: "Red Sup", team: "red", role: "SUP", lane: "bot", alive: true, pos: { x: 0.9, y: 0.5 }, hpRatio: 1, state: "laning" },
      ],
      minions: [],
      structures: [],
      objectives: [
        { key: "dragon", alive: true, nextSpawnAtSec: null, pos: { x: 0.67, y: 0.7 } },
        { key: "baron", alive: false, nextSpawnAtSec: 1200, pos: { x: 0.33, y: 0.3 } },
      ],
      neutralCamps: [],
      scoreboard: {
        blue: { kills: 0, towers: 0, dragons: 0, gold: 0 },
        red: { kills: 0, towers: 0, dragons: 0, gold: 0 },
      },
    },
    events: [],
  };
}

describe("LolMatchLive V3 adapter", () => {
  it("maps neutral timers from V3 camps/objectives", () => {
    const response = baseResponse();
    response.snapshot.neutralTimers = { nextDragonAtSec: 720, nextBaronAtSec: 1260, campsAlive: 2, campsRespawning: 1 };
    response.snapshot.neutralCamps = [
      { key: "wolves-blue", team: "blue", alive: true, nextSpawnAtSec: null, pos: { x: 0.2, y: 0.2 } },
      { key: "red-buff-red", team: "red", alive: false, nextSpawnAtSec: 900, pos: { x: 0.8, y: 0.8 } },
    ];

    const mapped = __testables.mapV3ResponseToV1RuntimeState(response, {});

    expect(mapped.neutralTimers.entities.dragon).toBeDefined();
    expect(mapped.neutralTimers.entities.baron).toBeDefined();
    expect(mapped.neutralTimers.entities["wolves-blue"]).toBeDefined();
    expect(mapped.neutralTimers.entities["red-buff-red"]).toBeDefined();
    expect(mapped.neutralTimers.entities.dragon.nextSpawnAt).toBe(720);
    expect(mapped.neutralTimers.entities.baron.nextSpawnAt).toBe(1260);
  });

  it("fills champion ids for all units from full draft even with partial role maps", () => {
    const selection: ChampionSelectionByPlayer = {
      home: { b1: "Aatrox", b2: "Viego", b3: "Ahri", b4: "Jinx", b5: "Nautilus" },
      away: { r1: "Gnar", r2: "LeeSin", r3: "Orianna", r4: "Xayah", r5: "Rakan" },
      homeRoles: { b1: "TOP", b3: "MID" },
      awayRoles: { r1: "TOP", r2: "JUNGLE" },
    };

    const championMap = __testables.buildV3ChampionByPlayerId(selection);
    const mapped = __testables.mapV3ResponseToV1RuntimeState(baseResponse(), championMap);

    expect(mapped.champions).toHaveLength(10);
    mapped.champions.forEach((champion) => {
      expect(champion.championId).not.toBe("");
    });
  });

  it("maps V3 minions into V1 runtime state", () => {
    const response = baseResponse();
    response.snapshot.minions = [
      { id: "m1", team: "blue", lane: "mid", kind: "melee", alive: true, hpRatio: 0.5, pos: { x: 0.4, y: 0.4 } },
      { id: "m2", team: "red", lane: "bot", kind: "siege", alive: true, hpRatio: 1, pos: { x: 0.5, y: 0.5 } },
    ];

    const mapped = __testables.mapV3ResponseToV1RuntimeState(response, {});

    expect(mapped.minions).toHaveLength(2);
    expect(mapped.minions[0].id).toBe("m1");
    expect(mapped.minions[1].id).toBe("m2");
    expect(mapped.minions[1].kind).toBe("ranged");
    expect(mapped.minions[1].hp).toBeGreaterThan(0);
  });
});
