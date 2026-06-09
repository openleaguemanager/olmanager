import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

// ─── Mocks ───────────────────────────────────────────────────────────────

vi.mock("react-i18next", () => ({
  initReactI18next: { type: "3rdParty", init: vi.fn() },
  useTranslation: () => ({
    i18n: { language: "es" },
    t: (key: string) => {
      const ES_TRANSLATIONS: Record<string, string> = {
        "placeholder.pendingDesign": "Pantalla pendiente de diseño.",
      };
      return ES_TRANSLATIONS[key] ?? key;
    },
  }),
}));

// ─── Tests ───────────────────────────────────────────────────────────────

describe("Placeholder", () => {
  it("renders title and pending design text", async () => {
    const { Placeholder } = await import("./Placeholder");
    render(<Placeholder title="Test Title" />);
    expect(screen.getByText("Test Title")).toBeInTheDocument();
    expect(screen.getByText("Pantalla pendiente de diseño.")).toBeInTheDocument();
  });
});
