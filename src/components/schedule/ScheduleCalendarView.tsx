import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { ChevronLeft, ChevronRight, Calendar as CalendarIcon, X } from "lucide-react";
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
  type BestOfContext,
} from "./ScheduleTab.helpers";
import { formatMatchDate, getTeamName } from "../../lib/helpers";

interface Props {
  gameState: GameStateData;
  fixtures: FixtureData[];
  onOpenFixtureResult: (stored: StoredFixtureDraftResult) => void;
}

const WEEKDAY_REFERENCE_MONDAY = new Date(Date.UTC(2024, 0, 1));
const MAX_FIXTURES_PER_CELL = 3;

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

function sortFixturesUserFirst(
  fixtures: FixtureData[],
  userTeamId: string,
): FixtureData[] {
  return [...fixtures].sort((a, b) => {
    const aIsUser = a.home_team_id === userTeamId || a.away_team_id === userTeamId;
    const bIsUser = b.home_team_id === userTeamId || b.away_team_id === userTeamId;
    if (aIsUser && !bIsUser) return -1;
    if (!aIsUser && bIsUser) return 1;
    return a.matchday - b.matchday;
  });
}

interface FixtureChipProps {
  fixture: FixtureData;
  gameState: GameStateData;
  bestOfContext: BestOfContext;
  userTeamId: string;
  onOpenFixtureResult: (stored: StoredFixtureDraftResult) => void;
  size?: "compact" | "full";
}

