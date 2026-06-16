# Guía del Sistema de Social Tweets en OLManager

## ¿Qué es el sistema de social tweets?

El sistema de social tweets genera mensajes automáticamente en la pestaña de Social del juego, después de cada partido en vivo. Estos mensajes simulan un feed de Twitter/X con diferentes perspectivas: equipo, fan, analista, jugador.

## Dónde se configura

### Archivos de datos
- **`data/social/templates.json`** → Templates de partidos (mensajes que se generan por partido)
- **`data/social/accounts.json`** → Cuentas sociales (fans, analistas, etc.)
- **`data/social/match_texts.json`** → Textos predefinidos para el equipo perdedor
- **`public/social-avatars/*.webp`** → Avatares de cuentas sociales

### Archivos del backend
- **`src-tauri/crates/olm_core/src/social.rs`** → Lógica de generación
- **`src-tauri/crates/olm_core/src/social_templates.rs`** → Sistema de templates y condiciones
- **`src-tauri/crates/olm_core/src/social_registry.rs`** → Cuentas por defecto

## Estructura de un template

```json
{
  "id": "team-banter-es-1",
  "language": "es",
  "slot": "TeamBanter",
  "weight": 5,
  "author_id": null,
  "conditions_json": null,
  "variants": [
    "GG {loser_short_name}. Buen partido y seguimos sumando. {score}",
    "Victoria importante. Gracias a todos los que nos apoyaron hoy. #Vamos{winner_short_name}",
    "Cerramos la serie con calma, buen macro y mucha confianza. {score}"
  ],
  "tags": ["match", "team", "global"],
  "active": true
}
```

### Campos
| Campo | Descripción | Ejemplo |
|-------|-------------|---------|
| `id` | Identificador único | `team-banter-es-1` |
| `language` | Idioma del template | `es`, `en`, `fr`, `de`, `it`, `pt`, `pt-BR`, `tr` |
| `slot` | Tipo de post | `TeamBanter`, `FanOpinion`, `AnalystTake`, `PlayerReaction` |
| `weight` | Frecuencia (1-8) | `1` (raro), `5` (normal), `8` (muy frecuente) |
| `author_id` | ID de la cuenta que escribe | `null` (auto), `analyst_manu`, `fan_random_lec` |
| `conditions_json` | Condiciones para que se active | `null` o `{"requires_stomp": true}` |
| `variants` | Array de mensajes (1-6) | Se elige uno por partido |
| `tags` | Tags para inferir sentimiento | `["match", "team", "global"]` |
| `active` | Si está activo | `true` / `false` |

## Slots disponibles

| Slot | Descripción | Autor típico |
|------|-------------|--------------|
| `TeamBanter` | Post del equipo ganador | Cuenta del equipo |
| `FanOpinion` | Opinión de fan genérico | `fan_random_lec` |
| `AnalystTake` | Análisis de experto | `analyst_manu` |
| `PlayerReaction` | Reacción del MVP | Jugador del partido |

## Tokens disponibles

| Token | Descripción | Ejemplo |
|-------|-------------|---------|
| `{score}` | Resultado | `2-1` |
| `{winner_name}` | Nombre completo del ganador | `G2 Esports` |
| `{winner_short_name}` | Siglas del ganador | `G2` |
| `{loser_name}` | Nombre completo del perdedor | `Fnatic` |
| `{loser_short_name}` | Siglas del perdedor | `FNC` |
| `{winner_objectives}` | Número de objetivos | `7` |
| `{player_name}` | Nombre del jugador destacado | `Caps` |

## Condiciones disponibles

### Requiere `conditions_json` con formato JSON

```json
{
  "requires_stomp": true,
  "requires_stomp": false,
  "matchup_team_ids": ["lec-g2-esports", "lec-fnatic"],
  "manager_result": "win",
  "manager_result": "loss",
  "requires_player_name": true
}
```

| Condición | Descripción | Ejemplo |
|-----------|-------------|---------|
| `requires_stomp` | Partido con dominio (>=2 mapas de diferencia) | `true` o `false` |
| `matchup_team_ids` | Matchup específico entre dos equipos | `["lec-g2-esports", "lec-fnatic"]` |
| `manager_result` | Resultado del equipo del jugador | `win` o `loss` |
| `requires_player_name` | Requiere un jugador destacado | `true` |
| `winner_team_id` | Ganador específico | `lec-g2-esports` |
| `loser_team_id` | Perdedor específico | `lec-fnatic` |
| `opponent_team_id` | Rival específico | `lec-fnatic` |

## Sentimientos

El sistema asigna sentimiento automáticamente según los tags:

| Tag | Ganador | Perdedor |
|-----|---------|----------|
| `stomp` | `Hype` | `Meltdown` |
| `close` | `Worried` | `Worried` |
| `hate` | `Angry` | `Angry` |
| `rivalry` | `Hype` | `Worried` |
| `global` (default) | `Calm` | `Worried` |

