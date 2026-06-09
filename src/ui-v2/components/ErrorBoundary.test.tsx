import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

// ─── Mocks ───────────────────────────────────────────────────────────────

const ES_TRANSLATIONS: Record<string, string> = {
  "errorBoundary.somethingWentWrong": "Algo salió mal",
  "errorBoundary.unexpectedError": "Ocurrió un error inesperado. Presioná F5 para recargar la aplicación.",
  "errorBoundary.reload": "Recargar (F5)",
};

vi.mock("react-i18next", () => ({
  initReactI18next: { type: "3rdParty", init: vi.fn() },
  useTranslation: () => ({
    i18n: { language: "es" },
    t: (key: string) => ES_TRANSLATIONS[key] ?? key,
  }),
}));

// ErrorBoundary uses i18n direct import — mock that too
vi.mock("@/i18n", () => ({
  default: {
    t: (key: string) => ES_TRANSLATIONS[key] ?? key,
    language: "es",
    changeLanguage: vi.fn(),
  },
}));

// ─── Tests ───────────────────────────────────────────────────────────────

function Bomb(): React.ReactNode {
  throw new Error("💥");
}

describe("ErrorBoundary", () => {
  it("renders children when no error", async () => {
    const ErrorBoundary = (await import("./ErrorBoundary")).default;
    render(
      <ErrorBoundary>
        <p>All good</p>
      </ErrorBoundary>,
    );
    expect(screen.getByText("All good")).toBeInTheDocument();
  });

  it("renders error UI when a child throws", async () => {
    // Suppress console.error from React error logging in tests
    vi.spyOn(console, "error").mockImplementation(() => {});
    const ErrorBoundary = (await import("./ErrorBoundary")).default;
    render(
      <ErrorBoundary>
        <Bomb />
      </ErrorBoundary>,
    );
    expect(screen.getByText("Algo salió mal")).toBeInTheDocument();
    expect(
      screen.getByText("Ocurrió un error inesperado. Presioná F5 para recargar la aplicación."),
    ).toBeInTheDocument();
    expect(screen.getByText("Recargar (F5)")).toBeInTheDocument();
  });
});
