import { useTranslation } from "react-i18next";
import { Card, CardContent, CardHeader, CardTitle } from "@/ui-v2/components/ui/card";

export function Placeholder({ title }: { title: string }) {
  const { t } = useTranslation();
  return (
    <div className="p-6">
      <Card>
        <CardHeader>
          <CardTitle>{title}</CardTitle>
        </CardHeader>
        <CardContent className="text-sm text-muted-foreground">
          {t("placeholder.pendingDesign")}
        </CardContent>
      </Card>
    </div>
  );
}
