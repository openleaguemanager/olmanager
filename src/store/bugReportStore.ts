import { create } from "zustand";

export interface BugReportContext {
  currentRoute: string;
  currentDashboardTab: string;
}

interface BugReportStore extends BugReportContext {
  setCurrentRoute: (route: string) => void;
  setCurrentDashboardTab: (tab: string) => void;
}

export const useBugReportStore = create<BugReportStore>((set) => ({
  currentRoute: "",
  currentDashboardTab: "",

  setCurrentRoute: (route) => set({ currentRoute: route }),
  setCurrentDashboardTab: (tab) => set({ currentDashboardTab: tab }),
}));