**Sentimientos disponibles:**
- `Hype` — Euforia, emoción
- `Calm` — Tranquilidad, normalidad
- `Worried` — Preocupación, nervios
- `Angry` — Furia, odio
- `Meltdown` — Colapso, desesperación
- `Copium` — Negación, autoengaño (más que nada para fans perdedores)

## Tipos de autor

| Tipo | Descripción | Ejemplo |
|------|-------------|---------|
| `Team` | Cuenta oficial del equipo | G2 Esports |
| `Fan` | Fan genérico o de equipo | @randomLECEnjoyer |
| `Analyst` | Experto/analista | @Cabramaravilla |
| `Player` | Jugador profesional | Caps |
| `Journalist` | Medio de comunicación | @RiftNewswire |
| `MemeAccount` | Cuenta de memes | @SoloQChaos |
| `Manager` | Manager del jugador | @TuUsuario |

## Sistema de rivalidades y odio

### Rivalidad (competitivo, respetuoso)
- **G2 vs FNC** (`lec-g2-esports` + `lec-fnatic`)
- **GX vs TH** (`lec-giantx-lec` + `lec-team-heretics-lec`)

**Tono:** Intenso pero con respeto. "El clásico", "Rivalry time", "La rivalidad continúa"

**Sentimiento:** `Hype` (ganador) / `Worried` (perdedor)

### Odio (agresivo, sin respeto)
- **KOI vs TH** (`lec-mad-lions` + `lec-team-heretics-lec`)
- **KC vs Vitality** (`lec-karmine-corp` + `lec-team-vitality`)

**Tono:** Agresivo, despectivo, frustrado. **NO se usa "rivalidad" ni "clásico"**
- "Otra vez estos. Odio este matchup."
- "Espero que pierdan."
- "El peor partido de la semana."
- "No puedo ver esto."

**Sentimiento:** `Angry` (ambos equipos)

## Ejemplos de templates

### 1. Template básico (ganador global)
```json
{
  "id": "team-banter-es-1",
  "language": "es",
  "slot": "TeamBanter",
  "weight": 5,
  "author_id": null,
  "conditions_json": null,
  "variants": [
    "GG {loser_short_name}. Buen partido y seguimos sumando. {score}",
    "Victoria importante. Gracias a todos los que nos apoyaron hoy. #Vamos{winner_short_name}",
    "Cerramos la serie con calma, buen macro y mucha confianza. {score}"
  ],
  "tags": ["match", "team", "global"],
  "active": true
}
```

### 2. Template stomp (dominio)
```json
{
  "id": "team-banter-stomp-es-1",
  "language": "es",
  "slot": "TeamBanter",
  "weight": 3,
  "author_id": null,
  "conditions_json": "{\"requires_stomp\":true}",
  "variants": [
    "Clean sweep. {score} y no fue ni close.",
    "Demolición total. {score} en {winner_short_name}.",
    "Eso fue un clinic. {score} sin respuestas."
  ],
  "tags": ["match", "team", "stomp"],
  "active": true
}
```

### 3. Template close game (partido reñido)
```json
{
  "id": "team-banter-close-es-1",
  "language": "es",
  "slot": "TeamBanter",
  "weight": 3,
  "author_id": null,
  "conditions_json": "{\"requires_stomp\":false}",
  "variants": [
    "Tensión hasta el final. {score} pero lo conseguimos.",
    "Mi corazón no aguanta estos {score}.",
    "Sufrimos pero ganamos. {score} y a la siguiente."
  ],
  "tags": ["match", "team", "close"],
  "active": true
}
```

### 4. Template rivalidad (G2 vs FNC)
```json
{
  "id": "fan-rivalry-g2-fnc-es-1",
  "language": "es",
  "slot": "FanOpinion",
  "weight": 5,
  "author_id": "fan_random_lec",
  "conditions_json": "{\"matchup_team_ids\":[\"lec-g2-esports\",\"lec-fnatic\"]}",
  "variants": [
    "El clásico nunca decepciona. {score}",
    "Rivalry time. La rivalidad continúa.",
    "G2 vs FNC. Siempre es especial."
  ],
  "tags": ["fan", "match", "rivalry"],
  "active": true
}
```

### 5. Template odio (KOI vs TH)
```json
{
  "id": "fan-hate-koi-th-es-1",
  "language": "es",
  "slot": "FanOpinion",
  "weight": 5,
  "author_id": "fan_random_lec",
  "conditions_json": "{\"matchup_team_ids\":[\"lec-mad-lions\",\"lec-team-heretics-lec\"]}",
  "variants": [
    "Otra vez estos. Odio este matchup.",
    "Espero que pierdan. No me importa quién.",
    "El peor partido de la semana."
  ],
  "tags": ["fan", "match", "hate"],
  "active": true
}
```

## Cómo agregar un nuevo template

1. **Abrir** `data/social/templates.json`
2. **Elegir** el idioma y el slot
3. **Escribir** el ID único (ej: `team-banter-stomp-en-2`)
4. **Definir** las condiciones si aplica
5. **Escribir** 3-6 variantes de mensajes
6. **Asignar** tags correctos para sentimiento
7. **Guardar** y reiniciar el juego

