import { useState, useCallback, useRef } from "react";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { openUrl } from "@tauri-apps/plugin-opener";

const RELEASES_URL = "https://github.com/drumst0ck/OLManager/releases/latest";

function isAutoUpdatePlatform(): boolean {
  return navigator.platform.startsWith("Win") || navigator.platform.startsWith("Linux");
}

export type UpdateStatus =
  | "idle"
  | "checking"
  | "upToDate"
  | "available"
  | "downloading"
  | "installing"
  | "error";

export type PlatformUpdateMode = "auto" | "manual";

export interface AutoUpdateState {
  status: UpdateStatus;
  currentVersion: string;
  newVersion: string;
  downloadProgress: number;
  errorMessage: string;
  mode: PlatformUpdateMode;
}

export function useAutoUpdate() {
  const [state, setState] = useState<AutoUpdateState>({
    status: "idle",
    currentVersion: "",
    newVersion: "",
    downloadProgress: 0,
    errorMessage: "",
    mode: isAutoUpdatePlatform() ? "auto" : "manual",
  });

  const updateRef = useRef<Update | null>(null);

  const checkForUpdates = useCallback(async () => {
    setState((prev) => ({ ...prev, status: "checking", errorMessage: "" }));
    try {
      const update = await check();
      if (!update) {
        setState((prev) => ({
          ...prev,
          status: "upToDate",
        }));
        return;
      }
      updateRef.current = update;
      setState((prev) => ({
        ...prev,
        status: "available",
        currentVersion: update.currentVersion,
        newVersion: update.version,
      }));
    } catch (err) {
      setState((prev) => ({
        ...prev,
        status: "error",
        errorMessage: err instanceof Error ? err.message : String(err),
      }));
    }
  }, []);

  const downloadAndInstall = useCallback(async () => {
    const update = updateRef.current;

    if (state.mode === "manual") {
      await openUrl(RELEASES_URL);
      return;
    }

    if (!update) {
      setState((prev) => ({ ...prev, status: "idle" }));
      return;
    }

    setState((prev) => ({ ...prev, status: "downloading", downloadProgress: 0 }));

    try {
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            setState((prev) => ({ ...prev, downloadProgress: 0 }));
            break;
          case "Progress":
            setState((prev) => ({
              ...prev,
              downloadProgress: Math.min(prev.downloadProgress + 2, 95),
            }));
            break;
          case "Finished":
            setState((prev) => ({
              ...prev,
              status: "installing",
              downloadProgress: 100,
            }));
            break;
        }
      });
    } catch (err) {
      setState((prev) => ({
        ...prev,
        status: "error",
        errorMessage: err instanceof Error ? err.message : String(err),
      }));
    }
  }, [state.mode]);

  return {
    ...state,
    checkForUpdates,
    downloadAndInstall,
  };
}