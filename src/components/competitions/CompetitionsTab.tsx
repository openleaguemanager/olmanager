import { useState } from "react";
import { useTranslation } from "react-i18next";
import {
  Trophy,
  Calendar as CalendarIcon,
  TrendingUp,
  Globe,
} from "lucide-react";
import type { GameStateData, LeagueData } from "../../store/gameStore";
import ScheduleCalendarView from "../schedule/ScheduleCalendarView";
import { Badge } from "../ui";
import type { StoredFixtureDraftResult } from "../schedule/ScheduleTab.helpers";

interface CompetitionsTabProps {
  gameState: GameStateData;
}

const COMPETITION_COLORS: Record<string, string> = {
  lec: "bg-blue-500/20 text-blue-300 border-blue-500/30",
  lcs: "bg-red-500/20 text-red-300 border-red-500/30",
  lck: "bg-green-500/20 text-green-300 border-green-500/30",
  lpl: "bg-yellow-500/20 text-yellow-300 border-yellow-500/30",
  lcp: "bg-purple-500/20 text-purple-300 border-purple-500/30",
  cblol: "bg-emerald-500/20 text-emerald-300 border-emerald-500/30",
};

function getCompetitionColor(id: string): string {
  return COMPETITION_COLORS[id] ?? "bg-gray-500/20 text-gray-300 border-gray-500/30";
}

export default function CompetitionsTab({ gameState }: CompetitionsTabProps) {
  const { t } = useTranslation();
  const [selectedCompId, setSelectedCompId] = useState<string | null>(null);

  const leagues = gameState.leagues;
  const selectedLeague = selectedCompId
    ? leagues.find((l) => l.id === selectedCompId) ?? null
    : null;

  // Build competition label map for calendar
  const competitionLabelMap = new Map<string, string>();
  leagues.forEach((l) => {
    l.fixtures.forEach((f) => competitionLabelMap.set(f.id, l.name));
  });

  // All fixtures for calendar (filter by selected or show all)
  const calendarFixtures = selectedLeague
    ? selectedLeague.fixtures
    : leagues.flatMap((l) => l.fixtures);

  return (
    <div className="max-w-6xl mx-auto space-y-6">
      {/* Header */}
      <div className="flex items-center gap-3">
        <Globe className="w-6 h-6 text-primary-500" />
        <h2 className="text-xl font-heading font-bold text-gray-800 dark:text-gray-100">
          {t("competitions.title", "Competiciones")}
        </h2>
      </div>

      {/* Competitions grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        {leagues.map((league) => (
          <CompetitionCard
            key={league.id}
            league={league}
            selected={selectedCompId === league.id}
            colorClass={getCompetitionColor(league.id)}
            onSelect={() =>
              setSelectedCompId(
                selectedCompId === league.id ? null : league.id,
              )
            }
          />
        ))}
      </div>

      {/* Selected competition detail */}
      {selectedLeague && (
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-heading font-bold text-gray-800 dark:text-gray-100">
              {selectedLeague.name} —{" "}
              {t("competitions.calendar", "Calendario")}
            </h3>
            <Badge variant="accent" size="sm">
              {selectedLeague.fixtures.length}{" "}
              {t("competitions.fixtures", "partidos")}
            </Badge>
          </div>

          <ScheduleCalendarView
            gameState={gameState}
            fixtures={calendarFixtures}
            competitionLabelMap={competitionLabelMap}
            onOpenFixtureResult={(_stored: StoredFixtureDraftResult) => {}}
          />
        </div>
      )}
    </div>
  );
}

// ─── Competition Card ───────────────────────────────────────────────────

interface CompetitionCardProps {
  league: LeagueData;
  selected: boolean;
  colorClass: string;
  onSelect: () => void;
}

function CompetitionCard({
  league,
  selected,
  colorClass,
  onSelect,
}: CompetitionCardProps) {
  const { t } = useTranslation();

  const leagueFixtures = league.fixtures;
  const totalMatches = leagueFixtures.length;
  const playedMatches = leagueFixtures.filter(
    (f) => f.status === "Completed",
  ).length;
  const playoffFixtures = leagueFixtures.filter(
    (f) => f.match_type === "Playoffs",
  ).length;

  return (
    <button
      type="button"
      onClick={onSelect}
      className={[
        "relative text-left rounded-xl border-2 p-5 transition-all duration-200 hover:shadow-lg",
        selected
          ? "border-primary-500 bg-primary-500/10 shadow-md shadow-primary-500/10"
          : "border-gray-200 dark:border-navy-600 bg-white dark:bg-navy-800 hover:border-primary-400/50",
      ].join(" ")}
    >
      <div className="flex items-start justify-between mb-3">
        <div className="flex items-center gap-3">
          <div
            className={`w-10 h-10 rounded-lg flex items-center justify-center ${colorClass} border`}
          >
            <Trophy className="w-5 h-5" />
          </div>
          <div>
            <h4 className="font-heading font-bold text-sm text-gray-800 dark:text-gray-100">
              {league.name}
            </h4>
            <p className="text-[11px] text-gray-500 dark:text-gray-400 uppercase tracking-wider">
              {t("competitions.season", "Temporada")} {league.season}
            </p>
          </div>
        </div>
        {selected && <div className="w-3 h-3 rounded-full bg-primary-500" />}
      </div>

      <div className="flex items-center gap-4 text-xs text-gray-500 dark:text-gray-400">
        <span className="flex items-center gap-1">
          <CalendarIcon className="w-3.5 h-3.5" />
          {totalMatches} {t("competitions.matches", "partidos")}
        </span>
        <span className="flex items-center gap-1">
          <TrendingUp className="w-3.5 h-3.5" />
          {playedMatches}/{totalMatches} {t("competitions.played", "jugados")}
        </span>
        {playoffFixtures > 0 && (
          <Badge variant="accent" size="sm">
            {t("competitions.playoffs", "Playoffs")}
          </Badge>
        )}
      </div>
    </button>
  );
}
