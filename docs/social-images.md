# Social Data System

This document describes the local data assets for the social system in OLManager.

## Overview

The social system stores accounts, match templates, and hardcoded text strings in local JSON files instead of embedding them in the Rust binary. This makes the data easier to maintain, update, and extend without recompiling the game.

## Directory Structure

```
data/social/
├── accounts.json       # Social account metadata (35+ accounts)
├── templates.json      # Match templates (32 templates across 8 languages)
└── match_texts.json    # Hardcoded text strings for match posts

public/social-avatars/
└── {id}.webp          # Downloaded and converted profile images
```

## How the Backend Loads JSON

At runtime, the Rust backend (`olm_core`) loads social data from the JSON files in `data/social/` via `src-tauri/crates/olm_core/src/social_data.rs`.

```rust
pub fn load_social_accounts(data_base: &Path) -> Option<Vec<SocialAccount>> {
    let path = data_base.join("social").join("accounts.json");
    // ...
}

pub fn load_social_templates(data_base: &Path) -> Option<Vec<SocialTemplate>> {
    let path = data_base.join("social").join("templates.json");
    // ...
}

pub fn load_match_texts(data_base: &Path) -> Option<MatchTexts> {
    let path = data_base.join("social").join("match_texts.json");
    // ...
}
```

- `data/social/` is the **runtime source of truth**.
- `src-tauri/crates/olm_core/src/social_registry.rs` still contains fallback defaults, but it is only used when the JSON files are missing (e.g., on a fresh install before the data is copied).
- The `tauri.conf.json` already bundles `../public/**/*`, so `social-avatars/` requires no additional config.

## How the Frontend Resolves Avatars

The frontend uses `src/lib/social/resolveSocialAvatar.ts` to determine the correct avatar path for any social account, post, or author.

```ts
export function resolveSocialAvatar(
  authorId?: string,
  profileImageUrl?: string | null,
  authorType?: string,
  avatar?: string | null
): string | null;
```

Logic summary:
1. If `avatar` is provided, use it directly.
2. If `profileImageUrl` is a local path (starts with `/social-avatars/`), use it.
3. If `profileImageUrl` is an external URL (legacy), return it as-is.
4. If `authorId` is provided and matches a known avatar ID, return `/social-avatars/{id}.webp`.
5. Otherwise, return `null` (UI will show a placeholder).

The resulting path is passed through `assetUrl()` (in `src/lib/assetUrl.ts`), which handles root-relative paths correctly for both Vite dev server and Tauri production builds.

Example usage in a component:
```tsx
<img src={assetUrl(resolveSocialAvatar(post.author_id, post.profile_image_url, post.author_type))} />
```

## Regenerating Avatars

> **Note:** The `scripts/download-social-avatars.ts` script was designed to read external Twitter/X URLs from `social_registry.rs`. After the migration to local paths, the Rust source no longer contains those URLs, so the script **cannot download new avatars** in its current state. It will report `0 accounts with profile images`.

### To add a new avatar manually:

