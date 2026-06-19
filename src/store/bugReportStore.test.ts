import { beforeEach, describe, expect, it } from "vitest";
import { useBugReportStore } from "./bugReportStore";

beforeEach(() => {
  useBugReportStore.setState({
    currentRoute: "",
    currentDashboardTab: "",
  });
});

describe("useBugReportStore", () => {
  it("starts with empty route and dashboard tab", () => {
    const state = useBugReportStore.getState();

    expect(state.currentRoute).toBe("");
    expect(state.currentDashboardTab).toBe("");
  });

  it("updates the current route", () => {
    useBugReportStore.getState().setCurrentRoute("/dashboard");

    expect(useBugReportStore.getState().currentRoute).toBe("/dashboard");
  });

  it("updates the current dashboard tab independently of the route", () => {
    useBugReportStore.getState().setCurrentRoute("/dashboard");
    useBugReportStore.getState().setCurrentDashboardTab("Training");

    expect(useBugReportStore.getState().currentRoute).toBe("/dashboard");
    expect(useBugReportStore.getState().currentDashboardTab).toBe("Training");
  });

  it("can clear the current dashboard tab", () => {
    useBugReportStore.getState().setCurrentDashboardTab("Finances");
    useBugReportStore.getState().setCurrentDashboardTab("");

    expect(useBugReportStore.getState().currentDashboardTab).toBe("");
  });
});
