use crate::domain::player::{Player, PlayerTrait};
use crate::domain::team::Team;
use crate::game::Game;
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Retention scoring weights (design: retention = Σ(weight × factor))
// ---------------------------------------------------------------------------
const W_LOL_OVR: f64 = 0.35;
const W_AVG_RATING: f64 = 0.20;
const W_AGE: f64 = 0.15;
const W_CONTRACT_SECURITY: f64 = 0.10;
const W_WAGE_VALUE: f64 = 0.10;
const W_TRAIT_BONUS: f64 = 0.10;

/// Soft cap multiplier on wage_budget: we will not exceed 110 % of budget.
const WAGE_SOFT_CAP_MULTIPLIER: f64 = 1.10;

/// Max teams processed per day in staggered mode.
const TEAMS_PER_DAY: usize = 3;

// ---------------------------------------------------------------------------
// Helpers (extracted for REFACTOR)
// ---------------------------------------------------------------------------

/// Age factor: younger → higher score.
/// Returns 1.0 for age ≤ 20, linear decrease to 0.0 at age ≥ 35.
pub fn age_factor(age: u8) -> f64 {
    if age <= 20 {
        1.0
    } else if age >= 35 {
        0.0
    } else {
        1.0 - (age as f64 - 20.0) / 15.0
    }
}

/// Contract security: more remaining years → higher score.
/// Returns 1.0 for ≥ 3 years, linear to 0.0 for 0 years.
pub fn contract_security(contract_end: Option<&str>, today: NaiveDate) -> f64 {
    let end = match contract_end.and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()) {
        Some(d) => d,
        None => return 0.0, // no contract = no security
    };
    let years_remaining: f64 =
        (end.signed_duration_since(today).num_days() as f64 / 365.25).clamp(0.0, f64::MAX);
    (years_remaining / 3.0).clamp(0.0, 1.0)
}

/// Wage-value ratio: wage ≤ expected → good (≥ 1.0).
/// Returns 1.0 if wage ≤ expected_wage, down to 0.3 at extreme overpay.
pub fn wage_value_ratio(wage: u32, expected_wage: u32) -> f64 {
    if expected_wage == 0 {
        return 0.5; // unknown baseline
    }
    let ratio = wage as f64 / expected_wage as f64;
    if ratio <= 1.0 {
        1.0
    } else {
        (1.0 - (ratio - 1.0) * 0.5).clamp(0.3, 1.0)
    }
}

/// Trait bonus: key traits increase retention score.
/// HyperCarry, ShotCaller, IceCold, Visionary → +0.2 each.
/// TeamPlayer, Sentinel, Workhorse → +0.1 each.
pub fn trait_bonus(traits: &[PlayerTrait]) -> f64 {
    let mut bonus: f64 = 0.0;
    for t in traits {
        match t {
            PlayerTrait::HyperCarry
            | PlayerTrait::ShotCaller
            | PlayerTrait::IceCold
            | PlayerTrait::Visionary => bonus += 0.2,
            PlayerTrait::TeamPlayer | PlayerTrait::Sentinel | PlayerTrait::Workhorse => bonus += 0.1,
            _ => {}
        }
    }
    bonus.clamp(0.0, 1.0)
}

/// Compute player age from date_of_birth string.
pub fn player_age(date_of_birth: &str, today: NaiveDate) -> u8 {
    let birth = match NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return 25, // fallback
    };
    let mut age = today.year() - birth.year();
    // Birthday check: use month*100+day to avoid leap-year ordinal skew
    if today.format("%m%d").to_string() < birth.format("%m%d").to_string() {
        age -= 1;
    }
    age.max(0) as u8
}

// ---------------------------------------------------------------------------
// Composite retention score
// ---------------------------------------------------------------------------

