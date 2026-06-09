import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { GameStateData } from "@/store/gameStore";

// ─── Mocks ───────────────────────────────────────────────────────────────

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

const ES_TRANSLATIONS: Record<string, string> = {
  "inbox.title": "Bandeja",
  "inbox.unread": "Sin leer ({{count}})",
  "common.search": "Buscar...",
  "inbox.sortNewest": "Más nuevos",
  "inbox.sortOldest": "Más antiguos",
  "inbox.markAllRead": "Marcar leído",
  "inbox.clearOld": "Limpiar antiguos",
  "inbox.all": "Todas ({{count}})",
  "inbox.noMessages": "Sin mensajes",
  "inbox.selectMessage": "Selecciona un mensaje",
  "inbox.close": "Cerrar",
  "inbox.deleteMessage": "Eliminar",
};

vi.mock("react-i18next", () => ({
  initReactI18next: { type: "3rdParty", init: vi.fn() },
  useTranslation: () => ({
    i18n: { language: "es" },
    t: (key: string, options?: Record<string, unknown>) => {
      let template = ES_TRANSLATIONS[key];
      if (template === undefined) return key;
      // Simple interpolation for {{count}}
      if (options && "count" in options) {
        return template.replace("{{count}}", String(options.count));
      }
      return template;
    },
  }),
}));

vi.mock("@/lib/i18n/backendI18n", () => ({
  resolveBackendText: (key?: string, fallback?: string) => fallback ?? key ?? "",
  resolveMessage: (v: unknown) => v,
}));

vi.mock("@/services/inboxService", () => ({
  markMessageRead: vi.fn().mockResolvedValue({}),
  markAllMessagesRead: vi.fn().mockResolvedValue({}),
  clearOldMessages: vi.fn().mockResolvedValue({}),
  deleteMessage: vi.fn().mockResolvedValue({}),
}));

vi.mock("@/lib/inbox/helpers", () => ({
  getFilteredMessages: (messages: unknown[]) => messages,
  getNavigationTarget: () => ({ tab: "Home", context: {} }),
  isNavigateAction: () => false,
  sortInboxMessages: (messages: unknown[]) => messages,
  UNREAD_FILTER: "__unread__",
}));

// ─── Tests ───────────────────────────────────────────────────────────────

describe("InboxTabV2", () => {
  it("renders inbox title", async () => {
    const { InboxTabV2 } = await import("./InboxTabV2");
    const gs = { messages: [] } as unknown as GameStateData;
    render(
      <InboxTabV2 gameState={gs} onGameUpdate={vi.fn()} />,
    );
    expect(screen.getByText("Bandeja")).toBeInTheDocument();
  });

  it("renders select message prompt when no message selected", async () => {
    const { InboxTabV2 } = await import("./InboxTabV2");
    const gs = { messages: [] } as unknown as GameStateData;
    render(
      <InboxTabV2 gameState={gs} onGameUpdate={vi.fn()} />,
    );
    expect(screen.getByText("Selecciona un mensaje")).toBeInTheDocument();
  });

  it("renders no messages when inbox is empty", async () => {
    const { InboxTabV2 } = await import("./InboxTabV2");
    const gs = { messages: [] } as unknown as GameStateData;
    render(
      <InboxTabV2 gameState={gs} onGameUpdate={vi.fn()} />,
    );
    expect(screen.getByText("Sin mensajes")).toBeInTheDocument();
  });

  it("renders action buttons for actions bar", async () => {
    const { InboxTabV2 } = await import("./InboxTabV2");
    const gs = { messages: [
      { id: "m1", subject: "Test", body: "Body", sender: "Board", date: "2025-03-15", read: false, category: "Info", priority: "normal", actions: [], sender_icon: null, sender_role: null },
    ] } as unknown as GameStateData;
    render(
      <InboxTabV2 gameState={gs} onGameUpdate={vi.fn()} />,
    );

    // Search placeholder
    expect(screen.getByPlaceholderText("Buscar...")).toBeInTheDocument();
  });

  it("renders search placeholder", async () => {
    const { InboxTabV2 } = await import("./InboxTabV2");
    const gs = { messages: [] } as unknown as GameStateData;
    render(
      <InboxTabV2 gameState={gs} onGameUpdate={vi.fn()} />,
    );
    expect(screen.getByPlaceholderText("Buscar...")).toBeInTheDocument();
  });
});
