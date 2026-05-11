# LoL Sim V3 Plan

`lol_sim_v3` va a ser una reimplementación paralela y más chica del simulador live. La meta no es simular LoL de forma física y perfecta; la meta es generar una partida creíble, balanceable y fácil de renderizar en el minimapa.

## Decisión base

Vamos a construir V3 con:

- **Fixed timestep** para que la simulación sea determinista y testeable.
- **Máquina de estados por agente** para que cada campeón tenga comportamiento explícito.
- **Intentions antes de resolución** para separar decisión de consecuencias.
- **Event Queue tipada** para que el frontend pueda animar acciones sin entender lógica interna.
- **Snapshots livianos** para sincronizar el estado visible cada ciertos ticks.

## Principios

1. **Semántica antes que física**: el motor decide jugadas de LoL, no micro-movimientos perfectos.
2. **Contrato primero**: Rust y frontend se comunican con eventos y snapshots bien definidos.
3. **Legibilidad**: si no podemos explicar por qué pasó un evento, el diseño está mal.
4. **Determinismo**: mismo seed + mismas entradas = mismo resultado.
5. **Migración segura**: V3 convive con `lol_sim_v2` hasta que pueda reemplazarlo.

## Roadmap detallado

### ~~Paso 1 — Definir el contrato V3~~ ✅

Crear los tipos públicos que el frontend va a consumir.

- `LolSimV3InitRequest`
- `LolSimV3TickRequest`
- `LolSimV3TickResponse`
- `LolSimV3Snapshot`
- `LolSimV3Event`
- `LolSimV3UnitView`

El objetivo es que el frontend no dependa del estado interno completo del motor.

### ~~Paso 2 — Crear el estado interno mínimo~~ ✅

Modelar solo lo necesario para una partida creíble inicial.

- Tiempo de partida.
- Seed/RNG determinista.
- Equipos blue/red.
- 10 campeones.
- Torres principales.
- Dragón simple.
- Nexus.
- Score básico: kills, gold, towers, dragons.

No incluir todavía visión compleja, items avanzados, habilidades específicas ni pathing sofisticado.

### ~~Paso 3 — Implementar agentes con estados explícitos~~ ✅

Cada campeón debe tener un estado claro.

Estados iniciales sugeridos:

- `Laning`
- `Pushing`
- `Roaming`
- `ObjectiveSetup`
- `Fighting`
- `Recalling`
- `Dead`

Cada estado debe responder dos preguntas:

1. ¿Qué intención produce este agente?
2. ¿Cuándo cambia a otro estado?

### ~~Paso 4 — Crear el sistema de intentions~~ ✅

Los agentes no modifican el mundo directamente. Primero emiten intenciones.

Ejemplos:

- `FarmLane`
- `TradeWithEnemy`
- `RotateToObjective`
- `TakeDragon`
- `PushTower`
- `Recall`
- `DefendBase`

Esto evita que cada agente haga cambios desordenados sobre el estado global.

### ~~Paso 5 — Resolver intentions por sistemas~~ ✅

Los sistemas toman las intenciones y producen consecuencias.

Sistemas iniciales:

- `LaneSystem`: presión de línea, farm, push.
- `CombatSystem`: trades, kills, bajas, respawn.
- `ObjectiveSystem`: dragón, torres, nexus.
- `EconomySystem`: gold, recompensas, ventaja.
- `MovementSystem`: posiciones renderizables en minimapa.

La regla importante: los sistemas producen eventos tipados, no texto libre como fuente principal.

### ~~Paso 6 — Diseñar la Event Queue~~ ✅

Eventos iniciales:

- `UnitMoved`
- `AgentStateChanged`
- `TradeStarted`
- `DamageApplied`
- `ChampionKilled`
- `TowerDestroyed`
- `DragonTaken`
- `NexusDestroyed`
- `GoldChanged`

Cada evento debe tener:

- `id`
- `t`
- `kind`
- payload tipado

El frontend puede usar estos eventos para animar el minimapa y alimentar el HUD.

### ~~Paso 7 — Generar snapshots livianos~~ ✅

El snapshot es una vista pública, no el estado interno.

Debe incluir:

- Tiempo actual.
- Winner, si existe.
- Unidades visibles para render.
- Estructuras visibles.
- Objetivos visibles.
- Stats del marcador.

