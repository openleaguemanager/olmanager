import type { ApiClient } from "./types"

let _client: ApiClient | null = null

export async function getApiClient(): Promise<ApiClient> {
  if (_client) return _client

  // Detect Tauri via window.__TAURI__ (more reliable than dynamic import)
  const isTauri = typeof window !== "undefined" && "__TAURI__" in window

  if (isTauri) {
    const { tauriAdapter } = await import("./adapters/tauri.adapter")
    _client = tauriAdapter
  } else {
    const { httpAdapter } = await import("./adapters/http.adapter")
    _client = httpAdapter
  }

  return _client
}

/** Call after getApiClient() has been called once. Throws if not initialized. */
export function getApiClientSync(): ApiClient {
  if (!_client) throw new Error("[ApiClient] No inicializado. Llama a getApiClient() primero.")
  return _client
}
