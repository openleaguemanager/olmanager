import { lazy, Suspense } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import DashboardV2 from "./dashboard/DashboardV2";

const MainMenu = lazy(() => import("@/pages/MainMenu"));
const TeamSelection = lazy(() => import("@/pages/TeamSelection"));
const MatchSimulation = lazy(() => import("@/pages/MatchSimulation"));
const Settings = lazy(() => import("@/pages/Settings"));

function LazyFallback() {
  return (
    <div className="flex h-screen items-center justify-center bg-background">
      <div className="size-8 animate-spin rounded-full border-4 border-primary border-t-transparent" />
    </div>
  );
}

export default function AppV2() {
  return (
    <div className="dark">
      <BrowserRouter>
        <Suspense fallback={<LazyFallback />}>
          <Routes>
            <Route path="/" element={<MainMenu />} />
            <Route path="/select-team" element={<TeamSelection />} />
            <Route path="/dashboard" element={<DashboardV2 />} />
            <Route path="/match" element={<MatchSimulation />} />
            <Route path="/settings" element={<Settings />} />
          </Routes>
        </Suspense>
      </BrowserRouter>
    </div>
  );
}
