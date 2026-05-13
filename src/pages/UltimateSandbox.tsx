import { useState } from "react";
import { useNavigate } from "react-router-dom";

import { Button } from "../components/ui";
import { UltimateLiveSandbox } from "../components/match/lol-prototype/ui/UltimateLiveSandbox";
import { ULTIMATE_SANDBOX_IDENTITIES } from "../components/match/lol-prototype/ui/ultimateSandbox";

export default function UltimateSandbox() {
  const navigate = useNavigate();
  const [selectedSignature, setSelectedSignature] = useState(
    ULTIMATE_SANDBOX_IDENTITIES[0]?.signatureId ?? "",
  );

  return (
    <div className="min-h-screen bg-gray-100 dark:bg-navy-900 text-gray-900 dark:text-white p-6">
      <div className="mx-auto max-w-6xl space-y-5">
        <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
          <div>
            <p className="text-xs font-heading uppercase tracking-[0.25em] text-primary-500">
              Debug visual
            </p>
            <h1 className="text-3xl font-heading font-bold uppercase">
              Ultimate Sandbox
            </h1>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Probá ultimates dentro de una live sim V2 aislada, con movimiento e IA real.
            </p>
          </div>
          <Button onClick={() => navigate("/")}>Volver al menú</Button>
        </div>

        <UltimateLiveSandbox
          selectedSignature={selectedSignature}
          onSelectedSignatureChange={setSelectedSignature}
        />
      </div>
    </div>
  );
}
