import { useEffect, useState, useMemo } from "react";
import { ArrowLeft, Users, AlertTriangle, Trophy, TrendingUp, Target, Crosshair, Sparkles, Shield } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useGameStore } from "../store/gameStore";
import { ROLE_ICON_PATHS } from "../lib/roleIcons";
import { Card, CardBody, CardHeader } from "../components/ui";

export interface ChampionPageProps {
  championKey: string;
  onClose: () => void;
}

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
  a?: string;
  b?: string;
  value?: number;
  champion_key?: string;
  champion_name?: string;
  role?: string;
  reason?: string;
}

/**
 * Extracts the opposing champion key from a counterpick/synergy entry.
 * The backend stores { a: "Aatrox", b: "Chogath", value: 1 } where "a" is the
 * subject champion and "b" is the counter/synergy target.
 */
function extractOpponentKey(item: CounterpickOrSynergyItem, subjectKey: string): string {
  // Prefer explicit champion_key if present
  if (item.champion_key) return item.champion_key;
  // Backend format: { a, b, value } — return "b" if "a" matches subject
  if (item.a === subjectKey && item.b) return item.b;
  // Fallback: if only one of a/b is present and doesn't match subject, use it
  if (item.b && item.b !== subjectKey) return item.b;
  if (item.a && item.a !== subjectKey) return item.a;
  return item.champion_name || "";
}

