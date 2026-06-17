# ADR-014: Two-Agent AI Decision System for Non-Player Teams

## Status

Accepted

## Date

2026-06-17

## Context

OLManager's non-player (AI) teams lacked proactive decision-making. Before this system, AI teams relied on:

- A passive contract renewal evaluation that only reacted to human-player renewals.
- A safety net (`roster_stability`) that generated emergency placeholder players when rosters fell below match-ready minimums.
- No buying, selling, or retention strategy — rosters decayed over time as contracts expired and players left.

The result was that non-player teams became progressively weaker, making league competition unbalanced and reducing the realism of the simulation.

At the same time, players on AI teams had no autonomy:
- They never requested transfers, even when unhappy or underpaid.
- They never demanded contract renewals.
- Their career development was entirely passive.

We needed a system that gave non-player teams and their players autonomous, context-aware decision-making — without schema changes (no new fields on existing models).

## Decision

Adopt a **two-agent architecture** with staggered processing:

### Team Agent (`ai_team_agent.rs`)
- Manages roster decisions for non-player teams: retention scoring, contract renewals, player sales, and free agent purchases.
- Processes 2–3 teams per day in round-robin fashion (staggered to spread workload and avoid performance spikes).
- Retention decisions weigh: `lol_ovr` (35 %), `avg_rating` (20 %), age (15 %), contract security (10 %), wage-value ratio (10 %), and trait bonuses (10 %).
- Sales decisions flag players via a deadweight score (low OVR + high wage + short contract).
- Purchase decisions fill role gaps (positions with 0 players) from the free agent pool.

### Player Agent (`ai_player_agent.rs`)
- Manages player career decisions: satisfaction scoring, transfer requests, renewal demands.
- Processes players on teams that were processed by the Team Agent **yesterday** (day-offset design).
- Satisfaction weighs: morale (40 %), manager trust (30 %), wage satisfaction (15 %), ambition alignment (10 %), loyalty (5 %).
- Actions are limited to setting flags: `transfer_listed` for transfer requests, `renewal_state` for renewal demands. Never modifies roster directly (PA-06).

### Orchestration
- Day N: Team Agent processes 2-3 AI teams (renewals, sales, purchases).
- Day N+1: Player Agent processes players on those same teams (satisfaction → decisions).
- Day N+1 (after Player Agent): Conflict resolution (`resolve_conflicts`) — Team Agent overrides Player Agent for under-contract players who are high-value (retention score ≥ 0.70) or critical depth (only player at a role with > 12 months remaining).
- Day N+1 (after conflict resolution): News generation (`generate_ai_transfer_news`).
- Call order in `turn/mod.rs`: `process_ai_team_agents` → `process_ai_player_agents` → `resolve_conflicts` → `generate_ai_transfer_news`.

### Key Design Constraints
- **No schema changes**: All decisions derive from existing `Player` and `Team` fields (stats, traits, morale, contract, wage, etc.).
- **Deterministic**: No RNG in agent decisions — same input state always produces same output (OR-06).
- **Safety net above**: Agents sit ABOVE `roster_stability`. The safety net catches edge cases the agents miss (e.g., mass departures, scenario corner cases).
- **Under-contract override**: Team Agent overrides Player Agent for under-contract players (OR-03).
- **Free agent final**: Player Agent decision is final for free agents — no override (OR-04).

## Consequences

### Positive
- AI teams now proactively manage rosters: renew stars, sell deadweight, fill gaps.
- Players on AI teams have career autonomy: they leave unhappy situations, demand better contracts.
- League competition is more realistic — AI teams maintain competitive rosters.
- No schema changes — zero migration risk.
- Deterministic decisions make testing and debugging straightforward.
- Staggered processing (2-3 teams/day) avoids CPU spikes.

### Negative
- Conflict resolution adds complexity: Team Agent can undo Player Agent decisions.
- Staggered processing means a team gets full agent attention only every ~10 days (for a 30-team league). This is acceptable because most decisions are not time-critical, but fast-reaction scenarios (e.g., morale crisis) might lag.
- The system adds ~400 lines of agent logic plus tests.

### Safety Net
- `roster_stability` remains as the ultimate fallback. If agents fail to maintain a minimum roster (e.g., all players leave, extreme edge case), `roster_stability::repair_league` generates emergency players. The agent system reduces how often this fires but does not replace it.

### Future Considerations
- If player churn is too low or too high, tune the satisfaction thresholds (`SATISFACTION_LOW`, `SATISFACTION_HIGH`, `TRULY_MISERABLE`).
- If AI teams over-perform or under-perform, tune retention/deadweight weights.
- The system can be extended to handle player development desires (playtime, role preference) without schema changes.
