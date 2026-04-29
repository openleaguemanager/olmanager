import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { ChevronLeft, ChevronRight, Calendar as CalendarIcon } from "lucide-react";
import { GameStateData, FixtureData } from "../../store/gameStore";
import { Card, CardBody, Badge } from "../ui";
import {
  StoredFixtureDraftResult,
  buildBestOfContext,
  getTeamLogoPath,
  inferBestOf,
  isoDateKey,
  normalizeLolScore,
  parseFixtureDate,
  readStoredFixtureDraftResult,
} from "./ScheduleTab.helpers";
import { getTeamName } from "../../lib/helpers";

interface Props {
  gameState: GameStateData;
  fixtures: FixtureData[];
  onOpenFixtureResult: (stored: StoredFixtureDraftResult) => void;
}

const WEEKDAY_REFERENCE_MONDAY = new Date(Date.UTC(2024, 0, 1));

function buildMonthGrid(viewMonth: Date): Date[] {
  const firstOfMonth = new Date(viewMonth.getFullYear(), viewMonth.getMonth(), 1);
  const dayOfWeekMondayBased = (firstOfMonth.getDay() + 6) % 7;
  const gridStart = new Date(firstOfMonth);
  gridStart.setDate(firstOfMonth.getDate() - dayOfWeekMondayBased);

  const cells: Date[] = [];
  for (let i = 0; i < 42; i++) {
    const cell = new Date(gridStart);
    cell.setDate(gridStart.getDate() + i);
    cells.push(cell);
  }
  return cells;
}

function pickInitialMonth(currentDateStr: string, fixtures: FixtureData[]): Date {
  const parsed = parseFixtureDate(currentDateStr);
  if (parsed) return new Date(parsed.getFullYear(), parsed.getMonth(), 1);
  const firstFixture = fixtures
    .map((f) => parseFixtureDate(f.date))
    .filter((d): d is Date => d !== null)
    .sort((a, b) => a.getTime() - b.getTime())[0];
  if (firstFixture) return new Date(firstFixture.getFullYear(), firstFixture.getMonth(), 1);
  const today = new Date();
  return new Date(today.getFullYear(), today.getMonth(), 1);
}

