import type { SocialPostData, SocialAccountData, TeamData, PlayerData } from "@/store/types";
import { resolvePlayerPhoto } from "@/lib/players/playerPhotos";
import { resolveTeamLogo } from "@/lib/teams/teamLogos";

function defaultTeamLogoSrc(teamId: string): string {
  const slug = teamId.replace(/^lec-/, "");
  if (slug === "shifters") {
    return "/teams-icons/shifters.webp";
  }
  return `/teams-icons/${slug}.webp`;
}

function academyLogoFromMetadata(team: TeamData): string | null {
  const academy = team.academy as
    | {
        branding?: { current_logo_url?: string | null };
        acquisition?: { original_logo_url?: string | null };
        source_identity?: { original_logo_url?: string | null };
        current_logo_url?: string | null;
        original_logo_url?: string | null;
      }
    | null
    | undefined;

  return (
    academy?.branding?.current_logo_url ??
    academy?.acquisition?.original_logo_url ??
    academy?.source_identity?.original_logo_url ??
    academy?.current_logo_url ??
    academy?.original_logo_url ??
    null
  );
}

function teamLogoSrc(team: TeamData): string {
  return team.logo_url ?? resolveTeamLogo(team.name) ?? defaultTeamLogoSrc(team.id) ?? academyLogoFromMetadata(team) ?? "";
}

function findPostTeam(post: SocialPostData, teams: TeamData[]): TeamData | null {
  const normalizedHandle = post.author_handle.replace(/^@/, "").toLowerCase();
  const byHandle = teams.find((team) => {
    const teamHandle = team.name
      .toLowerCase()
      .replace(/[^a-z0-9]/g, "")
      .slice(0, 15);
    const shortHandle = team.short_name
      .toLowerCase()
      .replace(/[^a-z0-9]/g, "")
      .slice(0, 15);
    return normalizedHandle === teamHandle || normalizedHandle === shortHandle;
  });
  if (byHandle) return byHandle;

  const firstTeamId = post.team_ids[0];
  if (!firstTeamId) return null;
  return teams.find((team) => team.id === firstTeamId) ?? null;
}

function findPostPlayer(post: SocialPostData, players: PlayerData[]): PlayerData | null {
  const firstPlayerId = post.player_ids[0];
  if (!firstPlayerId) return null;
  return players.find((player) => player.id === firstPlayerId) ?? null;
}

/**
 * Resolve the avatar image path for a social post.
 * Resolution order:
 * 1. Account profile_image_url (local path like /social-avatars/{id}.webp)
 * 2. Team logo (for Team author_type)
 * 3. Player photo (for Player author_type)
 * 4. null (caller should show initials fallback)
 */
export function resolveSocialAvatar(
  post: SocialPostData,
  accounts: SocialAccountData[],
  teams: TeamData[],
  players: PlayerData[],
): string | null {
  // 1. Use account profile_image_url
  const account = accounts.find(
    (a) => a.handle.toLowerCase() === post.author_handle.toLowerCase(),
  );
  if (account?.profile_image_url) {
    return account.profile_image_url;
  }

  // 2. Team logo for Team type
  if (post.author_type === "Team") {
    const team = findPostTeam(post, teams);
    if (team) {
      const logo = teamLogoSrc(team);
      if (logo) return logo;
    }
  }

  // 3. Player photo for Player type
  if (post.author_type === "Player") {
    const player = findPostPlayer(post, players);
    if (player) {
      return resolvePlayerPhoto(player.id, player.match_name, player.profile_image_url);
    }
  }

  return null;
}
