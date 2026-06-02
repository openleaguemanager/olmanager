export interface DownloadEvent {
  event: string;
  data?: unknown;
}

export interface Update {
  version: string;
  body?: string;
  date?: string | null;
  downloadAndInstall: (onEvent?: (event: DownloadEvent) => void) => Promise<void>;
}

export async function check(): Promise<Update | null> {
  return null;
}
