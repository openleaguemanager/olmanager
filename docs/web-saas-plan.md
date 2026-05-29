# OLManager → Web SaaS — Plan de arquitectura

> Estado: **propuesta** (no implementado). Documento de decisión para revisar
> antes de invertir tiempo. Última actualización: 2026-05-29.

---

## 1. Objetivo y decisión

Convertir OLManager (hoy app de escritorio Tauri) en un **SaaS web** donde el
usuario se registra, juega su carrera desde el navegador y —objetivo final—
compite con otros usuarios (ligas compartidas, rankings).

**Decisión arquitectónica: motor de juego en el servidor (servidor autoritativo).**

No es una preferencia, es una consecuencia de dos requisitos del producto:

1. **Competitivo/social** → para que un ranking o una liga entre usuarios
   signifique algo, el resultado de cada partido debe ser indiscutible. Si el
   motor corriera en el navegador (WASM), el cliente podría falsear resultados.
   El servidor **debe simular y ser la fuente de verdad**.
2. **Solo web** (se abandona Tauri) → una sola arquitectura, sin mantener la
   doble ruta IPC/HTTP.

Esto descarta el enfoque "motor en WASM, servidor fino". El motor vive en el
servidor.

---

## 2. Arquitectura actual (lo que ya tenemos)

```
┌─────────────────────────────────────────────┐
│ Tauri app (un binario nativo por usuario)    │
│                                              │
│  React frontend  ──invoke()──►  Rust backend │
│  (src/)            83 cmds      (src-tauri/)  │
│                                              │
│                          StateManager        │
│                          └─ Session (1)       │
│                             ├─ game: Game      │
│                             ├─ stats           │
│                             ├─ live_match      │
│                             └─ save_id         │
│                                              │
│                   crates puros (portables):  │
│                   ofm_core · engine · domain │
│                   db (rusqlite/SQLite local) │
└─────────────────────────────────────────────┘
   Saves: ficheros .db SQLite en disco local
```

**Lo bueno (muy reutilizable):**

- `ofm_core`, `engine`, `domain`: **cero dependencias de Tauri**. Rust puro.
- El motor **serializa la partida completa** a SQLite vía el crate `db`.
- Los comandos siguen el patrón **cargar → mutar → guardar** sobre un `Session`.
- El frontend ya es React/Vite (reutilizable casi tal cual).

**Lo que hay que cambiar:**

- `StateManager` mantiene **un solo `Session`** bajo `Mutex` → modelo
  mono-usuario. En web hay miles de usuarios y partidas.
- Persistencia **SQLite local** → nube (Postgres / blob storage).
- 83 `invoke()` de Tauri → endpoints HTTP/WebSocket.
- Auth: no existe. Hay que añadirla.

---

## 3. Arquitectura objetivo

```
┌────────────┐     HTTPS/WSS      ┌──────────────────────┐
│  Navegador │ ◄──────────────►   │  Game API (Rust axum) │
│            │                    │                      │
│  React app │   fetch / ws       │  reusa crates:       │
│  (Vercel)  │                    │  ofm_core·engine·... │
│            │                    │                      │
│  auth ─────┼──► Supabase Auth   │  por petición:       │
└────────────┘                    │   load Session(DB)   │
                                  │   → run command       │
       ┌──────────────┐           │   → persist Session   │
       │  Postgres     │ ◄────────┤  (Fly.io / Railway)  │
       │  (Supabase)   │  saves    └──────────────────────┘
       │  users, saves │
       │  shared leagues (fase 3) │
       └──────────────┘
```

**Componentes:**

| Pieza        | Tecnología                    | Por qué                                   |
| ------------ | ----------------------------- | ----------------------------------------- |
| Frontend     | React actual → Vercel         | Ya existe; solo cambia la capa de datos   |
| Auth + DB    | Supabase (Postgres + Auth)    | Ya se usa en OLMDBManager; gratis para empezar |
| Game server  | Rust **axum**                 | Reusa los crates; mismo lenguaje que el motor |
| Hosting API  | Fly.io o Railway              | Vercel no sirve para un servidor Rust con cómputo |
| Saves        | Postgres (blob) o Supabase Storage | Mínimo refactor del crate `db`        |

---

## 4. La costura central: `Session` → sesiones por-petición

Hoy:

