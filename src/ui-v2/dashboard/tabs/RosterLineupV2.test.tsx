import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { PlayerData } from "@/store/gameStore";

// ─── Mocks ───────────────────────────────────────────────────────────────

const ES_TRANSLATIONS: Record<string, string> = {
  "common.condition": "Condition",
  "common.morale": "Morale",
  "tactics.lol.roles.TOP": "Top",
  "tactics.lol.roles.JUNGLE": "Jungle",
  "tactics.lol.roles.MID": "Mid",
  "tactics.lol.roles.ADC": "ADC",
  "tactics.lol.roles.SUPPORT": "Support",
};

vi.mock("react-i18next", () => ({
  initReactI18next: { type: "3rdParty", init: vi.fn() },
  useTranslation: () => ({
    i18n: { language: "en" },
    t: (key: string, options?: Record<string, unknown>) => {
      if (options && typeof options === "object" && "defaultValue" in options) {
        return ES_TRANSLATIONS[key] ?? String(options.defaultValue);
      }
      return ES_TRANSLATIONS[key] ?? key;
    },
  }),
}));

vi.mock("@/lib/players/lolIdentity", () => ({
  resolvePlayerLolRole: () => "MID",
  fallbackChampionForRole: () => "",
}));

vi.mock("@/lib/players/playerPhotos", () => ({
  resolvePlayerPhoto: () => null,
}));

vi.mock("@/lib/players/lolPlayerStats", () => ({
  calculateLolOvr: () => 80,
}));

vi.mock("@/lib/champions/championImages", () => ({
  resolveChampionSplash: () => null,
}));

vi.mock("@/lib/champions/championIds", () => ({
  normalizeChampionKey: (v: string) => v,
}));

// ─── Tests ───────────────────────────────────────────────────────────────

describe("RosterLineupV2", () => {
  it("renders stat box labels using i18n keys", async () => {
    const { RosterLineupV2 } = await import("./RosterLineupV2");
    const roster: PlayerData[] = [
      {
        id: "p1",
        match_name: "Player One",
        team_id: "team-1",
        role: "MID",
        condition: 85,
        morale: 70,
      } as unknown as PlayerData,
    ];
    render(
      <RosterLineupV2
        roster={roster}
        championMasteries={[]}
        onNavigate={vi.fn()}
        onSelectPlayer={vi.fn()}
      />,
    );
    const conditionElements = screen.getAllByText("Condition");
    expect(conditionElements.length).toBeGreaterThanOrEqual(1);
    const moraleElements = screen.getAllByText("Morale");
    expect(moraleElements.length).toBeGreaterThanOrEqual(1);
  });

  it("renders OVR for players", async () => {
    const { RosterLineupV2 } = await import("./RosterLineupV2");
    const roster = [
      {
        id: "p1",
        match_name: "Player One",
        team_id: "team-1",
        condition: 85,
        morale: 70,
      } as unknown as PlayerData,
    ];
    render(
      <RosterLineupV2 roster={roster} championMasteries={[]} />,
    );
    const ovrElements = screen.getAllByText(/OVR/);
    expect(ovrElements.length).toBeGreaterThanOrEqual(1);
  });
});
