import { useEffect, useRef } from "react";
import { useGameStore } from "../store/gameStore";
import { playAudio, preloadAudio } from "../lib/audioManager";

const NOTIFICATION_SOUND_PATH = "/sounds/notification.ogg";

/**
 * Global sound manager for inbox notifications.
 * Plays a sound whenever a new message arrives, regardless of which page
 * the user is currently viewing (Dashboard, Match, Settings, etc.).
 */
export default function GlobalNotificationSound(): null {
  const gameState = useGameStore((s) => s.gameState);
  const previousMessageCountRef = useRef<number>(0);

  useEffect(() => {
    preloadAudio(NOTIFICATION_SOUND_PATH);
  }, []);

  useEffect(() => {
    if (!gameState) {
      previousMessageCountRef.current = 0;
      return;
    }

    const currentCount = gameState.messages.length;
    const previousCount = previousMessageCountRef.current;

    if (previousCount > 0 && currentCount > previousCount) {
      playAudio(NOTIFICATION_SOUND_PATH, { volume: 0.5 });
    }

    previousMessageCountRef.current = currentCount;
  }, [gameState?.messages.length]);

  return null;
}
