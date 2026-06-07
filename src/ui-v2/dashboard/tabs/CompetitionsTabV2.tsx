import { useState, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Building2, Calendar, Globe, ListOrdered, TrendingUp, Trophy, Users } from "lucide-react";

import type { GameStateData, LeagueData } from "@/store/gameStore";
import { compareStandingsByLolScore, getStandingKillDiff } from "@/store/gameStore";
import { getTeamLogoPath } from "@/components/schedule/ScheduleTab.helpers";
import { resolvePlayerPhoto } from "@/lib/players/playerPhotos";
import { calculateLolOvr } from "@/lib/players/lolPlayerStats";
import { cn } from "@/ui-v2/lib/utils";
import ScheduleCalendarView from "@/components/schedule/ScheduleCalendarView";
import type { StoredFixtureDraftResult } from "@/components/schedule/ScheduleTab.helpers";

interface CompetitionsTabV2Props {
  gameState: GameStateData;
  onSelectTeam?: (id: string) => void;
}

type DetailView = "calendar" | "standings" | "teams" | "players";

const REGION_COLORS: Record<string, string> = {
  lec: "border-l-blue-500", lcs: "border-l-red-500", lck: "border-l-green-500",
  lpl: "border-l-yellow-500", lcp: "border-l-purple-500", cblol: "border-l-emerald-500",
};

const TIER_1 = new Set(["lec", "lcs", "lck", "lpl", "lcp", "cblol"]);