```rust
// Un único Session global bajo Mutex (un usuario, una partida)
pub struct StateManager { session: Mutex<Session> }
```

En el servidor, el `Session` deja de ser global. Cada request trae
`user_id` (del token de auth) + `save_id`, y el flujo es:

```
1. Autenticar request → user_id
2. Cargar el blob de la partida (user_id, save_id) desde Postgres
3. Deserializar a `Game` / `Session`
4. Ejecutar el comando (misma función del motor que hoy)
5. Re-serializar y guardar el blob
6. Devolver el estado/resultado al frontend
```

Esto reutiliza **toda** la lógica de los comandos. El cambio es de *dónde*
vive el `Session` (memoria global → DB por usuario), no de *qué hace* el motor.

> Optimización futura: cachear sesiones activas en memoria (Redis o in-process)
> para partidas en curso y evitar serializar en cada acción. MVP no lo necesita.

---

## 5. Migración de persistencia (mínimo esfuerzo)

El motor ya sabe serializar la partida a SQLite. **No reescribimos el crate
`db` a Postgres** (sería semanas). En su lugar:

- **Opción elegida (MVP): blob.** Guardamos la partida serializada (el fichero
  SQLite como bytes, o el `Game` en JSON/bincode) como un campo `BYTEA` en una
  fila de Postgres `saves(user_id, save_id, data, updated_at)`. El servidor
  rehidrata SQLite en memoria (`:memory:`) o en un tempfile por petición.
  - ✅ Reutiliza el crate `db` entero.
  - ✅ Refactor mínimo.
  - ⚠️ No permite consultas SQL sobre el contenido de la partida desde Postgres
    (no las necesitamos en MVP).

- **Opción futura (fase 3, social):** esquema Postgres real para las **ligas
  compartidas** (ver §8), donde sí necesitas consultar standings/resultados
  cross-usuario. Las partidas individuales pueden seguir como blob.

---

## 6. Los 83 comandos, por categoría

Para portar `invoke` → HTTP. La mayoría son mutaciones simples sobre el
`Session`; unos pocos son de larga duración (simulación de partido en vivo) y
piden WebSocket.

| Categoría             | Ejemplos                                                      | Transporte |
| --------------------- | ------------------------------------------------------------- | ---------- |
| Ciclo de partida      | `start_new_game`, `select_team`, `load_game`, `save_game`, `get_active_game` | HTTP |
| Lectura/consulta      | `get_saves`, `get_*_stats_overview`, `get_*_match_history`, `get_scrim_context` | HTTP GET |
| Mutaciones de gestión | `set_training`, `set_lol_tactics`, `make_transfer_bid`, `hire_staff`, `set_starting_xi` … (la mayoría) | HTTP POST |
| Avance de tiempo      | `advance_time`, `skip_to_match_day`, `advance_to_next_season` | HTTP POST (puede tardar) |
| Partido en vivo       | `start_live_match`, `step_live_match`, `finish_live_match`, `get_match_snapshot` | **WebSocket** (streaming por tick) |
| Inbox/social          | `mark_message_read`, `resolve_message_action`, `delete_message` | HTTP POST |
| Mundo/admin           | `export_world_database`, `load_world_editor_database` | HTTP (o quitar en web) |

**Estrategia de port:** crear una capa `apiClient.ts` en el frontend con la
misma firma que `invoke(cmd, args)` pero que haga `fetch`. Así el resto del
frontend casi no cambia. Un mapa `cmd → endpoint`.

---

## 7. Modelo de datos (Postgres / Supabase)

### Fase 1-2 (carrera individual)

```sql
-- Provisto por Supabase Auth
auth.users (id, email, ...)

profiles (
  user_id      uuid primary key references auth.users,
  display_name text,
  created_at   timestamptz
)

saves (
  id          uuid primary key,
  user_id     uuid references auth.users,
  name        text,
  data        bytea,         -- partida serializada (blob)
  manager     text,          -- denormalizado para listar sin deserializar
  updated_at  timestamptz,
  created_at  timestamptz
)
-- RLS: un usuario solo ve/edita sus propios saves
```

### Fase 3 (social/competitivo) — el gran salto

Aquí cambia el modelo: ya no es "una partida aislada por usuario" sino
**mundos compartidos** donde varios usuarios gestionan equipos en la misma liga.