/// Compute a 0..1 retention score for a player relative to their team.
/// Uses EXISTING player/team fields only — no schema changes.
pub fn retention_score(player: &Player, team: &Team, today: NaiveDate) -> f64 {
    let lol_ovr = f64::from(player.lol_ovr) / 99.0; // normalise 0..1

    let rating = f64::from(player.stats.avg_rating) / 10.0; // avg_rating is 0..10, normalise

    let age = player_age(&player.date_of_birth, today);
    let age_f = age_factor(age);

    let cs = contract_security(player.contract_end.as_deref(), today);

    // Expected wage from contracts module
    let expected = crate::contracts::expected_wage(player, team, today);
    let wvr = wage_value_ratio(player.wage, expected);

    let tb = trait_bonus(&player.traits);

    lol_ovr * W_LOL_OVR
        + rating * W_AVG_RATING
        + age_f * W_AGE
        + cs * W_CONTRACT_SECURITY
        + wvr * W_WAGE_VALUE
        + tb * W_TRAIT_BONUS
}

// ---------------------------------------------------------------------------
// Roster report
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct RosterReport {
    /// Number of players per LolRole
    pub role_depth: HashMap<crate::domain::stats::LolRole, u32>,
    /// Average OVR per role
    pub avg_ovr_by_role: HashMap<crate::domain::stats::LolRole, f64>,
    /// Player IDs sorted by retention score (descending)
    pub retention_scores: Vec<(String, f64)>,
    /// Player IDs whose score suggests renewal
    pub renewal_candidates: Vec<String>,
    /// Roles with 0 players
    pub role_gaps: Vec<crate::domain::stats::LolRole>,
}

/// Assess a team's roster and produce a RosterReport.
pub fn assess_roster(team: &Team, players: &[Player], today: NaiveDate) -> RosterReport {
    use crate::domain::stats::LolRole;

    let mut role_depth: HashMap<LolRole, u32> = HashMap::new();
    let mut ovr_sum: HashMap<LolRole, u64> = HashMap::new();
    let mut ovr_count: HashMap<LolRole, u32> = HashMap::new();

    // Initialise all roles
    for role in &[
        LolRole::Top,
        LolRole::Jungle,
        LolRole::Mid,
        LolRole::Adc,
        LolRole::Support,
    ] {
        role_depth.insert(*role, 0);
        ovr_sum.insert(*role, 0);
        ovr_count.insert(*role, 0);
    }

    let team_players: Vec<&Player> = players
        .iter()
        .filter(|p| p.team_id.as_deref() == Some(&team.id))
        .collect();

    for p in &team_players {
        let role = p.natural_position;
        *role_depth.entry(role).or_insert(0) += 1;
        *ovr_sum.entry(role).or_insert(0) += u64::from(p.lol_ovr);
        *ovr_count.entry(role).or_insert(0) += 1;
    }

    let avg_ovr_by_role: HashMap<LolRole, f64> = ovr_sum
        .into_iter()
        .map(|(role, sum)| {
            let count = *ovr_count.get(&role).unwrap_or(&1);
            (role, sum as f64 / count.max(1) as f64)
        })
        .collect();

    let mut retention_scores: Vec<(String, f64)> = team_players
        .iter()
        .map(|p| {
            let score = retention_score(p, team, today);
            (p.id.clone(), score)
        })
        .collect();
    retention_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let renewal_candidates: Vec<String> = retention_scores
        .iter()
        .filter(|(_, score)| *score >= 0.7)
        .map(|(id, _)| id.clone())
        .collect();

    let role_gaps: Vec<LolRole> = role_depth
        .iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(role, _)| *role)
        .collect();

    RosterReport {
        role_depth,
        avg_ovr_by_role,
        retention_scores,
        renewal_candidates,
        role_gaps,
    }
}

// ---------------------------------------------------------------------------
// Renewal decisions
// ---------------------------------------------------------------------------