El frontend debería poder renderizar con `snapshot + events`, sin conocer reglas internas.

### ~~Paso 8 — Crear endpoints Tauri paralelos~~ ✅

Agregar comandos nuevos sin tocar todavía V2:

- `lol_sim_v3_init`
- `lol_sim_v3_tick`
- `lol_sim_v3_reset`
- `lol_sim_v3_dispose`
- `lol_sim_v3_run_to_completion`

Esto permite probar V3 detrás de un flag.

### ~~Paso 9 — Adaptar el frontend detrás de un flag~~ ✅

Crear cliente TypeScript para V3 y mantener V2 como fallback.

- `LolSimV3Client`
- contrato TS equivalente al contrato Rust
- flag local para elegir `v2` o `v3`

Primero hay que renderizar la misma pantalla live con menos features, no más.

### ~~Paso 10 — Testear determinismo y reglas base~~ ✅

Tests mínimos obligatorios:

- mismo seed produce mismo ganador
- no hay eventos con tiempo fuera de orden
- una partida siempre termina antes del máximo de ticks
- kills actualizan stats
- destruir nexus define winner
- snapshots no exponen estado interno innecesario

### Paso 11 — Migrar features gradualmente (en progreso) ⏭️

Estado de migración actual:

- ✅ 1. Lanes + farm + presión (base inicial integrada en systems)
- ✅ 2. Kills + respawns (respawn timer básico integrado)
- ✅ 3. Torres + nexus (daño estructural y cierre por nexus)
- ✅ 4. Dragón (captura y respawn de objetivo)
- ✅ 5. Baron (captura + recompensa + evento)
- ✅ 6. Recalls (retorno a base + curación + cambio de estado)
- ✅ 7. Roams (rotación con impacto de escaramuza)
- ✅ 8. Visión simple (control por proximidad para dragón/baron)
- ✅ 9. Tácticas del manager (agresión/objetivo/seguridad impactan prioridad)
- ✅ 10. Diferencias por campeones/arquetipos (modificadores de presión/daño/objetivo/roam)
- ✅ 11. Layout completo de estructuras importado desde V2 (torres por lane, inhibidores, torres de nexus y nexus)
- ✅ 12. Identidad visible de campeones (`championByPlayerId` + fallback determinista para iconos)
- ✅ 13. Pathing base por lane usando anchors derivados de los paths V2, sin colapsar todo a mid
- ✅ 14. Gating semántico de estructuras: outer → inner → inhib tower → inhib → nexus towers → nexus
- ✅ 15. Slice crítico V2→V3: init live pasa `championByPlayerId`, los iconos prefieren `unit.championId`, bot lane conserva sus torres en snapshot y `PushTower` solo daña estructuras enemigas si el campeón está cerca de la siguiente estructura válida.
- ✅ 16. Cadena de identidad end-to-end reforzada: draft→init V3→runtime/snapshot→adapter V3→paneles/render con mapeo determinista por slot (`blue-top`, `blue-jgl`, etc.) y fallback solo de último recurso (sin fuzzy matching primario).
- ✅ 17. Minions/waves baseline V2→V3: contrato público mínimo (`id/team/lane/kind/alive/hpRatio/pos`), spawn periódico por lane para ambos equipos, pathing por anchors de lane y adapter V3→V1 para render en live map sin romper la cadena de identidad de campeones
- ✅ 18. Paso 3 (presión real minion↔tower): daño estructural semántico condicionado por wave aliada avanzada en lane objetivo, con aporte menor de campeones cercanos y sin permitir daño a estructuras propias.
- ✅ 19. Paso 4 (tower aggro/shots): loop semántico de disparos por cadencia, targeting determinista con prioridad minions→champions en el mismo corredor de lane, daño periódico sin proyectiles y eventos `damage_applied`/`champion_killed` reutilizados.
- ✅ 20. Slice minion↔minion en lane: choque semántico de waves por corredor con cadencia determinista, daño recíproco y limpieza de línea para que el push dependa de fuerza de oleada.
- ✅ 21. Slice minion -> structure DPS: loop determinista de daño directo de minions sobre la siguiente estructura enemiga válida por lane (torre/inhibidor/nexus), con gate por progreso/proximidad y preservando coherencia de `damage_applied`/`tower_destroyed`/`nexus_destroyed`.
- ✅ 22. Slice tower dive/retarget/reset semántico: memoria corta de target por torre para evitar thrash, retarget a campeón agresor reciente en corredor sin escudo de minions, y reset determinista cuando el target muere/sale de corredor.
- ✅ 23. Slice neutrals/jungle timers: modelo liviano de camps neutrales por lado, spawns deterministas de dragon/baron por timer explícito, despawn/respawn semántico tras take y señal de economía/eventos coherente para objetivos y camps.
- ✅ 24. Slice de eventos V3 para UI: spawn de waves/camps en eventos concisos, eventos neutrales con metadata estable (`team/lane/key/counts/source`), daño/destrucción de torres con señal semántica y throttling anti-flood, y mapeo live V3→UI compatible con eventos legacy.
- ✅ 25. Slice stats/profiles/balance: perfil liviano por campeón (baseline por rol + influencia por championId), multiplicadores deterministas y acotados para combate/durabilidad/push/roam-objetivos, integración en sistemas core V3 y tests de dirección/determinismo/no-regresión.
- ✅ 26. Slice parity-fine (telemetría + anti-snowball + overlay mapping): snapshot público con campos aditivos (`lanePressure`, `towerTargets`, `neutralTimers`), tuning determinista anti-snowball para DPS minion→estructura con rampa por tiempo/wave + contexto defensor, y metadata de eventos endurecida para overlays avanzados sin romper feed legacy.
- ✅ 27. Slice contrato aditivo/determinista (fase+rol/lane+objetivos): snapshot ahora expone `phaseContributions`, `roleLaneContributions`, `objectivePressureSummary`; metadata de familias mayores de eventos se normaliza con defaults consistentes (`v/key/overlayType/source/importance`) y frontend live aplica fallback seguro cuando falte metadata.

