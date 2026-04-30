import { useCallback } from "react";
import { useNavigate } from "react-router-dom";
import ChampionsGrid from "../champions/ChampionsGrid";
import type { ChampionData } from "../../store/types";

interface ChampionsWorldTabProps {
  champions?: ChampionData[];
}

export default function ChampionsWorldTab({ champions }: ChampionsWorldTabProps) {
  const navigate = useNavigate();

  const handleChampionClick = useCallback((championKey: string) => {
    navigate(`/champion/${championKey}`);
  }, [navigate]);

  return (
    <div className="space-y-6">
      {/* Champions Grid */}
      <section className="rounded-2xl border border-gray-200 dark:border-navy-600 bg-white dark:bg-navy-800 p-4">
        <ChampionsGrid champions={champions} onChampionClick={handleChampionClick} />
      </section>
    </div>
  );
}