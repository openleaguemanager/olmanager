import type { GameStateData } from "@/store/gameStore";
import { useActiveLeague } from "@/store/gameStore";
import SocialTab from "@/ui-v2/_legacy/components/social/SocialTab";
import { Clock, MessageSquare } from "lucide-react";
import { useTranslation } from "react-i18next";

interface SocialTabV2Props {
  gameState: GameStateData;
  onGameUpdate: (state: GameStateData) => void;
}

export function SocialTabV2({ gameState, onGameUpdate }: SocialTabV2Props) {
  const { t } = useTranslation();
  const activeLeague = useActiveLeague(gameState);
  const isLEC = activeLeague?.competition_id === "lec";

  if (!isLEC) {
    return (
      <div className="flex min-h-0 flex-1 flex-col items-center justify-center gap-6 p-6 text-center">
        <div className="flex h-20 w-20 items-center justify-center rounded-full bg-orange-100">
          <MessageSquare className="h-10 w-10 text-orange-500" />
        </div>
        <div className="max-w-md space-y-2">
          <h2 className="font-heading text-xl font-bold uppercase tracking-wide text-zinc-900">
            {t("social.comingSoonTitle")}
          </h2>
          <p
            className="text-sm text-zinc-500"
            dangerouslySetInnerHTML={{ __html: t("social.comingSoonBody") }}
          />
          <p className="text-sm text-zinc-400">
            {t("social.comingSoonSub")}
          </p>
        </div>
        <div className="flex items-center gap-2 rounded-full bg-zinc-100 px-4 py-2 text-xs font-medium text-zinc-500">
          <Clock className="h-4 w-4" />
          <span>{t("social.currentlyPlaying")} {activeLeague?.name ?? "Unknown League"}</span>
        </div>
      </div>
    );
  }

  return (
    <div className="flex min-h-0 flex-1 flex-col overflow-y-auto p-6 scrollbar-v2">
      <SocialTab gameState={gameState} onGameUpdate={onGameUpdate} />
    </div>
  );
}
