# Task Breakdown: Migrate PlayerAttributes to LoL Stats

## Phase 1: Foundation & Data Model (Tasks 1-8)

### Task 1.1: Update PlayerAttributes struct definition
**File**: `src-tauri/crates/domain/src/player.rs`  
**Priority**: P0 - Blocking  
**Estimate**: 2h  
**Description**: Replace 19 football attributes with 9 LoL stats in the struct definition. Add serde aliases for backward compatibility.

**Acceptance Criteria**:
- [ ] Struct has exactly 9 fields: mechanics, laning, teamfighting, macro_play, consistency, shotcalling, champion_pool, discipline, mental_resilience
- [ ] Each field has appropriate serde alias for legacy mapping
- [ ] All fields are u8 type
- [ ] Default value function returns 50

---

### Task 1.2: Implement custom deserializer for backward compatibility
**File**: `src-tauri/crates/domain/src/player.rs`  
**Priority**: P0 - Blocking  
**Estimate**: 4h  
**Description**: Implement custom Deserialize trait that maps legacy football attributes to new LoL stats.

**Acceptance Criteria**:
- [ ] Legacy format with 19 fields deserializes correctly
- [ ] New format with 9 fields deserializes directly
- [ ] Mixed format prefers new fields
- [ ] Missing fields default to 50
- [ ] Unit tests for all mapping combinations

---

### Task 1.3: Update calculate_lol_ovr function
**File**: `src-tauri/crates/ofm_core/src/potential.rs`  
**Priority**: P0 - Blocking  
**Estimate**: 1h  
**Description**: Modify OVR calculation to average the 9 LoL stats directly instead of mapped football attributes.

**Acceptance Criteria**:
- [ ] Function averages mechanics, laning, teamfighting, macro_play, consistency, shotcalling, champion_pool, discipline, mental_resilience
- [ ] Result is rounded and clamped 25-99
- [ ] Unit tests updated with new test cases
- [ ] Existing tests that relied on old mapping updated

---

### Task 1.4: Remove build_attributes_from_seed function
**File**: `src-tauri/src/commands/game.rs`  
**Priority**: P0 - Blocking  
**Estimate**: 1h  
**Description**: Delete the legacy mapping function. Update build_lol_stats_from_seed to return PlayerAttributes directly.

**Acceptance Criteria**:
- [ ] build_attributes_from_seed function removed
- [ ] build_lol_stats_from_seed returns PlayerAttributes
- [ ] All call sites updated to use new return type
- [ ] No compilation errors

---

### Task 1.5: Update build_lol_stats_from_seed return type
**File**: `src-tauri/src/commands/game.rs`  
**Priority**: P0 - Blocking  
**Estimate**: 1h  
**Description**: Modify function signature and all usages to work with PlayerAttributes instead of [u8; 9].

**Acceptance Criteria**:
- [ ] Function returns PlayerAttributes
- [ ] Internal array construction still used, then converted to struct
- [ ] All callers updated
- [ ] Tests updated

---

### Task 1.6: Update trait computation logic
**File**: `src-tauri/crates/domain/src/player.rs`  
**Priority**: P1 - High  
**Estimate**: 3h  
**Description**: Update compute_traits() to derive traits from LoL stats using new thresholds.

**Acceptance Criteria**:
- [ ] All trait conditions updated per design document mapping table
- [ ] Goalkeeper traits (SafeHands, CatReflexes, AerialDominance) removed or deprecated
- [ ] New trait conditions tested with boundary values
- [ ] Trait computation tests pass

---

### Task 1.7: Add default value handling for missing fields
**File**: `src-tauri/crates/domain/src/player.rs`  
**Priority**: P1 - High  
**Estimate**: 1h  
**Description**: Ensure serde default handling works correctly for partial data.

**Acceptance Criteria**:
- [ ] Missing LoL stat fields default to 50
- [ ] Legacy fields without mapping default appropriately
- [ ] Test cases for partial deserialization