Orden recomendado:

1. Lanes + farm + presión.
2. Kills + respawns.
3. Torres + nexus.
4. Dragón.
5. Baron.
6. Recalls.
7. Roams.
8. Visión simple.
9. Tácticas del manager.
10. Diferencias por campeones/arquetipos.

No migrar todo de V2 de golpe. Primero lograr una partida simple que se vea bien. El baseline actual ya prioriza lanes, estructuras, identidad visual y daño estructural semántico con proximidad; minions y waves quedan como el siguiente slice si el contrato público lo justifica.

Backlog restante tras este slice:

- Gap residual: scoreboard todavía no refleja economía/tempo por rol como series temporales; hoy la telemetría aditiva vive en snapshot por tick.
- Gap residual: falta validación visual end-to-end de overlays externos (consumiendo metadata normalizada) y guía de adopción para clientes legacy.

## Qué queda fuera al principio

- Habilidades específicas por campeón.
- Pathfinding complejo.
- Visión granular tipo LoL real.
- Items detallados.
- Micro-combate frame a frame.
- Simulación física de minions. TODO: agregar scaffolding público de minions/waves en un slice separado, manteniendo contratos livianos.

Estas cosas pueden venir después si aportan diversión real al juego.

## Estructura esperada

```txt
lol_sim_v3/
  README.md
  api.rs
  engine.rs
  state.rs
  agents.rs
  intentions.rs
  systems.rs
  events.rs
  snapshot.rs
  tests.rs
```

## Criterio de éxito

V3 está listo para reemplazar V2 cuando:

- Una partida completa puede correr live y en fast-forward.
- El minimapa puede animarse desde eventos tipados.
- Los resultados son deterministas por seed.
- El código permite agregar una nueva regla sin tocar cinco módulos no relacionados.
- El jugador puede entender por qué ganó o perdió.

## Estado actual

- ~~Paso 1 completado~~
- ~~Paso 2 completado~~
- ~~Paso 3 completado~~
- ~~Paso 4 completado~~
- ~~Paso 5 completado~~
- ~~Paso 6 completado~~
- ~~Paso 7 completado~~
- ~~Paso 8 completado~~
- ~~Paso 9 completado~~
- ~~Paso 10 completado~~
- **Paso 11 en progreso: migración gradual base importada desde V2 para mapa, estructuras, pathing e identidad visual. Slice de stats/profiles/balance completado; próximo foco sugerido: paridad de telemetría por fase/rol y tuning anti-snowball minion↔estructura.**
