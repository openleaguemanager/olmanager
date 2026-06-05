import { ArrowLeft, Calendar as CalendarIcon, Loader2, Play, Save, Search } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Button } from "@/ui-v2/components/ui/button";
import { Separator } from "@/ui-v2/components/ui/separator";

interface Props {
  activeTabLabel: string;
  currentDate: string;
  hasProfileHistory: boolean;
  isAdvancing: boolean;
  isSaving: boolean;
  saveFlash: boolean;
  hasMatchToday: boolean;
  onBack: () => void;
  onSave: () => void;
  onContinue: () => void;
}

export function DashboardHeaderV2({
  activeTabLabel,
  currentDate,
  hasProfileHistory,
  isAdvancing,
  isSaving,
  saveFlash,
  hasMatchToday,
  onBack,
  onSave,
  onContinue,
}: Props) {
  const { t } = useTranslation();

  return (
    <header className="flex h-14 shrink-0 items-center gap-3 border-b border-border bg-background/60 px-4 backdrop-blur">
      {hasProfileHistory && (
        <Button variant="ghost" size="icon-sm" onClick={onBack} aria-label="Back">
          <ArrowLeft className="size-4" />
        </Button>
      )}

      <h1 className="font-heading text-base font-semibold uppercase tracking-wider">
        {activeTabLabel}
      </h1>

      <Separator orientation="vertical" className="h-6" />

      <div className="flex items-center gap-1.5 text-sm text-muted-foreground">
        <CalendarIcon className="size-4" />
        <span>{currentDate}</span>
      </div>

      <div className="ml-auto flex items-center gap-2">
        <Button variant="ghost" size="icon-sm" aria-label="Search">
          <Search className="size-4" />
        </Button>

        <Button variant="outline" size="sm" onClick={onSave} disabled={isSaving}>
          {isSaving ? <Loader2 className="size-4 animate-spin" /> : <Save className="size-4" />}
          <span className="ml-1.5">
            {saveFlash ? t("dashboard.saved", { defaultValue: "Saved" }) : t("dashboard.save", { defaultValue: "Save" })}
          </span>
        </Button>

        <Button onClick={onContinue} disabled={isAdvancing} size="sm">
          {isAdvancing ? <Loader2 className="size-4 animate-spin" /> : <Play className="size-4" />}
          <span className="ml-1.5">
            {hasMatchToday
              ? t("continueMenu.goToField", { defaultValue: "Play Match" })
              : t("dashboard.continue", { defaultValue: "Continue" })}
          </span>
        </Button>
      </div>
    </header>
  );
}
