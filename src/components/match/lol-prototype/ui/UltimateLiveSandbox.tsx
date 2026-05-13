import { useEffect, useMemo, useRef, useState } from "react";

import { Button } from "../../../ui";
import type { MatchSnapshot } from "../../types";
import { getWalls } from "../assets/map";
import type { LolSimV1RuntimeState } from "../backend/contract-v1";
import { LolSimV2Client } from "../backend/tauri-client";
import type { SimEvent, Vec2 } from "../engine/types";
import { renderSimulation } from "./render";
import {
  createUltimateSandboxEvent,
  ULTIMATE_SANDBOX_IDENTITIES,
  type UltimateSandboxIdentity,
} from "./ultimateSandbox";
import type { UltimateIdentityEventMetadata } from "./ultimateIdentityVfx";
import { drawUltimateIdentityEvents } from "./ultimateIdentityVfx";

const CASTER_ID = "sandbox-home-mid";
const DEBUG_SPEED = 4;
const SLOW_SPEED = 1;

const ROSTER = [
  { id: "sandbox-home-top", name: "Blue Top", role: "TOP", champion: "Garen" },
  { id: "sandbox-home-jungle", name: "Blue Jungle", role: "JUNGLE", champion: "Vi" },
  { id: CASTER_ID, name: "Ultimate Tester", role: "MID", champion: "Ahri" },
  { id: "sandbox-home-adc", name: "Blue ADC", role: "ADC", champion: "Jinx" },
  { id: "sandbox-home-support", name: "Blue Support", role: "SUPPORT", champion: "Leona" },
  { id: "sandbox-away-top", name: "Red Top", role: "TOP", champion: "Darius" },
  { id: "sandbox-away-jungle", name: "Red Jungle", role: "JUNGLE", champion: "LeeSin" },
  { id: "sandbox-away-mid", name: "Red Mid", role: "MID", champion: "Lux" },
  { id: "sandbox-away-adc", name: "Red ADC", role: "ADC", champion: "Caitlyn" },
  { id: "sandbox-away-support", name: "Red Support", role: "SUPPORT", champion: "Thresh" },
] as const;

function player(id: string, name: string, role: string) {
  return {
    id,
    name,
    role,
    condition: 100,
    fitness: 100,
    mechanics: 78,
    laning: 76,
    teamfighting: 80,
    macro_play: 77,
    consistency: 75,
    shotcalling: 72,
    champion_pool: 82,
    discipline: 74,
    mental_resilience: 78,
    traits: [],
  };
}

function createDebugSnapshot(): MatchSnapshot {
  const home = ROSTER.slice(0, 5).map((slot) => player(slot.id, slot.name, slot.role));
  const away = ROSTER.slice(5).map((slot) => player(slot.id, slot.name, slot.role));
  return {
    phase: "Live",
    current_minute: 0,
    home_score: 0,
    away_score: 0,
    possession: "Home",
    ball_zone: "Mid",
    home_team: { id: "ultimate-sandbox-blue", name: "Blue Debug", formation: "1-3-1", play_style: "balanced", players: home },
    away_team: { id: "ultimate-sandbox-red", name: "Red Debug", formation: "1-3-1", play_style: "balanced", players: away },
    home_bench: [],
    away_bench: [],
    home_possession_pct: 50,
    away_possession_pct: 50,
    events: [],
    home_subs_made: 0,
    away_subs_made: 0,
    max_subs: 0,
    home_roles: { captain: CASTER_ID, shotcaller: CASTER_ID },
    away_roles: { captain: "sandbox-away-mid", shotcaller: "sandbox-away-mid" },
    substitutions: [],
    allows_extra_time: false,
    home_yellows: {},
    away_yellows: {},
    sent_off: [],
  };
}

function championMap(identity: UltimateSandboxIdentity): Record<string, string> {
  return Object.fromEntries(
    ROSTER.map((slot) => [slot.id, slot.id === CASTER_ID ? identity.championKey : slot.champion]),
  );
}

function ultimateProfiles(identity: UltimateSandboxIdentity) {
  return {
    [identity.championKey]: {
      archetype: "burst",
      icon: "",
      signatureId: identity.signatureId,
      visualEventId: `ultimate.${identity.signatureId}`,
    },
  };
}

function fallbackEventOrigin(state: LolSimV1RuntimeState | null): { origin: Vec2; target: Vec2 } | null {
  const caster = state?.champions.find((champion) => champion.id === CASTER_ID);
  const target = state?.champions.find((champion) => champion.team !== caster?.team && champion.alive);
  if (!caster || !target) return null;
  return { origin: caster.pos, target: target.pos };
}

