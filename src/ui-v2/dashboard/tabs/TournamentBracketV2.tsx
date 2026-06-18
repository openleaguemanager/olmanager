import { Card, CardContent, CardHeader, CardTitle } from "@/ui-v2/components/ui/card";
import { Badge } from "@/ui-v2/components/ui/badge";
import { cn } from "@/ui-v2/lib/utils";
import { getTeamLogoPath } from "@/lib/schedule/helpers";
import { getTeamName } from "@/lib/common/helpers";
import type { GameStateData, FixtureData } from "@/store/gameStore";
import { Trophy } from "lucide-react";

interface Props {
  gameState: GameStateData;
  leagueId: string;
}

export function TournamentBracketV2({ gameState, leagueId }: Props) {
  const league = gameState.leagues.find((l) => (l.competition_id ?? l.id) === leagueId);
  if (!league) return null;

  const fixtures = league.fixtures ?? [];
  const userTeamId = gameState.manager.team_id;
  const tstate = league.tournament_state;

  const koFixtures = fixtures.filter((f) => f.match_type === "TournamentKnockout");
  const groupFixtures = fixtures.filter((f) => f.match_type === "TournamentGroup");
  const playInFixtures = fixtures.filter((f) => f.match_type === "TournamentPlayIn");
  const swissFixtures = fixtures.filter((f) => f.match_type === "TournamentSwiss");

  const renderFixture = (f: FixtureData) => {
    const completed = f.status === "Completed";
    const homeScore = f.result?.home_wins ?? 0;
    const awayScore = f.result?.away_wins ?? 0;
    const homeLogo = getTeamLogoPath(gameState.teams, f.home_team_id);
    const awayLogo = getTeamLogoPath(gameState.teams, f.away_team_id);
    const isUserMatch = f.home_team_id === userTeamId || f.away_team_id === userTeamId;

    return (
      <div
        key={f.id}
        className={cn(
          "flex items-center gap-2 rounded-lg border bg-card p-2.5",
          isUserMatch && "border-primary/40 bg-primary/5",
          !isUserMatch && "border-border"
        )}
      >
        <div className="flex min-w-0 flex-1 flex-col gap-1">
          <div className="flex items-center gap-2">
            {homeLogo && <img src={homeLogo} alt="" className="size-4 object-contain" />}
            <span className={cn("text-xs font-medium truncate", f.home_team_id === userTeamId && "text-primary font-bold")}>
              {getTeamName(gameState.teams, f.home_team_id)}
            </span>
          </div>
          <div className="flex items-center gap-2">
            {awayLogo && <img src={awayLogo} alt="" className="size-4 object-contain" />}
            <span className={cn("text-xs font-medium truncate", f.away_team_id === userTeamId && "text-primary font-bold")}>
              {getTeamName(gameState.teams, f.away_team_id)}
            </span>
          </div>
        </div>
        <div className="flex flex-col items-center gap-0.5">
          <span className={cn("text-xs font-bold tabular-nums", completed ? "text-foreground" : "text-muted-foreground/40")}>
            {completed ? homeScore : "—"}
          </span>
          <span className={cn("text-xs font-bold tabular-nums", completed ? "text-foreground" : "text-muted-foreground/40")}>
            {completed ? awayScore : "—"}
          </span>
        </div>
        <Badge variant="outline" className="text-[10px]">BO{f.best_of}</Badge>
      </div>
    );
  };

  const renderSwissStandings = () => {
    if (!tstate?.swiss_records?.length) return null;
    const sorted = [...tstate.swiss_records].sort((a, b) => b.wins - a.wins || b.buchholz - a.buchholz);
    return (
      <div className="space-y-1">
        {sorted.map((rec, idx) => (
          <div key={rec.team_id} className="flex items-center justify-between rounded-md border bg-card px-3 py-1.5 text-xs">
            <span className="font-medium">{idx + 1}. {getTeamName(gameState.teams, rec.team_id)}</span>
            <span className="tabular-nums text-muted-foreground">{rec.wins}–{rec.losses} <span className="text-[10px]">(B{rec.buchholz})</span></span>
          </div>
        ))}
      </div>
    );
  };

  const renderGslGroups = () => {
    if (!tstate?.gsl_groups?.length) return null;
    return (
      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
        {tstate.gsl_groups.map((g, gi) => (
          <div key={gi} className="rounded-lg border bg-card p-3">
            <h5 className="mb-2 font-heading text-xs font-bold uppercase tracking-wider text-muted-foreground">Group {String.fromCharCode(65 + gi)}</h5>
            <div className="space-y-1 text-xs">
              {g.teams.map((tid) => {
                const isAdvanced = g.advanced_teams?.includes(tid);
                return (
                  <div key={tid} className={cn("flex items-center gap-2 rounded px-2 py-1", isAdvanced ? "bg-primary/10 text-primary" : "text-foreground")}>
                    <span className="truncate">{getTeamName(gameState.teams, tid)}</span>
                    {isAdvanced && <Badge variant="outline" className="text-[10px]">Advanced</Badge>}
                  </div>
                );
              })}
            </div>
          </div>
        ))}
      </div>
    );
  };

  return (
    <div className="flex flex-col gap-4">
      <Card>
        <CardHeader className="flex-row items-center gap-2 space-y-0">
          <Trophy className="size-4 text-primary" />
          <CardTitle className="font-heading text-sm uppercase tracking-widest text-muted-foreground">
            {league.name}
          </CardTitle>
          {tstate && (
            <Badge variant="secondary" className="ml-auto text-[10px] uppercase">
              {tstate.current_phase}
            </Badge>
          )}
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          {tstate?.current_phase === "Group" && groupFixtures.length > 0 && (
            <div>
              <h4 className="mb-2 font-heading text-xs font-bold uppercase tracking-wider text-muted-foreground">Groups</h4>
              {renderGslGroups()}
              <div className="mt-3 grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3">
                {groupFixtures.map(renderFixture)}
              </div>
            </div>
          )}
          {tstate?.current_phase === "Swiss" && (
            <div>
              <h4 className="mb-2 font-heading text-xs font-bold uppercase tracking-wider text-muted-foreground">Swiss Standings</h4>
              {renderSwissStandings()}
              {swissFixtures.length > 0 && (
                <div className="mt-3 grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3">
                  {swissFixtures.map(renderFixture)}
                </div>
              )}
            </div>
          )}
          {tstate?.current_phase === "Knockout" && koFixtures.length > 0 && (
            <div>
              <h4 className="mb-2 font-heading text-xs font-bold uppercase tracking-wider text-muted-foreground">Knockout</h4>
              <div className="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3">
                {koFixtures.map(renderFixture)}
              </div>
            </div>
          )}
          {tstate?.current_phase === "PlayIn" && playInFixtures.length > 0 && (
            <div>
              <h4 className="mb-2 font-heading text-xs font-bold uppercase tracking-wider text-muted-foreground">Play-In</h4>
              <div className="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3">
                {playInFixtures.map(renderFixture)}
              </div>
            </div>
          )}
          {!tstate && fixtures.length === 0 && (
            <p className="text-center text-sm text-muted-foreground">No fixtures generated yet.</p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
