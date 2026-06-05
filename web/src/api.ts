import { supabase } from "./supabase";

const API_BASE = (import.meta.env.VITE_API_BASE as string) || "";

/** A save row as returned by GET /api/saves. */
export interface SaveSummary {
  id: string;
  name: string;
  manager: string | null;
  updated_at: string;
}

/** Minimal shape of the serialized game we read in the web UI so far. */
export interface GameState {
  clock: { current_date: string };
  day_phase: string;
  manager: { nickname: string; first_name: string; last_name: string };
  teams: { id: string; name: string; short_name?: string; competition_id?: string | null }[];
  players: unknown[];
  leagues: { id: string; name: string; season: number }[];
}

async function authHeader(): Promise<Record<string, string>> {
  const { data } = await supabase.auth.getSession();
  const token = data.session?.access_token;
  return token ? { Authorization: `Bearer ${token}` } : {};
}

async function request<T>(path: string, init: RequestInit = {}): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    ...(await authHeader()),
    ...((init.headers as Record<string, string>) ?? {}),
  };
  const res = await fetch(`${API_BASE}${path}`, { ...init, headers });
  if (!res.ok) {
    let detail = res.statusText;
    try {
      const body = await res.json();
      detail = body.error ?? detail;
    } catch {
      /* ignore */
    }
    throw new Error(`${res.status}: ${detail}`);
  }
  return res.json() as Promise<T>;
}

export interface NewGameInput {
  first_name: string;
  last_name: string;
  nickname?: string;
  date_of_birth: string;
  nationality: string;
  name?: string;
}

export const api = {
  me: () => request<{ user_id: string }>("/api/me"),

  listSaves: () => request<{ saves: SaveSummary[] }>("/api/saves"),

  createSave: (input: NewGameInput) =>
    request<{ id: string; game: GameState }>("/api/saves", {
      method: "POST",
      body: JSON.stringify(input),
    }),

  loadSave: (id: string) =>
    request<{ id: string; game: GameState }>(`/api/saves/${id}`),

  selectTeam: (id: string, teamId: string) =>
    request<{ id: string; game: GameState }>(`/api/saves/${id}/select-team`, {
      method: "POST",
      body: JSON.stringify({ team_id: teamId }),
    }),

  advance: (id: string) =>
    request<{ id: string; game: GameState }>(`/api/saves/${id}/advance`, {
      method: "POST",
    }),

  deleteSave: (id: string) =>
    request<{ deleted: string }>(`/api/saves/${id}`, { method: "DELETE" }),
};
