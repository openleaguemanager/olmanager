import type { PlayerData, StaffData, TeamData } from "../store/gameStore";

export type FinanceHealthLevel = "stable" | "watch" | "warning" | "critical";

export interface TeamFinanceSnapshot {
  annualWageBill: number;
  annualWageBudget: number;
  annualSponsorIncome: number;
  weeklyWageBudget: number;
  projectedAnnualNet: number;
  cashRunwayMonths: number | null;
  wageBudgetUsagePercent: number;
  wageBudgetStatus: FinanceHealthLevel;
  runwayStatus: FinanceHealthLevel;
  overallStatus: FinanceHealthLevel;
}

const HEALTH_PRIORITY: Record<FinanceHealthLevel, number> = {
  stable: 0,
  watch: 1,
  warning: 2,
  critical: 3,
};

function safeNumber(value: unknown, fallback = 0): number {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

export function getAnnualWageBill(
  players: PlayerData[],
  staff: StaffData[] = [],
): number {
  return [...players, ...staff].reduce((sum, person) => {
    return sum + Math.max(0, safeNumber(person.wage));
  }, 0);
}
export function annualAmountToMonthlyCommitment(amount: number): number {
  return Math.floor(Math.max(0, safeNumber(amount)) / 12);
}
export function getCashRunwayMonths(
  balance: number,
  projectedAnnualNet: number,
): number | null {
  const projectedWeeklyNet = safeNumber(projectedAnnualNet) / 52;
  if (projectedWeeklyNet >= 0) {
    return null;
  }

  return Math.max(0, Math.floor(safeNumber(balance) / Math.abs(projectedWeeklyNet)));
}

function getWageBudgetStatus(usagePercent: number): FinanceHealthLevel {
  if (usagePercent > 110) {
    return "critical";
  }

  if (usagePercent > 100) {
    return "warning";
  }

  if (usagePercent >= 85) {
    return "watch";
  }

  return "stable";
}

function getRunwayStatus(
  balance: number,
  runwayMonths: number | null,
): FinanceHealthLevel {
  if (balance < 0) {
    return "critical";
  }

  if (runwayMonths === null) {
    return "stable";
  }

  if (runwayMonths <= 4) {
    return "critical";
  }

  if (runwayMonths <= 8) {
    return "warning";
  }

  if (runwayMonths <= 12) {
    return "watch";
  }

  return "stable";
}

function getMostSevereLevel(
  left: FinanceHealthLevel,
  right: FinanceHealthLevel,
): FinanceHealthLevel {
  return HEALTH_PRIORITY[left] >= HEALTH_PRIORITY[right] ? left : right;
}

export function getTeamFinanceSnapshot(
  team: TeamData,
  players: PlayerData[],
  staff: StaffData[] = [],
): TeamFinanceSnapshot {
  const annualWageBill = getAnnualWageBill(players, staff);
  const annualWageBudget = safeNumber(team.wage_budget);
  const annualSponsorIncome = safeNumber(team.sponsorship?.base_value);
  const projectedAnnualNet = annualSponsorIncome - annualWageBill;
  const finance = safeNumber(team.finance);
  const cashRunwayMonths = getCashRunwayMonths(finance, projectedAnnualNet);
  const wageBudgetUsagePercent = Math.round(
    (annualWageBill / Math.max(1, annualWageBudget)) * 100,
  );
  const wageBudgetStatus = getWageBudgetStatus(wageBudgetUsagePercent);
  const runwayStatus = getRunwayStatus(finance, cashRunwayMonths);

  return {
    annualWageBill,
    annualWageBudget,
    annualSponsorIncome,
    projectedAnnualNet,
    cashRunwayMonths,
    wageBudgetUsagePercent,
    wageBudgetStatus,
    runwayStatus,
    overallStatus: getMostSevereLevel(wageBudgetStatus, runwayStatus),
  };
}
