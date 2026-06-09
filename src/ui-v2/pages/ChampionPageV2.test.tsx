import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

// ─── Mocks ───────────────────────────────────────────────────────────────

vi.mock("@/lib/champions/championImages", () => ({
  resolveChampionTile: () => null,
  resolveChampionSplash: () => null,
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockRejectedValue(new Error("no backend")),
}));

const ES_TRANSLATIONS: Record<string, string> = {
  "championPage.champion": "Campeón",
  "championPage.wr": "WR",
  "championPage.pr": "PR",
  "championPage.br": "BR",
  "championPage.kda": "KDA",
  "championPage.kills": "Kills",
  "championPage.deaths": "Deaths",
  "championPage.assists": "Assists",
  "championPage.gold": "Gold",
  "championPage.damage": "Damage",
  "championPage.cs": "CS",
  "championPage.roles": "Roles",
  "championPage.noRoles": "Sin datos de roles",
  "championPage.bestAgainst": "Mejor contra",
  "championPage.worstAgainst": "Peor contra",
  "championPage.topPlayers": "Top jugadores",
  "championPage.noData": "Sin datos",
  "championPage.vision": "Visión",
  "championPage.duration": "Duración",
  "championPage.weekly": "Semanal",
  "championPage.noHistory": "Sin historial",
};

vi.mock("react-i18next", () => ({
  initReactI18next: { type: "3rdParty", init: vi.fn() },
  useTranslation: () => ({
    i18n: { language: "es" },
    t: (key: string) => ES_TRANSLATIONS[key] ?? key,
  }),
}));

// ─── Tests ───────────────────────────────────────────────────────────────

describe("ChampionPageV2", () => {
  it("renders hero section with champion label", async () => {
    const ChampionPageV2 = (await import("./ChampionPageV2")).default;
    render(<ChampionPageV2 championKey="Ahri" onClose={vi.fn()} />);
    // Champion name from championKey when no data
    const names = screen.getAllByText(/Ahri/i);
    expect(names.length).toBeGreaterThanOrEqual(1);
    // The "Campeón" label
    expect(screen.getByText("Campeón")).toBeInTheDocument();
  });

  it("renders stat cards with labels when no data", async () => {
    const ChampionPageV2 = (await import("./ChampionPageV2")).default;
    render(<ChampionPageV2 championKey="Ahri" onClose={vi.fn()} />);
    // Hero stats
    expect(screen.getByText("WR")).toBeInTheDocument();
    expect(screen.getByText("PR")).toBeInTheDocument();
    expect(screen.getByText("BR")).toBeInTheDocument();
    expect(screen.getByText("KDA")).toBeInTheDocument();
    // Performance stat cards
    expect(screen.getByText("Kills")).toBeInTheDocument();
    expect(screen.getByText("Deaths")).toBeInTheDocument();
    expect(screen.getByText("Assists")).toBeInTheDocument();
    expect(screen.getByText("Gold")).toBeInTheDocument();
    expect(screen.getByText("Damage")).toBeInTheDocument();
    expect(screen.getByText("CS")).toBeInTheDocument();
  });

  it("renders roles section with empty state", async () => {
    const ChampionPageV2 = (await import("./ChampionPageV2")).default;
    render(<ChampionPageV2 championKey="Ahri" onClose={vi.fn()} />);
    expect(screen.getByText("Roles")).toBeInTheDocument();
    expect(screen.getByText("Sin datos de roles")).toBeInTheDocument();
  });

  it("renders matchups with titles", async () => {
    const ChampionPageV2 = (await import("./ChampionPageV2")).default;
    render(<ChampionPageV2 championKey="Ahri" onClose={vi.fn()} />);
    expect(screen.getByText("Mejor contra")).toBeInTheDocument();
    expect(screen.getByText("Peor contra")).toBeInTheDocument();
  });

  it("renders sidebar sections", async () => {
    const ChampionPageV2 = (await import("./ChampionPageV2")).default;
    render(<ChampionPageV2 championKey="Ahri" onClose={vi.fn()} />);
    expect(screen.getByText("Top jugadores")).toBeInTheDocument();
    // Multiple "Sin datos" from matchups and top players block
    const sinDatos = screen.getAllByText("Sin datos");
    expect(sinDatos.length).toBeGreaterThanOrEqual(1);
    expect(screen.getByText("Semanal")).toBeInTheDocument();
    expect(screen.getByText("Sin historial")).toBeInTheDocument();
  });
});
