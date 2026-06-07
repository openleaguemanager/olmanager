import { describe, expect, it } from "vitest";

import type {
  AcademyAcquisitionOptionData,
  AcademyAcquisitionOptionsResponseData,
  AcquireAcademyTeamRequestData,
  TeamData,
} from "./types";

describe("academy acquisition contracts", () => {
  it("models backend acquisition options with source identity and rebrand metadata", () => {
    const option: AcademyAcquisitionOptionData = {
      source_team_id: "mkoi-fenix",
      source_team_name: "Movistar KOI Fénix",
      source_team_short_name: "MKOI F",
      source_team_logo_url: "https://cdn.example/logo.webp",
      erl_league_id: "nlc",
      league_name: "Northern League of Legends Championship",
      country: "ES",
      region: "Europe",
      reputation: 4,
      development_level: 3,
      acquisition_cost: 260000,
      assignment_rule: "Domestic",
      fallback_reason: null,
      rebrand_allowed: true,
      source_identity: {
        source_team_id: "mkoi-fenix",
        original_name: "Movistar KOI Fénix",
        original_short_name: "MKOI F",
        original_logo_url: "https://cdn.example/logo.webp",
      },
    };

        custom_logo_url: "https://cdn.example/custom-logo.webp",
        original_logo_url: "https://cdn.example/logo.webp",
          acquisition_cost: 260000,
          acquired_at: "2026-08-10T00:00:00Z",
        },
        branding: {
          current_name: "Movistar KOI Fénix Academy",
          current_short_name: "MKOI F",
          current_logo_url: "https://cdn.example/custom-logo.webp",
        },
        source_identity: {
          source_team_id: "mkoi-fenix",
          original_name: "Movistar KOI Fénix",
          original_short_name: "MKOI F",
          original_logo_url: "https://cdn.example/logo.webp",
        },
        erl_assignment: {
          erl_league_id: "nlc",
          country_rule: "Domestic",
          fallback_reason: null,
          reputation: 4,
          creation_cost: 260000,
          created_at: "2026-08-10T00:00:00Z",
        },
      },
    };

    expect(team.academy?.acquisition?.source_team_id).toBe("mkoi-fenix");
  });
});