### Reglas para escribir buenos templates
- Usar tokens para hacerlos dinámicos
- 3-6 variantes por template (más = más variedad)
- Incluir emojis y hashtags cuando sea natural
- No usar nombres de jugadores individuales (los rosters cambian)
- Para odio: ser agresivo pero no cruzar líneas
- Para rivalidad: mantener respeto competitivo
- Vary sentence length: short punchy + longer reflective
- Use regional slang cuando sea natural

## Cómo agregar un matchup nuevo

1. **Identificar** los IDs de los equipos:
   - Ver en `data/teams/lec_teams.json` el campo `"id"`
   - Ejemplo: KOI es `lec-mad-lions` (legacy naming)

2. **Crear** 2 templates (uno por cada slot que quieras):
   - FanOpinion
   - AnalystTake
   - TeamBanter (opcional)

3. **Usar** `matchup_team_ids` con ambos IDs:
   ```json
   {"matchup_team_ids": ["id-equipo-1", "id-equipo-2"]}
   ```

4. **Asignar** tag `"hate"` o `"rivalry"` según el tono

## Cómo agregar un nuevo idioma

1. **Copiar** templates de un idioma existente
2. **Traducir** los mensajes
3. **Cambiar** `language` al código del idioma
4. **Asegurar** que haya al menos 1 template por slot
5. **Opcional:** Añadir cuentas sociales en ese idioma en `accounts.json`

## IDs de equipos del LEC

| Equipo | ID |
|--------|-----|
| Fnatic | `lec-fnatic` |
| G2 Esports | `lec-g2-esports` |
| Team Heretics | `lec-team-heretics-lec` |
| KOI (MAD Lions) | `lec-mad-lions` |
| Vitality | `lec-team-vitality` |
| Karmine Corp | `lec-karmine-corp` |
| GiantX | `lec-giantx-lec` |
| SK Gaming | `lec-sk-gaming` |
| Natus Vincere | `lec-natus-vincere` |
| Movistar KOI | `lec-mad-lions` (same org) |
| Shifters | `lec-shifters` |

## Cuentas sociales disponibles

| ID | Tipo | Equipo | Handle |
|----|------|--------|--------|
| `fan_fnc_catxalote` | Fan | FNC | @CATXALOTE_ |
| `fan_g2_demons` | Fan | G2 | @DemonsGxd |
| `fan_th_serranito` | Fan | TH | @serraanitoo_ |
| `fan_koi_mrparrot` | Fan | KOI | @MrParrot23 |
| `fan_vit_rocket` | Fan | VIT | @VIT_Rocket |
| `fan_kc_kharasu` | Fan | KC | @Kharasu17 |
| `fan_sk_coriolis` | Fan | SK | @Cori0lis |
| `analyst_manu` | Analyst | — | @Cabramaravilla |
| `fan_lec_bouzys` | Fan | FNC | @Bouzyslol |
| `fan_random_lec` | Fan | — | @randomLECEnjoyer |

## Workflow recomendado para crear contenido

1. **Planificar** qué tipo de posts se quieren (rivalidad, stomp, close, etc.)
2. **Escribir** los templates en JSON
3. **Validar** el JSON con un linter
4. **Probar** en el juego con un partido
5. **Ajustar** según los resultados
6. **Repetir** para otros idiomas

## Notas importantes

- **No es necesario reiniciar el build** — los templates se leen en tiempo real desde el JSON
- **No es necesario usar SocialEditor** — fue eliminado, todo se edita en JSON
- **Los templates se leen en tiempo real** — cambiar el JSON y jugar un partido nuevo carga los cambios
- **El sistema de pesos (weight)** evita que un template domine si tiene muchas variantes
- **Las condiciones** se evalúan en AND: todas las condiciones deben cumplirse para que el template se active
- **El orden de prioridad** es: templates de usuario (en la base de datos) > templates de JSON > templates embebidos

## Ejemplo completo: Agregar un nuevo template

**Situación:** Queremos un template para cuando un equipo bajo (underdog) gana a uno alto.

**JSON:**
```json
{
  "id": "analyst-upset-es-1",
  "language": "es",
  "slot": "AnalystTake",
  "weight": 5,
  "author_id": "analyst_manu",
  "conditions_json": null,
  "variants": [
    "Upset confirmado. {winner_short_name} derrotó a {loser_short_name}. Nadie lo vio venir.",
    "El underdog se impuso. {score} contra {loser_short_name} y el mundo se sorprende.",
    "Carambola. {winner_short_name} acaba de cambiar la liga con este {score}."
  ],
  "tags": ["analysis", "match", "upset"],
  "active": true
}
```

**Nota:** Aunque no hay condición `upset` automática, el tag `"upset"` le dará sentimiento `Hype` al ganador.

---

**Para más información:**
- Ver `docs/social-images.md` para el sistema de imágenes
- Ver `src-tauri/crates/olm_core/src/social.rs` para la lógica de generación
- Ver `src-tauri/crates/olm_core/src/social_templates.rs` para el sistema de condiciones
