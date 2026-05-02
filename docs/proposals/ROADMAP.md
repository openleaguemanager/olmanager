# OLManager Roadmap

> Open League Manager — Manager de Esports para League of Legends

[![Discord](https://img.sh.shields.io/discord/placeholder?label=Discord&style=social)](https://discord.gg/placeholder)
[![GitHub Stars](https://img.shields.io/github/stars/placeholder?label=Stars&style=social)](https://github.com/placeholder)

## Visión General

OLManager es un manager de esports para League of Legends diseñado para simular la gestión de equipos en competencias profesionales tipo LEC (League of Legends European Championship). El proyecto transita desde su origen en fútbol (OpenFootManager) hacia un sistema completo de gestión de equipos de esports.

**Objetivo estratégico:** Construir una plataforma modular y extensible que permita a los usuarios gestionar equipos, jugadores, presupuestos, estrategias de juego y estadísticas en un entorno de simulación realista.

---

## Estado Actual

| Métrica | Valor |
|--------|-------|
| **Versión** | 0.1.2 (pre-alpha) |
| **Análisis técnico** | `docs/proposals/analisis.md` — 44 hallazgos documentados |
| **Stack** | React 19 + TypeScript 6.0 + Vite 8 + TailwindCSS 4 + Tauri v2 (Rust) |
| **LOC Frontend** | ~71.500 TS/TSX, 228 componentes |
| **LOC Backend** | ~77.000 Rust, 173 archivos, 4 crates |
| **DB** | SQLite per-save (37 migraciones versionadas) |
| **Tests** | 107 frontend (Vitest) + 125 Rust tests (5 legacy rotos) |
| **i18n** | 7 idiomas configurados |
| **Commits** | Conventional commits |

### ✅ Fase 1 Completada (2026-05-02)

La Fase 1 de hardening y foundation está completa. Ver `docs/proposals/analisis.md` para el análisis técnico original.

| Issue resuelto | PR | Estado |
|---------------|-----|--------|
| Security hardening (path traversal, CSP, capabilities) | #101 | ✅ |
| StateManager unification (4 Mutex → 1 Session) | #101 | ✅ |
| Break god files (avatar.rs extraído a game_setup/) | #101 | ✅ |
| CI/CD audit gates (cargo audit, npm audit, tests blocking) | #101 | ✅ |
| Legacy tests (123 db tests pass, legacy marcados) | #101 | ✅ |
| Input validation (validator + Zod) | #101 | ✅ |
| AppError enum (thiserror + códigos) | #101 | ✅ |
| Architecture docs (ADRs + Mermaid C4) | #101 | ✅ |
| Unwrap audit (production unwraps → expect) | #103 | ✅ |
| Cross-stack types (ts-rs derives en 100+ tipos) | #104 | ✅ |

### Deuda Técnica Remanente (post-Fase 1)

- ⚠️ **Componentes monolíticos frontend**: `ChampionDraft.tsx` (3.149 LOC), `MatchSimulation.tsx` (1.922 LOC)
- ⚠️ **`lol_sim_v2.rs` test compilation**: funciones faltantes (6.281 LOC, pre-existing)
- ⚠️ **JSON-en-TEXT**: modelo de datos en SQLite (6 campos en players)
- ⚠️ **100+ warnings de clippy**: pre-existing en workspace, no blocking en CI
- ⚠️ **19 RustSec advisories**: pre-existing, cargo audit non-blocking

---

## Fases del Roadmap

### ✅ Fase 1: Hardening y Foundation — COMPLETADA (2026-05-02)

**Objetivo:** Endurecer la seguridad, pagar deuda técnica crítica y establecer CI/CD sólido antes de agregar features.

**Prioridad:** 🔴 Alta — **✅ 100% completado**

#### 🎯 Hitos (todos ✅)

- ✅ **Seguridad**: CSP habilitado, path traversal eliminado en avatar endpoints, capabilities restringidas
- ✅ **CI/CD endurecido**: `cargo audit`, `npm audit`, tests bloqueantes en core crates
- ✅ **Tipos cross-stack**: `ts-rs` integrado con derives en 100+ tipos, feature-gated
- ✅ **Tests legacy**: rotos marcados como `#[ignore]` con tracking issues, `continue-on-error` eliminado
- ✅ **StateManager**: unificado en single `Mutex<Session>` con `with_session()`/`with_session_mut()`

#### PRs de Fase 1

| PR | Descripción |
|----|-------------|
| [#101](https://github.com/OpenLeagueManager/OLManager/pull/101) | Principal: security, StateManager, CI/CD, tests, validation, AppError, docs |
| [#102](https://github.com/OpenLeagueManager/OLManager/pull/102) | ts-rs scaffold inicial |
| [#103](https://github.com/OpenLeagueManager/OLManager/pull/103) | Unwrap audit (production → expect) |
| [#104](https://github.com/OpenLeagueManager/OLManager/pull/104) | ts-rs derives en 100+ tipos (completa #93) |

---

### Fase 2: Estabilización, Features Core y Release Beta — Mediano Plazo (v0.3 Beta)

**Objetivo:** Pagar deuda técnica restante de Fase 1, estabilizar simulación, implementar features core de gestión y release beta.

**Prioridad:** 🟡 Media

#### 🎯 Hitos

- [ ] 🔲 **Fase 1 cleanup**: completar items que quedaron pendientes
- [ ] 🔲 **Motor de simulación**: lol_sim_v2 compilando + live_match funcional
- [ ] 🔲 **AppError + i18n**: migración completa de todos los comandos
- [ ] 🔲 **Sistema de temporada completa**: Winter/Spring/Summer/Season Finals
- [ ] 🔲 **Sistema de finanzas**: presupuesto, salarios, transferencias
- [ ] 🔲 **Dashboard de estadísticas del equipo**
- [ ] 🔲 **Release beta**: v0.3.0-beta taggeada y publicada

#### 📋 Tareas

##### 🧹 Fase 1 Cleanup (prioridad: 🔴 alta)

- [ ] **Cross-stack type generation (#93)**: annotar ~58 tipos restantes con `#[derive(TS)]`, generar `bindings.ts`
- [ ] **AppError full migration**: migrar todos los comandos (>50) de `Result<T, String>` a `Result<T, AppError>`
- [ ] **i18n de errores**: frontend mapea errores por `code` en vez de string libre
- [ ] **Input validation expansion**: extender `validator` + Zod a más comandos (transferencias, staff, squad)
- [ ] **`lol_sim_v2` test compilation**: fixear funciones faltantes (`baron_push_target_for_lane`, `pick_combat_target`, etc.)
- [ ] **Pre-existing clippy cleanup**: resolver ~100 warnings heredados en workspace (empezar por `domain`, luego `engine`, luego `ofm_core`)

##### 🏗️ Arquitectura y DX (prioridad: 🟡 media)

- [ ] **`tracing` migration**: reemplazar `log` por `tracing` + `tracing-subscriber` con spans por comando Tauri
- [ ] **Logging config**: `Info` en release, `Debug` opt-in, rotación `KeepN(10)` (50 MB tope)
- [ ] **Modelo de datos**: migrar campos consultables de JSON-en-TEXT a columnas reales (atributos de player: `pace`, `stamina`, etc.)
- [ ] **Índices SQLite**: añadir índices funcionales con `json_extract` donde aún haya JSON
- [ ] **Componentes monolíticos frontend**: romper `ChampionDraft.tsx` (3.149 LOC), `MatchSimulation.tsx` (1.922 LOC) en Container/Presentational
- [ ] **`useEffect` audit**: activar `eslint-plugin-react-hooks/exhaustive-deps: error`, migrar fetch a TanStack Query
- [ ] **Fix `ChampionRuntime` visibility**: warning `private_interfaces` en `lol_sim_v2.rs`
- [ ] **Rust profile tuning**: añadir `[profile.release]` con LTO, strip, panic=abort

##### 🎮 Features Core (prioridad: 🟡 media)

- [ ] **Calendario de temporada**: implementar splits LEC (Winter/Spring/Summer) + Season Finals
  - [ ] Generación de fixtures para Spring y Summer split
  - [ ] Playoffs por split (top 6/8)
  - [ ] Season Finals con Championship Points
  - [ ] UI de calendario en Dashboard
- [ ] **Sistema de finanzas**:
  - [ ] Presupuesto por temporada (salary cap)
  - [ ] Contratos multi-año con incrementos
  - [ ] Renovaciones y cláusulas de rescisión
  - [ ] Patrocinadores con objetivos
- [ ] **Mercado de transferencias**:
  - [ ] Ventana de transferencias (Offseason / Mid-season)
  - [ ] Free agency con negociación
  - [ ] Trades entre equipos
  - [ ] UI de mercado en TransfersTab
- [ ] **Modo espectador**: ver partidos sin interactuar (skip mode existente, pulir visualización)
- [ ] **Dashboard de estadísticas**: visualizaciones de rendimiento del equipo (KDA, gold dif, visión, etc.)
- [ ] **Staff management**: contratar/despedir coaches, scouts, analysts con efectos en gameplay
- [ ] **Documentar API de comandos Tauri**: listado de comandos, params, returns

##### 🧪 Testing (prioridad: 🟢 baja)

- [ ] Añadir **Playwright** smoke tests (5 flujos críticos: crear → avanzar → simular → guardar → recargar)
- [ ] Añadir **`proptest`** para propiedades del motor de simulación

#### Métricas de Éxito

- ✅ Todos los comandos usan `AppError` con códigos i18n
- ✅ `lol_sim_v2` compila y pasa tests
- ✅ Usuario puede completar temporada completa (Winter→Spring→Summer→Season Finals)
- ✅ Sistema de finanzas funcional (presupuesto > 0 después de gastos)
- ✅ Ventana de transferencias operativa
- ✅ Release beta (v0.3.0-beta) taggeada y publicada
- ✅ Logging estructurado con spans por comando

---

### Fase 3: Ecosistema y Distribución — Largo Plazo (v1.0 Stable)

**Objetivo:** Construir ecosistema completo, abrir a comunidad, distribuir con actualizaciones automáticas y alcanzar estabilidad de producción.

**Prioridad:** 🟢 Baja

#### 🎯 Hitos

- [ ] 🔲 Sistema de scouting (buscar jugadores en el mercado)
- [ ] 🔲 Competiciones y rankings multi-temporada
- [ ] 🔲 **`tauri-plugin-updater`** con auto-update y firmas
- [ ] 🔲 **Firma de binarios**: Windows EV + macOS Developer ID + GPG signatures
- [ ] 🔲 **Perfil release optimizado**: LTO, codegen-units=1, strip, panic=abort
- [ ] 🔲 Modo multijugador básico (compartir partidas)
- [ ] 🔲 Primera release estable (v1.0.0)
- [ ] 🔲 Publicación OSS (anuncio oficial)

#### 📋 Tareas

- [ ] Implementar mercado de transferencias
- [ ] Crear sistema de ligas/torneos con estadísticas
- [ ] Añadir otras regiones (LCK, LCS, LPL, PCS, VCS)
- [ ] Configurar `tauri-plugin-updater` con endpoint en GitHub Releases
- [ ] Firmar manifests con minisign/ed25519
- [ ] Firmar Windows con certificado EV (DigiCert/SSL.com)
- [ ] Notarizar macOS con Apple Developer ID
- [ ] Publicar SHA256 de cada artefacto + GPG signature en el tag
- [ ] Configurar `[profile.release]` con LTO, strip, panic=abort
- [ ] Desarrollar API REST pública (opcional)
- [ ] Configurar containerización (Docker para simulación headless)
- [ ] Escribir documentación completa para contribuyentes

#### Métricas de Éxito

- ✅ v1.0.0 publicada con changelog y firmas
- ✅ `tauri-plugin-updater` funcional (auto-update de alpha a stable)
- ✅ Comunidad puede contribuir siguiendo flow issue-first
- ✅ docs/ actualizada para usuarios y desarrolladores

---

## Proceso de Trabajo

### Flujo Issue-First

Siguiendo [`GOVERNANCE.md`](docs/GOVERNANCE.md), el desarrollo sigue este flujo:

```
1. Abrir issue con template → 2. Review de maintainer → 3. Apply label status:approved
4. Crear branch desde development → 5. Abrir PR con type:* label → 6. Merge a development
```

### Labels Utilizados

| Categoría | Labels |
|-----------|--------|
| **Status** | `status:needs-review`, `status:approved` |
| **Type** | `type:feature`, `type:bug`, `type:docs`, `type:chore`, `type:refactor`, `type:test`, `type:release`, `type:security` |

### Ramas

- `main` — Estable, solo releases
- `development` — Integración (default para PRs)
- `type/slug` — Ramas de feature/fix/docs/chore

---

## Métricas de Progreso

### KPIs por Fase

| Fase | KPI Principal | KPI Secundario |
|------|---------------|----------------|
| **Fase 1** | ✅ **Completada**. 9/9 issues, 4 PRs mergeados | CI tests: core crates pasan |
| **Fase 2** | Features core: 6 (season, finances, transfers, sim, dashboard, staff) | Release beta publicada |
| **Fase 3** | v1.0.0 released | Auto-updater funcional |

### Badges de Progreso

```markdown
[![Version](https://img.shields.io/badge/version-0.1.2-blue)](ROADMAP.md)
[![Phase](https://img.shields.io/badge/phase-1-green)](ROADMAP.md)
[![CI Status](https://img.shields.io/github/checks-status/placeholder/development)](actions)
```

---

## Cómo Seguir el Progreso

- **Roadmap (este archivo)** — Estado general y fases
- **`docs/proposals/analisis.md`** — Análisis técnico completo con 44 hallazgos detallados
- **GitHub Issues** — Tareas individuales con labels
- **GitHub Project Board** — Vista kanban del desarrollo
- **GitHub Releases** — Changelogs y downloads
- **Discussions** — Q&A y feedback comunitario

---

## Cómo Contribuir

¡Todas las contribuciones son bienvenidas! Para contribuir:

1. **Revisa issues abiertos** — Busca `status:approved` para trabajo confirmado
2. **Abre un issue** — Usa el template para bugs o features
3. **Espera approval** — Un maintainer revisará y aplicará `status:approved`
4. **Crea tu branch** — Desde `development` con formato `type/slug`
5. **Abre PR** — Linkea el issue, añade un `type:*` label
6. **Pasa CI** — Ensure `frontend-install` y `rust-check` pasan

### Requisitos de PR

- [ ] Branch desde `development`
- [ ] Issue linkeado con label `status:approved`
- [ ] Exactly uno `type:*` label
- [ ] Commits conventional
- [ ] Checks: `frontend-install` + `rust-check`

### Configuración Local

```bash
# Frontend
npm install
npm run dev

# Backend (Rust)
cargo build --workspace
cargo test --workspace

# full CI
npm run test
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

---

## Historial de Versiones

| Versión | Fecha | Notas |
|---------|-------|-------|
| 0.1.2 | 2026-05-02 | Pre-alpha actual. **Fase 1 completada** (9/9 issues) |
| 0.2.0-alpha | ⏳ Pendiente | Alpha con Phase 1 cleanup y Fase 2 features |
| 0.3.0-beta | ⏳ Pendiente | Beta con features core + release |
| 1.0.0 | ⏳ Pendiente | Primera stable con auto-updater |

---

*Última actualización: 2026-05-02 — Roadmap actualizado tras análisis técnico arquitectónico (`docs/proposals/analisis.md`)*
