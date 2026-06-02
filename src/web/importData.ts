import { supabase } from "./supabase";

const API_BASE = (import.meta.env.VITE_API_BASE as string | undefined) ?? "";

export interface ImportSummary {
  data_files: number;
  photo_files: number;
  player_count: number;
  team_count: number;
  staff_count: number;
  skipped: number;
}

async function authHeaders(): Promise<Record<string, string>> {
  const { data } = await supabase.auth.getSession();
  const token = data.session?.access_token;
  return token ? { Authorization: `Bearer ${token}` } : {};
}

async function readImportSummary(res: Response): Promise<ImportSummary> {
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

  const body = (await res.json()) as { summary: ImportSummary };
  return body.summary;
}

/** Upload an OLMDBManager export zip to the server for extraction. */
export async function importExportZip(file: File): Promise<ImportSummary> {
  const form = new FormData();
  form.append("file", file);

  const res = await fetch(`${API_BASE}/api/admin/import-export`, {
    method: "POST",
    headers: await authHeaders(),
    body: form,
  });

  return readImportSummary(res);
}

/** Pull the configured public OLMDBManager export URL into this OLManager server. */
export async function autoImportDatabase(): Promise<ImportSummary> {
  const res = await fetch(`${API_BASE}/api/admin/auto-import`, {
    method: "POST",
    headers: await authHeaders(),
  });
  return readImportSummary(res);
}

export async function getCatalogSummary(): Promise<ImportSummary> {
  const res = await fetch(`${API_BASE}/api/admin/catalog-summary`, {
    headers: await authHeaders(),
  });
  return readImportSummary(res);
}
