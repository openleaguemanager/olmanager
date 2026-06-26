/// <reference types="node" />
import { describe, it, expect } from "vitest";
import fs from "fs";
import path from "path";

const PROJECT_ROOT = process.cwd();

const accountsPath = path.join(PROJECT_ROOT, "data", "social", "accounts.json");
const templatesPath = path.join(PROJECT_ROOT, "data", "social", "templates.json");
const matchTextsPath = path.join(PROJECT_ROOT, "data", "social", "match_texts.json");
const avatarsDir = path.join(PROJECT_ROOT, "public", "social-avatars");

describe("Social Data Consistency", () => {
  it("accounts.json is valid JSON", () => {
    const raw = fs.readFileSync(accountsPath, "utf-8");
    const parsed = JSON.parse(raw);
    expect(Array.isArray(parsed)).toBe(true);
  });

  it("templates.json is valid JSON", () => {
    const raw = fs.readFileSync(templatesPath, "utf-8");
    const parsed = JSON.parse(raw);
    expect(parsed).toHaveProperty("templates");
    expect(Array.isArray(parsed.templates)).toBe(true);
  });

  it("match_texts.json is valid JSON", () => {
    const raw = fs.readFileSync(matchTextsPath, "utf-8");
    const parsed = JSON.parse(raw);
    expect(typeof parsed).toBe("object");
  });

  it("all accounts with non-null profile_image_url have a corresponding avatar file", () => {
    const accounts = JSON.parse(fs.readFileSync(accountsPath, "utf-8"));
    const avatarFiles = new Set(fs.readdirSync(avatarsDir));

    const missing: string[] = [];
    for (const account of accounts) {
      const url = account.profile_image_url;
      if (url && typeof url === "string") {
        const fileName = path.basename(url);
        if (!avatarFiles.has(fileName)) {
          missing.push(`${account.id} -> ${fileName}`);
        }
      }
    }

    if (missing.length > 0) {
      console.warn("Missing avatar files:", missing);
    }

    expect(missing).toEqual([]);
  });

  it("all templates with author_id reference an existing account", () => {
    const accounts = JSON.parse(fs.readFileSync(accountsPath, "utf-8"));
    const accountIds = new Set(accounts.map((a: any) => a.id));
    const templates = JSON.parse(fs.readFileSync(templatesPath, "utf-8")).templates;

    const missing: string[] = [];
    for (const template of templates) {
      if (template.author_id && !accountIds.has(template.author_id)) {
        missing.push(`${template.id} -> ${template.author_id}`);
      }
    }

    expect(missing).toEqual([]);
  });

  it("match_texts.json contains expected keys", () => {
    const data = JSON.parse(fs.readFileSync(matchTextsPath, "utf-8"));
    const expectedKeys = [
      "team_loser",
      "team_loser_stomp",
      "team_loser_close",
      "fan_reaction_won",
      "fan_reaction_lost",
      "bouzys_vs_fnatic",
    ];
    for (const key of expectedKeys) {
      expect(data).toHaveProperty(key);
      expect(typeof data[key]).toBe("object");
    }
  });
});
