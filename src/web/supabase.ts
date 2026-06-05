import { createClient, type SupabaseClient } from "@supabase/supabase-js";

const url = import.meta.env.VITE_SUPABASE_URL as string | undefined;
const key = import.meta.env.VITE_SUPABASE_PUBLISHABLE_KEY as string | undefined;

if (import.meta.env.MODE === "web" && (!url || !key)) {
  console.error("Missing VITE_SUPABASE_URL or VITE_SUPABASE_PUBLISHABLE_KEY");
}

declare global {
  var __olmSupabaseClient: SupabaseClient | undefined;
}

export const supabase =
  globalThis.__olmSupabaseClient ??
  createClient(url ?? "http://localhost", key ?? "web-disabled", {
    auth: {
      persistSession: true,
      autoRefreshToken: true,
    },
  });

globalThis.__olmSupabaseClient = supabase;
