# Delta Spec: LoL Player Attributes Migration

## Domain: lol-player-attributes

### ADDED Requirements

#### Requirement: LoL Stats Definition

The system MUST define a `PlayerAttributes` struct with exactly 9 LoL-specific stats ranging from 0-100.

| Stat | Description | Mapping from Legacy |
|------|-------------|---------------------|
| mechanics | Technical skill and champion execution | dribbling |
| laning | 1v1 and 2v2 lane phase performance | shooting |
| teamfighting | Coordination in 5v5 engagements | teamwork |
| macro_play | Map awareness and objective control | vision |
| consistency | Performance stability across games | decisions |
| shotcalling | In-game leadership and calls | leadership |
| champion_pool | Champion versatility and mastery | agility |
| discipline | Focus and tilt resistance | composure |
| mental_resilience | Pressure handling and recovery | stamina |

#### Scenario: New Player Generation

- GIVEN a new player is generated from seed data
- WHEN the system creates `PlayerAttributes`
- THEN it MUST populate all 9 LoL stats directly from `build_lol_stats_from_seed()`
- AND the legacy mapping function MUST be removed

#### Scenario: Stat Value Validation

- GIVEN any LoL stat value
- WHEN the value is set or modified
- THEN it MUST be clamped to the range 25-99 inclusive
- AND default values for missing stats MUST be 50

### MODIFIED Requirements

#### Requirement: Overall Rating Calculation

The `calculate_lol_ovr()` function MUST compute OVR as the direct average of all 9 LoL stats without intermediate mapping.

(Previously: averaged 9 mapped football attributes derived from LoL stats)

#### Scenario: OVR Calculation with New Stats

- GIVEN a player with LoL stats [mechanics=75, laning=70, teamfighting=80, macro_play=72, consistency=68, shotcalling=65, champion_pool=78, discipline=70, mental_resilience=74]
- WHEN `calculate_lol_ovr()` is called
- THEN it MUST return 72 (rounded average of all 9 stats)

#### Scenario: OVR Edge Cases

- GIVEN a player with all stats at minimum (25)
- WHEN OVR is calculated
- THEN it MUST return 25
- GIVEN a player with all stats at maximum (99)
- WHEN OVR is calculated
- THEN it MUST return 99

### REMOVED Requirements

#### Requirement: Legacy Football Attributes

(Reason: Replaced by LoL-specific stats. Migration handled via serde deserializer)

The following 19 football attributes are REMOVED:
- pace, stamina, strength, agility
- passing, shooting, tackling, dribbling, defending
- positioning, vision, decisions, composure, aggression, teamwork, leadership
- handling, reflexes, aerial

---

## Domain: player-serde-migration

### ADDED Requirements

#### Requirement: Backward Compatibility Deserializer

The system MUST implement a custom serde deserializer that maps legacy football attribute names to new LoL stats using the established mapping from `build_attributes_from_seed()`.

#### Scenario: Loading Legacy Save File

- GIVEN a JSON player with legacy attributes `{ "pace": 70, "stamina": 72, "shooting": 65, ... }`
- WHEN the player is deserialized
- THEN the system MUST map legacy fields to LoL stats using intelligent defaults:
  - pace → mechanics (averaged with dribbling if present)
  - shooting → laning
  - teamwork → teamfighting
  - vision → macro_play
  - decisions → consistency
  - leadership → shotcalling
  - agility → champion_pool
  - composure → discipline
  - stamina → mental_resilience
- AND missing fields MUST default to 50

#### Scenario: Loading New Format Save File

- GIVEN a JSON player with new LoL attributes `{ "mechanics": 75, "laning": 70, ... }`
- WHEN the player is deserialized
- THEN it MUST deserialize directly without transformation
- AND all 9 stats MUST be present in the resulting struct

#### Scenario: Mixed Legacy and New Format

- GIVEN a JSON with both legacy and new format fields
- WHEN the player is deserialized
- THEN new format fields MUST take precedence
- AND legacy fields MUST be ignored if new format is present

---

## Domain: player-generation

### MODIFIED Requirements

#### Requirement: Player Generation from Seed

The `build_lol_stats_from_seed()` function becomes the PRIMARY generation function; `build_attributes_from_seed()` is REMOVED.

(Previously: `build_lol_stats_from_seed()` returned an array that was then mapped to football attributes via `build_attributes_from_seed()`)

#### Scenario: Seed-Based Player Creation

- GIVEN a `DraftPlayerSeed` with role="mid" and rating=75
- WHEN a player is generated
- THEN `build_lol_stats_from_seed()` MUST return 9 stats with role-based bias:
  - Mid: higher mechanics (+2), laning (+2)
  - Top: higher mechanics (+1), teamfighting (+1), discipline (+1), mental_resilience (+2)
  - Jungle: higher macro_play (+2), shotcalling (+2)
  - ADC: higher mechanics (+2), laning (+2)
  - Support: higher macro_play (+2), shotcalling (+2), discipline (+1)
- AND all stats MUST be within 25-99 range
- AND the average MUST approximate the target rating (±3)

---

## Domain: player-training

### MODIFIED Requirements

#### Requirement: Training System Integration

The training system MUST adjust LoL stats directly instead of mapping through football attributes.

(Previously: trained football attributes which were then mapped back to LoL stats conceptually)

#### Scenario: Individual Training Focus

- GIVEN a player with training_focus="Mechanics"
- WHEN daily training is processed
- THEN the mechanics stat MUST receive the primary training bonus
- AND related stats (laning, consistency) MAY receive secondary bonuses
- AND the training gain MUST respect the effective_potential_cap

#### Scenario: Team Training by Focus

