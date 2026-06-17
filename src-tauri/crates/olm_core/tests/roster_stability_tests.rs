use chrono::{TimeZone, Utc};
use olm_core::clock::GameClock;
use olm_core::domain::league::{
    Fixture, FixtureStatus, League, LeagueKind, MatchType, StandingEntry,
};
use olm_core::domain::manager::Manager;
use olm_core::domain::player::{Player, PlayerAttributes};
use olm_core::domain::season::TransferWindowStatus;
use olm_core::domain::stats::LolRole;
use olm_core::domain::team::Team;
use olm_core::game::Game;
use olm_core::roster_stability::{
    RepairAction, RosterStabilityReason, evaluate_team, repair_league, repair_team,
};

fn attrs() -> PlayerAttributes {
    PlayerAttributes {
        mental_resilience: 60,
        champion_pool: 60,
        laning: 60,
        mechanics: 60,
        macro_play: 60,
        consistency: 60,
        discipline: 60,
        teamfighting: 60,
        shotcalling: 60,
    }
}

fn player(id: &str, team_id: Option<&str>, role: LolRole, contract_end: Option<&str>) -> Player {
    let mut player = Player::new(
        id.to_string(),
        id.to_string(),
        format!("{id} Test"),
        "2000-01-01".to_string(),
        "Spain".to_string(),
        role,
        attrs(),
    );
    player.team_id = team_id.map(str::to_string);
    player.contract_end = contract_end.map(str::to_string);
    player.wage = 50_000;
    player.market_value = 500_000;
    player
}

fn team(id: &str, manager_id: Option<&str>, lineup: Vec<&str>) -> Team {
    let mut team = Team::new(
        id.to_string(),
        format!("{id} Esports"),
        id.chars().take(3).collect::<String>().to_uppercase(),
        "Spain".to_string(),
        "Madrid".to_string(),
        "Arena".to_string(),
        10_000,
    );
    team.manager_id = manager_id.map(str::to_string);
    team.active_lineup_ids = lineup.into_iter().map(str::to_string).collect();
    team
}

fn league_with_team(team_id: &str) -> League {
    League {
        id: "league-1".to_string(),
        name: "League One".to_string(),
        season: 2026,
        fixtures: vec![Fixture {
            id: "fixture-1".to_string(),
            matchday: 1,
            date: "2026-08-02".to_string(),
            home_team_id: team_id.to_string(),
            away_team_id: "other-ai".to_string(),
            match_type: MatchType::League,
            best_of: 1,
            status: FixtureStatus::Scheduled,
            result: None,
        }],
        standings: vec![StandingEntry::new(team_id.to_string())],
        competition_id: Some("competition-1".to_string()),
        logo: None,
        league_kind: LeagueKind::Main,
        split_index: 0,
        tier: 1,
        active: true,
    }
}

fn game_with(players: Vec<Player>, lineup: Vec<&str>) -> Game {
    let clock = GameClock::new(Utc.with_ymd_and_hms(2026, 8, 1, 12, 0, 0).unwrap());
    let mut manager = Manager::new(
        "manager-1".to_string(),
        "Jane".to_string(),
        "Doe".to_string(),
        "1980-01-01".to_string(),
        "Spain".to_string(),
    );
    manager.hire("user-team".to_string());

    let mut game = Game::new(
        clock,
        manager,
        vec![
            team("ai-team", None, lineup),
            team("other-ai", None, vec![]),
        ],
        players,
        vec![],
        vec![],
    );
    game.leagues = vec![league_with_team("ai-team")];
    game.user_competition_id = Some("competition-1".to_string());
    game
}

fn full_roster(team_id: &str) -> Vec<Player> {
    vec![
        player("top", Some(team_id), LolRole::Top, Some("2028-06-30")),
        player("jungle", Some(team_id), LolRole::Jungle, Some("2028-06-30")),
        player("mid", Some(team_id), LolRole::Mid, Some("2028-06-30")),
        player("adc", Some(team_id), LolRole::Adc, Some("2028-06-30")),
        player(
            "support",
            Some(team_id),
            LolRole::Support,
            Some("2028-06-30"),
        ),
    ]
}

