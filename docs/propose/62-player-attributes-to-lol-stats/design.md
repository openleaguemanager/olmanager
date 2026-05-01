# Technical Design: Migrate PlayerAttributes to LoL Stats

## Overview

This design document details the technical approach for replacing 19 football-specific player attributes with 9 League of Legends stats. The change affects the domain model, serialization layer, business logic, and frontend presentation.

## Architecture Decisions

### ADR-1: Serde Aliases for Backward Compatibility

**Decision**: Use serde's `alias` and `default` attributes plus a custom `Deserialize` implementation for backward compatibility.

**Rationale**: 
- Player attributes are stored as JSON in the database
- Existing save files contain legacy football attribute names
- A custom deserializer allows intelligent mapping from old to new format
- No database schema migration required

**Tradeoffs**:
- (+) No breaking change for existing saves
- (+) Clean migration path without data export/import
- (-) Custom deserializer adds complexity
- (-) Legacy mapping logic persists in codebase temporarily

### ADR-2: Remove Intermediate Mapping Layer

**Decision**: Delete `build_attributes_from_seed()` and use `build_lol_stats_from_seed()` directly.

**Rationale**:
- The mapping from LoL stats → football attributes → LoL OVR was always temporary
- Direct LoL stat usage simplifies the domain model
- Eliminates confusion about which attribute system is authoritative

**Tradeoffs**:
- (+) Cleaner, more maintainable code
- (+) No ambiguity about stat semantics
- (-) Requires updating all call sites (105+ matches)

### ADR-3: Trait System Retention with Mapping Update

**Decision**: Keep the existing trait system but update thresholds to map from LoL stats.

**Rationale**:
- Traits provide valuable gameplay flavor
- Many traits have conceptual equivalents in LoL (e.g., "Visionary" → high macro_play)
- Goalkeeper-specific traits will be deprecated/removed

## Data Model Changes

### Before: Football Attributes (19 fields)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAttributes {
    // Physical (4)
    pub pace: u8,
    pub stamina: u8,
    pub strength: u8,
    pub agility: u8,
    
    // Technical (5)
    pub passing: u8,
    pub shooting: u8,
    pub tackling: u8,
    pub dribbling: u8,
    pub defending: u8,
    
    // Mental (7)
    pub positioning: u8,
    pub vision: u8,
    pub decisions: u8,
    pub composure: u8,
    pub aggression: u8,
    pub teamwork: u8,
    pub leadership: u8,
    
    // Goalkeeper (3)
    pub handling: u8,
    pub reflexes: u8,
    pub aerial: u8,
}
```

### After: LoL Stats (9 fields)

```rust
#[derive(Debug, Clone, Serialize)]
pub struct PlayerAttributes {
    #[serde(alias = "dribbling")]
    pub mechanics: u8,
    
    #[serde(alias = "shooting")]
    pub laning: u8,
    
    #[serde(alias = "teamwork")]
    pub teamfighting: u8,
    
    #[serde(alias = "vision")]
    pub macro_play: u8,
    
    #[serde(alias = "decisions")]
    pub consistency: u8,
    
    #[serde(alias = "leadership")]
    pub shotcalling: u8,
    
    #[serde(alias = "agility")]
    pub champion_pool: u8,
    
    #[serde(alias = "composure")]
    pub discipline: u8,
    
    #[serde(alias = "stamina")]
    pub mental_resilience: u8,
}
```

## Migration Strategy

### Phase 1: Custom Deserializer Implementation

Implement `Deserialize` manually to handle legacy format:

```rust
impl<'de> Deserialize<'de> for PlayerAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct LegacyAttributes {
            // Legacy fields with defaults
            #[serde(default = "default_attr")]
            pace: u8,
            #[serde(default = "default_attr")]
            stamina: u8,
            // ... all 19 legacy fields
            
