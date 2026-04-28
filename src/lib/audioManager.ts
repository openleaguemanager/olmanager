/**
 * Centralized audio manager for OLManager using the Web Audio API.
 *
 * This provides zero-latency, hardware-accelerated audio playback that
 * is far more reliable on mobile / Android than HTMLAudioElement.
 *
 * Architecture:
 * - One shared AudioContext (lazy-created on first user interaction).
 * - One master GainNode for global volume.
 * - AudioBuffers are decoded once and reused forever.
 * - Each play() spawns a fresh AudioBufferSourceNode (required by the spec).
 */
import { useSettingsStore } from "../store/settingsStore";

/** Decoded audio data, keyed by file path. */
const AUDIO_BUFFERS = new Map<string, AudioBuffer>();

/** In-flight fetch promises so we don't double-request the same file. */
const FETCH_PROMISES = new Map<string, Promise<void>>();

/** Master gain that every sound routes through. */
let masterGain: GainNode | null = null;

/** Shared AudioContext. Lazy-created because browsers require a user
 *  gesture before the context can leave the "suspended" state. */
let audioCtx: AudioContext | null = null;

function getAudioContext(): AudioContext {
  if (audioCtx) return audioCtx;

  const AC = (window as any).AudioContext || (window as any).webkitAudioContext;
  const ctx = new AC();
  audioCtx = ctx;

  const gain = ctx.createGain();
  gain.connect(ctx.destination);

  // Start with the saved master volume
  const settings = useSettingsStore.getState().settings;
  gain.gain.value = settings.master_volume;

  masterGain = gain;
  return ctx;
}

function getMasterGain(): GainNode {
  getAudioContext();
  return masterGain!;
}

function getPrefersReducedMotion(): boolean {
  if (typeof window === "undefined") return false;
  return window.matchMedia("(prefers-reduced-motion: reduce)").matches;
}

/**
 * Ensure the AudioContext is running. Browsers suspend it until the
 * first user interaction, so we call this before every play().
 */
async function resumeContextIfNeeded(): Promise<void> {
  const ctx = getAudioContext();
  if (ctx.state === "suspended") {
    try {
      await ctx.resume();
    } catch {
      // Ignore – playback will simply be silent until the user interacts.
    }
  }
}

export interface PlayOptions {
  /** Volume of this specific sound, 0–1. Final volume = master * local. */
  volume?: number;
  /** Force loop (useful for music). Default false. */
  loop?: boolean;
  /** If true, the sound is treated as music and obeys `music_enabled`. */
  isMusic?: boolean;
}

export interface ManagedAudioNode {
  /** Stop the sound immediately. */
  stop(): void;
  /** Underlying Web Audio source node (advanced use). */
  source: AudioBufferSourceNode;
}

/**
 * Pre-load an audio file via fetch + decode so it's ready for instant
 * playback. Safe to call multiple times for the same src.
 */
export function preloadAudio(src: string): Promise<void> {
  // Already decoded?
  if (AUDIO_BUFFERS.has(src)) return Promise.resolve();

  // Already fetching?
  const existing = FETCH_PROMISES.get(src);
  if (existing) return existing;

  const promise = (async () => {
    try {
      const response = await fetch(src);
      if (!response.ok) {
        console.warn("[AudioManager] Failed to fetch:", src, response.status);
        return;
      }
      const arrayBuffer = await response.arrayBuffer();
      const ctx = getAudioContext();
      const audioBuffer = await ctx.decodeAudioData(arrayBuffer);
      AUDIO_BUFFERS.set(src, audioBuffer);
    } catch (err) {
      console.warn("[AudioManager] Preload failed:", src, err);
    } finally {
      FETCH_PROMISES.delete(src);
    }
  })();

  FETCH_PROMISES.set(src, promise);
  return promise;
}

/**
 * Play a one-shot sound effect or start music.
 *
 * @returns A lightweight handle with a `stop()` method, or `null` if
 *          playback was blocked by settings / accessibility.
 */
export function playAudio(
  src: string,
  options: PlayOptions = {},
): ManagedAudioNode | null {
  const { volume = 1, loop = false, isMusic = false } = options;

  const settings = useSettingsStore.getState().settings;

  // Respect user toggles
  if (isMusic && !settings.music_enabled) return null;
  if (!isMusic && !settings.sound_effects_enabled) return null;

  // Respect accessibility
  if (getPrefersReducedMotion()) return null;

  const buffer = AUDIO_BUFFERS.get(src);
  if (!buffer) {
    // Buffer not ready yet – silently skip. The caller should have
    // pre-loaded it; we don't block here.
    console.warn("[AudioManager] Buffer not ready, skipping:", src);
    return null;
  }

  resumeContextIfNeeded();

  const ctx = getAudioContext();
  const master = getMasterGain();

  // Per-sound gain so we can control individual volume
  const soundGain = ctx.createGain();
  soundGain.gain.value = Math.max(0, Math.min(1, volume));

  const source = ctx.createBufferSource();
  source.buffer = buffer;
  source.loop = loop;

  // Chain: source → soundGain → masterGain → destination
  source.connect(soundGain);
  soundGain.connect(master);

  source.start(0);

  const handle: ManagedAudioNode = {
    stop() {
      try {
        source.stop();
      } catch {
        // Already stopped – ignore.
      }
      try {
        source.disconnect();
        soundGain.disconnect();
      } catch {
        // Ignore double-disconnect.
      }
    },
    source,
  };

  // Clean up nodes automatically when the sound finishes
  source.onended = () => {
    try {
      source.disconnect();
      soundGain.disconnect();
    } catch {
      // ignore
    }
  };

  return handle;
}

/**
 * Stop a previously returned audio handle.
 */
export function stopAudio(audio: ManagedAudioNode | null): void {
  if (!audio) return;
  audio.stop();
}

/**
 * Update the master volume in real time.
 */
export function updateMasterVolume(): void {
  if (!masterGain) return;
  const settings = useSettingsStore.getState().settings;
  masterGain.gain.value = settings.master_volume;
}