- GIVEN a team with training_focus="MacroSystems"
- WHEN team training is processed
- THEN all team players' macro_play stat MUST receive bonus
- AND shotcalling MAY receive secondary bonus
- AND gains MUST be modulated by facility level and coaching quality

#### Scenario: Training Intensity Impact

- GIVEN a team with TrainingIntensity::Intense
- WHEN training is processed
- THEN stat gains MUST be multiplied by 1.3
- AND condition depletion MUST be multiplied by 1.5

---

## Domain: player-scouting

### MODIFIED Requirements

#### Requirement: Scouting Report Format

Scouting reports MUST display LoL stats instead of football attributes.

(Previously: showed football attributes or partially mapped LoL stats)

#### Scenario: Scout Report Generation

- GIVEN a completed scouting assignment
- WHEN the report is generated
- THEN it MUST include all 9 LoL stats visible to the scout
- AND stats above the scout's judging_ability threshold MUST be accurate
- AND stats below threshold MUST show as approximate ranges (??)

#### Scenario: Scout Report Accuracy

- GIVEN a scout with judging_ability=80
- WHEN evaluating a player
- THEN stats above 80 MUST be shown as exact values
- AND stats 60-80 MUST be shown with ±3 variance
- AND stats below 60 MUST be hidden or marked as "??"

---

## Domain: player-traits

### MODIFIED Requirements

#### Requirement: Trait Derivation from LoL Stats

The `compute_traits()` function MUST derive traits directly from LoL stats using equivalent thresholds.

(Previously: derived from football attributes)

| Trait | Old Condition | New Condition |
|-------|--------------|---------------|
| Speedster | pace >= 85 | mechanics >= 85 |
| Tank | strength>=85 && stamina>=75 | teamfighting>=85 && mental_resilience>=75 |
| Agile | agility >= 85 | champion_pool >= 85 |
| Tireless | stamina >= 90 | mental_resilience >= 90 |
| Playmaker | passing>=80 && vision>=80 | macro_play>=80 && shotcalling>=80 |
| Sharpshooter | shooting >= 85 | laning >= 85 |
| Dribbler | dribbling >= 85 | mechanics >= 85 |
| BallWinner | tackling>=80 && aggression>=70 | discipline>=80 && teamfighting>=70 |
| Rock | defending>=85 && positioning>=75 | teamfighting>=85 && macro_play>=75 |
| Leader | leadership>=85 && teamwork>=75 | shotcalling>=85 && teamfighting>=75 |
| CoolHead | composure>=85 && decisions>=80 | discipline>=85 && consistency>=80 |
| Visionary | vision >= 85 | macro_play >= 85 |
| HotHead | aggression>=85 && composure<50 | low discipline, high teamfighting |
| TeamPlayer | teamwork >= 85 | teamfighting >= 85 |
| SafeHands | handling >= 85 | (removed - goalkeeper trait) |
| CatReflexes | reflexes >= 85 | (removed - goalkeeper trait) |
| AerialDominance | aerial >= 85 | (removed - goalkeeper trait) |
| CompleteForward | shooting>=75 && dribbling>=75 && pace>=70 && strength>=70 | mechanics>=75 && laning>=75 && champion_pool>=70 |
| Engine | stamina>=85 && pace>=70 && teamwork>=75 | mental_resilience>=85 && mechanics>=70 && teamfighting>=75 |
| SetPieceSpecialist | passing>=80 && shooting>=75 && vision>=75 | macro_play>=80 && laning>=75 && shotcalling>=75 |

---

## Domain: frontend-player-attributes

### MODIFIED Requirements

#### Requirement: TypeScript Type Definition

The frontend `PlayerData.attributes` type MUST be updated to reflect the 9 LoL stats.

(Previously: 19 football attributes)

#### Scenario: Frontend Type Safety

- GIVEN the TypeScript `PlayerData` interface
- WHEN accessing player.attributes
- THEN it MUST expose only the 9 LoL stat fields
- AND type checking MUST reject legacy football attribute names

#### Scenario: UI Display Update

- GIVEN the PlayerProfileAttributesCard component
- WHEN rendering player attributes
- THEN it MUST group LoL stats logically:
  - Mechanical: mechanics, laning, champion_pool
  - Tactical: teamfighting, macro_play, shotcalling
  - Mental: consistency, discipline, mental_resilience
- AND each stat MUST have a descriptive tooltip
- AND stat names MUST be i18n-compatible

---

## Domain: database-migration

### ADDED Requirements

#### Requirement: Legacy Save Migration

The system MUST provide a one-time migration for existing save files that converts football attributes to LoL stats.

#### Scenario: Save File Version Detection

- GIVEN a save file with version < 2.0
- WHEN the game loads
- THEN it MUST detect legacy format via absence of LoL stat fields
- AND trigger the migration path
- AND save the file in new format after migration

#### Scenario: Migration Idempotency

- GIVEN a save file that has already been migrated
- WHEN the game loads again
- THEN it MUST recognize the new format
- AND skip migration
- AND not corrupt existing data

---

## Summary

| Domain | Added | Modified | Removed |
|--------|-------|----------|---------|
| lol-player-attributes | 2 | 2 | 1 |
| player-serde-migration | 3 | 0 | 0 |
| player-generation | 1 | 1 | 0 |
| player-training | 0 | 3 | 0 |
| player-scouting | 0 | 2 | 0 |
| player-traits | 0 | 1 | 0 |
| frontend-player-attributes | 0 | 2 | 0 |
| database-migration | 2 | 0 | 0 |
| **Total** | **8** | **11** | **1** |

### Test Coverage Requirements

- Unit tests for serde deserialization (legacy → new format)
- Unit tests for `calculate_lol_ovr()` with edge cases
- Unit tests for trait derivation with new stat mappings
- Integration tests for save file migration
- Frontend tests for attribute display components
