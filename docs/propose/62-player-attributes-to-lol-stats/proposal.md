# Proposal: Migrate PlayerAttributes to LoL Stats

## Intent

The current `PlayerAttributes` struct uses 19 football-specific attributes (pace, stamina, strength, agility, passing, shooting, tackling, dribbling, defending, positioning, vision, decisions, composure, aggression, teamwork, leadership, handling, reflexes, aerial). As the game transitions to a League of Legends-themed manager, we need to replace these with LoL-appropriate stats that reflect competitive League of Legends gameplay. This change will replace the football attributes with 9 LoL stats (mechanics, laning, teamfighting, macro_play, consistency, shotcalling, champion_pool, discipline, mental_resilience), aligning the domain model with the new thematic direction and simplifying the mapping already present in the codebase.

## Scope

### In Scope
- Replace `PlayerAttributes` struct in `domain/src/player.rs` with 9 LoL stats.
- Update all Rust references (105+ matches across `domain`, `ofm_core`, `db`, `commands`).
- Update frontend TypeScript types and components that reference football attributes.
- Ensure backward compatibility for existing save files via serde migration (custom deserializer that maps old field names to new ones with reasonable defaults).
- Update `calculate_lol_ovr` function to compute OVR directly from LoL stats (no mapping needed).
- Update `build_lol_stats_from_seed` and `build_attributes_from_seed` functions (merge into one, as the mapping becomes obsolete).
- Update training, scouting, and other systems that reference specific football attributes.

### Out of Scope
- Changing simulation logic beyond adapting to the new stats (i.e., no rebalancing of modifiers).
- Adding new UI components for stat visualization (future work).
- Database schema changes (player attributes stored as JSON, so only migration of existing data).
- Changing the 9 LoL stats themselves (already defined and in use).

## Capabilities

### New Capabilities
- `<lol-player-attributes>`: Replaces football-specific player attributes with League of Legends stats, affecting player generation, training, scouting, match simulation, and overall rating.

### Modified Capabilities
- `<player-overall-rating>`: `calculate_lol_ovr` now averages the 9 LoL stats directly, removing the mapping layer.
- `<player-generation>`: `build_lol_stats_from_seed` becomes the primary generation function; `build_attributes_from_seed` is removed.
- `<player-training>`: Training adjustments target LoL stats (mechanics, laning, teamfighting, etc.) instead of football attributes.
- `<player-scouting>`: Scouting reports show LoL stats instead of football attributes.

## Approach

1. **Define new `PlayerAttributes` struct** with 9 LoL stats fields, using serde aliases for backward compatibility (e.g., `#[serde(alias = "pace")]` mapping to appropriate new field or default).
2. **Implement custom deserialization** that maps old football attribute names to new LoL stats with intelligent defaults (e.g., pace → mechanics, stamina → mental_resilience, etc.) using the existing mapping in `build_attributes_from_seed` as reference.
3. **Update `calculate_lol_ovr`** to average the 9 LoL stats fields directly.
4. **Update all Rust code** (105+ matches) to use new field names; adjust any logic that differentiated between football attributes (e.g., goalkeeper handling/reflexes/aerial become irrelevant; keep as low defaults or remove).
5. **Update frontend TypeScript types** (`PlayerAttributes` interface) and components that display attributes (player cards, training UI, scouting UI).
6. **Update training system** to train LoL stats (e.g., "Mechanics", "Laning", "Teamfighting" focus).
7. **Update scouting system** to report LoL stats.
8. **Add data migration** for existing saves: custom deserializer that maps old JSON fields to new ones using the same mapping as `build_attributes_from_seed`. If a field is missing, assign a default (50).
9. **Update tests** that rely on football attributes.

## Affected Areas

| Area | Impact | Description |
|------|--------|-------------|
| `src-tauri/crates/domain/src/player.rs` | Modified | Replace `PlayerAttributes` struct with LoL stats. |
| `src-tauri/crates/ofm_core/src/potential.rs` | Modified | Update `calculate_lol_ovr` to use new fields. |
| `src-tauri/src/commands/game.rs` | Modified | Update `build_lol_stats_from_seed` and `build_attributes_from_seed`; merge into one function. |
| `src-tauri/crates/ofm_core/src/training.rs` | Modified | Update training adjustments to target LoL stats. |
| `src-tauri/crates/ofm_core/src/scouting.rs` | Modified | Update scouting reports. |
| `src-tauri/crates/db/src/repositories/player_repo.rs` | Modified | Update deserialization logic. |
| `src-tauri/crates/db/src/legacy_migration.rs` | Modified | Add migration for old saves. |
| `src/components/**/PlayerCard.tsx` | Modified | Update UI to show LoL stats. |
| `src/components/**/TrainingTab.tsx` | Modified | Update training UI. |
| `src/components/**/ScoutingReport.tsx` | Modified | Update scouting UI. |
| `src/types/player.ts` | Modified | Update TypeScript interface. |
| Various test files | Modified | Update test helpers and assertions. |

## Risks

| Risk | Likigation | Mitigation |
|------|------------|------------|
| Breaking existing saves | High | Implement custom deserializer with aliases and defaults; test with existing save files. |
| UI confusion | Medium | Update UI labels to reflect LoL stats; keep tooltips explaining each stat. |
| Missing references (105+ matches) | Medium | Use global search/replace with careful review; run full test suite after changes. |
| Incorrect stat mapping | Medium | Use existing mapping from `build_attributes_from_seed` as reference; verify with gameplay experts. |

## Rollback Plan

Revert the branch; the change is self-contained and does not affect database schemas (player attributes stored as JSON). Existing saves that already use the new struct will not load after rollback, but that's acceptable for a pre-release change.

## Dependencies

- None (this is a standalone refactor).

## Success Criteria

- [ ] All Rust code compiles with new LoL stats replacing football attributes.
- [ ] Existing save files (with football attributes) load correctly via custom deserializer.
- [ ] `calculate_lol_ovr` returns same overall rating as before (or with documented adjustments).
- [ ] Training system improves LoL stats as expected.
- [ ] Scouting reports show LoL stats correctly.
- [ ] Frontend UI displays LoL stats with proper labels and tooltips.
- [ ] All existing tests pass; new tests added for stat mapping and deserialization.
- [ ] No references to football attribute names remain in the codebase (except serde aliases).