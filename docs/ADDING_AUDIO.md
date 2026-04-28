# Adding Audio to OLManager

This guide explains how to add new sound effects or music to Open League Manager.

## Audio System Architecture

OLManager uses the **Web Audio API** (not HTMLAudioElement) for all audio playback. This provides:

- **Zero-latency playback** — audio buffers are decoded once and reused
- **Hardware acceleration** — especially important on Android/mobile
- **Real-time volume control** — master volume applied via a single `GainNode`
- **No memory leaks** — each sound spawns a lightweight `AudioBufferSourceNode`, not a full DOM element

## File Locations

| Location | Purpose |
|----------|---------|
| `public/sounds/` | Raw audio files (OGG, WAV, MP3) served as static assets |
| `src/lib/audioManager.ts` | Core audio engine — decoding, playback, volume |
| `src/components/GlobalClickSound.tsx` | Global click sound on all buttons |
| `src/components/GlobalNotificationSound.tsx` | Notification sound for new inbox messages |
| `src/App.tsx` | Pre-loads critical sounds on app startup |

## Supported Formats

| Format | Recommended | Notes |
|--------|-------------|-------|
| **OGG** | ✅ Yes | Best compression, fully open, patent-free |
| **WAV** | ✅ Yes | Uncompressed, instant decode, larger files |
| **MP3** | ⚠️ Okay | Patent-encumbered, slightly slower decode |

> **License requirement**: All audio assets must be compatible with GPL-3.0. Preferred sources: CC0, CC-BY, CC-BY-SA, or public domain. See [Audio Licensing](#audio-licensing) below.

## How to Add a New Sound Effect

### 1. Add the audio file

Copy your audio file into `public/sounds/`:

```bash
cp my-sound.ogg public/sounds/my-sound.ogg
```

### 2. Pre-load it

In `src/App.tsx`, add a `preloadAudio()` call so the sound is decoded before it's needed:

```tsx
import { preloadAudio } from "./lib/audioManager";

useEffect(() => {
  preloadAudio("/sounds/click.ogg");
  preloadAudio("/sounds/notification.ogg");
  preloadAudio("/sounds/my-sound.ogg");  // ← add this
}, []);
```

> Pre-loading is **optional but strongly recommended**. Without it, the first playback may be silent while the browser decodes the file.

### 3. Play the sound from your component

```tsx
import { playAudio } from "../lib/audioManager";

function MyComponent() {
  const handleAction = () => {
    playAudio("/sounds/my-sound.ogg", { volume: 0.5 });
    // ... rest of your logic
  };

  return <button onClick={handleAction}>Do Thing</button>;
}
```

### PlayOptions

```ts
playAudio("/sounds/file.ogg", {
  volume: 0.5,   // 0–1, multiplied by the user's master volume
  loop: false,   // set true for background music
  isMusic: false // set true to respect the "Music" toggle in settings
});
```

### Return value

`playAudio()` returns a `ManagedAudioNode | null`:

```ts
const handle = playAudio("/sounds/loop.ogg", { loop: true });

// Later, stop it:
handle?.stop();
```

## How to Add Background Music

Music uses the same API but with `loop: true` and `isMusic: true`:

```tsx
import { playAudio, stopAudio } from "../lib/audioManager";

const musicRef = useRef<ManagedAudioNode | null>(null);

useEffect(() => {
  musicRef.current = playAudio("/sounds/menu-music.ogg", {
    loop: true,
    isMusic: true,
    volume: 0.3,
  });

  return () => {
    stopAudio(musicRef.current);
  };
}, []);
```

Music with `isMusic: true` automatically respects the user's **Music** toggle in Settings.

## Global Sound Effects (Built-in)

Two global sound components are already wired into `App.tsx`:

| Component | Trigger | Sound file |
|-----------|---------|------------|
| `GlobalClickSound` | Any `<button>` click | `public/sounds/click.ogg` |
| `GlobalNotificationSound` | New inbox message | `public/sounds/notification.ogg` |

To disable sound on a specific button, add `data-no-sound`:

```tsx
<button data-no-sound>This button is silent</button>
```

## Audio Settings

The following settings are persisted in `AppSettings` (`src/store/settingsStore.ts`):

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `master_volume` | `number` (0–1) | `0.5` | Global volume multiplier |
| `sound_effects_enabled` | `boolean` | `true` | Master switch for SFX |
| `music_enabled` | `boolean` | `true` | Master switch for music |

When `master_volume` changes, `updateMasterVolume()` in `audioManager.ts` updates the Web Audio `GainNode` in real time.

## Audio Licensing

**All audio assets must be GPL-3.0 compatible.**

### Recommended licenses

| License | Compatible? | Attribution required? |
|---------|-------------|----------------------|
| **CC0** | ✅ Yes | No |
| **CC-BY 4.0** | ✅ Yes | Yes (in `AUDIO_PROVENANCE.md` or similar) |
| **CC-BY-SA 4.0** | ✅ Yes | Yes + ShareAlike |
| **Public Domain** | ✅ Yes | No |
| **CC-BY-NC** | ❌ No | — |
| **Proprietary / All Rights Reserved** | ❌ No | — |

### Recommended sources

- **Kenney.nl** — `public/sounds/click.ogg` and `notification.ogg` come from here (CC0)
- **OpenGameArt.org** — Filter by CC0 / CC-BY / CC-BY-SA / OGA-BY
- **Freesound.org** — Filter by CC0 or CC-BY
- **Incompetech** (Kevin MacLeod) — CC-BY music

### Attribution

If you add audio that requires attribution, add an entry to `docs/AUDIO_PROVENANCE.md` (create the file if it doesn't exist):

```markdown
## CC-BY 3.0

- `"Battle Theme"` by Kevin MacLeod (incompetech.com)
  - Licensed under CC BY 3.0
  - File: `public/sounds/battle-theme.ogg`
```

## Performance Tips

1. **Keep files small** — OGG at 44.1kHz mono is usually < 10KB for a 0.2s click sound
2. **Always pre-load** — Call `preloadAudio()` in `App.tsx` for any sound used in the first 5 seconds
3. **Don't over-use `new Audio()`** — The Web Audio API already handles this via `AudioBufferSourceNode`; you never need to create `new Audio()` manually
4. **Test on Android** — Web Audio API behaves slightly differently on mobile WebViews; verify with `npm run tauri android dev` if possible

## Troubleshooting

| Symptom | Likely cause | Fix |
|---------|-------------|-----|
| Sound doesn't play at all | Buffer not pre-loaded | Add `preloadAudio()` in `App.tsx` |
| Sound plays once then never again | Using a single `HTMLAudioElement` | Use `playAudio()` which creates fresh `AudioBufferSourceNode`s |
| Volume slider has no effect | Not calling `updateMasterVolume()` | Call it when `master_volume` changes |
| Sound only plays after first click | AudioContext suspended | `resumeContextIfNeeded()` is called automatically inside `playAudio()` |
| No sound on Android | Autoplay policy | First user interaction (click/tap) will unlock the context automatically |
