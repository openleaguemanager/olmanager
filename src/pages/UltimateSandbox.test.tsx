import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import type { ComponentPropsWithoutRef } from "react";

import UltimateSandbox from "./UltimateSandbox";

const navigateMock = vi.fn();

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock("react-router-dom", () => ({
  useNavigate: () => navigateMock,
}));

vi.mock("../components/ui", () => ({
  Button: ({ children, ...props }: ComponentPropsWithoutRef<"button">) => (
    <button {...props}>{children}</button>
  ),
}));

vi.mock("../components/match/lol-prototype/assets/map", () => ({
  getWalls: () => [],
}));

vi.mock("../components/match/lol-prototype/ui/render", () => ({
  renderSimulation: vi.fn(),
}));

const mockedInvoke = vi.mocked(invoke);

const baseState = {
  timeSec: 0,
  running: true,
  winner: null,
  showWalls: false,
  champions: [
    { id: "sandbox-home-mid", name: "Ultimate Tester", championId: "Ahri", team: "blue", role: "MID", pos: { x: 0.3, y: 0.7 }, hp: 200, maxHp: 200, alive: true },
    { id: "sandbox-away-mid", name: "Red Mid", championId: "Lux", team: "red", role: "MID", pos: { x: 0.7, y: 0.3 }, hp: 200, maxHp: 200, alive: true },
  ],
  minions: [],
  structures: [],
  objectives: {},
  neutralTimers: { dragonSoulUnlocked: false, elderUnlocked: false, entities: {} },
  stats: { blue: { kills: 0, towers: 0, dragons: 0, barons: 0, gold: 2500 }, red: { kills: 0, towers: 0, dragons: 0, barons: 0, gold: 2500 } },
  events: [],
  speed: 4,
};

describe("UltimateSandbox", () => {
  beforeEach(() => {
    navigateMock.mockReset();
    mockedInvoke.mockReset();
    vi.spyOn(window, "requestAnimationFrame").mockReturnValue(1);
    vi.spyOn(window, "cancelAnimationFrame").mockImplementation(() => undefined);
    mockedInvoke.mockImplementation(async (command: string) => {
      if (command === "lol_sim_v2_init") return { sessionId: "sandbox", state: baseState };
      if (command === "lol_sim_v2_debug_force_ultimate") {
        return {
          sessionId: "sandbox",
          state: {
            ...baseState,
            events: [{
              t: 1,
              text: "Ultimate Tester debug forced Ultimate (burst)",
              type: "info",
              metadata: {
                event: "champion_ultimate_cast",
                actorId: "sandbox-home-mid",
                targetId: "sandbox-away-mid",
                shape: "circle",
                originPos: { x: 0.3, y: 0.7 },
                targetPos: { x: 0.7, y: 0.3 },
                direction: { x: 0.7, y: -0.7 },
              },
            }],
          },
          casted: true,
          reason: null,
        };
      }
      if (command === "lol_sim_v2_dispose") return { sessionId: "sandbox", disposed: true };
      return { sessionId: "sandbox", state: baseState };
    });
  });

  it("muestra los controles de Live Sim Debug", async () => {
    render(<UltimateSandbox />);

    expect(screen.getByText("Ultimate Sandbox")).toBeInTheDocument();
    expect(screen.getByText("Forzar ultimate")).toBeInTheDocument();
    expect(screen.getByText("Pausar")).toBeInTheDocument();
    expect(screen.getByText("Slow mode")).toBeInTheDocument();
    expect(screen.getByText("Live Sim Debug")).toBeInTheDocument();

    await waitFor(() => expect(mockedInvoke).toHaveBeenCalledWith("lol_sim_v2_init", expect.any(Object)));
  });

  it("forzar ultimate llama al debug hook V2 y muestra el último event real", async () => {
    render(<UltimateSandbox />);

    await waitFor(() => expect(mockedInvoke).toHaveBeenCalledWith("lol_sim_v2_init", expect.any(Object)));
    fireEvent.click(screen.getByText("Forzar ultimate"));

    await waitFor(() => {
      expect(mockedInvoke).toHaveBeenCalledWith(
        "lol_sim_v2_debug_force_ultimate",
        expect.objectContaining({ request: expect.objectContaining({ casterId: "sandbox-home-mid" }) }),
      );
    });

    expect(await screen.findByText("Ultimate Tester debug forced Ultimate (burst)")).toBeInTheDocument();
    expect(screen.getByText("Ultimate forzada en backend V2.")).toBeInTheDocument();
  });
});