---

### Task 1.8: Create legacy migration module
**File**: `src-tauri/crates/db/src/legacy_migration.rs`  
**Priority**: P1 - High  
**Estimate**: 3h  
**Description**: Create dedicated module for save file migration logic with version detection.

**Acceptance Criteria**:
- [ ] Migration module created with version detection
- [ ] One-time migration path implemented
- [ ] Idempotent migration (won't double-migrate)
- [ ] Logging for migration events

---

## Phase 2: Training System Updates (Tasks 2.1-2.5)

### Task 2.1: Update training focus definitions
**File**: `src-tauri/crates/domain/src/team.rs` (or training focus module)  
**Priority**: P1 - High  
**Estimate**: 1h  
**Description**: Ensure training focus enum values map to LoL stats.

**Acceptance Criteria**:
- [ ] TrainingFocus enum values reviewed and updated if needed
- [ ] Each focus maps to appropriate LoL stat(s)
- [ ] Documentation updated

---

### Task 2.2: Update individual training adjustments
**File**: `src-tauri/crates/ofm_core/src/training.rs`  
**Priority**: P1 - High  
**Estimate**: 2h  
**Description**: Modify training logic to adjust LoL stats directly instead of mapped football attributes.

**Acceptance Criteria**:
- [ ] Individual training targets correct LoL stat
- [ ] Secondary bonuses updated for related stats
- [ ] Potential cap enforcement still works
- [ ] Tests updated

---

### Task 2.3: Update team training adjustments
**File**: `src-tauri/crates/ofm_core/src/training.rs`  
**Priority**: P1 - High  
**Estimate**: 2h  
**Description**: Update team-wide training to affect appropriate LoL stats.

**Acceptance Criteria**:
- [ ] Team training focus affects correct LoL stat
- [ ] Facility level bonuses apply correctly
- [ ] Staff coaching effects work with new stats

---

### Task 2.4: Update training intensity effects
**File**: `src-tauri/crates/ofm_core/src/training.rs`  
**Priority**: P2 - Medium  
**Estimate**: 1h  
**Description**: Ensure training intensity multipliers work with LoL stat gains.

**Acceptance Criteria**:
- [ ] Intense training gives 1.3x LoL stat gains
- [ ] Condition depletion still works
- [ ] Light training gives reduced gains

---

### Task 2.5: Update training tests
**File**: `src-tauri/crates/ofm_core/src/training.rs` (tests)  
**Priority**: P1 - High  
**Estimate**: 2h  
**Description**: Update all training-related tests to use LoL stats.

**Acceptance Criteria**:
- [ ] Test helpers updated to create players with LoL stats
- [ ] All existing tests pass with new stats
- [ ] New tests for LoL stat training gains

---

## Phase 3: Scouting System Updates (Tasks 3.1-3.4)

### Task 3.1: Update scout report generation
**File**: `src-tauri/crates/ofm_core/src/scouting.rs`  
**Priority**: P1 - High  
**Estimate**: 2h  
**Description**: Modify scout reports to include LoL stats instead of football attributes.

**Acceptance Criteria**:
- [ ] Scout report contains all 9 LoL stats
- [ ] Accuracy based on scout judging_ability works
- [ ] Hidden stats (below threshold) show as "??"

---

### Task 3.2: Update scouting report accuracy logic
**File**: `src-tauri/crates/ofm_core/src/scouting.rs`  
**Priority**: P2 - Medium  
**Estimate**: 1h  
**Description**: Ensure stat visibility thresholds work with LoL stats.

**Acceptance Criteria**:
- [ ] Stats > judging_ability shown exactly
- [ ] Stats 60-80 shown with ±3 variance
- [ ] Stats < 60 hidden

---

### Task 3.3: Update scouting-related types
**File**: `src-tauri/crates/domain/src/scouting.rs` (if exists) or scouting module  
**Priority**: P1 - High  
**Estimate**: 1h  
**Description**: Update type definitions for scout reports to use LoL stats.

**Acceptance Criteria**:
- [ ] Scout report struct uses LoL stat names
- [ ] Serde serialization updated
- [ ] Frontend types will be updated in Phase 5

---

### Task 3.4: Update scouting tests
**File**: `src-tauri/crates/ofm_core/src/scouting.rs` (tests)  
**Priority**: P2 - Medium  
**Estimate**: 1h  
**Description**: Update scouting tests to work with LoL stats.

**Acceptance Criteria**:
- [ ] Test players created with LoL stats
- [ ] Report accuracy tests updated
- [ ] All scouting tests pass

---

## Phase 4: Frontend TypeScript Updates (Tasks 4.1-4.6)

### Task 4.1: Update PlayerData TypeScript interface
**File**: `src/store/types.ts`  
**Priority**: P0 - Blocking  
**Estimate**: 1h  
**Description**: Replace 19 football attributes with 9 LoL stats in TypeScript type.

**Acceptance Criteria**:
- [ ] PlayerData.attributes has 9 LoL stat fields
- [ ] Legacy football attributes removed
- [ ] Type checking passes

---

### Task 4.2: Update PlayerProfileAttributesCard component
**File**: `src/components/playerProfile/PlayerProfileAttributesCard.tsx`  
**Priority**: P1 - High  
**Estimate**: 2h  
**Description**: Update UI to display LoL stats in organized groups.

**Acceptance Criteria**:
- [ ] Component displays 9 LoL stats
- [ ] Stats grouped logically (Mechanical, Tactical, Mental)
- [ ] Labels and tooltips updated
- [ ] i18n keys added for stat names

---

### Task 4.3: Update attribute grouping logic
**File**: `src/components/playerProfile/PlayerProfile.attributes.ts` (if exists)  
**Priority**: P1 - High  
**Estimate**: 1h  
**Description**: Update attribute grouping helper for LoL stats.

**Acceptance Criteria**:
- [ ] Mechanical group: mechanics, laning, champion_pool
- [ ] Tactical group: teamfighting, macro_play, shotcalling
- [ ] Mental group: consistency, discipline, mental_resilience
- [ ] Average calculations work per group

---

### Task 4.4: Update TrainingTab component
**File**: `src/components/training/TrainingTab.tsx` (or similar)  
**Priority**: P1 - High  
**Estimate**: 2h  
**Description**: Update training UI to reference LoL stats.

**Acceptance Criteria**:
- [ ] Training focus dropdown shows LoL stat names
- [ ] Individual training targets display correctly
- [ ] Training preview shows expected LoL stat gains

---

### Task 4.5: Update ScoutingReport component
**File**: `src/components/scouting/ScoutingReport.tsx` (or similar)  
**Priority**: P1 - High  
**Estimate**: 2h  
**Description**: Update scouting UI to display LoL stats in reports.

**Acceptance Criteria**:
- [ ] Scout reports show 9 LoL stats
- [ ] Hidden stats display as "??"
- [ ] Stat bars/colors work with LoL stats

---

### Task 4.6: Update any other attribute references
**File**: Various frontend files  
**Priority**: P2 - Medium  
**Estimate**: 2h  
**Description**: Search and update all remaining frontend references to football attributes.

**Acceptance Criteria**:
- [ ] Global search for old attribute names returns 0 results
- [ ] PlayerCard shows relevant LoL stat
- [ ] Any stat comparison logic updated

---

## Phase 5: Testing & Validation (Tasks 5.1-5.8)

### Task 5.1: Update test helpers in domain crate
**File**: `src-tauri/crates/domain/src/player.rs` (test helpers)  
**Priority**: P0 - Blocking  
**Estimate**: 2h  
**Description**: Update sample_attributes() and other test helpers to use LoL stats.

**Acceptance Criteria**:
- [ ] sample_attributes() returns PlayerAttributes with LoL stats
- [ ] All test compilation errors resolved
- [ ] Test defaults are reasonable (50-70 range)

---

### Task 5.2: Update potential.rs tests
**File**: `src-tauri/crates/ofm_core/src/potential.rs` (tests)  
**Priority**: P0 - Blocking  
**Estimate**: 1h  
**Description**: Update OVR calculation tests for LoL stats.

**Acceptance Criteria**:
- [ ] Test attrs() helper uses LoL stats
- [ ] OVR calculation tests pass
- [ ] Edge case tests (min, max, average)

---

### Task 5.3: Update game.rs tests
**File**: `src-tauri/src/commands/game.rs` (tests)  
**Priority**: P1 - High  
**Estimate**: 2h  
**Description**: Update player generation tests for new return type.

**Acceptance Criteria**:
- [ ] Tests compile with new build_lol_stats_from_seed signature
- [ ] Seed-based generation tests pass
- [ ] Stat distribution tests updated

---

### Task 5.4: Create serde migration tests
**File**: `src-tauri/crates/domain/src/player.rs` (tests) or new test file  
**Priority**: P0 - Blocking  
**Estimate**: 3h  
**Description**: Comprehensive tests for backward compatibility deserializer.

**Acceptance Criteria**:
- [ ] Test: Full legacy format → correct LoL stats
- [ ] Test: New format → direct mapping
- [ ] Test: Mixed format → prefers new
- [ ] Test: Partial legacy → defaults for missing
- [ ] Test: Partial new → defaults for missing

---

### Task 5.5: Create integration tests for save migration
**File**: `src-tauri/crates/db/src/legacy_migration.rs` (tests)  
**Priority**: P1 - High  
**Estimate**: 3h  
**Description**: Test end-to-end save file migration.

**Acceptance Criteria**:
- [ ] Legacy save file loads and migrates correctly
- [ ] Migration is idempotent
- [ ] New saves don't trigger migration
- [ ] Migration logging works

---

### Task 5.6: Update trait computation tests
**File**: `src-tauri/crates/domain/src/player.rs` (tests)  
**Priority**: P1 - High  
**Estimate**: 2h  
**Description**: Update trait derivation tests for LoL stat mappings.

**Acceptance Criteria**:
- [ ] Each trait test uses correct LoL stat thresholds
- [ ] Boundary value tests (threshold ±1)
- [ ] Removed goalkeeper traits handled

---

### Task 5.7: Update frontend tests
**File**: Various `.test.ts` files  
**Priority**: P2 - Medium  
**Estimate**: 2h  
**Description**: Update frontend unit tests that reference player attributes.

**Acceptance Criteria**:
- [ ] Mock player data uses LoL stats
- [ ] Component tests pass
- [ ] TypeScript type errors resolved

---

### Task 5.8: Run full test suite
**File**: Entire codebase  
**Priority**: P0 - Blocking  
**Estimate**: 2h  
**Description**: Execute all tests and fix any remaining failures.

**Acceptance Criteria**:
- [ ] `cargo test` passes in all crates
- [ ] `npm test` passes for frontend
- [ ] No test compilation errors
- [ ] Test coverage maintained or improved

---

## Phase 6: Documentation & Cleanup (Tasks 6.1-6.4)

### Task 6.1: Update code documentation
**File**: All modified files  
**Priority**: P2 - Medium  
**Estimate**: 2h  
**Description**: Update doc comments to reference LoL stats instead of football attributes.

**Acceptance Criteria**:
- [ ] All doc comments use LoL stat names
- [ ] Function documentation updated
- [ ] Module documentation reflects changes

---

### Task 6.2: Update README or developer docs
**File**: `docs/` or README files  
**Priority**: P3 - Low  
**Estimate**: 1h  
**Description**: Document the attribute system for developers.

**Acceptance Criteria**:
- [ ] Attribute system documented
- [ ] Migration guide for developers
- [ ] Trait conditions documented

---

### Task 6.3: Remove dead code
**File**: Throughout codebase  
**Priority**: P2 - Medium  
**Estimate**: 1h  
**Description**: Delete commented-out code and unused imports.

**Acceptance Criteria**:
- [ ] No commented legacy attribute code
- [ ] Unused imports removed
- [ ] Clippy warnings resolved

---

### Task 6.4: Final code review preparation
**File**: All modified files  
**Priority**: P1 - High  
**Estimate**: 2h  
**Description**: Prepare for code review with clean commits and documentation.

**Acceptance Criteria**:
- [ ] Commits organized by phase
- [ ] No debugging code or print statements
- [ ] CHANGELOG updated

---

## Phase 7: Manual QA & Verification (Tasks 7.1-7.4)

### Task 7.1: Manual save file migration test
**Priority**: P0 - Blocking  
**Estimate**: 2h  
**Description**: Test migration with real save files from production.

**Acceptance Criteria**:
- [ ] Legacy save loads without errors
- [ ] Player stats look reasonable after migration
- [ ] OVR values consistent
- [ ] Save in new format loads correctly

---

### Task 7.2: UI/UX verification
**Priority**: P1 - High  
**Estimate**: 2h  
**Description**: Manual testing of all UI components showing player attributes.

**Acceptance Criteria**:
- [ ] Player profile shows 9 LoL stats correctly
- [ ] Training UI displays correct stat names
- [ ] Scouting reports show LoL stats
- [ ] Tooltips explain each stat

---

### Task 7.3: Gameplay verification
**Priority**: P1 - High  
**Estimate**: 3h  
**Description**: Play through game features to verify stat usage.

**Acceptance Criteria**:
- [ ] Player generation creates reasonable stats
- [ ] Training improves stats as expected
- [ ] Scouting reveals stats correctly
- [ ] OVR calculation feels balanced
- [ ] Traits are assigned appropriately

---

### Task 7.4: Regression testing
**Priority**: P1 - High  
**Estimate**: 3h  
**Description**: Test unrelated features to ensure no regressions.

**Acceptance Criteria**:
- [ ] Matches simulate correctly
- [ ] Transfers work
- [ ] Contracts and wages calculated properly
- [ ] Save/load cycle works

---

## Summary

| Phase | Tasks | Total Estimate |
|-------|-------|----------------|
| Phase 1: Foundation | 8 | 18h |
| Phase 2: Training | 5 | 8h |
| Phase 3: Scouting | 4 | 6h |
| Phase 4: Frontend | 6 | 10h |
| Phase 5: Testing | 8 | 17h |
| Phase 6: Documentation | 4 | 6h |
| Phase 7: QA | 4 | 10h |
| **Total** | **39** | **75h** |

## Task Dependencies

```
Phase 1 (Foundation)
├── Task 1.1 (struct update) ──┬──► Task 1.2 (deserializer)
│                              └──► Task 1.3 (ovr calc)
├── Task 1.4 (remove fn) ──────► Task 1.5 (update fn)
├── Task 1.6 (traits)
└── Task 1.8 (migration)

Phase 2 (Training) ──► Phase 1 complete
Phase 3 (Scouting) ──► Phase 1 complete
Phase 4 (Frontend) ──► Phase 1 complete
Phase 5 (Testing) ───► Phases 1-4 complete
Phase 6 (Docs) ──────► Phase 5 complete
Phase 7 (QA) ────────► All phases complete
```

## Notes

- **Critical Path**: Tasks 1.1 → 1.2 → 1.3 → 5.1 → 5.4 → 5.8 → 7.1
- **Parallel Work**: Training (Phase 2), Scouting (Phase 3), and Frontend (Phase 4) can be worked on simultaneously after Phase 1
- **Risk Areas**: 
  - Custom deserializer (Task 1.2) - most complex piece
  - Frontend type updates (Task 4.1) - affects many files
  - Save migration (Tasks 1.8, 5.5, 7.1) - data integrity critical
