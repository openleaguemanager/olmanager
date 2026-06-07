import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { ArrowLeft, ChevronRight, Globe, Trophy, Users } from "lucide-react";
import type { CompetitionSummary } from "@/store/gameStore";
import { cn } from "@/ui-v2/lib/utils";

interface LeaguePickerV2Props {
  competitions: CompetitionSummary[];
  onSelect: (id: string) => void;
  onBack: () => void;
}

const REGION_BG: Record<string, string> = {
  "LEC": "from-blue-600/30 via-blue-800/10 to-transparent",
  "LCS": "from-red-600/30 via-red-800/10 to-transparent",
  "LCK": "from-green-600/30 via-green-800/10 to-transparent",
  "LPL": "from-yellow-500/30 via-yellow-700/10 to-transparent",
  "LCP": "from-purple-600/30 via-purple-800/10 to-transparent",
  "CBLOL": "from-emerald-600/30 via-emerald-800/10 to-transparent",
};

export function LeaguePickerV2({ competitions, onSelect, onBack }: LeaguePickerV2Props) {
  const { t } = useTranslation();
  const [visible, setVisible] = useState(false);
  useEffect(() => setVisible(true), []);

  return (
    <div className="flex h-full flex-col bg-background">
      {/* Header */}
      <header className="relative flex h-14 shrink-0 items-center gap-3 border-b border-border bg-gradient-to-r from-primary/5 to-transparent px-6">
        <button
          type="button"
          onClick={onBack}
          className="flex size-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
        >
          <ArrowLeft className="size-4" />
        </button>
        <div>
          <h1 className="font-heading text-lg font-black uppercase tracking-widest text-foreground">
            {t("teamSelect.selectLeague", "Select League")}
          </h1>
          <p className="text-xs text-muted-foreground/70">
            {t("teamSelect.selectLeagueSubtitle", "Choose a competition")}
          </p>
        </div>
        <Trophy className="ml-auto size-5 text-muted-foreground/20" />
      </header>

      {/* Grid */}
      <div className="flex-1 overflow-y-auto p-6 md:p-8 scrollbar-v2">
        <div className="mx-auto grid max-w-2xl grid-cols-1 gap-4 md:grid-cols-2">
          {competitions.map((comp, i) => {
            const region = Object.keys(REGION_BG).find((k) => comp.name.toUpperCase().includes(k) || comp.region.toUpperCase().includes(k));
            const bgGradient = region ? REGION_BG[region] : "from-primary/20 via-primary/5 to-transparent";

            return (
              <button
                key={comp.id}
                type="button"
                onClick={() => onSelect(comp.id)}
                className={cn(
                  "group relative overflow-hidden rounded-xl border border-border bg-card p-6 text-left transition-all duration-300",
                  "hover:-translate-y-1 hover:border-primary/40 hover:shadow-xl hover:shadow-primary/5",
                  visible && "animate-fade-in-up",
                )}
                style={{ animationDelay: `${i * 60}ms` }}
              >
                {/* Regional gradient glow */}
                <div className={cn("absolute -right-20 -top-20 size-40 rounded-full opacity-0 blur-3xl transition-opacity duration-500 group-hover:opacity-100", bgGradient.replace("from-", "bg-").split("/")[0]?.replace("from-", "bg-") ?? "bg-primary/10")} />
                <div className={cn("absolute inset-0 bg-gradient-to-br opacity-0 transition-opacity duration-500 group-hover:opacity-100", bgGradient)} />

                <div className="relative z-10 flex items-start gap-4">
                  <div className={cn(
                    "flex size-14 shrink-0 items-center justify-center overflow-hidden rounded-xl border-2 transition-all duration-300",
                    "bg-muted group-hover:border-primary/30 group-hover:shadow-lg group-hover:shadow-primary/10"
                  )}>
                    {comp.logo ? (
                      <img src={comp.logo} alt={comp.name} className="size-10 object-contain" />
                    ) : (
                      <Globe className="size-6 text-muted-foreground" />
                    )}
                  </div>
                  <div className="min-w-0 flex-1">
                    <h3 className="truncate font-heading text-base font-bold uppercase tracking-wide text-foreground group-hover:text-primary transition-colors">
                      {comp.name}
                    </h3>
                    <p className="mt-0.5 text-xs text-muted-foreground">{comp.region}</p>
                    <div className="mt-3 flex items-center gap-3 text-xs text-muted-foreground/60">
                      <span className="flex items-center gap-1.5">
                        <Users className="size-3.5" />
                        {comp.team_count} {t("teamSelect.teams", "teams")}
                      </span>
                      <ChevronRight className="size-3.5 opacity-0 transition-all group-hover:translate-x-1 group-hover:opacity-100" />
                    </div>
                  </div>
                </div>
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
}
