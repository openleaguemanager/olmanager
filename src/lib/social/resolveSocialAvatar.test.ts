import { describe, expect, it } from "vitest";

import { resolveSocialAvatar } from "./resolveSocialAvatar";
import type { SocialPostData, SocialAccountData, TeamData, PlayerData } from "@/store/types";

function createPost(overrides: Partial<SocialPostData> = {}): SocialPostData {
  return {
    id: "post-1",
    date: "2025-01-01",
    author_name: "Test Author",
    author_handle: "@testauthor",
    author_type: "Fan",
    body: "Hello world",
    likes: 0,
    reposts: 0,
    replies: 0,
    sentiment: "Hype",
    category: "FanOpinion",
    tags: [],
    team_ids: [],
    player_ids: [],
    fixture_id: null,
    read: false,
    ...overrides,
  };
}

function createAccount(overrides: Partial<SocialAccountData> = {}): SocialAccountData {
  return {
    id: "account-1",
    language: "all",
    display_name: "Test Account",
    handle: "@testauthor",
    author_type: "Fan",
    profile_image_url: null,
    favorite_team_ids: [],
    active: true,
    ...overrides,
  };
}

function createTeam(overrides: Partial<TeamData> = {}): TeamData {
  return {
    id: "team-1",
    name: "Alpha FC",
    short_name: "ALP",
    country: "BR",
    city: "Rio",
    finance: 0,
    manager_id: null,
    reputation: 50,
    wage_budget: 0,
    transfer_budget: 0,
    season_income: 0,
    season_expenses: 0,
    draft_strategy: "Balanced",
    training_focus: "Physical",
    training_intensity: "Medium",
    training_schedule: "Balanced",
    founded_year: 1900,
    colors: { primary: "#111111", secondary: "#ffffff" },
    form: [],
    history: [],
    ...overrides,
  };
}

function createPlayer(overrides: Partial<PlayerData> = {}): PlayerData {
  return {
    id: "player-1",
    match_name: "J. Smith",
    full_name: "John Smith",
    date_of_birth: "2000-01-01",
    nationality: "BR",
    position: "TOP",
    natural_position: "TOP",
    alternate_positions: [],
    training_focus: null,
    attributes: {},
    condition: 80,
    morale: 80,
    team_id: "team-1",
    contract_end: null,
    wage: 0,
    market_value: 0,
    stats: { assists: 0 },
    career: [],
    transfer_listed: false,
    loan_listed: false,
    transfer_offers: [],
    traits: [],
    ...overrides,
  };
}

describe("resolveSocialAvatar", () => {
  it("returns account profile_image_url when available", () => {
    const post = createPost({ author_handle: "@alpha" });
    const accounts = [createAccount({ handle: "@alpha", profile_image_url: "/social-avatars/alpha.webp" })];
    const result = resolveSocialAvatar(post, accounts, [], []);
    expect(result).toBe("/social-avatars/alpha.webp");
  });

  it("returns null when no account matches and no team/player fallback", () => {
    const post = createPost({ author_handle: "@unknown" });
    const result = resolveSocialAvatar(post, [], [], []);
    expect(result).toBeNull();
  });

  it("prefers account avatar over team logo for Team type", () => {
    const post = createPost({
      author_handle: "@alphafc",
      author_type: "Team",
      team_ids: ["team-1"],
    });
    const accounts = [createAccount({ handle: "@alphafc", profile_image_url: "/social-avatars/alphafc.webp" })];
    const teams = [createTeam({ id: "team-1", name: "Alpha FC" })];
    const result = resolveSocialAvatar(post, accounts, teams, []);
    expect(result).toBe("/social-avatars/alphafc.webp");
  });

  it("falls back to team logo for Team type when no account avatar", () => {
    const post = createPost({
      author_handle: "@alphafc",
      author_type: "Team",
      team_ids: ["team-1"],
    });
    const accounts = [createAccount({ handle: "@alphafc", profile_image_url: null })];
    const teams = [createTeam({ id: "team-1", name: "Alpha FC" })];
    const result = resolveSocialAvatar(post, accounts, teams, []);
    expect(result).toMatch(/^\/teams-icons\/.*\.webp$/);
  });

  it("falls back to player photo for Player type when no account avatar", () => {
    const post = createPost({
      author_handle: "@jsmith",
      author_type: "Player",
      player_ids: ["player-1"],
    });
    const accounts = [createAccount({ handle: "@jsmith", profile_image_url: null })];
    const players = [createPlayer({ id: "player-1", match_name: "J. Smith" })];
    const result = resolveSocialAvatar(post, accounts, [], players);
    expect(result).toMatch(/^\/player-photos\/.*\.webp$/);
  });

  it("is case-insensitive when matching account handle", () => {
    const post = createPost({ author_handle: "@AlphaFC" });
    const accounts = [createAccount({ handle: "@alphafc", profile_image_url: "/social-avatars/alphafc.webp" })];
    const result = resolveSocialAvatar(post, accounts, [], []);
    expect(result).toBe("/social-avatars/alphafc.webp");
  });

  it("finds team by handle match even without team_ids", () => {
    const post = createPost({
      author_handle: "@AlphaFC",
      author_type: "Team",
      team_ids: [],
    });
    const teams = [createTeam({ id: "team-1", name: "Alpha FC" })];
    const result = resolveSocialAvatar(post, [], teams, []);
    expect(result).toMatch(/^\/teams-icons\/.*\.webp$/);
  });

  it("finds player by player_ids for Player type", () => {
    const post = createPost({
      author_handle: "@jsmith",
      author_type: "Player",
      player_ids: ["player-1"],
    });
    const players = [createPlayer({ id: "player-1", match_name: "J. Smith" })];
    const result = resolveSocialAvatar(post, [], [], players);
    expect(result).toMatch(/^\/player-photos\/.*\.webp$/);
  });

  it("returns null for Fan type with no matching account", () => {
    const post = createPost({ author_handle: "@randomfan", author_type: "Fan" });
    const result = resolveSocialAvatar(post, [], [], []);
    expect(result).toBeNull();
  });
});