export default function ScheduleCalendarView({
  gameState,
  fixtures,
  onOpenFixtureResult,
}: Props) {
  const { t, i18n } = useTranslation();
  const userTeamId = gameState.manager.team_id ?? "";
  const todayKey = gameState.clock?.current_date?.substring(0, 10) ?? "";

  const [viewMonth, setViewMonth] = useState<Date>(() =>
    pickInitialMonth(gameState.clock?.current_date ?? "", fixtures),
  );

  const bestOfContext = useMemo(() => buildBestOfContext(fixtures), [fixtures]);

  const fixturesByDay = useMemo(() => {
    const map = new Map<string, FixtureData[]>();
    fixtures.forEach((f) => {
      const key = f.date.substring(0, 10);
      const list = map.get(key) ?? [];
      list.push(f);
      map.set(key, list);
    });
    map.forEach((list) => list.sort((a, b) => a.matchday - b.matchday));
    return map;
  }, [fixtures]);

  const monthCells = useMemo(() => buildMonthGrid(viewMonth), [viewMonth]);

  const monthLabel = useMemo(() => {
    return new Intl.DateTimeFormat(i18n.language, {
      month: "long",
      year: "numeric",
    }).format(viewMonth);
  }, [viewMonth, i18n.language]);

  const weekdayLabels = useMemo(() => {
    const formatter = new Intl.DateTimeFormat(i18n.language, { weekday: "short" });
    return Array.from({ length: 7 }, (_, idx) => {
      const day = new Date(WEEKDAY_REFERENCE_MONDAY);
      day.setUTCDate(WEEKDAY_REFERENCE_MONDAY.getUTCDate() + idx);
      return formatter.format(day);
    });
  }, [i18n.language]);

  const goPrev = () => setViewMonth((m) => new Date(m.getFullYear(), m.getMonth() - 1, 1));
  const goNext = () => setViewMonth((m) => new Date(m.getFullYear(), m.getMonth() + 1, 1));
  const goToday = () =>
    setViewMonth(pickInitialMonth(gameState.clock?.current_date ?? "", fixtures));

  return (
    <Card>
      <div className="flex items-center justify-between gap-3 p-4 border-b border-gray-100 dark:border-navy-600 bg-gray-50 dark:bg-navy-800 rounded-t-xl">
        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={goPrev}
            aria-label={t("schedule.previousMonth", "Mes anterior")}
            className="inline-flex items-center justify-center w-8 h-8 rounded-md text-gray-500 dark:text-gray-300 hover:text-primary-500 hover:bg-white dark:hover:bg-navy-700 transition-colors"
          >
            <ChevronLeft className="w-4 h-4" />
          </button>
          <h4 className="font-heading font-bold text-sm uppercase tracking-wider text-gray-700 dark:text-gray-200 min-w-[160px] text-center">
            {monthLabel}
          </h4>
          <button
            type="button"
            onClick={goNext}
            aria-label={t("schedule.nextMonth", "Mes siguiente")}
            className="inline-flex items-center justify-center w-8 h-8 rounded-md text-gray-500 dark:text-gray-300 hover:text-primary-500 hover:bg-white dark:hover:bg-navy-700 transition-colors"
          >
            <ChevronRight className="w-4 h-4" />
          </button>
        </div>
        <button
          type="button"
          onClick={goToday}
          className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-white dark:bg-navy-700 border border-gray-200 dark:border-navy-600 text-xs font-heading font-bold uppercase tracking-wider text-gray-600 dark:text-gray-200 hover:text-primary-500 transition-colors"
        >
          <CalendarIcon className="w-3.5 h-3.5" />
          {t("schedule.today", "Hoy")}
        </button>
      </div>
      <CardBody className="p-3">
        <div className="grid grid-cols-7 gap-1 mb-1">
          {weekdayLabels.map((label, idx) => (
            <div
              key={`${label}-${idx}`}
              className="text-center text-[10px] font-heading font-bold uppercase tracking-wider text-gray-400 dark:text-gray-500 py-1"
            >
              {label}
            </div>
          ))}
        </div>
        <div className="grid grid-cols-7 gap-1">
          {monthCells.map((cell) => {
            const cellKey = isoDateKey(cell);
            const inMonth = cell.getMonth() === viewMonth.getMonth();
            const isToday = cellKey === todayKey;
            const cellFixtures = fixturesByDay.get(cellKey) ?? [];
            const hasUserMatch = cellFixtures.some(
              (f) => f.home_team_id === userTeamId || f.away_team_id === userTeamId,
            );

            return (
              <div
                key={cellKey}
                className={[
                  "min-h-[120px] rounded-md border p-2 flex flex-col gap-1 transition-colors",
                  inMonth
                    ? "bg-white dark:bg-navy-800 border-gray-200 dark:border-navy-600"
                    : "bg-gray-50/60 dark:bg-navy-900/40 border-gray-100 dark:border-navy-700 opacity-60",
                  isToday ? "ring-2 ring-primary-500/60 border-primary-400" : "",
                  hasUserMatch && !isToday ? "border-accent-400/60" : "",
                ].join(" ")}
              >
                <div className="flex items-center justify-between">
                  <span
                    className={`text-[11px] font-heading font-bold tabular-nums ${
                      isToday
                        ? "text-primary-500"
                        : inMonth
                          ? "text-gray-600 dark:text-gray-300"
                          : "text-gray-400 dark:text-gray-500"
                    }`}
                  >
                    {cell.getDate()}
                  </span>
                  {cellFixtures.length > 1 ? (
                    <span className="text-[9px] text-gray-400 dark:text-gray-500 tabular-nums">
                      ×{cellFixtures.length}
                    </span>
                  ) : null}
                </div>
                <div className="flex flex-col gap-1 overflow-hidden">
                  {cellFixtures.slice(0, 3).map((f) => {
                    const stored = readStoredFixtureDraftResult(f.id);
                    const bo = inferBestOf(f, bestOfContext);
                    const completed = f.status === "Completed";
                    const score = normalizeLolScore(f, stored, userTeamId, bo);
                    const homeLogo = getTeamLogoPath(gameState.teams, f.home_team_id);
                    const awayLogo = getTeamLogoPath(gameState.teams, f.away_team_id);
                    const isUserMatch =
                      f.home_team_id === userTeamId || f.away_team_id === userTeamId;
                    const userIsHome = f.home_team_id === userTeamId;
                    const userResultTone = (() => {
                      if (!isUserMatch || !completed || !score) return "";
                      const userWins = userIsHome
                        ? score.home > score.away
                        : score.away > score.home;
                      return userWins
                        ? "bg-blue-500/15 dark:bg-blue-500/20 border-blue-500/30"
                        : "bg-red-500/15 dark:bg-red-500/20 border-red-500/30";
                    })();

                    const clickable = completed && stored;
                    return (
                      <button
                        type="button"
                        key={f.id}
                        disabled={!clickable}
                        onClick={() => clickable && onOpenFixtureResult(stored)}
                        className={[
                          "group flex items-center gap-1 px-1 py-0.5 rounded border text-left transition-colors",
                          userResultTone ||
                            (isUserMatch
                              ? "bg-primary-50/70 dark:bg-primary-500/10 border-primary-400/30"
                              : "bg-gray-50 dark:bg-navy-700/50 border-gray-200/60 dark:border-navy-600"),
                          clickable ? "hover:border-primary-400 cursor-pointer" : "cursor-default",
                        ].join(" ")}
                        title={`${getTeamName(gameState.teams, f.home_team_id)} ${
                          score ? `${score.home}-${score.away}` : "vs"
                        } ${getTeamName(gameState.teams, f.away_team_id)}`}
                      >
                        {homeLogo ? (
                          <img
                            src={homeLogo}
                            alt=""
                            className="w-3.5 h-3.5 object-contain shrink-0"
                            loading="lazy"
                          />
                        ) : (
                          <span className="w-3.5 h-3.5 shrink-0" />
                        )}
                        <span className="text-[10px] font-heading font-bold tabular-nums text-gray-700 dark:text-gray-200 px-0.5">
                          {score ? `${score.home}-${score.away}` : "vs"}
                        </span>
                        {awayLogo ? (
                          <img
                            src={awayLogo}
                            alt=""
                            className="w-3.5 h-3.5 object-contain shrink-0"
                            loading="lazy"
                          />
                        ) : (
                          <span className="w-3.5 h-3.5 shrink-0" />
                        )}
                        <Badge variant="neutral" size="sm" className="ml-auto !text-[8px] !px-1 !py-0">
                          BO{bo}
                        </Badge>
                      </button>
                    );
                  })}
                  {cellFixtures.length > 3 ? (
                    <span className="text-[9px] text-gray-400 dark:text-gray-500 px-1">
                      +{cellFixtures.length - 3} {t("schedule.moreMatches", "más")}
                    </span>
                  ) : null}
                </div>
              </div>
            );
          })}
        </div>
      </CardBody>
    </Card>
  );
}
