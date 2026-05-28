import { Card, CardContent, CardHeader, CardTitle } from "@/ui-v2/components/ui/card";
import { Button } from "@/ui-v2/components/ui/button";
import { Badge } from "@/ui-v2/components/ui/badge";

export default function AppV2() {
  return (
    <div className="dark min-h-screen bg-background text-foreground">
      <div className="mx-auto max-w-5xl p-8 space-y-6">
        <header className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-semibold tracking-tight">OLManager</h1>
            <p className="text-muted-foreground text-sm">UI v2 — base lista</p>
          </div>
          <Badge>v2</Badge>
        </header>

        <Card>
          <CardHeader>
            <CardTitle>Andamiaje funcionando</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <p className="text-muted-foreground text-sm">
              Tokens del tema activos (zinc + acento naranja). A partir de aquí construimos las pantallas.
            </p>
            <div className="flex gap-2">
              <Button>Primario</Button>
              <Button variant="outline">Outline</Button>
              <Button variant="ghost">Ghost</Button>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