            // New fields (for forward compatibility)
            #[serde(default)]
            mechanics: Option<u8>,
            #[serde(default)]
            laning: Option<u8>,
            // ... all 9 new fields
        }
        
        let legacy = LegacyAttributes::deserialize(deserializer)?;
        
        // If new format present, use it directly
        if let (Some(m), Some(l), Some(t), Some(mp), Some(c), Some(s), Some(cp), Some(d), Some(mr)) = 
            (legacy.mechanics, legacy.laning, legacy.teamfighting, 
             legacy.macro_play, legacy.consistency, legacy.shotcalling,
             legacy.champion_pool, legacy.discipline, legacy.mental_resilience) {
            return Ok(PlayerAttributes {
                mechanics: m, laning: l, teamfighting: t,
                macro_play: mp, consistency: c, shotcalling: s,
                champion_pool: cp, discipline: d, mental_resilience: mr,
            });
        }
        
        // Otherwise, map from legacy
        Ok(PlayerAttributes {
            mechanics: avg(legacy.pace, legacy.dribbling),
            laning: legacy.shooting,
            teamfighting: legacy.teamwork,
            macro_play: legacy.vision,
            consistency: legacy.decisions,
            shotcalling: legacy.leadership,
            champion_pool: legacy.agility,
            discipline: legacy.composure,
            mental_resilience: legacy.stamina,
        })
    }
}
```

### Phase 2: Legacy Field Mapping Reference

| Legacy Field | Maps To | Formula |
|--------------|---------|---------|
| pace | mechanics | avg(pace, dribbling) |
| dribbling | mechanics | avg(pace, dribbling) |
| shooting | laning | direct |
| teamwork | teamfighting | direct |
| vision | macro_play | direct |
| decisions | consistency | direct |
| leadership | shotcalling | direct |
| agility | champion_pool | direct |
| composure | discipline | direct |
| stamina | mental_resilience | direct |
| passing, tackling, strength, defending, positioning, aggression, handling, reflexes, aerial | — | ignored (defaults used) |

### Phase 3: Save File Detection

Add a version field to save files to detect legacy format:

```rust
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    #[serde(default)]
    pub version: u32, // 0 or missing = legacy, 1+ = new format
    pub game: Game,
}
```

## API Changes

### Rust Backend

#### Modified Functions

| Function | File | Change |
|----------|------|--------|
| `calculate_lol_ovr()` | potential.rs | Average 9 LoL stats directly |
| `build_lol_stats_from_seed()` | game.rs | Returns PlayerAttributes instead of [u8; 9] |
| `build_attributes_from_seed()` | game.rs | **REMOVED** |
| `compute_traits()` | player.rs | Update trait thresholds |
| `apply_training()` | training.rs | Train LoL stats directly |
| `generate_scout_report()` | scouting.rs | Report LoL stats |

#### New Functions

| Function | File | Purpose |
|----------|------|---------|
| `migrate_legacy_attributes()` | legacy_migration.rs | One-time save migration |

### TypeScript Frontend

#### Type Changes

```typescript
// Before
interface PlayerData {
  attributes: {
    pace: number; stamina: number; strength: number; agility: number;
    passing: number; shooting: number; tackling: number; 
    dribbling: number; defending: number;
    positioning: number; vision: number; decisions: number;
    composure: number; aggression: number; teamwork: number;
    leadership: number;
    handling: number; reflexes: number; aerial: number;
  };
}

// After
interface PlayerData {
  attributes: {
    mechanics: number;
    laning: number;
    teamfighting: number;
    macro_play: number;
    consistency: number;
    shotcalling: number;
    champion_pool: number;
    discipline: number;
    mental_resilience: number;
  };
}
```

#### Component Updates

| Component | Changes |
|-----------|---------|
| `PlayerProfileAttributesCard.tsx` | Update attribute groups, labels, tooltips |
| `TrainingTab.tsx` | Update training focus options |
| `ScoutingReport.tsx` | Display LoL stats |
| `PlayerCard.tsx` | Show primary LoL stat (mechanics) as summary |

## Testing Strategy

### Unit Tests

1. **Deserializer Tests**
   - Legacy format with all 19 fields → correct LoL stats
   - New format with all 9 fields → direct mapping
   - Mixed format (both old and new) → prefer new
   - Missing fields → default to 50

2. **OVR Calculation Tests**
   - All stats equal → returns that value
   - Average calculation with rounding
   - Min/max clamping at 25/99

3. **Trait Derivation Tests**
   - Each trait condition with new stat mappings
   - Boundary values (threshold - 1, threshold, threshold + 1)
   - Multiple traits on same player

### Integration Tests

1. **Save Migration**
   - Load legacy save → verify correct migration
   - Load already-migrated save → no double migration
   - Save after migration → new format persisted

2. **End-to-End Flow**
   - Generate player from seed → correct stats
   - Train player → stats improve
   - Scout player → report shows LoL stats
   - Calculate OVR → uses LoL stats

### Regression Tests

- Existing gameplay features (matches, transfers, contracts)
- UI interactions (player profile, team setup)
- Save/load cycle

## Rollback Plan

If critical issues are discovered:

1. Revert the PR/branch
2. Players saved in new format will fail to load (acceptable for pre-release)
3. Legacy saves remain unaffected

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Data loss during migration | Comprehensive backup before migration; idempotent migration logic |
| Incorrect stat mapping | Unit tests for each mapping; spot-check with gameplay experts |
| UI confusion | Clear tooltips explaining each LoL stat; i18n keys for localization |
| Trait calculation errors | Boundary tests; compare pre/post migration trait counts |

## Performance Considerations

- Custom deserializer adds minimal overhead (one-time per player load)
- Smaller struct (9 vs 19 fields) reduces memory footprint
- Direct OVR calculation is faster (no mapping layer)

## Files Modified

### Backend (Rust)
- `domain/src/player.rs` — PlayerAttributes struct, compute_traits()
- `ofm_core/src/potential.rs` — calculate_lol_ovr()
- `ofm_core/src/training.rs` — Training adjustments
- `ofm_core/src/scouting.rs` — Scout report generation
- `src/commands/game.rs` — Player generation functions
- `db/src/legacy_migration.rs` — Save migration logic

### Frontend (TypeScript)
- `src/store/types.ts` — PlayerData interface
- `src/components/playerProfile/*.tsx` — Attribute display
- `src/components/training/*.tsx` — Training UI
- `src/components/scouting/*.tsx` — Scouting UI

### Tests
- All test files using PlayerAttributes test helpers
- Snapshot tests may need updates

## Success Metrics

- [ ] All 105+ Rust references updated
- [ ] All TypeScript types updated
- [ ] Legacy save files load correctly
- [ ] New save files use new format
- [ ] No references to football attributes in non-migration code
- [ ] All tests pass
- [ ] Manual QA confirms correct OVR calculations
