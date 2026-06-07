import type { GameStateData } from "@/store/gameStore";
import ScheduleTab from "@/components/schedule/ScheduleTab";
import { cn } from "@/ui-v2/lib/utils";

interface ScheduleTabV2Props {
  gameState: GameStateData;
  onSelectTeam: (id: string) => void;
}

export function ScheduleTabV2({ gameState, onSelectTeam }: ScheduleTabV2Props) {
  return (
    <div className="competitions-v2 flex h-full flex-col overflow-y-auto p-6 scrollbar-v2">
      <ScheduleTab
        gameState={gameState}
        onSelectTeam={onSelectTeam}
      />
    </div>
  );
}
