import { useEffect, useState, useMemo } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { ArrowLeft } from "lucide-react";
import { useGameStore } from "../store/gameStore";
import { ROLE_ICON_PATHS } from "../lib/roleIcons";

/**
 * Maps DB role names to ROLE_ICON_PATHS keys (uppercase)
 */
function mapRoleToIconPath(role: string): string | undefined {
  const normalized = role.toUpperCase();
  if (normalized === "TOP") return ROLE_ICON_PATHS.TOP;
  if (normalized === "JUNGLE") return ROLE_ICON_PATHS.JUNGLE;
  if (normalized === "JUNGLER") return ROLE_ICON_PATHS.JUNGLE;
  if (normalized === "MID") return ROLE_ICON_PATHS.MID;
  if (normalized === "ADC" || normalized === "BOT") return ROLE_ICON_PATHS.ADC;
  if (normalized === "SUPPORT") return ROLE_ICON_PATHS.SUPPORT;
  return undefined;
}

/**
 * Fallback champion tile URL from Data Dragon
 */
function fallbackTileUrl(championKey: string): string {
  return `https://ddragon.leagueoflegends.com/cdn/img/champion/tiles/${championKey}_0.jpg`;
}

/**
 * Fallback champion splash URL from Data Dragon
 */
function fallbackSplashUrl(championKey: string): string {
  return `https://ddragon.leagueoflegends.com/cdn/img/champion/splash/${championKey}_0.jpg`;
}

function parseJsonField<T>(json: string | null, fallback: T): T {
  if (!json) return fallback;
  try {
    const parsed = JSON.parse(json);
    return parsed ?? fallback;
  } catch {
    return fallback;
  }
}

interface CounterpickOrSynergyItem {
  champion_key?: string;
  champion_name?: string;
  role?: string;
  reason?: string;
}

function inferRoleHints(items: CounterpickOrSynergyItem[]): string[] {
  const rolesSet = new Set<string>();
  items.forEach((item) => {
    if (item.role) {
      rolesSet.add(item.role);
    }
  });
  return Array.from(rolesSet);
}

