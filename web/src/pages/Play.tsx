import { useEffect, useMemo, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { api, type GameState } from "../api";

export function Play() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [game, setGame] = useState<GameState | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    if (!id) return;
    api
      .loadSave(id)
      .then((r) => setGame(r.game))
      .catch((e) => setError(e instanceof Error ? e.message : String(e)));
  }, [id]);

  // Has the world been assembled yet? (lightweight games start empty)
  const hasWorld = (game?.teams.length ?? 0) > 0;
  const myTeamId = game?.manager && hasWorld ? findMyTeam(game) : null;

  async function pickTeam(teamId: string) {
    if (!id) return;
    setBusy(true);
    setError(null);
    try {
      const r = await api.selectTeam(id, teamId);
      setGame(r.game);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setBusy(false);
    }
  }

  async function advance() {
    if (!id) return;
    setBusy(true);
    setError(null);
    try {
      const r = await api.advance(id);
      setGame(r.game);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setBusy(false);
    }
  }

  if (error && !game) return <div className="center error">{error}</div>;
  if (!game) return <div className="center muted">Cargando partida…</div>;

  return (
    <div className="container">
      <div className="between" style={{ marginBottom: 20 }}>
        <div>
          <div className="eyebrow">OLManager</div>
          <h1 className="title">{game.manager.nickname || game.manager.first_name}</h1>
        </div>
        <button className="secondary" onClick={() => navigate("/")}>
          ← Mis partidas
        </button>
      </div>

      {error && <div className="error">{error}</div>}

      {!hasWorld ? (
        <TeamPicker busy={busy} onPick={pickTeam} />
      ) : (
        <Dashboard game={game} myTeamId={myTeamId} busy={busy} onAdvance={advance} />
      )}
    </div>
  );
}

function findMyTeam(game: GameState): string | null {
  // The server assigns the manager to a team; we don't yet expose team_id on
  // the manager in the web DTO, so just surface the first team's competition.
  return game.teams[0]?.id ?? null;
}

function TeamPicker({ busy, onPick }: { busy: boolean; onPick: (id: string) => void }) {
  // The lightweight game has no teams yet, so we offer the known LEC roster as
  // entry points. Selecting any one assembles the full world server-side.
  const lecTeams = [
    ["lec-g2-esports", "G2 Esports"],
    ["lec-fnatic", "Fnatic"],
    ["lec-karmine-corp", "Karmine Corp"],
    ["lec-team-vitality", "Team Vitality"],
    ["lec-natus-vincere", "Natus Vincere"],
    ["lec-sk-gaming", "SK Gaming"],
    ["lec-team-heretics-lec", "Team Heretics"],
    ["lec-giantx-lec", "GIANTX"],
  ];
  return (
    <div className="card">
      <div className="eyebrow">Elige tu equipo</div>
      <p className="subtitle">Al elegir se genera el mundo completo (todas las ligas).</p>
      <div className="grid" style={{ gridTemplateColumns: "1fr 1fr 1fr 1fr" }}>
        {lecTeams.map(([tid, name]) => (
          <button key={tid} className="secondary" disabled={busy} onClick={() => onPick(tid)}>
            {name}
          </button>
        ))}
      </div>
    </div>
  );
}

function Dashboard({
  game,
  myTeamId,
  busy,
  onAdvance,
}: {
  game: GameState;
  myTeamId: string | null;
  busy: boolean;
  onAdvance: () => void;
}) {
  const date = new Date(game.clock.current_date).toLocaleDateString();
  const myLeague = useMemo(() => game.leagues[0], [game.leagues]);

  return (
    <div className="stack">
      <div className="card between">
        <div>
          <div className="eyebrow">Fecha</div>
          <div className="title mono" style={{ fontSize: 22 }}>
            {date}
          </div>
          <span className="tag">{game.day_phase}</span>
        </div>
        <button disabled={busy} onClick={onAdvance}>
          {busy ? "…" : "Avanzar día →"}
        </button>
      </div>

      <div className="grid" style={{ gridTemplateColumns: "1fr 1fr" }}>
        <div className="card">
          <div className="eyebrow">Mundo</div>
          <div className="stack" style={{ marginTop: 8 }}>
            <Stat label="Equipos" value={game.teams.length} />
            <Stat label="Jugadores" value={game.players.length} />
            <Stat label="Ligas" value={game.leagues.length} />
          </div>
        </div>

        <div className="card">
          <div className="eyebrow">Tu competición</div>
          {myLeague ? (
            <div className="stack" style={{ marginTop: 8 }}>
              <div style={{ fontWeight: 600 }}>{myLeague.name}</div>
              <span className="tag green">Temporada {myLeague.season}</span>
              {myTeamId && <div className="muted" style={{ fontSize: 13 }}>Tu equipo: {myTeamId}</div>}
            </div>
          ) : (
            <div className="muted">Sin liga.</div>
          )}
        </div>
      </div>
    </div>
  );
}

function Stat({ label, value }: { label: string; value: number | string }) {
  return (
    <div className="between">
      <span className="muted">{label}</span>
      <span className="mono" style={{ fontWeight: 600 }}>
        {value}
      </span>
    </div>
  );
}
