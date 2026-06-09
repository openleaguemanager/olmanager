import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

// ─── Mocks ───────────────────────────────────────────────────────────────

vi.mock("react-router-dom", () => ({
  useNavigate: () => vi.fn(),
}));

vi.mock("@/store/gameStore", () => ({
  useGameStore: () => ({
    setGameState: vi.fn(),
    setGameActive: vi.fn(),
  }),
}));

vi.mock("@/ui-v2/_legacy/components/teamSelection/teamSelection.helpers", () => ({
  loadLeagueSelectionData: () => Promise.resolve({ competitions: [] }),
  selectTeam: vi.fn(),
}));

vi.mock("@/lib/squad/helpers", () => ({
  buildActiveLineupIds: () => [],
}));

// Fully mock d3 to avoid topojson loading
vi.mock("d3", () => ({
  geoNaturalEarth1: () => {
    const fn: any = () => fn;
    fn.fitSize = () => fn;
    fn.center = () => fn;
    fn.scale = () => fn;
    fn.translate = () => fn;
    fn.rotate = () => fn;
    return fn;
  },
  geoPath: () => {
    const fn: any = () => "";
    fn.projection = () => fn;
    fn.centroid = () => fn;
    fn.pointRadius = () => fn;
    return fn;
  },
  select: () => ({
    append: () => ({
      attr: () => ({
        attr: () => ({
          style: () => ({
            style: () => ({
              attr: () => ({
                append: () => ({
                  attr: () => ({
                    append: () => vi.fn(),
                    text: () => vi.fn(),
                  }),
                  selectAll: () => ({
                    data: () => ({
                      join: () => ({
                        attr: () => ({
                          attr: () => ({
                            attr: () => ({
                              style: () => ({
                                style: () => ({
                                  on: () => vi.fn(),
                                }),
                              }),
                            }),
                          }),
                        }),
                      }),
                    }),
                    remove: () => vi.fn(),
                  }),
                }),
                select: () => ({
                  transition: () => ({
                    duration: () => ({
                      attr: () => ({
                        on: () => vi.fn(),
                      }),
                    }),
                  }),
                  attr: () => vi.fn(),
                }),
              }),
            }),
          }),
        }),
      }),
      remove: () => vi.fn(),
    }),
    selectAll: () => ({
      each: () => vi.fn(),
    }),
    node: () => null,
  }),
  selectAll: () => ({
    remove: () => vi.fn(),
  }),
}));

vi.mock("topojson-client", () => ({
  feature: () => ({ features: [] }),
}));

const ES_TRANSLATIONS: Record<string, string> = {
  "leaguePicker.europe": "Europe",
  "leaguePicker.northAmerica": "North America",
  "leaguePicker.korea": "Korea",
  "leaguePicker.china": "China",
  "leaguePicker.asiaPacific": "Asia-Pacific",
  "leaguePicker.southAmerica": "South America",
  "leaguePicker.mapError": "Error loading the map",
  "leaguePicker.clickRegion": "Click a region on the map",
  "worldSelect.creatingWorld": "Creating world...",
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

// ─── Tests ───────────────────────────────────────────────────────────────

describe("LeaguePickerMapV2", () => {
  it("renders creating world text in loading phase", async () => {
    const LeaguePickerMapV2 = (await import("./LeaguePickerMapV2")).default;
    render(<LeaguePickerMapV2 />);
    expect(screen.getByText("Creating world...")).toBeInTheDocument();
  });

  it("has getLeagueConfig function that returns translated labels", async () => {
    const mod = await import("./LeaguePickerMapV2");
    // getLeagueConfig should be exported
    expect(typeof mod.getLeagueConfig).toBe("function");

    const i18nMock = (key: string) => ES_TRANSLATIONS[key] ?? key;
    const config = mod.getLeagueConfig(i18nMock as any);

    expect(config.LEC.label).toBe("Europe");
    expect(config.LCS.label).toBe("North America");
    expect(config.LCK.label).toBe("Korea");
    expect(config.LPL.label).toBe("China");
    expect(config.LCP.label).toBe("Asia-Pacific");
    expect(config.CBLOL.label).toBe("South America");
  });
});