function FixtureChip({
  fixture,
  gameState,
  bestOfContext,
  userTeamId,
  onOpenFixtureResult,
  size = "compact",
}: FixtureChipProps) {
  const stored = readStoredFixtureDraftResult(fixture.id);
  const bo = inferBestOf(fixture, bestOfContext);
  const completed = fixture.status === "Completed";
  const score = normalizeLolScore(fixture, stored, userTeamId, bo);
  const homeLogo = getTeamLogoPath(gameState.teams, fixture.home_team_id);
  const awayLogo = getTeamLogoPath(gameState.teams, fixture.away_team_id);
  const isUserMatch =
    fixture.home_team_id === userTeamId || fixture.away_team_id === userTeamId;
  const userIsHome = fixture.home_team_id === userTeamId;
  const userResultTone = (() => {
    if (!isUserMatch || !completed || !score) return "";
    const userWins = userIsHome
      ? score.home > score.away
      : score.away > score.home;
    return userWins
      ? "bg-blue-500/15 dark:bg-blue-500/20 border-blue-500/30"
      : "bg-red-500/15 dark:bg-red-500/20 border-red-500/30";
  })();

  const clickable = completed && stored !== null;
  const isFull = size === "full";

  const homeName = getTeamName(gameState.teams, fixture.home_team_id);
  const awayName = getTeamName(gameState.teams, fixture.away_team_id);

  return (
    <button
      type="button"
      disabled={!clickable}
      onClick={() => clickable && stored && onOpenFixtureResult(stored)}
      className={[
        "group flex items-center gap-1 rounded border text-left transition-colors",
        isFull ? "px-3 py-2 gap-2" : "px-1 py-0.5",
        userResultTone ||
          (isUserMatch
            ? "bg-primary-50/70 dark:bg-primary-500/10 border-primary-400/30"
            : "bg-gray-50 dark:bg-navy-700/50 border-gray-200/60 dark:border-navy-600"),
        clickable ? "hover:border-primary-400 cursor-pointer" : "cursor-default",
      ].join(" ")}
      title={`${homeName} ${score ? `${score.home}-${score.away}` : "vs"} ${awayName}`}
    >
      {homeLogo ? (
        <img
          src={homeLogo}
          alt=""
          className={`object-contain shrink-0 ${isFull ? "w-5 h-5" : "w-3.5 h-3.5"}`}
          loading="lazy"
        />
      ) : (
        <span className={`shrink-0 ${isFull ? "w-5 h-5" : "w-3.5 h-3.5"}`} />
      )}
      {isFull ? (
        <span className="text-xs font-semibold text-gray-700 dark:text-gray-200 truncate">
          {homeName}
        </span>
      ) : null}
      <span
        className={`font-heading font-bold tabular-nums text-gray-700 dark:text-gray-200 ${
          isFull ? "text-sm px-2" : "text-[10px] px-0.5"
        }`}
      >
        {score ? `${score.home}-${score.away}` : "vs"}
      </span>
      {isFull ? (
        <span className="text-xs font-semibold text-gray-700 dark:text-gray-200 truncate">
          {awayName}
        </span>
      ) : null}
      {awayLogo ? (
        <img
          src={awayLogo}
          alt=""
          className={`object-contain shrink-0 ${isFull ? "w-5 h-5" : "w-3.5 h-3.5"}`}
          loading="lazy"
        />
      ) : (
        <span className={`shrink-0 ${isFull ? "w-5 h-5" : "w-3.5 h-3.5"}`} />
      )}
      <Badge
        variant="neutral"
        size="sm"
        className={
          isFull
            ? "ml-auto"
            : "ml-auto !text-[8px] !px-1 !py-0"
        }
      >
        BO{bo}
      </Badge>
    </button>
  );
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
  const [openDayKey, setOpenDayKey] = useState<string | null>(null);

  const bestOfContext = useMemo(() => buildBestOfContext(fixtures), [fixtures]);

  const fixturesByDay = useMemo(() => {
    const map = new Map<string, FixtureData[]>();
    fixtures.forEach((f) => {
      const key = f.date.substring(0, 10);
      const list = map.get(key) ?? [];
      list.push(f);
      map.set(key, list);
    });
    map.forEach((list, key) => {
      map.set(key, sortFixturesUserFirst(list, userTeamId));
    });
    return map;
  }, [fixtures, userTeamId]);

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

  const openDayFixtures = openDayKey ? fixturesByDay.get(openDayKey) ?? [] : [];

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
            const overflow = cellFixtures.length - MAX_FIXTURES_PER_CELL;

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
                  {cellFixtures.slice(0, MAX_FIXTURES_PER_CELL).map((f) => (
                    <FixtureChip
                      key={f.id}
                      fixture={f}
                      gameState={gameState}
                      bestOfContext={bestOfContext}
                      userTeamId={userTeamId}
                      onOpenFixtureResult={onOpenFixtureResult}
                    />
                  ))}
                  {overflow > 0 ? (
                    <button
                      type="button"
                      onClick={() => setOpenDayKey(cellKey)}
                      className="text-[9px] font-heading font-bold uppercase tracking-wider text-primary-500 hover:text-primary-600 dark:hover:text-primary-300 px-1 text-left transition-colors"
                    >
                      +{overflow} {t("schedule.moreMatches", "más")}
                    </button>
                  ) : null}
                </div>
              </div>
            );
          })}
        </div>
      </CardBody>

      {openDayKey && openDayFixtures.length > 0 ? (
        <div
          className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
          onClick={() => setOpenDayKey(null)}
        >
          <div
            className="bg-white dark:bg-navy-800 rounded-xl shadow-2xl border border-gray-200 dark:border-navy-600 w-full max-w-lg max-h-[80vh] flex flex-col"
            onClick={(event) => event.stopPropagation()}
          >
            <div className="flex items-center justify-between gap-3 px-5 py-4 border-b border-gray-100 dark:border-navy-600">
              <div className="flex flex-col gap-0.5">
                <h3 className="text-sm font-heading font-bold uppercase tracking-wider text-gray-500 dark:text-gray-400">
                  {t("schedule.matchesOnDay", "Partidos del día")}
                </h3>
                <p className="text-base font-heading font-bold text-gray-800 dark:text-gray-100">
                  {formatMatchDate(openDayKey, i18n.language)}
                </p>
              </div>
              <button
                type="button"
                onClick={() => setOpenDayKey(null)}
                aria-label={t("common.close", "Cerrar")}
                className="inline-flex items-center justify-center w-8 h-8 rounded-md text-gray-500 dark:text-gray-300 hover:text-primary-500 hover:bg-gray-100 dark:hover:bg-navy-700 transition-colors"
              >
                <X className="w-4 h-4" />
              </button>
            </div>
            <div className="flex flex-col gap-2 p-5 overflow-y-auto">
              {openDayFixtures.map((f) => (
                <FixtureChip
                  key={f.id}
                  fixture={f}
                  gameState={gameState}
                  bestOfContext={bestOfContext}
                  userTeamId={userTeamId}
                  onOpenFixtureResult={(stored) => {
                    setOpenDayKey(null);
                    onOpenFixtureResult(stored);
                  }}
                  size="full"
                />
              ))}
            </div>
          </div>
        </div>
      ) : null}
    </Card>
  );
}