export function CompetitionsTabV2({ gameState, onSelectTeam }: CompetitionsTabV2Props) {
  const { t } = useTranslation();
  const [selectedCid, setSelectedCid] = useState<string | null>(null);
  const [view, setView] = useState<DetailView>("standings");

  const leagues = useMemo(
    () => (gameState.leagues ?? []).filter((l) => TIER_1.has(l.competition_id ?? l.id)),
    [gameState.leagues],
  );
  const selectedLeague = selectedCid
    ? leagues.find((l) => (l.competition_id ?? l.id) === selectedCid) ?? null
    : null;
  const selectedTeamIds = selectedCid
    ? gameState.teams.filter((t) => t.competition_id === selectedCid).map((t) => t.id)
    : [];
  const selectedPlayers = selectedTeamIds.length > 0
    ? gameState.players.filter((p) => p.team_id != null && selectedTeamIds.includes(p.team_id))
    : [];
  const sortedStandings = selectedLeague
    ? [...selectedLeague.standings].sort(compareStandingsByLolScore)
    : [];
  const calendarFixtures = selectedLeague
    ? selectedLeague.fixtures ?? []
    : leagues.flatMap((l) => l.fixtures ?? []);
  const compLabelMap = new Map<string, string>();
  leagues.forEach((l) => (l.fixtures ?? []).forEach((f) => compLabelMap.set(f.id, l.name)));

  return (
    <div className="competitions-v2 flex h-full flex-col gap-5 overflow-y-auto p-6 scrollbar-v2">
      {/* Header */}
      <div className="flex items-center gap-2">
        <Globe className="size-5 text-primary" />
        <h2 className="font-heading text-lg font-bold uppercase tracking-wider text-foreground">
          {t("competitions.title", "Competiciones")}
        </h2>
      </div>

      {/* Competition cards */}
      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
        {leagues.map((league) => {
          const cid = league.competition_id ?? league.id;
          const selected = selectedCid === cid;
          const fixtures = league.fixtures ?? [];
          const played = fixtures.filter((f) => f.status === "Completed").length;
          const playoffCount = fixtures.filter((f) => f.match_type === "Playoffs").length;
          const teamsCount = gameState.teams.filter((t) => t.competition_id === cid).length;
          const regionColor = REGION_COLORS[cid] ?? "border-l-gray-500";

          return (
            <button
              key={league.id}
              type="button"
              onClick={() => setSelectedCid(selected ? null : cid)}
              className={cn(
                "relative rounded-xl border-2 px-5 py-4 text-left transition-all hover:shadow-md",
                selected
                  ? "border-primary bg-primary/5 shadow-primary/10"
                  : "border-border bg-card hover:border-primary/40",
              )}
            >
              {/* Region accent bar */}
              <div className={cn("absolute left-0 top-3 bottom-3 w-1 rounded-r-full", regionColor.replace("border-l-", "bg-"))} />

              <div className="flex items-start gap-3 pl-3">
                <div className="flex size-10 shrink-0 items-center justify-center overflow-hidden rounded-lg border border-border bg-muted">
                  <CompetitionLogo league={league} cid={cid} />
                </div>
                <div className="min-w-0 flex-1">
                  <div className="flex items-center gap-2">
                    <h4 className="truncate font-heading text-sm font-bold uppercase tracking-wide text-foreground">
                      {league.name}
                    </h4>
                    {selected && <span className="size-2 shrink-0 rounded-full bg-primary" />}
                  </div>
                  <p className="text-[11px] uppercase tracking-wider text-muted-foreground">
                    {t("competitions.season", "Season")} {league.season}
                  </p>
                </div>
              </div>

              <div className="mt-3 flex flex-wrap items-center gap-3 pl-3 text-xs text-muted-foreground">
                <span className="flex items-center gap-1"><Building2 className="size-3.5" />{teamsCount} equipos</span>
                <span className="flex items-center gap-1"><Calendar className="size-3.5" />{fixtures.length} partidos</span>
                <span className="flex items-center gap-1"><TrendingUp className="size-3.5" />{played}/{fixtures.length}</span>
                {playoffCount > 0 && (
                  <span className="rounded-md border border-amber-500/30 bg-amber-500/10 px-1.5 py-0.5 font-heading text-[10px] font-bold uppercase tracking-wider text-amber-400">
                    Playoffs
                  </span>
                )}
              </div>
            </button>
          );
        })}
      </div>

      {/* Detail panel */}
      {selectedLeague && (
        <div className="flex flex-col gap-4">
          {/* View tabs */}
          <div className="flex flex-wrap gap-1.5 rounded-lg border border-border bg-muted/30 p-1">
            {(["standings", "calendar", "teams", "players"] as const).map((v) => (
              <button
                key={v}
                type="button"
                onClick={() => setView(v)}
                className={cn(
                  "flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-medium transition-colors",
                  view === v
                    ? "bg-card text-foreground shadow-sm"
                    : "text-muted-foreground hover:text-foreground",
                )}
              >
                {v === "standings" && <ListOrdered className="size-3.5" />}
                {v === "calendar" && <Calendar className="size-3.5" />}
                {v === "teams" && <Building2 className="size-3.5" />}
                {v === "players" && <Users className="size-3.5" />}
                {v === "standings" && "Clasificación"}
                {v === "calendar" && "Calendario"}
                {v === "teams" && `Equipos (${selectedTeamIds.length})`}
                {v === "players" && `Jugadores (${selectedPlayers.length})`}
              </button>
            ))}
          </div>

          {/* Standings */}
          {view === "standings" && (
            <div className="overflow-hidden rounded-xl border border-border">
              <table className="w-full text-left">
                <thead>
                  <tr className="border-b border-border bg-muted/30 text-[10px] uppercase tracking-widest text-muted-foreground">
                    <th className="w-10 px-4 py-3 font-heading font-bold">#</th>
                    <th className="px-4 py-3 font-heading font-bold">{t("standings.team", "Equipo")}</th>
                    <th className="px-3 py-3 text-center font-heading font-bold">{t("standings.played", "PJ")}</th>
                    <th className="px-3 py-3 text-center font-heading font-bold">{t("standings.won", "G")}</th>
                    <th className="px-3 py-3 text-center font-heading font-bold">{t("standings.lost", "P")}</th>
                    <th className="px-3 py-3 text-center font-heading font-bold">GM</th>
                    <th className="px-3 py-3 text-center font-heading font-bold">GP</th>
                    <th className="px-3 py-3 text-center font-heading font-bold">D</th>
                    <th className="px-4 py-3 text-center font-heading font-bold">{t("standings.points", "Pts")}</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-border/40">
                  {sortedStandings.map((entry, idx) => {
                    const team = gameState.teams.find((t) => t.id === entry.team_id);
                    const logo = team ? getTeamLogoPath(gameState.teams, team.id) : null;
                    const diff = getStandingKillDiff(entry);
                    const isUser = team?.id === gameState.manager.team_id;
                    return (
                      <tr
                        key={entry.team_id}
                        onClick={() => onSelectTeam?.(entry.team_id)}
                        className={cn(
                          "cursor-pointer transition-colors hover:bg-muted/20",
                          isUser && "bg-primary/5",
                        )}
                      >
                        <td className="px-4 py-2.5 font-heading text-xs font-bold tabular-nums text-muted-foreground">
                          {idx + 1}
                        </td>
                        <td className="px-4 py-2.5">
                          <div className="flex items-center gap-2.5">
                            {logo && <img src={logo} alt="" className="size-5 object-contain" />}
                            <span className={cn("text-sm", isUser ? "font-bold text-primary" : "font-medium text-foreground")}>
                              {team?.name ?? entry.team_id}
                            </span>
                          </div>
                        </td>
                        <td className="px-3 py-2.5 text-center text-xs tabular-nums text-muted-foreground">{entry.played}</td>
                        <td className="px-3 py-2.5 text-center text-xs tabular-nums text-emerald-400 font-medium">{entry.won}</td>
                        <td className="px-3 py-2.5 text-center text-xs tabular-nums text-red-400 font-medium">{entry.lost}</td>
                        <td className="px-3 py-2.5 text-center text-xs tabular-nums text-muted-foreground">{entry.maps_won ?? entry.kills_for ?? 0}</td>
                        <td className="px-3 py-2.5 text-center text-xs tabular-nums text-muted-foreground">{entry.maps_lost ?? entry.kills_against ?? 0}</td>
                        <td className={cn("px-3 py-2.5 text-center text-xs tabular-nums font-medium", diff >= 0 ? "text-emerald-400" : "text-red-400")}>
                          {diff >= 0 ? "+" : ""}{diff}
                        </td>
                        <td className="px-4 py-2.5 text-center font-heading text-sm font-bold tabular-nums text-foreground">
                          {entry.points}
                        </td>
                      </tr>
                    );
                  })}
                  {sortedStandings.length === 0 && (
                    <tr><td colSpan={9} className="py-8 text-center text-sm text-muted-foreground">Sin datos de clasificación</td></tr>
                  )}
                </tbody>
              </table>
            </div>
          )}

          {/* Calendar */}
          {view === "calendar" && (
            <div className="schedule-calendar-container">
            <ScheduleCalendarView
              gameState={gameState}
              fixtures={calendarFixtures}
              competitionLabelMap={compLabelMap}
              onOpenFixtureResult={(_s: StoredFixtureDraftResult) => {}}
            />
            </div>
          )}

          {/* Teams */}
          {view === "teams" && (
            <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
              {selectedTeamIds.map((id) => {
                const team = gameState.teams.find((t) => t.id === id);
                if (!team) return null;
                const logo = getTeamLogoPath(gameState.teams, team.id);
                return (
                  <button
                    key={team.id}
                    type="button"
                    onClick={() => onSelectTeam?.(team.id)}
                    className="flex items-center gap-3 rounded-xl border border-border bg-card px-4 py-3 text-left transition-all hover:border-primary/40 hover:bg-muted/20"
                  >
                    {logo && <img src={logo} alt={team.name} className="size-10 shrink-0 rounded-lg border border-border bg-muted p-1 object-contain" />}
                    <div className="min-w-0">
                      <h4 className="truncate font-heading text-sm font-bold uppercase tracking-wide text-foreground">{team.name}</h4>
                      <p className="text-xs text-muted-foreground">{team.short_name}</p>
                    </div>
                  </button>
                );
              })}
              {selectedTeamIds.length === 0 && (
                <p className="col-span-full py-8 text-center text-sm text-muted-foreground">No hay equipos en esta competición.</p>
              )}
            </div>
          )}

          {/* Players */}
          {view === "players" && (
            <div className="overflow-hidden rounded-xl border border-border">
              <table className="w-full text-left">
                <thead>
                  <tr className="border-b border-border bg-muted/30 text-[10px] uppercase tracking-widest text-muted-foreground">
                    <th className="px-4 py-3 font-heading font-bold">{t("players.name", "Jugador")}</th>
                    <th className="px-4 py-3 font-heading font-bold">{t("players.role", "Rol")}</th>
                    <th className="px-4 py-3 font-heading font-bold">{t("players.team", "Equipo")}</th>
                    <th className="px-4 py-3 text-center font-heading font-bold">{t("players.overall", "OVR")}</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-border/40">
                  {selectedPlayers.slice(0, 100).map((player) => {
                    const team = gameState.teams.find((t) => t.id === player.team_id);
                    const ovr = calculateLolOvr(player);
                    const photo = resolvePlayerPhoto(player.id, player.match_name, player.profile_image_url);
                    return (
                      <tr key={player.id} className="transition-colors hover:bg-muted/20">
                        <td className="px-4 py-2.5">
                          <div className="flex items-center gap-2.5">
                            {photo && <img src={photo} alt={player.match_name} className="size-7 rounded-full object-cover" />}
                            <div>
                              <p className="text-sm font-medium text-foreground">{player.match_name}</p>
                              <p className="text-[11px] text-muted-foreground">{player.full_name}</p>
                            </div>
                          </div>
                        </td>
                        <td className="px-4 py-2.5">
                          <span className="inline-flex items-center rounded-md border border-border bg-card px-1.5 py-0.5 font-heading text-[10px] font-bold uppercase tracking-wider text-muted-foreground">
                            {player.position}
                          </span>
                        </td>
                        <td className="px-4 py-2.5 text-sm text-muted-foreground">
                          {team?.name ?? t("common.freeAgent", "FA")}
                        </td>
                        <td className="px-4 py-2.5 text-center font-heading text-sm font-bold tabular-nums"
                          style={{ color: ovr >= 75 ? "#fb923c" : ovr >= 60 ? "#fbbf24" : "#94a3b8" }}>
                          {ovr}
                        </td>
                      </tr>
                    );
                  })}
                  {selectedPlayers.length === 0 && (
                    <tr><td colSpan={4} className="py-8 text-center text-sm text-muted-foreground">No hay jugadores en esta competición.</td></tr>
                  )}
                </tbody>
              </table>
              {selectedPlayers.length > 100 && (
                <p className="border-t border-border px-4 py-3 text-center text-xs text-muted-foreground">
                  Mostrando 100 de {selectedPlayers.length} jugadores
                </p>
              )}
            </div>
          )}
        </div>
      )}

      {!selectedCid && (
        <div className="flex flex-1 items-center justify-center py-12">
          <p className="font-heading text-sm uppercase tracking-wider text-muted-foreground">
            Seleccioná una competición para ver detalles
          </p>
        </div>
      )}
    </div>
  );
}

/** Competition logo with fallback */
function CompetitionLogo({ league, cid }: { league: LeagueData; cid: string }) {
  const src = league.logo ?? `/competitions-icons/${cid}.webp`;
  return (
    <img
      src={src}
      alt={league.name}
      className="size-full object-contain p-1"
      onError={(e) => {
        e.currentTarget.style.display = "none";
      }}
    />
  );
}