export default function ChampionPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { t } = useTranslation();
  const [showFullImage, setShowFullImage] = useState(false);

  // Get champions from game store - stable selector
  const champions = useGameStore((state) => state.gameState?.champions);

  // Find champion by ID - memoized to avoid re-computing on every render
  const champion = useMemo(() => {
    if (!champions || !id) return undefined;
    return champions.find((c) => c.id.toString() === id);
  }, [champions, id]);

  // Handle keyboard escape
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        navigate(-1);
      }
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [navigate]);

  // Don't render if no champion (show loading or not found)
  if (!champions) {
    return (
      <div className="min-h-screen bg-navy-900 flex items-center justify-center">
        <div className="text-gray-400">{t("common.loading", "Cargando...")}</div>
      </div>
    );
  }

  if (!champion) {
    return (
      <div className="min-h-screen bg-navy-900 flex items-center justify-center">
        <div className="text-center space-y-4">
          <p className="text-gray-400 text-lg">{t("champions.notFound", "Campeón no encontrado")}</p>
          <button
            type="button"
            onClick={() => navigate(-1)}
            className="flex items-center gap-2 mx-auto text-primary-400 hover:text-primary-300 transition-colors"
          >
            <ArrowLeft className="h-5 w-5" />
            <span className="font-heading">{t("common.back", "Volver")}</span>
          </button>
        </div>
      </div>
    );
  }

  // Parse JSON fields
  const roles = parseJsonField<string[]>(champion.roles_json, []);
  const counterpicks = parseJsonField<CounterpickOrSynergyItem[]>(
    champion.counterpicks_json,
    [],
  );
  const synergies = parseJsonField<CounterpickOrSynergyItem[]>(
    champion.synergies_json,
    [],
  );

  // Determine image URLs
  const splashUrl =
    champion.image_splash_url || fallbackSplashUrl(champion.champion_key);
  const tileUrl =
    champion.image_tile_url || fallbackTileUrl(champion.champion_key);

  return (
    <div className="min-h-screen bg-navy-900">
      {/* Back button */}
      <div className="sticky top-0 z-10 bg-navy-900/90 backdrop-blur-sm border-b border-navy-600">
        <div className="max-w-6xl mx-auto px-4 py-3">
          <button
            type="button"
            onClick={() => navigate(-1)}
            className="flex items-center gap-2 text-gray-300 hover:text-white transition-colors"
          >
            <ArrowLeft className="h-5 w-5" />
            <span className="font-heading">{t("common.back", "Volver")}</span>
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="max-w-6xl mx-auto px-4 py-6 space-y-6">
        {/* Splash/Tile Image */}
        <div
          className="relative h-80 w-full cursor-pointer rounded-2xl overflow-hidden shadow-2xl"
          onClick={() => setShowFullImage(!showFullImage)}
        >
          <img
            src={showFullImage ? splashUrl : tileUrl}
            alt={champion.name}
            className="h-full w-full object-cover"
          />
          <div className="absolute inset-0 bg-gradient-to-t from-navy-900 via-navy-900/50 to-transparent" />

          {/* Champion Name Overlay */}
          <div className="absolute bottom-0 left-0 right-0 p-6">
            <h1 className="text-4xl font-heading font-bold text-white">
              {champion.name}
            </h1>
            <div className="flex gap-2 mt-3">
              {roles.map((role) => {
                const iconPath = mapRoleToIconPath(role);
                if (!iconPath) return null;
                return (
                  <div
                    key={role}
                    className="flex items-center gap-1 rounded-lg bg-black/40 px-3 py-1.5"
                  >
                    <img
                      src={iconPath}
                      alt={role}
                      className="h-5 w-5"
                      title={role}
                    />
                    <span className="text-sm font-heading text-gray-200">
                      {role}
                    </span>
                  </div>
                );
              })}
            </div>
          </div>
        </div>

        {/* Content Sections */}
        <div className="space-y-6">
          {/* Counterpicks Section */}
          {counterpicks.length > 0 && (
            <section className="rounded-xl border border-red-400/30 bg-red-500/5 p-6">
              <h3 className="mb-4 flex items-center gap-2 text-xl font-heading font-bold uppercase tracking-wider text-red-300">
                {t("champions.counterpicks", "Counterpicks")}
              </h3>
              <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3">
                {counterpicks.map((cp, idx) => {
                  const champKey = cp.champion_key || cp.champion_name || `unknown-${idx}`;
                  const imgUrl = fallbackTileUrl(champKey);
                  return (
                    <div
                      key={`cp-${idx}`}
                      className="flex items-center gap-3 rounded-lg border border-red-400/20 bg-navy-800/50 p-3"
                    >
                      <img
                        src={imgUrl}
                        alt={cp.champion_name || champKey}
                        className="h-12 w-12 rounded object-cover"
                        onError={(e) => {
                          const img = e.currentTarget;
                          img.onerror = null;
                          img.src = fallbackTileUrl(champKey);
                        }}
                      />
                      <div className="flex-1 min-w-0">
                        <p className="text-sm font-heading font-semibold text-gray-100 truncate">
                          {cp.champion_name || champKey}
                        </p>
                        {cp.role && (
                          <p className="text-xs text-gray-400">
                            {cp.role}
                          </p>
                        )}
                      </div>
                    </div>
                  );
                })}
              </div>
              {inferRoleHints(counterpicks).length > 0 && (
                <p className="mt-3 text-sm text-gray-400">
                  <span className="font-heading">Roles: </span>
                  {inferRoleHints(counterpicks).join(", ")}
                </p>
              )}
            </section>
          )}

          {/* Synergies Section */}
          {synergies.length > 0 && (
            <section className="rounded-xl border border-emerald-400/30 bg-emerald-500/5 p-6">
              <h3 className="mb-4 flex items-center gap-2 text-xl font-heading font-bold uppercase tracking-wider text-emerald-300">
                {t("champions.synergies", "Sinergias")}
              </h3>
              <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3">
                {synergies.map((syn, idx) => {
                  const champKey = syn.champion_key || syn.champion_name || `unknown-${idx}`;
                  const imgUrl = fallbackTileUrl(champKey);
                  return (
                    <div
                      key={`syn-${idx}`}
                      className="flex items-center gap-3 rounded-lg border border-emerald-400/20 bg-navy-800/50 p-3"
                    >
                      <img
                        src={imgUrl}
                        alt={syn.champion_name || champKey}
                        className="h-12 w-12 rounded object-cover"
                        onError={(e) => {
                          const img = e.currentTarget;
                          img.onerror = null;
                          img.src = fallbackTileUrl(champKey);
                        }}
                      />
                      <div className="flex-1 min-w-0">
                        <p className="text-sm font-heading font-semibold text-gray-100 truncate">
                          {syn.champion_name || champKey}
                        </p>
                        {syn.role && (
                          <p className="text-xs text-gray-400">
                            {syn.role}
                          </p>
                        )}
                      </div>
                    </div>
                  );
                })}
              </div>
              {inferRoleHints(synergies).length > 0 && (
                <p className="mt-3 text-sm text-gray-400">
                  <span className="font-heading">Roles: </span>
                  {inferRoleHints(synergies).join(", ")}
                </p>
              )}
            </section>
          )}

          {/* Empty state if no counterpicks or synergies */}
          {counterpicks.length === 0 && synergies.length === 0 && (
            <div className="rounded-xl border border-navy-600 bg-navy-800/50 p-8 text-center">
              <p className="text-base text-gray-400">
                {t(
                  "champions.noData",
                  "No hay información de counterpicks o sinergias disponible.",
                )}
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}