export function extractLatestUltimateEvent(state: LolSimV1RuntimeState | null): SimEvent | null {
  const events = state?.events ?? [];
  for (let index = events.length - 1; index >= 0; index -= 1) {
    const event = events[index] as SimEvent;
    const metadata = event.metadata as { event?: string } | null | undefined;
    if (metadata?.event === "champion_ultimate_cast") return event;
  }
  return null;
}

function formatVec(vec: Vec2 | null | undefined): string {
  if (!vec) return "—";
  return `${vec.x.toFixed(3)}, ${vec.y.toFixed(3)}`;
}

interface Props {
  selectedSignature: string;
  onSelectedSignatureChange: (signature: string) => void;
}

export function UltimateLiveSandbox({ selectedSignature, onSelectedSignatureChange }: Props) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const clientRef = useRef<LolSimV2Client | null>(null);
  const rafRef = useRef<number | null>(null);
  const tickInFlightRef = useRef(false);
  const lastFrameRef = useRef(0);
  const [state, setState] = useState<LolSimV1RuntimeState | null>(null);
  const stateRef = useRef<LolSimV1RuntimeState | null>(null);
  const [running, setRunning] = useState(true);
  const [slowMode, setSlowMode] = useState(false);
  const [status, setStatus] = useState<"loading" | "live" | "fallback">("loading");
  const [forceReason, setForceReason] = useState<string | null>(null);
  const walls = useMemo(() => getWalls(), []);
  const selectedIdentity = useMemo(
    () => ULTIMATE_SANDBOX_IDENTITIES.find((identity) => identity.signatureId === selectedSignature) ?? ULTIMATE_SANDBOX_IDENTITIES[0],
    [selectedSignature],
  );
  const championsByPlayer = useMemo(() => championMap(selectedIdentity), [selectedIdentity]);
  const latestUltimate = useMemo(() => extractLatestUltimateEvent(state), [state]);
  const latestMetadata = (latestUltimate?.metadata ?? null) as UltimateIdentityEventMetadata | null;

  useEffect(() => {
    stateRef.current = state;
  }, [state]);

  useEffect(() => {
    const client = new LolSimV2Client(`ultimate-live-sandbox-${selectedIdentity.signatureId}-${Date.now()}`);
    clientRef.current = client;
    let disposed = false;
    setStatus("loading");
    setState(null);
    setForceReason(null);

    void client
      .init({
        seed: "4242424242",
        aiMode: "hybrid",
        snapshot: createDebugSnapshot(),
        championByPlayerId: championsByPlayer,
        championProfilesById: {},
        championUltimatesById: ultimateProfiles(selectedIdentity),
      })
      .then((response) => {
        if (disposed || clientRef.current !== client) return;
        setState(response.state);
        setStatus("live");
      })
      .catch(() => {
        if (disposed || clientRef.current !== client) return;
        setStatus("fallback");
      });

    return () => {
      disposed = true;
      if (clientRef.current === client) clientRef.current = null;
      void client.dispose().catch(() => undefined);
    };
  }, [championsByPlayer, selectedIdentity]);

  useEffect(() => {
    const draw = (timestamp: number) => {
      const canvas = canvasRef.current;
      if (!canvas) return;
      const rect = canvas.getBoundingClientRect();
      const size = Math.max(320, Math.floor(Math.min(rect.width || 960, 720)));
      if (canvas.width !== size || canvas.height !== size) {
        canvas.width = size;
        canvas.height = size;
      }

      const client = clientRef.current;
      const currentState = stateRef.current;
      if (client && currentState && status === "live" && !tickInFlightRef.current) {
        const last = lastFrameRef.current || timestamp;
        const dt = Math.min(0.05, (timestamp - last) / 1000);
        lastFrameRef.current = timestamp;
        tickInFlightRef.current = true;
        void client
          .tick({ dtSec: dt, running, speed: slowMode ? SLOW_SPEED : DEBUG_SPEED })
          .then((response) => setState(response.state))
          .catch(() => setStatus("fallback"))
          .finally(() => {
            tickInFlightRef.current = false;
          });
      }

      if (currentState && status === "live") {
        renderSimulation(canvas, currentState, walls, championsByPlayer);
      } else {
        const ctx = canvas.getContext("2d");
        if (ctx) {
          ctx.fillStyle = "#0f172a";
          ctx.fillRect(0, 0, canvas.width, canvas.height);
          ctx.fillStyle = "#e2e8f0";
          ctx.font = "16px Inter, sans-serif";
          ctx.fillText("Fallback dummy renderer", 24, 36);
          const fallback = createUltimateSandboxEvent(selectedIdentity, performance.now() / 1000);
          const positions = fallbackEventOrigin(currentState);
          if (positions) {
            const metadata = fallback.metadata as UltimateIdentityEventMetadata;
            metadata.originPos = positions.origin;
            metadata.targetPos = positions.target;
          }
          drawUltimateIdentityEvents(ctx, [fallback], [], fallback.t + 0.5, canvas.width, canvas.height);
        }
      }

      rafRef.current = requestAnimationFrame(draw);
    };

    rafRef.current = requestAnimationFrame(draw);
    return () => {
      if (rafRef.current) cancelAnimationFrame(rafRef.current);
      rafRef.current = null;
      lastFrameRef.current = 0;
    };
  }, [championsByPlayer, running, selectedIdentity, slowMode, status, walls]);

  const forceUltimate = async () => {
    const client = clientRef.current;
    if (!client || status !== "live") {
      setForceReason("Live sim no disponible; usando fallback visual.");
      setStatus("fallback");
      return;
    }

    const response = await client.debugForceUltimate({ casterId: CASTER_ID });
    setState(response.state);
    setForceReason(response.casted ? "Ultimate forzada en backend V2." : response.reason ?? "No se pudo castear.");
  };

  return (
    <div className="rounded-2xl border border-gray-200 dark:border-navy-600 bg-white dark:bg-navy-800 p-4 shadow-xl">
      <div className="mb-4 grid gap-3 lg:grid-cols-[1fr_auto_auto_auto] lg:items-end">
        <label className="block">
          <span className="mb-1 block text-xs font-heading font-bold uppercase tracking-wide text-gray-500 dark:text-gray-400">
            Campeón / signature ({ULTIMATE_SANDBOX_IDENTITIES.length})
          </span>
          <select
            className="w-full rounded-lg border border-gray-300 dark:border-navy-600 bg-gray-50 dark:bg-navy-900 p-3 text-sm outline-none focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20"
            value={selectedSignature}
            onChange={(event) => onSelectedSignatureChange(event.target.value)}
          >
            {ULTIMATE_SANDBOX_IDENTITIES.map((identity) => (
              <option key={identity.signatureId} value={identity.signatureId}>
                {identity.championName} — {identity.signatureId} ({identity.primitive})
              </option>
            ))}
          </select>
        </label>
        <Button onClick={forceUltimate}>Forzar ultimate</Button>
        <Button onClick={() => setRunning((value) => !value)}>{running ? "Pausar" : "Resume"}</Button>
        <Button onClick={() => setSlowMode((value) => !value)}>{slowMode ? "Velocidad normal" : "Slow mode"}</Button>
      </div>

      <div className="grid gap-4 xl:grid-cols-[minmax(0,1fr)_320px]">
        <canvas
          ref={canvasRef}
          width={960}
          height={720}
          aria-label="Ultimate live sim debug canvas"
          className="aspect-square w-full rounded-xl border border-gray-900/10 dark:border-white/10 bg-slate-950"
        />
        <aside className="rounded-xl border border-gray-200 dark:border-navy-600 bg-gray-50 dark:bg-navy-900 p-4 text-sm">
          <p className="font-heading text-xs uppercase tracking-[0.2em] text-primary-500">Live Sim Debug</p>
          <dl className="mt-3 space-y-2">
            <div><dt className="text-gray-500">Backend</dt><dd>V2 {status}</dd></div>
            <div><dt className="text-gray-500">Caster live</dt><dd>{state?.champions.find((champion) => champion.id === CASTER_ID)?.name ?? "—"} / {selectedIdentity.championName}</dd></div>
            <div><dt className="text-gray-500">Último ultimate event</dt><dd>{latestUltimate?.text ?? "—"}</dd></div>
            <div><dt className="text-gray-500">Caster</dt><dd>{latestMetadata?.actorId ?? "—"}</dd></div>
            <div><dt className="text-gray-500">Target</dt><dd>{latestMetadata?.targetId ?? "—"}</dd></div>
            <div><dt className="text-gray-500">Shape</dt><dd>{latestMetadata?.shape ?? "—"}</dd></div>
            <div><dt className="text-gray-500">Origin</dt><dd>{formatVec(latestMetadata?.originPos)}</dd></div>
            <div><dt className="text-gray-500">TargetPos</dt><dd>{formatVec(latestMetadata?.targetPos)}</dd></div>
            <div><dt className="text-gray-500">Direction</dt><dd>{formatVec(latestMetadata?.direction)}</dd></div>
            <div><dt className="text-gray-500">Reason</dt><dd>{forceReason ?? "—"}</dd></div>
          </dl>
        </aside>
      </div>
    </div>
  );
}
