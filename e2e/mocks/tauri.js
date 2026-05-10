// Mock de window.__TAURI_INTERNALS__ para Playwright
(function () {
  // --------------- helpers ---------------
  function makePlayer(id, name, teamId, role, ovrBase) {
    const seed = id * 7;
    return {
      id: "player-" + id,
      match_name: name,
      full_name: name,
      date_of_birth: "2000-01-01",
      nationality: "EUN",
      birth_country: null,
      position: role, natural_position: role, alternate_positions: [],
      attributes: {
        mechanics: clamp(ovrBase + (seed % 20)),
        laning: clamp(ovrBase + ((seed * 3) % 20)),
        teamfighting: clamp(ovrBase + ((seed * 7) % 20)),
        macro_play: clamp(ovrBase + ((seed * 5) % 20)),
        consistency: clamp(ovrBase + ((seed * 11) % 20)),
        shotcalling: clamp(ovrBase + ((seed * 13) % 20)),
        champion_pool: clamp(ovrBase + ((seed * 17) % 20)),
        discipline: clamp(ovrBase + ((seed * 19) % 20)),
        mental_resilience: clamp(ovrBase + ((seed * 23) % 20)),
      },
      condition: 100, morale: 100, fitness: 75, injury: null,
      team_id: teamId, traits: [], contract_end: null,
      wage: 25000, market_value: 1200000,
      stats: { assists: 0, avg_rating: 0, appearances: 0, goals: 0, clean_sheets: 0, minutes_played: 0 },
      career: [], training_focus: null,
      transfer_listed: false, loan_listed: false, transfer_offers: [],
      morale_core: { manager_trust: 50, unresolved_issue: null, recent_treatment: null, pending_promise: null, talk_cooldown_until: null, renewal_state: null },
    };
  }

  function clamp(v) { return Math.max(25, Math.min(95, v)); }

  // --------------- static data ---------------
  var teams = [
    { id: "lec-fnatic", name: "Fnatic", short_name: "FNC", country: "GB", city: "London", logo_url: "/teams-icons/fnatic.webp", colors: { primary: "#1f2937", secondary: "#f3f4f6" }, reputation: 500, finance: 3500000, wage_budget: 770000, transfer_budget: 1225000, stadium_name: "Fnatic Arena", stadium_capacity: 28000 },
    { id: "lec-g2-esports", name: "G2 Esports", short_name: "G2", country: "DE", city: "Berlin", logo_url: "/teams-icons/g2-esports.webp", colors: { primary: "#1f2937", secondary: "#f3f4f6" }, reputation: 650, finance: 4500000, wage_budget: 990000, transfer_budget: 1575000, stadium_name: "G2 Arena", stadium_capacity: 28000 },
    { id: "lec-giantx-lec", name: "GIANTX", short_name: "GX", country: "ES", city: "M\u00e1laga", logo_url: "/teams-icons/giantx-lec.webp", colors: { primary: "#1f2937", secondary: "#f3f4f6" }, reputation: 500, finance: 3500000, wage_budget: 770000, transfer_budget: 1225000, stadium_name: "GIANTX Arena", stadium_capacity: 28000 },
    { id: "lec-karmine-corp", name: "Karmine Corp", short_name: "KC", country: "FR", city: "Paris", logo_url: "/teams-icons/karmine-corp.webp", colors: { primary: "#1f2937", secondary: "#f3f4f6" }, reputation: 650, finance: 4500000, wage_budget: 990000, transfer_budget: 1575000, stadium_name: "Karmine Corp Arena", stadium_capacity: 28000 },
    { id: "lec-movistar-koi", name: "Movistar KOI", short_name: "MKOI", country: "ES", city: "Madrid", logo_url: "/teams-icons/movistar-koi.webp", colors: { primary: "#1f2937", secondary: "#f3f4f6" }, reputation: 650, finance: 4500000, wage_budget: 990000, transfer_budget: 1575000, stadium_name: "Movistar KOI Arena", stadium_capacity: 28000 },
    { id: "lec-natus-vincere", name: "Natus Vincere", short_name: "NAVI", country: "UA", city: "Kyiv", logo_url: "/teams-icons/natus-vincere.webp", colors: { primary: "#1f2937", secondary: "#f3f4f6" }, reputation: 500, finance: 3500000, wage_budget: 770000, transfer_budget: 1225000, stadium_name: "Natus Vincere Arena", stadium_capacity: 28000 },
    { id: "lec-shifters", name: "Shifters", short_name: "shft", country: "TR", city: "Istanbul", logo_url: "/teams-icons/shifters.webp", colors: { primary: "#1f2937", secondary: "#f3f4f6" }, reputation: 320, finance: 3000000, wage_budget: 660000, transfer_budget: 1050000, stadium_name: "Shifters Arena", stadium_capacity: 28000 },
    { id: "lec-sk-gaming", name: "SK Gaming", short_name: "SK", country: "DE", city: "Berlin", logo_url: "/teams-icons/sk-gaming.webp", colors: { primary: "#1f2937", secondary: "#f3f4f6" }, reputation: 320, finance: 3000000, wage_budget: 660000, transfer_budget: 1050000, stadium_name: "SK Gaming Arena", stadium_capacity: 28000 },
    { id: "lec-team-heretics-lec", name: "Team Heretics", short_name: "TH", country: "ES", city: "Madrid", logo_url: "/teams-icons/team-heretics-lec.webp", colors: { primary: "#1f2937", secondary: "#f3f4f6" }, reputation: 500, finance: 3500000, wage_budget: 770000, transfer_budget: 1225000, stadium_name: "Team Heretics Arena", stadium_capacity: 28000 },
    { id: "lec-team-vitality", name: "Team Vitality", short_name: "VIT", country: "FR", city: "Paris", logo_url: "/teams-icons/team-vitality.webp", colors: { primary: "#1f2937", secondary: "#f3f4f6" }, reputation: 650, finance: 4500000, wage_budget: 990000, transfer_budget: 1575000, stadium_name: "Team Vitality Arena", stadium_capacity: 28000 },
  ];

  var roles = ["Top", "Jungle", "Mid", "Adc", "Support"];
  var players = [];
  var namePool = [
    ["Bwipo","Razork","Humanoid","Noah","Jun"],
    ["BrokenBlade","Yike","Caps","Hans Sama","Labrov"],
    ["Th3Antonio","Xerxe","Jackies","Patrik","Mersa"],
    ["Canna","Yike","Vladi","Caliste","Targamas"],
    ["Myrwn","Elyoya","Fresskowy","Supa","Alvaro"],
    ["Szygenda","Lyncas","Nisqy","Ice","Hylissang"],
    ["Shlatan","??","??","??","??"],
    ["Irrelevant","Isma","RKR","Jackspektra","Bertho"],
    ["Carlsen","Sheo","Perkz","Flakked","Kaiser"],
    ["Naak Nako","Lyncas","Czajek","Carzzy","Hylissang"],
  ];

  var week = 1;
  var savedGames = [];

  for (var ti = 0; ti < teams.length; ti++) {
    for (var ri = 0; ri < roles.length; ri++) {
      var pid = ti * 10 + ri;
      var name = (namePool[ti] && namePool[ti][ri]) || ("Player " + pid);
      players.push(makePlayer("m-" + pid, name, teams[ti].id, roles[ri], 55 + ri * 5));
    }
  }

  var manager = {
    id: "mgr_user", nickname: "JD", first_name: "John", last_name: "Doe",
    date_of_birth: "2000-01-15", nationality: "ES", avatar_path: null,
    reputation: 500, satisfaction: 50, fan_approval: 50, team_id: null,
    career_stats: { total_matches: 0, total_wins: 0, total_losses: 0, total_draws: 0, win_rate: 0, titles: 0, best_finish: null, total_transfers: 0, total_spent: 0, total_received: 0, current_streak: 0, longest_win_streak: 0, seasons_completed: 0 },
    career_history: [],
  };

  function buildGameState(teamId, overrides) {
    return Object.assign({
      clock: { current_date: "2025-01-0" + week, start_date: "2025-01-01" },
      manager: Object.assign({}, manager, { team_id: teamId }),
      teams: teams,
      players: players,
      staff: [],
      messages: [],
      news: [],
      league: {
        id: "lec-2025", name: "LEC 2025",
        standings: teams.map(function(t, i) { return { team_id: t.id, wins: Math.min(10, week - 1), losses: Math.min(10, 10 - week + 1), points: 0, position: i + 1, form: [] }; }),
        season: 1, current_week: week, total_weeks: 18,
        fixtures: [{ week: week, home_team_id: "lec-fnatic", away_team_id: "lec-g2-esports", date: "2025-01-0" + week }],
      },
      scouting_assignments: [],
      board_objectives: [],
      champions: [],
    }, overrides || {});
  }

  window.__TAURI_INTERNALS__ = {
    invoke: function(cmd, args) {
      switch (cmd) {
        // --- Onboarding ---
        case "start_new_game":
          return Promise.resolve("ok");
        case "get_team_selection_data":
          return Promise.resolve({ manager: Object.assign({}, manager, { team_id: null }), teams: teams, players: players });
        case "select_team":
          week = 1;
          return Promise.resolve(buildGameState(args.teamId));

        // --- Dashboard ---
        case "get_active_game":
          return Promise.resolve(buildGameState(args && args.teamId));
        case "get_champions":
          return Promise.resolve([]);
        case "save_game":
          savedGames.push(Date.now());
          return Promise.resolve();
        case "exit_to_menu":
          return Promise.resolve();

        // --- Advance Time ---
        case "advance_time_with_mode":
          week++;
          return Promise.resolve({ action: "advanced", game: buildGameState("lec-fnatic"), round_summary: { matches: [] } });
        case "check_blocking_actions":
          return Promise.resolve([]);
        case "skip_to_match_day":
          week++;
          return Promise.resolve({ action: "advanced", game: buildGameState("lec-fnatic"), days_skipped: 1 });

        // --- Training ---
        case "set_training":
        case "set_training_schedule":
        case "set_training_groups":
        case "set_weekly_scrims":
        case "set_weekly_scrim_plans":
        case "set_weekly_scrim_slots":
        case "set_weekly_scrim_objective":
        case "finalize_weekly_scrim_setup":
        case "auto_configure_weekly_scrim_setup":
        case "set_player_training_focus":
        case "choose_post_scrim_decision":
        case "choose_daily_scrim_action":
        case "delegate_scrim_decision":
        case "delegate_champion_training":
        case "cancel_todays_scrims":
          return Promise.resolve(buildGameState("lec-fnatic"));
        case "get_scrim_context":
          return Promise.resolve({ state: "Training", slot_index: null, opponent_team_id: null, resolved_opponent_team_id: null, objective: null, report: null, can_edit_plan: true, can_cancel: false, can_review: false, can_view_weekly_plan: true, has_official_match: false, primary_action: "Training", push_through_recommended: false });

        // --- Settings ---
        case "get_settings":
          return Promise.resolve({ language: "en", theme: "system", notifications: true });
        case "save_settings":
          return Promise.resolve();

        // --- Save/Load ---
        case "get_saves":
          return Promise.resolve([{ id: "save-1", name: "Test Save", date: "2025-01-15", clock: "Week 1", team_name: "Fnatic" }]);
        case "load_game":
          return Promise.resolve("John Doe");
        case "delete_save":
          return Promise.resolve(true);

        // --- Player ---
        case "get_player":
          return Promise.resolve(players[0]);
        case "set_player_champion_training_target":
          return Promise.resolve(buildGameState("lec-fnatic"));

        // --- Staff ---
        case "get_staff":
          return Promise.resolve([]);
        case "hire_staff":
          return Promise.resolve(buildGameState("lec-fnatic"));
        case "release_staff":
          return Promise.resolve(buildGameState("lec-fnatic"));

        // --- Transfers ---
        case "make_transfer_bid":
        case "preview_transfer_bid_financial_impact":
        case "respond_to_offer":
        case "release_player_contract":
          return Promise.resolve(buildGameState("lec-fnatic"));

        // --- Scouting ---
        case "send_scout":
        case "start_potential_research":
          return Promise.resolve(buildGameState("lec-fnatic"));

        // --- Academy ---
        case "get_academy_acquisition_options":
          return Promise.resolve({ academies: [], can_acquire: false });
        case "acquire_academy_team":
        case "promote_academy_player":
        case "demote_main_player_to_academy":
          return Promise.resolve(buildGameState("lec-fnatic"));

        // --- Social ---
        case "get_social_accounts":
          return Promise.resolve([]);
        case "get_social_feed":
          return Promise.resolve([]);
        case "get_social_templates":
          return Promise.resolve([]);
        case "create_manager_social_post":
        case "save_social_accounts":
        case "save_social_templates":
          return Promise.resolve();

        // --- Jobs ---
        case "get_available_jobs":
          return Promise.resolve([]);
        case "apply_for_job":
          return Promise.resolve({ success: false, message: "Not implemented" });

        // --- Inbox ---
        case "get_messages":
        case "urgent_messages":
          return Promise.resolve([]);
        case "mark_message_read":
        case "mark_all_messages_read":
        case "delete_message":
        case "delete_messages":
        case "resolve_message_action":
          return Promise.resolve();

        default:
          console.warn("[Tauri mock] Unhandled invoke:", cmd, args);
          return Promise.reject(new Error("Mock not implemented: " + cmd));
      }
    },
  };
})();