export default function ChampionPage({ championKey, onClose }: ChampionPageProps) {
  const { t } = useTranslation();
  const [showFullImage, setShowFullImage] = useState(false);

  // Get champions from game store - stable selector
  const champions = useGameStore((state) => state.gameState?.champions);

  // Find champion by champion_key
  const champion = useMemo(() => {
    if (!champions || !championKey) return undefined;
    return champions.find((c) => c.champion_key === championKey);
  }, [champions, championKey]);

  // Handle keyboard escape
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onClose();
      }
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [onClose]);

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
            onClick={onClose}
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

  // Render a list of counterpick/synergy items
  function renderChampionList(
    items: CounterpickOrSynergyItem[],
    sectionClass: string,
    titleKey: string,
    titleDefault: string,
  ) {
    if (items.length === 0) return null;
    return (
      <section className={sectionClass}>
        <h3 className="mb-4 flex items-center gap-2 text-xl font-heading font-bold uppercase tracking-wider">
          {t(titleKey, titleDefault)}
        </h3>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3">
          {items.map((item, idx) => {
            const champKey = extractOpponentKey(item, champion.champion_key);
            const imgUrl = champKey ? fallbackTileUrl(champKey) : "";
            return (
              <div
                key={`item-${idx}`}
                className="flex items-center gap-3 rounded-lg border border-navy-600/30 bg-navy-800/50 p-3"
              >
                {imgUrl ? (
                  <img
                    src={imgUrl}
                    alt={champKey}
                    className="h-12 w-12 rounded object-cover"
                    onError={(e) => {
                      const img = e.currentTarget;
                      img.onerror = null;
                      img.src = champKey ? fallbackTileUrl(champKey) : "";
                    }}
                  />
                ) : (
                  <div className="h-12 w-12 rounded bg-navy-700" />
                )}
                <div className="flex-1 min-w-0">
                  <p className="text-sm font-heading font-semibold text-gray-100 truncate">
                    {champKey}
                  </p>
                  {item.value !== undefined && (
                    <p className="text-xs text-gray-400">
                      {item.value} {item.value === 1 ? "game" : "games"}
                    </p>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </section>
    );
  }

  return (
    <div className="min-h-screen bg-navy-900">
      {/* Back button - matching PlayerProfile style */}
      <div className="max-w-6xl mx-auto px-4 pt-4">
        <button
          onClick={onClose}
          className="flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200 transition-colors mb-4"
        >
          <ArrowLeft className="w-4 h-4" />
          <span className="font-heading font-bold uppercase tracking-wider">
            {t("common.back")}
          </span>
        </button>
      </div>

      <div className="max-w-6xl mx-auto px-4 pb-8">
        {/* Hero Card - matching PlayerProfileHeroCard */}
        <Card accent="primary" className="mb-5">
          <div className="relative p-8 rounded-t-xl overflow-hidden">
            {splashUrl ? (
              <>
                <div
                  className="absolute inset-0 bg-cover opacity-100"
                  style={{
                    backgroundImage: `url(${showFullImage ? splashUrl : tileUrl})`,
                    backgroundPosition: "center 12%",
                    cursor: "pointer",
                  }}
                  onClick={() => setShowFullImage(!showFullImage)}
                />
                <div className="absolute inset-0 bg-linear-to-r from-black/88 via-black/28 to-transparent" />
              </>
            ) : (
              <div className="absolute inset-0 bg-linear-to-r from-navy-700 to-navy-800" />
            )}

            <div className="relative z-10 flex items-center gap-6">
              {/* Champion Avatar - matching player photo style */}
              <div className="relative w-24 h-24 shrink-0">
                <div className="w-24 h-24 rounded-2xl overflow-hidden border-2 border-primary-500/40">
                  <img
                    src={tileUrl}
                    alt={champion.name}
                    className="w-full h-full object-cover"
                    loading="lazy"
                  />
                </div>
              </div>

              {/* Champion Info */}
              <div className="flex-1">
                <h2 className="text-3xl font-heading font-bold text-white uppercase tracking-wide">
                  {champion.name}
                </h2>
                <div className="flex items-center gap-3 mt-2">
                  {roles.map((role) => {
                    const iconPath = mapRoleToIconPath(role);
                    if (!iconPath) return null;
                    return (
                      <div
                        key={role}
                        className="flex items-center gap-1 rounded-lg bg-black/40 px-2 py-1"
                      >
                        <img
                          src={iconPath}
                          alt={role}
                          className="h-5 w-5"
                          title={role}
                        />
                        <span className="text-xs font-heading text-gray-200">
                          {role}
                        </span>
                      </div>
                    );
                  })}
                </div>
                <p className="text-gray-400 text-sm mt-2 flex items-center gap-1.5">
                  {champion.champion_key}
                </p>
              </div>

              {/* QuickStats - Desktop */}
              <div className="hidden md:flex items-center gap-3">
                <div className="grid grid-cols-3 gap-3 flex-1">
                  <QuickStat
                    label={t("champions.winRate", "Win Rate")}
                    value="--"
                    color="text-accent-300"
                  />
                  <QuickStat
                    label={t("champions.pickRate", "Pick Rate")}
                    value="--"
                    color="text-primary-400"
                  />
                  <QuickStat
                    label={t("champions.banRate", "Ban Rate")}
                    value="--"
                    color="text-red-400"
                  />
                  <QuickStat
                    label={t("champions.kda", "KDA")}
                    value="--"
                    color="text-gray-200"
                  />
                  <QuickStat
                    label={t("champions.tier", "Tier")}
                    value="--"
                    color="text-white"
                  />
                  <QuickStat
                    label={t("champions.difficulty", "Dificultad")}
                    value="--"
                    color="text-white"
                  />
                </div>
              </div>
            </div>
          </div>

        {/* Content Sections */}
        <div className="space-y-6">
          {/* Counterpicks Section */}
          {renderChampionList(
            counterpicks,
            "rounded-xl border border-red-400/30 bg-red-500/5 p-6",
            "champions.counterpicks",
            "Counterpicks",
          )}

          {/* Synergies Section */}
          {renderChampionList(
            synergies,
            "rounded-xl border border-emerald-400/30 bg-emerald-500/5 p-6",
            "champions.synergies",
            "Sinergias",
          )}

          {/* Right column - Synergies + Stats placeholder */}
          <div className="lg:col-span-2 flex flex-col gap-5">
            <Card>
              <CardHeader>
                <span className="text-emerald-400">{t("champions.synergies", "Sinergias")}</span>
              </CardHeader>
              <CardBody>
                {synergies.length > 0 ? (
                  <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
                    {synergies.map((item, idx) => {
                      const champKey = extractOpponentKey(item, champion.champion_key);
                      const imgUrl = champKey ? fallbackTileUrl(champKey) : "";
                      return (
                        <div
                          key={`syn-${idx}`}
                          className="flex items-center gap-2 rounded-lg border border-emerald-400/20 bg-navy-800/50 p-2"
                        >
                          {imgUrl ? (
                            <img
                              src={imgUrl}
                              alt={champKey}
                              className="h-10 w-10 rounded object-cover"
                              onError={(e) => {
                                const img = e.currentTarget;
                                img.onerror = null;
                                img.src = champKey ? fallbackTileUrl(champKey) : "";
                              }}
                            />
                          ) : (
                            <div className="h-10 w-10 rounded bg-navy-700" />
                          )}
                          <div className="flex-1 min-w-0">
                            <p className="text-sm font-heading font-semibold text-gray-100 truncate">
                              {champKey}
                            </p>
                            {item.value !== undefined && (
                              <p className="text-[10px] text-gray-400">
                                {item.value} {item.value === 1 ? "game" : "games"}
                              </p>
                            )}
                          </div>
                        </div>
                      );
                    })}
                  </div>
                ) : (
                  <div className="text-center py-6">
                    <Users className="w-8 h-8 text-gray-400 dark:text-gray-500 mx-auto mb-2" />
                    <p className="text-sm text-gray-400 dark:text-gray-500">
                      {t("champions.noSynergies", "Sin sinergias registradas")}
                    </p>
                  </div>
                )}
              </CardBody>
            </Card>

            {/* Stats placeholder card */}
            <Card>
              <CardHeader>{t("champions.stats", "Estadísticas")}</CardHeader>
              <CardBody>
                <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
                  <div className="text-center p-3 rounded-lg bg-navy-800/50">
                    <Trophy className="w-6 h-6 mx-auto mb-2 text-accent-400" />
                    <p className="text-xs text-gray-400">{t("champions.winRate", "Win Rate")}</p>
                    <p className="text-lg font-heading font-bold text-gray-200">--</p>
                  </div>
                  <div className="text-center p-3 rounded-lg bg-navy-800/50">
                    <TrendingUp className="w-6 h-6 mx-auto mb-2 text-primary-400" />
                    <p className="text-xs text-gray-400">{t("champions.pickRate", "Pick Rate")}</p>
                    <p className="text-lg font-heading font-bold text-gray-200">--</p>
                  </div>
                  <div className="text-center p-3 rounded-lg bg-navy-800/50">
                    <Target className="w-6 h-6 mx-auto mb-2 text-red-400" />
                    <p className="text-xs text-gray-400">{t("champions.banRate", "Ban Rate")}</p>
                    <p className="text-lg font-heading font-bold text-gray-200">--</p>
                  </div>
                  <div className="text-center p-3 rounded-lg bg-navy-800/50">
                    <Crosshair className="w-6 h-6 mx-auto mb-2 text-gray-300" />
                    <p className="text-xs text-gray-400">{t("champions.kda", "KDA")}</p>
                    <p className="text-lg font-heading font-bold text-gray-200">--</p>
                  </div>
                  <div className="text-center p-3 rounded-lg bg-navy-800/50">
                    <Sparkles className="w-6 h-6 mx-auto mb-2 text-yellow-400" />
                    <p className="text-xs text-gray-400">{t("champions.tier", "Tier")}</p>
                    <p className="text-lg font-heading font-bold text-gray-200">--</p>
                  </div>
                  <div className="text-center p-3 rounded-lg bg-navy-800/50">
                    <Shield className="w-6 h-6 mx-auto mb-2 text-blue-400" />
                    <p className="text-xs text-gray-400">{t("champions.difficulty", "Dificultad")}</p>
                    <p className="text-lg font-heading font-bold text-gray-200">--</p>
                  </div>
                </div>
              </CardBody>
            </Card>
          </div>
        </div>
      </div>
    </div>
  );
}