#[test]
fn valid_roster_evaluates_without_repair_actions() {
    let game = game_with(
        full_roster("ai-team"),
        vec!["top", "jungle", "mid", "adc", "support"],
    );

    let evaluation = evaluate_team(&game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("team should evaluate");

    assert!(evaluation.match_eligible);
    assert_eq!(evaluation.eligible_player_count, 5);
    assert!(evaluation.missing_roles.is_empty());
    assert!(evaluation.stale_lineup_ids.is_empty());
}

#[test]
fn below_minimum_roster_is_repaired_with_generated_players() {
    let mut game = game_with(
        vec![
            player("top", Some("ai-team"), LolRole::Top, Some("2028-06-30")),
            player(
                "jungle",
                Some("ai-team"),
                LolRole::Jungle,
                Some("2028-06-30"),
            ),
            player("mid", Some("ai-team"), LolRole::Mid, Some("2028-06-30")),
            player("adc", Some("ai-team"), LolRole::Adc, Some("2028-06-30")),
        ],
        vec!["top", "jungle", "mid", "adc"],
    );

    let report = repair_team(&mut game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("generated emergency player should repair below-minimum roster");
    let evaluation = evaluate_team(&game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("team should evaluate after repair");

    assert!(evaluation.match_eligible);
    assert!(report.actions.iter().any(|action| matches!(
        action,
        RepairAction::GeneratedReplacement {
            role: LolRole::Support,
            ..
        }
    )));
}

#[test]
fn missing_role_prefers_role_fit_free_agent() {
    let mut players = vec![
        player("top", Some("ai-team"), LolRole::Top, Some("2028-06-30")),
        player(
            "jungle",
            Some("ai-team"),
            LolRole::Jungle,
            Some("2028-06-30"),
        ),
        player("mid", Some("ai-team"), LolRole::Mid, Some("2028-06-30")),
        player("adc", Some("ai-team"), LolRole::Adc, Some("2028-06-30")),
        player("adc2", Some("ai-team"), LolRole::Adc, Some("2028-06-30")),
        player("free-support", None, LolRole::Support, Some("2028-06-30")),
    ];
    players[5].market_value = 100_000;
    let mut game = game_with(players, vec!["top", "jungle", "mid", "adc", "adc2"]);

    let report = repair_team(&mut game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("free support should repair role gap");

    assert_eq!(
        game.players
            .iter()
            .find(|player| player.id == "free-support")
            .unwrap()
            .team_id
            .as_deref(),
        Some("ai-team")
    );
    assert!(report.actions.iter().any(|action| matches!(
        action,
        RepairAction::AssignedFreeAgent { player_id, role: LolRole::Support } if player_id == "free-support"
    )));
}

#[test]
fn stale_lineup_ids_are_reconciled_to_current_eligible_players() {
    let mut game = game_with(
        full_roster("ai-team"),
        vec!["top", "released-player", "mid", "adc", "support"],
    );

    let report = repair_team(&mut game, "ai-team", RosterStabilityReason::LoadMigration)
        .expect("stale lineup should reconcile");
    let lineup = &game
        .teams
        .iter()
        .find(|team| team.id == "ai-team")
        .unwrap()
        .active_lineup_ids;

    assert_eq!(lineup.len(), 5);
    assert!(!lineup.iter().any(|id| id == "released-player"));
    assert!(lineup.iter().any(|id| id == "jungle"));
    assert!(
        report
            .actions
            .iter()
            .any(|action| matches!(action, RepairAction::ReconciledLineup { .. }))
    );
}

#[test]
fn closed_window_emergency_repairs_and_reports_policy_exception() {
    let mut game = game_with(
        vec![
            player("top", Some("ai-team"), LolRole::Top, Some("2028-06-30")),
            player(
                "jungle",
                Some("ai-team"),
                LolRole::Jungle,
                Some("2028-06-30"),
            ),
            player("mid", Some("ai-team"), LolRole::Mid, Some("2028-06-30")),
            player("adc", Some("ai-team"), LolRole::Adc, Some("2028-06-30")),
        ],
        vec!["top", "jungle", "mid", "adc"],
    );
    game.season_context.transfer_window.status = TransferWindowStatus::Closed;

    let report = repair_team(
        &mut game,
        "ai-team",
        RosterStabilityReason::BackgroundSimulation,
    )
    .expect("emergency policy should bypass closed transfer window");

    assert!(
        report
            .policy_exceptions
            .iter()
            .any(|exception| exception == "transfer_window_closed")
    );
}

#[test]
fn closed_window_expired_bench_without_repair_does_not_report_policy_exception() {
    let mut players = full_roster("ai-team");
    players.push(player(
        "expired-bench",
        Some("ai-team"),
        LolRole::Support,
        Some("2026-07-31"),
    ));
    let mut game = game_with(players, vec!["top", "jungle", "mid", "adc", "support"]);
    game.season_context.transfer_window.status = TransferWindowStatus::Closed;

    let report = repair_team(
        &mut game,
        "ai-team",
        RosterStabilityReason::BackgroundSimulation,
    )
    .expect("eligible active roster should remain a no-op despite expired bench players");

    assert!(report.before.match_eligible);
    assert!(report.after.match_eligible);
    assert!(report.actions.is_empty());
    assert!(
        !report
            .policy_exceptions
            .iter()
            .any(|exception| exception == "transfer_window_closed")
    );
}

#[test]
fn free_agent_starvation_uses_deterministic_generated_replacement() {
    let mut game = game_with(
        vec![
            player("top", Some("ai-team"), LolRole::Top, Some("2028-06-30")),
            player(
                "jungle",
                Some("ai-team"),
                LolRole::Jungle,
                Some("2028-06-30"),
            ),
            player("mid", Some("ai-team"), LolRole::Mid, Some("2028-06-30")),
            player(
                "support",
                Some("ai-team"),
                LolRole::Support,
                Some("2028-06-30"),
            ),
        ],
        vec!["top", "jungle", "mid", "support"],
    );

    let report = repair_team(&mut game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("generated ADC should repair free-agent starvation");

    assert!(game.players.iter().any(|player| {
        player.id == "emergency-ai-team-prematch-adc"
            && player.team_id.as_deref() == Some("ai-team")
            && player.natural_position == LolRole::Adc
    }));
    assert!(report.actions.iter().any(|action| matches!(
        action,
        RepairAction::GeneratedReplacement { player_id, role: LolRole::Adc } if player_id == "emergency-ai-team-prematch-adc"
    )));
}

#[test]
fn generated_replacement_collision_uses_next_deterministic_id() {
    let mut game = game_with(
        vec![
            player("top", Some("ai-team"), LolRole::Top, Some("2028-06-30")),
            player(
                "jungle",
                Some("ai-team"),
                LolRole::Jungle,
                Some("2028-06-30"),
            ),
            player("mid", Some("ai-team"), LolRole::Mid, Some("2028-06-30")),
            player(
                "support",
                Some("ai-team"),
                LolRole::Support,
                Some("2028-06-30"),
            ),
            player(
                "emergency-ai-team-prematch-adc",
                Some("other-ai"),
                LolRole::Adc,
                Some("2028-06-30"),
            ),
        ],
        vec!["top", "jungle", "mid", "support"],
    );

    let report = repair_team(&mut game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("ID collision should still make deterministic repair progress");

    assert!(game.players.iter().any(|player| {
        player.id == "emergency-ai-team-prematch-adc-2"
            && player.team_id.as_deref() == Some("ai-team")
            && player.natural_position == LolRole::Adc
    }));
    assert!(report.actions.iter().any(|action| matches!(
        action,
        RepairAction::GeneratedReplacement { player_id, role: LolRole::Adc } if player_id == "emergency-ai-team-prematch-adc-2"
    )));
}

#[test]
fn generated_replacement_wrong_role_collision_uses_next_deterministic_id() {
    let mut game = game_with(
        vec![
            player("top", Some("ai-team"), LolRole::Top, Some("2028-06-30")),
            player(
                "jungle",
                Some("ai-team"),
                LolRole::Jungle,
                Some("2028-06-30"),
            ),
            player("mid", Some("ai-team"), LolRole::Mid, Some("2028-06-30")),
            player(
                "support",
                Some("ai-team"),
                LolRole::Support,
                Some("2028-06-30"),
            ),
            player(
                "emergency-ai-team-prematch-adc",
                Some("ai-team"),
                LolRole::Support,
                Some("2028-06-30"),
            ),
        ],
        vec![
            "top",
            "jungle",
            "mid",
            "support",
            "emergency-ai-team-prematch-adc",
        ],
    );

    let report = repair_team(&mut game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("wrong-role ID collision should still make deterministic repair progress");

    assert!(game.players.iter().any(|player| {
        player.id == "emergency-ai-team-prematch-adc-2"
            && player.team_id.as_deref() == Some("ai-team")
            && player.natural_position == LolRole::Adc
    }));
    assert!(report.actions.iter().any(|action| matches!(
        action,
        RepairAction::GeneratedReplacement { player_id, role: LolRole::Adc } if player_id == "emergency-ai-team-prematch-adc-2"
    )));
}

#[test]
fn duplicate_lineup_ids_are_not_match_eligible_until_reconciled() {
    let mut game = game_with(
        full_roster("ai-team"),
        vec!["top", "top", "mid", "adc", "support"],
    );

    let before = evaluate_team(&game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("team should evaluate");
    let report = repair_team(&mut game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("duplicate lineup ids should reconcile");
    let after = evaluate_team(&game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("team should evaluate after repair");

    assert!(!before.match_eligible);
    assert!(after.match_eligible);
    assert_eq!(after.stale_lineup_ids, Vec::<String>::new());
    assert!(report.actions.iter().any(|action| matches!(
        action,
        RepairAction::ReconciledLineup { lineup_ids, .. } if lineup_ids == &vec![
            "top".to_string(),
            "jungle".to_string(),
            "mid".to_string(),
            "adc".to_string(),
            "support".to_string(),
        ]
    )));
}

#[test]
fn short_non_empty_lineup_is_not_match_eligible_until_reconciled() {
    let mut game = game_with(full_roster("ai-team"), vec!["top", "jungle", "mid", "adc"]);

    let before = evaluate_team(&game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("team should evaluate");
    let report = repair_team(&mut game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("short non-empty lineup should reconcile");
    let after = evaluate_team(&game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("team should evaluate after repair");

    assert!(!before.match_eligible);
    assert!(after.match_eligible);
    assert_eq!(after.eligible_player_count, 5);
    assert!(report.actions.iter().any(|action| matches!(
        action,
        RepairAction::ReconciledLineup { lineup_ids, .. } if lineup_ids.len() == 5
    )));
}

#[test]
fn empty_lineup_is_not_match_eligible_until_reconciled() {
    let mut game = game_with(full_roster("ai-team"), vec![]);

    let before = evaluate_team(&game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("team should evaluate");
    let report = repair_team(&mut game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("empty lineup should reconcile when a lineup is required");
    let after = evaluate_team(&game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("team should evaluate after repair");

    assert!(!before.match_eligible);
    assert!(after.match_eligible);
    assert_eq!(after.stale_lineup_ids, Vec::<String>::new());
    assert!(report.actions.iter().any(|action| matches!(
        action,
        RepairAction::ReconciledLineup { lineup_ids, .. } if lineup_ids == &vec![
            "top".to_string(),
            "jungle".to_string(),
            "mid".to_string(),
            "adc".to_string(),
            "support".to_string(),
        ]
    )));
}

#[test]
fn lineup_missing_required_role_is_not_match_eligible_until_reconciled() {
    let mut players = full_roster("ai-team");
    players.push(player(
        "adc2",
        Some("ai-team"),
        LolRole::Adc,
        Some("2028-06-30"),
    ));
    let mut game = game_with(players, vec!["top", "jungle", "mid", "adc", "adc2"]);

    let before = evaluate_team(&game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("team should evaluate");
    let report = repair_team(&mut game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("lineup missing support should reconcile");
    let after = evaluate_team(&game, "ai-team", RosterStabilityReason::PreMatch)
        .expect("team should evaluate after repair");

    assert!(!before.match_eligible);
    assert!(after.match_eligible);
    assert!(report.actions.iter().any(|action| matches!(
        action,
        RepairAction::ReconciledLineup { removed_ids, lineup_ids }
            if removed_ids == &vec!["adc2".to_string()]
                && lineup_ids == &vec![
                    "top".to_string(),
                    "jungle".to_string(),
                    "mid".to_string(),
                    "adc".to_string(),
                    "support".to_string(),
                ]
    )));
}

#[test]
fn generated_replacement_id_exhaustion_returns_error_without_mutating_game() {
    let mut players = vec![
        player("top", Some("ai-team"), LolRole::Top, Some("2028-06-30")),
        player(
            "jungle",
            Some("ai-team"),
            LolRole::Jungle,
            Some("2028-06-30"),
        ),
        player("mid", Some("ai-team"), LolRole::Mid, Some("2028-06-30")),
        player(
            "support",
            Some("ai-team"),
            LolRole::Support,
            Some("2028-06-30"),
        ),
    ];
    players.push(player(
        "emergency-ai-team-prematch-adc",
        Some("other-ai"),
        LolRole::Adc,
        Some("2028-06-30"),
    ));
    for suffix in 2..=17 {
        players.push(player(
            &format!("emergency-ai-team-prematch-adc-{suffix}"),
            Some("other-ai"),
            LolRole::Adc,
            Some("2028-06-30"),
        ));
    }
    let mut game = game_with(players, vec!["top", "jungle", "mid", "support"]);
    let original_players = game
        .players
        .iter()
        .map(|player| {
            (
                player.id.clone(),
                player.team_id.clone(),
                player.contract_end.clone(),
                player.natural_position,
            )
        })
        .collect::<Vec<_>>();
    let original_lineup = game
        .teams
        .iter()
        .find(|team| team.id == "ai-team")
        .unwrap()
        .active_lineup_ids
        .clone();

    let error = repair_team(&mut game, "ai-team", RosterStabilityReason::PreMatch)
        .expect_err("exhausted generated IDs should return a fatal domain error");

    assert_eq!(error.reason, RosterStabilityReason::PreMatch);
    assert_eq!(error.missing_roles, vec![LolRole::Adc]);
    assert_eq!(
        game.players
            .iter()
            .map(|player| {
                (
                    player.id.clone(),
                    player.team_id.clone(),
                    player.contract_end.clone(),
                    player.natural_position,
                )
            })
            .collect::<Vec<_>>(),
        original_players
    );
    assert_eq!(
        game.teams
            .iter()
            .find(|team| team.id == "ai-team")
            .unwrap()
            .active_lineup_ids,
        original_lineup
    );
}

#[test]
fn repair_league_repairs_all_schedulable_ai_main_teams() {
    let mut game = game_with(
        vec![
            player("top", Some("ai-team"), LolRole::Top, Some("2028-06-30")),
            player(
                "jungle",
                Some("ai-team"),
                LolRole::Jungle,
                Some("2028-06-30"),
            ),
            player("mid", Some("ai-team"), LolRole::Mid, Some("2028-06-30")),
            player("adc", Some("ai-team"), LolRole::Adc, Some("2028-06-30")),
        ],
        vec!["top", "jungle", "mid", "adc"],
    );

    let reports = repair_league(&mut game, RosterStabilityReason::LoadMigration)
        .expect("league repair should repair schedulable AI teams through the PR1 seam");
    let evaluation = evaluate_team(&game, "ai-team", RosterStabilityReason::LoadMigration)
        .expect("team should evaluate after repair");

    assert_eq!(reports.len(), 2);
    assert!(reports.iter().any(|report| report.team_id == "ai-team"
        && report.reason == RosterStabilityReason::LoadMigration));
    assert!(evaluation.match_eligible);
}

#[test]
fn repair_league_error_does_not_commit_partial_team_repairs() {
    let mut players = vec![
        player("top", Some("ai-team"), LolRole::Top, Some("2028-06-30")),
        player(
            "jungle",
            Some("ai-team"),
            LolRole::Jungle,
            Some("2028-06-30"),
        ),
        player("mid", Some("ai-team"), LolRole::Mid, Some("2028-06-30")),
        player("adc", Some("ai-team"), LolRole::Adc, Some("2028-06-30")),
        player(
            "other-top",
            Some("other-ai"),
            LolRole::Top,
            Some("2028-06-30"),
        ),
        player(
            "other-jungle",
            Some("other-ai"),
            LolRole::Jungle,
            Some("2028-06-30"),
        ),
        player(
            "other-mid",
            Some("other-ai"),
            LolRole::Mid,
            Some("2028-06-30"),
        ),
        player(
            "other-support",
            Some("other-ai"),
            LolRole::Support,
            Some("2028-06-30"),
        ),
    ];
    players.push(player(
        "emergency-other-ai-load-migration-adc",
        Some("ai-team"),
        LolRole::Adc,
        Some("2028-06-30"),
    ));
    for suffix in 2..=17 {
        players.push(player(
            &format!("emergency-other-ai-load-migration-adc-{suffix}"),
            Some("ai-team"),
            LolRole::Adc,
            Some("2028-06-30"),
        ));
    }
    let mut game = game_with(players, vec!["top", "jungle", "mid", "adc"]);
    let original_players = game
        .players
        .iter()
        .map(|player| {
            (
                player.id.clone(),
                player.team_id.clone(),
                player.contract_end.clone(),
                player.natural_position,
            )
        })
        .collect::<Vec<_>>();
    let original_teams = game
        .teams
        .iter()
        .map(|team| (team.id.clone(), team.active_lineup_ids.clone()))
        .collect::<Vec<_>>();

    let error = repair_league(&mut game, RosterStabilityReason::LoadMigration)
        .expect_err("league repair should fail atomically when any team cannot be repaired");

    assert_eq!(error.team_id, "other-ai");
    assert_eq!(error.missing_roles, vec![LolRole::Adc]);
    assert_eq!(
        game.players
            .iter()
            .map(|player| {
                (
                    player.id.clone(),
                    player.team_id.clone(),
                    player.contract_end.clone(),
                    player.natural_position,
                )
            })
            .collect::<Vec<_>>(),
        original_players
    );
    assert_eq!(
        game.teams
            .iter()
            .map(|team| (team.id.clone(), team.active_lineup_ids.clone()))
            .collect::<Vec<_>>(),
        original_teams
    );
}

#[test]
fn evaluate_team_missing_team_uses_supplied_reason() {
    let game = game_with(
        full_roster("ai-team"),
        vec!["top", "jungle", "mid", "adc", "support"],
    );

    let error = evaluate_team(&game, "missing-team", RosterStabilityReason::LoadMigration)
        .expect_err("missing team should include caller context in error");

    assert_eq!(error.reason, RosterStabilityReason::LoadMigration);
    assert_eq!(error.team_id, "missing-team");
}