1. Obtain the source image (e.g., from the account's current profile page).
2. Convert it to WebP using `sharp` or any image tool:
   ```bash
   npx sharp input.jpg --webp --quality 85 --output public/social-avatars/{id}.webp
   ```
3. Add the account entry to `data/social/accounts.json` with `profile_image_url: "/social-avatars/{id}.webp"`.
4. Update `src-tauri/crates/olm_core/src/social_registry.rs` with the same fallback entry (for users without the JSON file).

### To regenerate all existing avatars:

If the source URLs were still present in the Rust code, you could run:
```bash
npx tsx scripts/download-social-avatars.ts
```
This is preserved for historical reference and may be restored if the source-of-truth for URLs is moved to a separate config file.

## JSON Schemas

### accounts.json

```json
{
  "id": "fan_random_lec",
  "language": "all",
  "display_name": "LEC Enjoyer",
  "handle": "@randomLECEnjoyer",
  "author_type": "Fan",
  "profile_image_url": "/social-avatars/fan_random_lec.webp",
  "favorite_team_ids": ["lec-fnatic"],
  "active": true
}
```

**Fields:**
- `id` (string): Unique identifier for the account
- `language` (string): Language code or "all" for universal accounts
- `display_name` (string): Human-readable name
- `handle` (string): Social media handle with @ prefix
- `author_type` (string): One of `Team`, `Player`, `Fan`, `Analyst`, `Journalist`, `MemeAccount`, `Manager`
- `profile_image_url` (string|null): Local path to the avatar image, or null if no image
- `favorite_team_ids` (string[]): Array of team IDs this account supports (empty for neutral accounts)
- `active` (boolean): Whether the account is currently active

### templates.json

```json
{
  "id": "team-global-es",
  "language": "es",
  "slot": "TeamBanter",
  "weight": 5,
  "author_id": null,
  "conditions_json": null,
  "variants": ["GG {loser_short_name}. ..."],
  "tags": ["match", "team", "global"],
  "active": true
}
```

**Fields:**
- `id` (string): Unique template identifier
- `language` (string): Language code
- `slot` (string): One of `TeamBanter`, `FanOpinion`, `AnalystTake`, `PlayerReaction`
- `weight` (number): Selection weight (higher = more likely)
- `author_id` (string|null): Account ID for fan/analyst templates, null for team/player templates
- `conditions_json` (string|null): JSON-stringified conditions object, or null
- `variants` (string[]): Array of text template strings with placeholders
- `tags` (string[]): Categorization tags
- `active` (boolean): Whether the template is active

**Placeholders used in variants:**
- `{winner_short_name}` / `{loser_short_name}` — Team short names
- `{winner_name}` / `{loser_name}` — Team full names
- `{score}` — Match score (e.g., "2-1")
- `{winner_objectives}` — Winner objective count
- `{team}` / `{opponent}` — Generic team references
- `{winner}` — Used in bouzys_vs_fnatic text

### match_texts.json

Structure:
```json
{
  "team_loser": {
    "es": ["text1", "text2", "text3"],
    "en": ["text1", "text2", "text3"],
    "fr": ["..."],
    "de": ["..."],
    "it": ["..."],
    "pt": ["..."],
    "pt-BR": ["..."],
    "tr": ["..."]
  },
  "fan_reaction_won": { "es": [...], ... },
  "fan_reaction_lost": { "es": [...], ... },
  "bouzys_vs_fnatic": { "es": [...], ... }
}
```

## Troubleshooting

### Missing avatar images

If an account references a local avatar (`/social-avatars/{id}.webp`) but the file does not exist:
- The UI will show a placeholder (default avatar).
- The integration test `src/lib/social/socialDataConsistency.test.ts` will fail and list the missing files.
- To fix: either add the missing `.webp` file or set the account's `profile_image_url` to `null` in `data/social/accounts.json` (and update the fallback in `social_registry.rs`).

### Missing JSON files

If `data/social/accounts.json` or `templates.json` are missing at runtime:
- The Rust backend falls back to the hardcoded defaults in `social_registry.rs`.
- Match texts (`match_texts.json`) have no fallback; if missing, the match post system will use empty strings.

### JSON validation errors

If the JSON files are malformed:
- The Rust loader silently returns `None` and falls back to defaults.
- Run `npx vitest run src/lib/social/socialDataConsistency.test.ts` to verify JSON validity.

## Adding New Accounts

1. Add the account to `data/social/accounts.json`.
2. If the account has an avatar, place the `.webp` file in `public/social-avatars/`.
3. Update the fallback in `src-tauri/crates/olm_core/src/social_registry.rs` (optional but recommended).
4. Run the integration test to verify consistency.

## Adding New Templates

1. Add the template to `data/social/templates.json`.
2. Ensure the `id` is unique.
3. Use the correct `slot` and `language` values.
4. Set appropriate `weight` and `tags`.
5. Add `conditions_json` if the template requires specific conditions (e.g., player name).

## Notes

- `data/social/` files are the **runtime source of truth**.
- Profile images are stored as WebP for better compression. The `sharp` library is in `devDependencies`.
- The `public/social-avatars/` directory is served as static assets by Vite.
- All accounts with `profile_image_url: null` use a default placeholder in the UI.
- There are no remaining external Twitter/X URLs (`pbs.twimg.com`) in the social components or JSON data.
- The migration helper in `social.rs` (line ~730) still checks for legacy `pbs.twimg.com` URLs in existing save data and replaces them with local paths.
