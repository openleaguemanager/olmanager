import { useEffect, useRef } from "react";
import { playAudio, preloadAudio } from "../lib/audioManager";

const CLICK_SOUND_PATH = "/sounds/click.ogg";

/**
 * Global click sound effect for all interactive buttons.
 *
 * Attaches a document-level click listener in capture phase and plays a
 * subtle sound whenever a `<button>` or `[role="button"]` element is
 * clicked.
 */
export default function GlobalClickSound(): null {
  const lastPlayRef = useRef<number>(0);

  useEffect(() => {
    preloadAudio(CLICK_SOUND_PATH);
  }, []);

  useEffect(() => {
    function isInteractiveButton(target: EventTarget | null): boolean {
      if (!(target instanceof Element)) return false;

      if (target.closest("[data-no-sound]")) return false;

      if (target instanceof HTMLInputElement && target.type === "range") {
        return false;
      }
      if (target.closest('input[type="range"]')) return false;

      const button = target.closest("button, [role='button']");
      if (!button) return false;

      if (button instanceof HTMLButtonElement && button.disabled) {
        return false;
      }

      return true;
    }

    const handleClick = (event: MouseEvent) => {
      if (!isInteractiveButton(event.target)) return;

      const now = Date.now();
      if (now - lastPlayRef.current < 30) return;
      lastPlayRef.current = now;

      playAudio(CLICK_SOUND_PATH, { volume: 0.25 });
    };

    document.addEventListener("click", handleClick, true);
    return () => document.removeEventListener("click", handleClick, true);
  }, []);

  return null;
}
