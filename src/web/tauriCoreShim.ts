import { supabase } from "./supabase";

const API_BASE = (import.meta.env.VITE_API_BASE as string | undefined) ?? "";
const ACTIVE_SAVE_KEY = "olmanager.web.activeSaveId";
const SETTINGS_KEY = "olmanager.web.settings";

interface SaveSummary {
  id: string;
  name: string;
  manager: string | null;
  updated_at: string;
}

type InvokeArgs = Record<string, unknown> | undefined;

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
  const response = await fetch(`${API_BASE}${path}`, { ...init, headers });
  if (!response.ok) {
    let detail = response.statusText;
    try {
      const body = await response.json();
      detail = body.error ?? detail;
    } catch {
      /* keep status text */
    }
    throw new Error(`${response.status}: ${detail}`);
  }
  if (response.status === 204) {
    return undefined as T;
  }
  return response.json() as Promise<T>;
}

function activeSaveId(): string {
  const id = localStorage.getItem(ACTIVE_SAVE_KEY);
  if (!id) {
    throw new Error("No active save selected");
  }
  return id;
}

function setActiveSave(id: string | null): void {
  if (id) {
    localStorage.setItem(ACTIVE_SAVE_KEY, id);
  } else {
    localStorage.removeItem(ACTIVE_SAVE_KEY);
  }
}

function managerDisplayName(game: any): string {
  const manager = game?.manager ?? {};
  return manager.nickname?.trim?.() || `${manager.first_name ?? ""} ${manager.last_name ?? ""}`.trim();
}

function toLegacySaveEntry(save: SaveSummary) {
  return {
    id: save.id,
    name: save.name,
    manager_name: save.manager ?? save.name,
    db_filename: "",
    checksum: "",
    created_at: save.updated_at,
    last_played_at: save.updated_at,
  };
}

export async function invoke<T = unknown>(command: string, args?: InvokeArgs): Promise<T> {
  switch (command) {
    case "debug_log": {
      if (import.meta.env.DEV) {
        console.debug(String(args?.message ?? ""));
      }
      return undefined as T;
    }
    case "get_saves": {
      const result = await request<{ saves: SaveSummary[] }>("/api/saves");
      return result.saves.map(toLegacySaveEntry) as T;
    }
    case "start_new_game_lightweight": {
      const result = await request<{ id: string; game: unknown }>("/api/saves", {
        method: "POST",
        body: JSON.stringify({
          first_name: args?.firstName,
          last_name: args?.lastName,
          nickname: args?.nickname ?? null,
          date_of_birth: args?.dob,
          nationality: args?.nationality,
          name: args?.nickname || `${args?.firstName ?? ""} ${args?.lastName ?? ""}`.trim() || "Career",
        }),
      });
      setActiveSave(result.id);
      return "ok" as T;
    }
    case "load_game": {
      const saveId = String(args?.saveId ?? "");
      const result = await request<{ id: string; game: unknown }>(`/api/saves/${saveId}`);
      setActiveSave(result.id);
      return managerDisplayName(result.game) as T;
    }
    case "delete_save": {
      const saveId = String(args?.saveId ?? "");
      await request<{ deleted: string }>(`/api/saves/${saveId}`, { method: "DELETE" });
      if (localStorage.getItem(ACTIVE_SAVE_KEY) === saveId) {
        setActiveSave(null);
      }
      return true as T;
    }
    case "exit_to_menu":
      setActiveSave(null);
      return undefined as T;
    case "get_settings": {
      const saved = localStorage.getItem(SETTINGS_KEY);
      return (saved ? JSON.parse(saved) : {}) as T;
    }
    case "save_settings":
      localStorage.setItem(SETTINGS_KEY, JSON.stringify(args?.settings ?? {}));
      return undefined as T;
    case "clear_all_saves": {
      const result = await request<{ saves: SaveSummary[] }>("/api/saves");
      await Promise.all(
        result.saves.map((save) =>
          request(`/api/saves/${save.id}`, {
            method: "DELETE",
          }),
        ),
      );
      setActiveSave(null);
      return undefined as T;
    }
    default:
      return request<T>(`/api/saves/${activeSaveId()}/cmd/${command}`, {
        method: "POST",
        body: JSON.stringify(args ?? {}),
      });
  }
}
