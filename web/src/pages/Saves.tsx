import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { api, type SaveSummary } from "../api";
import { useAuth } from "../auth";

export function Saves() {
  const { signOut } = useAuth();
  const navigate = useNavigate();
  const [saves, setSaves] = useState<SaveSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);

  // New-game form
  const [form, setForm] = useState({
    first_name: "",
    last_name: "",
    nickname: "",
    date_of_birth: "1995-01-01",
    nationality: "ES",
    name: "Mi carrera",
  });

  async function refresh() {
    setError(null);
    try {
      const { saves } = await api.listSaves();
      setSaves(saves);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    refresh();
  }, []);

  async function create(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setCreating(true);
    try {
      const { id } = await api.createSave(form);
      navigate(`/play/${id}`);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setCreating(false);
    }
  }

  async function remove(id: string) {
    if (!confirm("¿Borrar esta partida?")) return;
    try {
      await api.deleteSave(id);
      refresh();
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  }

  return (
    <div className="container">
      <div className="between" style={{ marginBottom: 24 }}>
        <div>
          <div className="eyebrow">OLManager</div>
          <h1 className="title">Tus partidas</h1>
        </div>
        <button className="secondary" onClick={signOut}>
          Salir
        </button>
      </div>

      {error && <div className="error">{error}</div>}

      <div className="grid" style={{ gridTemplateColumns: "1fr 1fr", alignItems: "start" }}>
        {/* Saves list */}
        <div className="stack">
          <div className="eyebrow">Cargar</div>
          {loading ? (
            <div className="muted">Cargando…</div>
          ) : saves.length === 0 ? (
            <div className="muted">No tienes partidas todavía.</div>
          ) : (
            saves.map((s) => (
              <div key={s.id} className="save-item" onClick={() => navigate(`/play/${s.id}`)}>
                <div>
                  <div style={{ fontWeight: 600 }}>{s.name}</div>
                  <div className="muted" style={{ fontSize: 13 }}>
                    {s.manager ?? "—"} · {new Date(s.updated_at).toLocaleString()}
                  </div>
                </div>
                <button
                  className="danger"
                  onClick={(e) => {
                    e.stopPropagation();
                    remove(s.id);
                  }}
                >
                  Borrar
                </button>
              </div>
            ))
          )}
        </div>

        {/* New game */}
        <div className="card">
          <div className="eyebrow">Nueva partida</div>
          <form onSubmit={create} className="stack" style={{ marginTop: 12 }}>
            <label>
              Nombre del manager
              <input
                value={form.first_name}
                onChange={(e) => setForm({ ...form, first_name: e.target.value })}
                required
              />
            </label>
            <label>
              Apellido
              <input
                value={form.last_name}
                onChange={(e) => setForm({ ...form, last_name: e.target.value })}
                required
              />
            </label>
            <label>
              Apodo (opcional)
              <input
                value={form.nickname}
                onChange={(e) => setForm({ ...form, nickname: e.target.value })}
              />
            </label>
            <div className="row">
              <label style={{ flex: 1 }}>
                Nacimiento
                <input
                  type="date"
                  value={form.date_of_birth}
                  onChange={(e) => setForm({ ...form, date_of_birth: e.target.value })}
                />
              </label>
              <label style={{ width: 100 }}>
                País
                <input
                  value={form.nationality}
                  onChange={(e) => setForm({ ...form, nationality: e.target.value })}
                  maxLength={3}
                />
              </label>
            </div>
            <label>
              Nombre de la partida
              <input
                value={form.name}
                onChange={(e) => setForm({ ...form, name: e.target.value })}
              />
            </label>
            <button type="submit" disabled={creating}>
              {creating ? "Creando…" : "Crear y jugar"}
            </button>
          </form>
        </div>
      </div>
    </div>
  );
}