/// Evaluate which players on a team should get renewal offers.
/// Sets `player.morale_core.renewal_state` for eligible players.
pub fn evaluate_renewals(team: &Team, players: &mut [Player], report: &RosterReport, today: NaiveDate) {
    use crate::domain::player::{ContractRenewalState, RenewalSessionStatus};

    let soft_cap_wage = (team.wage_budget as f64 * WAGE_SOFT_CAP_MULTIPLIER) as i64;
    let current_wage_bill: i64 = players
        .iter()
        .filter(|p| p.team_id.as_deref() == Some(&team.id))
        .map(|p| i64::from(p.wage))
        .sum();

    let mut processed: std::collections::HashSet<String> = std::collections::HashSet::new();

    for player in players.iter_mut() {
        if player.team_id.as_deref() != Some(&team.id) {
            continue;
        }

        // No-deadlock guard: skip already processed
        if !processed.insert(player.id.clone()) {
            continue;
        }

        // Find their retention score from the report
        let score = report
            .retention_scores
            .iter()
            .find(|(id, _)| id == &player.id)
            .map(|(_, s)| *s)
            .unwrap_or(0.5);

        // Spec TA-02: Skip underperformer
        let is_underperformer =
            player.stats.avg_rating < 6.0 || player.lol_ovr < 65;
        if is_underperformer {
            continue;
        }

        // Skip players with zero wage (data anomaly / free agent edge case)
        if player.wage == 0 {
            continue;
        }

        // Check budget headroom
        let projected_wage = if player.wage > 0 {
            // Offer slight increase
            (player.wage as f64 * 1.10) as i64
        } else {
            player.wage as i64
        };

        let projected_bill = current_wage_bill - i64::from(player.wage) + projected_wage;
        if projected_bill > soft_cap_wage {
            continue; // budget exhausted
        }

        // Spec TA-02: Renew high performer
        let is_high_performer = player.stats.avg_rating > 7.5
            && player.lol_ovr > 80
            && contract_security(player.contract_end.as_deref(), today) < 0.5; // < ~18 months

        if score >= 0.7 || is_high_performer {
            if player.morale_core.renewal_state.is_none() {
                player.morale_core.renewal_state = Some(ContractRenewalState {
                    status: RenewalSessionStatus::Open,
                    ..ContractRenewalState::default()
                });
            } else if let Some(ref mut state) = player.morale_core.renewal_state {
                if state.status == RenewalSessionStatus::Idle {
                    state.status = RenewalSessionStatus::Open;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Staggered processing
// ---------------------------------------------------------------------------

/// Select non-player, non-academy teams for processing today using round-robin.
fn select_ai_teams_for_today(game: &Game) -> Vec<String> {
    let ai_teams: Vec<&Team> = game
        .teams
        .iter()
        .filter(|t| {
            t.team_kind == crate::domain::team::TeamKind::Main && t.manager_id.is_none()
        })
        .collect();

    if ai_teams.is_empty() {
        return Vec::new();
    }

    // Use day-of-year as round-robin offset
    let day_of_year = game
        .clock
        .current_date
        .format("%j")
        .to_string()
        .parse::<usize>()
        .unwrap_or(0);
    let count = TEAMS_PER_DAY.min(ai_teams.len());
    let start = day_of_year % ai_teams.len();

    (0..count)
        .map(|i| {
            let idx = (start + i) % ai_teams.len();
            ai_teams[idx].id.clone()
        })
        .collect()
}

/// Public entry point called from turn/mod.rs.
/// Processes up to 3 AI teams per day: assess roster + evaluate renewals.
pub fn process_ai_team_agents(game: &mut Game) {
    let today = game.clock.current_date.date_naive();

    let team_ids = select_ai_teams_for_today(game);

    for team_id in &team_ids {
        // Borrow team from game — the borrow checker allows
        // simultaneous &game.teams + &mut game.players since they
        // are distinct struct fields.
        let team = match game.teams.iter().find(|t| t.id == *team_id) {
            Some(t) => t,
            None => continue,
        };

        let report = assess_roster(team, &game.players, today);

        // Apply renewal decisions (mutable borrow of game.players)
        evaluate_renewals(team, &mut game.players, &report, today);

        // Stub: evaluate_sales (PR 2), evaluate_purchases (PR 2)
        // No-ops for now.
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::GameClock;
    use crate::domain::manager::Manager;
    use crate::domain::player::{
        LolRole, Player, PlayerAttributes, PlayerSeasonStats,
    };
    use crate::domain::player::RenewalSessionStatus;
    use crate::domain::team::Team;
    use chrono::{TimeZone, Utc};

    fn default_attrs() -> PlayerAttributes {
        PlayerAttributes {
            mechanics: 70,
            laning: 70,
            teamfighting: 70,
            macro_play: 70,
            consistency: 70,
            shotcalling: 70,
            champion_pool: 70,
            discipline: 70,
            mental_resilience: 70,
        }
    }

    fn make_player(
        id: &str,
        name: &str,
        team_id: &str,
        role: LolRole,
        lol_ovr: u8,
        avg_rating: f32,
        wage: u32,
        contract_end: Option<&str>,
        age: &str,
        traits: Vec<PlayerTrait>,
    ) -> Player {
        let mut player = Player::new(
            id.to_string(),
            name.to_string(),
            format!("Full {}", name),
            age.to_string(),
            "GB".to_string(),
            role,
            default_attrs(),
        );
        player.team_id = Some(team_id.to_string());
        player.lol_ovr = lol_ovr;
        player.stats = PlayerSeasonStats {
            avg_rating,
            ..PlayerSeasonStats::default()
        };
        player.wage = wage;
        player.contract_end = contract_end.map(|s| s.to_string());
        player.traits = traits;
        player
    }

    fn make_team(id: &str, name: &str) -> Team {
        let mut team = Team::new(
            id.to_string(),
            name.to_string(),
            name[..3].to_string(),
            "DE".to_string(),
            "Berlin".to_string(),
            "Arena".to_string(),
            10_000,
        );
        team.wage_budget = 500_000;
        team.transfer_budget = 200_000;
        team
    }

    fn test_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 6, 15).unwrap()
    }

    // Phase 1 — Scoring core

    #[test]
    fn test_age_factor_young() {
        assert!((age_factor(20) - 1.0).abs() < 0.001);
        assert!((age_factor(18) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_age_factor_old() {
        assert!((age_factor(35) - 0.0).abs() < 0.001);
        assert!((age_factor(40) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_age_factor_mid() {
        let f = age_factor(27);
        assert!(f > 0.0 && f < 1.0);
        assert!((f - (1.0 - 7.0 / 15.0)).abs() < 0.001);
    }

    #[test]
    fn test_contract_security_no_contract() {
        assert!((contract_security(None, test_date()) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_contract_security_long() {
        let end = "2029-06-15";
        let sec = contract_security(Some(end), test_date());
        assert!(sec > 0.9);
    }

    #[test]
    fn test_contract_security_short() {
        let end = "2026-09-15";
        let sec = contract_security(Some(end), test_date());
        assert!(sec > 0.0 && sec < 0.3);
    }

    #[test]
    fn test_wage_value_ratio_good_value() {
        assert!((wage_value_ratio(50_000, 60_000) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_wage_value_ratio_overpaid() {
        let r = wage_value_ratio(100_000, 50_000);
        assert!(r < 1.0 && r >= 0.3);
    }

    #[test]
    fn test_trait_bonus_key_traits() {
        let traits = vec![PlayerTrait::HyperCarry, PlayerTrait::ShotCaller];
        let bonus = trait_bonus(&traits);
        assert!((bonus - 0.4).abs() < 0.001);
    }

    #[test]
    fn test_trait_bonus_no_traits() {
        let bonus = trait_bonus(&[]);
        assert!((bonus - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_retention_score_high_performer() {
        let team = make_team("t1", "Test Team");
        let player = make_player(
            "p1", "Star", "t1", LolRole::Mid,
            95,   // lol_ovr
            8.5,  // avg_rating
            60_000, // wage
            Some("2027-06-15"), // contract_end
            "2002-01-01", // age ~24
            vec![PlayerTrait::HyperCarry],
        );
        let score = retention_score(&player, &team, test_date());
        assert!(score > 0.7, "retention_score should be high for star, got {score}");
    }

    #[test]
    fn test_retention_score_low_performer() {
        let team = make_team("t1", "Test Team");
        let player = make_player(
            "p2", "Bust", "t1", LolRole::Top,
            55,   // low lol_ovr
            5.0,  // low avg_rating
            80_000, // overpaid
            Some("2026-09-15"), // contract ending soon
            "1995-01-01", // age ~31
            vec![],
        );
        let score = retention_score(&player, &team, test_date());
        assert!(score < 0.5, "retention_score should be low for bust, got {score}");
    }

    // Phase 1 — assess_roster

    #[test]
    fn test_assess_roster_basic() {
        let team = make_team("t1", "Test Team");
        let player = make_player(
            "p1", "Star", "t1", LolRole::Mid,
            90, 8.0, 60_000, Some("2027-06-15"), "2002-01-01", vec![],
        );
        let report = assess_roster(&team, &[player], test_date());
        assert_eq!(*report.role_depth.get(&LolRole::Mid).unwrap(), 1);
        assert!(*report.role_depth.get(&LolRole::Top).unwrap() == 0);
        assert!(report.role_gaps.contains(&LolRole::Top));
        assert!(!report.retention_scores.is_empty());
    }

    #[test]
    fn test_assess_roster_empty() {
        let team = make_team("t1", "Test Team");
        let report = assess_roster(&team, &[], test_date());
        assert_eq!(report.role_gaps.len(), 5); // all roles are gaps
    }

    #[test]
    fn test_assess_roster_role_gaps_identified() {
        let team = make_team("t1", "Test Team");
        let players = vec![
            make_player("p1", "Top", "t1", LolRole::Top, 80, 7.0, 50_000, Some("2027-06-15"), "2002-01-01", vec![]),
            make_player("p2", "Mid", "t1", LolRole::Mid, 85, 7.5, 55_000, Some("2027-06-15"), "2002-01-01", vec![]),
        ];
        let report = assess_roster(&team, &players, test_date());
        assert!(report.role_gaps.contains(&LolRole::Jungle));
        assert!(report.role_gaps.contains(&LolRole::Adc));
        assert!(report.role_gaps.contains(&LolRole::Support));
        assert!(!report.role_gaps.contains(&LolRole::Top));
        assert!(!report.role_gaps.contains(&LolRole::Mid));
    }

    // Phase 2 — Renewal decisions

    #[test]
    fn test_evaluate_renewals_high_performer_gets_offer() {
        let team = make_team("t1", "Test Team");
        let today = test_date();
        let mut players = vec![
            make_player(
                "p1", "Star", "t1", LolRole::Mid,
                85, 8.0, 50_000,
                Some("2026-09-15"), // < 6 months
                "2002-01-01",
                vec![],
            ),
        ];
        let report = assess_roster(&team, &players, today);
        evaluate_renewals(&team, &mut players, &report, today);

        let state = players[0].morale_core.renewal_state.as_ref();
        assert!(state.is_some(), "high performer should get renewal state");
        if let Some(s) = state {
            assert_eq!(s.status, RenewalSessionStatus::Open);
        }
    }

    #[test]
    fn test_evaluate_renewals_skip_underperformer() {
        let team = make_team("t1", "Test Team");
        let today = test_date();
        let mut players = vec![
            make_player(
                "p2", "Bust", "t1", LolRole::Top,
                55, 5.0, 50_000,
                Some("2026-09-15"),
                "1995-01-01",
                vec![],
            ),
        ];
        let report = assess_roster(&team, &players, today);
        evaluate_renewals(&team, &mut players, &report, today);

        assert!(
            players[0].morale_core.renewal_state.is_none(),
            "underperformer should NOT get renewal state"
        );
    }

    #[test]
    fn test_evaluate_renewals_budget_exhausted() {
        let mut team = make_team("t1", "Poor Team");
        team.wage_budget = 50_000; // tight budget
        let today = test_date();
        let mut players = vec![
            make_player(
                "p1", "Star", "t1", LolRole::Mid,
                85, 8.0, 90_000, // already expensive
                Some("2026-09-15"),
                "2002-01-01",
                vec![PlayerTrait::HyperCarry],
            ),
        ];
        // Override lol_ovr since assess_roster reads it directly (no Game::new refresh here)
        players[0].lol_ovr = 85;

        let report = assess_roster(&team, &players, today);
        evaluate_renewals(&team, &mut players, &report, today);

        // Soft cap = 50_000 * 1.10 = 55_000, projected = 90_000 * 1.10 = 99_000 > 55_000
        let state = &players[0].morale_core.renewal_state;
        let is_open = matches!(state, Some(s) if s.status == RenewalSessionStatus::Open);
        assert!(!is_open, "renewal should be skipped when budget is exhausted");
    }

    #[test]
    fn test_no_deadlock_same_player_not_processed_twice() {
        let team = make_team("t1", "Test Team");
        let today = test_date();
        let mut players = vec![
            make_player(
                "p1", "Star", "t1", LolRole::Mid,
                85, 8.0, 50_000,
                Some("2026-09-15"),
                "2002-01-01",
                vec![],
            ),
            // Duplicate same player ID to test deadlock guard
            make_player(
                "p1", "Star Clone", "t1", LolRole::Mid,
                85, 8.0, 50_000,
                Some("2026-09-15"),
                "2002-01-01",
                vec![],
            ),
        ];
        // Override lol_ovr since assess_roster reads it directly (no Game::new refresh here)
        players[0].lol_ovr = 85;
        players[1].lol_ovr = 85;

        let report = assess_roster(&team, &players, today);
        evaluate_renewals(&team, &mut players, &report, today);

        // Both have same ID, but the no-deadlock guard ensures
        // only one is processed (players[0]). The second is skipped,
        // so it retains its default state (None) — spec I-04.
        // Verify no panic, and first entry gets processed.
        assert!(
            players[0].morale_core.renewal_state.is_some(),
            "first occurrence should get renewal state"
        );
        assert!(
            players[1].morale_core.renewal_state.is_none(),
            "duplicate should be skipped by deadlock guard"
        );
    }

    #[test]
    fn test_process_ai_team_agents_no_panic() {
        use crate::domain::team::TeamKind;

        let clock = GameClock::new(Utc.with_ymd_and_hms(2026, 6, 15, 12, 0, 0).unwrap());
        let manager = Manager::new(
            "mgr1".to_string(), "Test".to_string(), "Manager".to_string(),
            "1980-01-01".to_string(), "GB".to_string(),
        );
        let mut team = make_team("ai_team", "AI Team");
        team.team_kind = TeamKind::Main;
        team.manager_id = None; // AI team

        let player = make_player(
            "p1", "Star", "ai_team", LolRole::Mid,
            85, 8.0, 50_000, Some("2026-09-15"), "2002-01-01", vec![],
        );
        let mut game = Game::new(clock, manager, vec![team], vec![player], vec![], vec![]);
        // Game::new refreshes lol_ovr from attributes — override to test high performer logic
        game.players[0].lol_ovr = 85;
        process_ai_team_agents(&mut game);

        // Verify no panic, renewal states set
        let star = game.players.iter().find(|p| p.id == "p1").unwrap();
        assert!(
            star.morale_core.renewal_state.is_some(),
            "star player should have renewal state set"
        );
    }

    #[test]
    fn test_process_ai_team_agents_empty_roster() {
        use crate::domain::team::TeamKind;

        let clock = GameClock::new(Utc.with_ymd_and_hms(2026, 6, 15, 12, 0, 0).unwrap());
        let manager = Manager::new(
            "mgr1".to_string(), "Test".to_string(), "Manager".to_string(),
            "1980-01-01".to_string(), "GB".to_string(),
        );
        let mut team = make_team("ai_team", "AI Team");
        team.team_kind = TeamKind::Main;
        team.manager_id = None;

        let mut game = Game::new(clock, manager, vec![team], vec![], vec![], vec![]);
        process_ai_team_agents(&mut game);
        // Should not panic with 0 eligible players
    }

    #[test]
    fn test_player_age_computation() {
        let today = NaiveDate::from_ymd_opt(2026, 6, 15).unwrap();
        assert_eq!(player_age("2000-01-01", today), 26);
        assert_eq!(player_age("1995-06-15", today), 31);
        assert_eq!(player_age("2020-06-15", today), 6);
    }
}