```sql
worlds (              -- un universo compartido (temporada online)
  id, name, season, status, tick_schedule, created_at
)

world_members (       -- qué usuario controla qué equipo en ese mundo
  world_id, user_id, team_id, joined_at
)

world_state (         -- estado del mundo simulado por el servidor
  world_id, data bytea, current_date, updated_at
)

rankings (            -- materializado para leaderboards
  world_id, user_id, team_id, points, wins, losses, rank
)
```

El servidor corre un **scheduler** que avanza el tiempo de cada mundo (p. ej.
los partidos se juegan a una hora fija), simula con el motor existente, y
actualiza standings. Las decisiones de cada usuario (alineación, tácticas) se
aplican antes del tick. **Esto es esencialmente un juego nuevo encima del
motor** y merece su propio documento de diseño.

---

## 8. Roadmap por fases

| Fase | Entregable | Esfuerzo (solo dev) | Riesgo |
| ---- | ---------- | ------------------- | ------ |
| **0. Spike servidor** | axum envolviendo el motor; endpoints `new_game` + `get_active_game` probados con curl; reutiliza crates | ~1 sem | Bajo |
| **1. SaaS skeleton** | Supabase Auth, registro/login, tabla `saves`, guardar/cargar partida en la nube, RLS | ~2-3 sem | Medio |
| **2. Port de comandos** | capa `apiClient` en frontend (invoke→fetch), todos los comandos HTTP, partido en vivo por WebSocket, deploy frontend Vercel + API Fly | ~3-4 sem | Medio |
| **3. Social/competitivo** | mundos compartidos, scheduler de ticks, rankings, matchmaking | **meses** | Alto |

La fase 3 es, con diferencia, la más grande: cambia el modelo de datos y añade
simulación programada server-side. Recomendado abordarla solo cuando 0-2 estén
estables y haya usuarios reales jugando carreras individuales.

---

## 9. Recomendaciones de proceso

1. **Rama/repos separados.** No mezclar con `feat/ui-v2`. Sugerencia:
   - `olmanager-server` (nuevo crate binario axum, puede vivir en el mismo
     workspace `src-tauri/` reusando los crates, o en repo aparte).
   - El frontend se queda en este repo; se añade el `apiClient` detrás de un
     flag para poder seguir usando Tauri durante la transición.
2. **Terminar/estabilizar la UI v2 primero.** La web reutiliza ese mismo
   frontend; el trabajo de UI es prerequisito, no se tira.
3. **Fase 0 como prueba de fuego.** Si el motor corre como API en una semana,
   el resto es repetición mecánica. Si aparecen bloqueos (estado global,
   concurrencia), los detectamos barato.

---

## 10. Riesgos y preguntas abiertas

- **Concurrencia del motor**: ¿el motor asume single-thread / estado global en
  algún sitio más allá de `StateManager`? Hay que auditarlo antes de la fase 2.
- **Coste de cómputo**: `advance_time` y la simulación de partidos son lo caro.
  Con muchos usuarios concurrentes hay que medir y posiblemente encolar.
- **Tamaño del blob de partida**: los saves tienen cientos de jugadores +
  historial. Medir el peso serializado y si conviene comprimir.
- **Anti-cheat en fase 3**: el servidor autoritativo lo resuelve, pero hay que
  asegurar que ninguna decisión de gameplay se calcule en cliente.
- **Migración de saves de escritorio**: ¿se importan los `.db` locales de
  usuarios actuales? Probablemente sí como "subir partida".
- **Tiempo real del partido en vivo**: ¿streaming tick-a-tick por WS o se
  resuelve el partido entero server-side y se reproduce en cliente? Decisión de
  fase 2.

---

## 11. Stack final propuesto (resumen)

```
Frontend:   React + Vite (actual) → Vercel
Auth:       Supabase Auth
DB:         Supabase Postgres (saves como blob; esquema real para fase 3)
Game API:   Rust axum reusando ofm_core/engine/domain/db → Fly.io o Railway
Realtime:   WebSocket para partido en vivo
```

---

## 12. Siguiente paso

Cuando se apruebe este plan, el primer trabajo es la **Fase 0**: un crate
binario `olmanager-server` con axum que exponga `new_game` y `get_active_game`
reutilizando el motor, probado con `curl`. Eso valida la hipótesis central
(el motor corre como servicio) antes de construir auth, frontend o nube.
