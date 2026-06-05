import lesTeamsData from "../../data/erls/teams/les_teams.json";
import lflTeamsData from "../../data/erls/teams/lfl_teams.json";
import primeLeagueTeamsData from "../../data/erls/teams/prm_teams.json";

function normalizeKey(value: string): string {
  return value
    .toLowerCase()
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .replace(/[^a-z0-9]/g, "");
}

function buildLogoMapFromJson(data: { teams: { name: string; logo_url?: string | null }[] }): Map<string, string> {
  const map = new Map<string, string>();
  for (const team of data.teams) {
    const key = normalizeKey(team.name);
    if (!key) continue;
    if (!team.logo_url) continue;
    map.set(key, team.logo_url);
  }
  return map;
}

const EXAMPLE_TEAM_LOGO_MAP = new Map<string, string>([
  ...buildLogoMapFromJson(lesTeamsData).entries(),
  ...buildLogoMapFromJson(lflTeamsData).entries(),
  ...buildLogoMapFromJson(primeLeagueTeamsData).entries(),
]);

// Slugs that have a local file under /teams-icons/<slug>.webp.
// Includes both full-name keys and short-name keys so any spelling matches.
const LOCAL_TEAMS_ICONS: Record<string, string> = {
  // Fnatic
  fnatic: "fnatic",
  fnc: "fnatic",
  // G2
  g2: "g2-esports",
  g2esports: "g2-esports",
  // GIANTX
  giantx: "giantx-lec",
  gx: "giantx-lec",
  // Karmine Corp
  karminecorp: "karmine-corp",
  kc: "karmine-corp",
  // Movistar Koi
  movistarkoi: "movistar-koi",
  mkoi: "movistar-koi",
  koi: "movistar-koi",
  madlionskoi: "movistar-koi",
  // Natus Vincere
  natusvincere: "natus-vincere",
  navi: "natus-vincere",
  // Shifters
  shifters: "shifters",
  shft: "shifters",
  // SK Gaming
  skgaming: "sk-gaming",
  sk: "sk-gaming",
  // Team Heretics
  teamheretics: "team-heretics-lec",
  heretics: "team-heretics-lec",
  th: "team-heretics-lec",
  // Team Vitality
  teamvitality: "team-vitality",
  vitality: "team-vitality",
  vit: "team-vitality",
};

export function resolveTeamLogo(teamName?: string | null, logoUrl?: string | null): string | null {
  if (logoUrl) return logoUrl;
  const key = normalizeKey(teamName ?? "");
  if (!key) return null;
  const local = LOCAL_TEAMS_ICONS[key];
  if (local) return `/teams-icons/${local}.webp`;
  return EXAMPLE_TEAM_LOGO_MAP.get(key) ?? null;
}
