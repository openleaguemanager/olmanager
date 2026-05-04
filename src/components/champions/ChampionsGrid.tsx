import { useCallback, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Search, ArrowUpDown } from "lucide-react";
import type { ChampionData } from "../../store/types";

interface ChampionsGridProps {
  champions?: ChampionData[];
  onChampionClick: (championKey: string) => void;
}

type DraftRole = "TOP" | "JUNGLE" | "MID" | "ADC" | "SUPPORT";

const LOL_ROLE_ORDER: DraftRole[] = ["TOP", "JUNGLE", "MID", "ADC", "SUPPORT"];

const LOL_ROLE_ICON_URLS: Record<DraftRole, string> = {
  TOP: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-clash/global/default/assets/images/position-selector/positions/icon-position-top.png",
  JUNGLE: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-clash/global/default/assets/images/position-selector/positions/icon-position-jungle.png",
  MID: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-clash/global/default/assets/images/position-selector/positions/icon-position-middle.png",
  ADC: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-clash/global/default/assets/images/position-selector/positions/icon-position-bottom.png",
  SUPPORT: "https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-clash/global/default/assets/images/position-selector/positions/icon-position-utility.png",
};

function parseRoles(rolesJson: string): string[] {
  try {
    const parsed = JSON.parse(rolesJson);
    if (Array.isArray(parsed)) return parsed;
    return [];
  } catch {
    return [];
  }
}

function championTileUrl(championKey: string): string {
  return `https://ddragon.leagueoflegends.com/cdn/img/champion/tiles/${championKey}_0.jpg`;
}

export default function ChampionsGrid({ champions, onChampionClick }: ChampionsGridProps) {
  const { t } = useTranslation();
  const [search, setSearch] = useState("");
  const [roleFilter, setRoleFilter] = useState<"ALL" | DraftRole>("ALL");
  const [sortDir, setSortDir] = useState<"asc" | "desc">("asc");

  const toggleSort = () => setSortDir((prev) => (prev === "asc" ? "desc" : "asc"));

  const filtered = useMemo(() => {
    if (!champions) return [];
    const q = search.trim().toLowerCase();
    return champions
      .filter((c) => {
        if (q && !c.name.toLowerCase().includes(q) && !c.champion_key.toLowerCase().includes(q)) return false;
        if (roleFilter !== "ALL") {
          const roles = parseRoles(c.roles_json);
          if (!roles.some((r) => r.toUpperCase() === roleFilter || (r === "Bot" && roleFilter === "ADC"))) return false;
        }
        return true;
      })
      .sort((a, b) => sortDir === "asc" ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name));
  }, [champions, search, roleFilter, sortDir]);

  const handleClick = useCallback(
    (championKey: string) => onChampionClick(championKey),
    [onChampionClick],
  );

  if (!champions || champions.length === 0) return null;

  return (
    <div className="space-y-4">
      {/* Search + Filter bar */}
      <div className="flex flex-wrap gap-3 items-center">
        <div className="relative flex-1 min-w-[180px] max-w-xs">
          <Search className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder={t("champions.searchPlaceholder", "Buscar campeón...")}
            className="w-full pl-9 pr-3 py-2 rounded-lg bg-white dark:bg-navy-800 border border-gray-200 dark:border-navy-600 text-sm text-gray-800 dark:text-gray-200 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-primary-500/50"
          />
        </div>
        <div className="flex gap-1.5">
          <button
            onClick={() => setRoleFilter("ALL")}
            className={`px-3 py-1.5 rounded-lg text-xs font-heading font-bold uppercase tracking-wider transition-all ${roleFilter === "ALL" ? "bg-primary-500 text-white shadow-sm" : "bg-white dark:bg-navy-800 text-gray-500 dark:text-gray-400 border border-gray-200 dark:border-navy-600"}`}
          >
            <img src="/role-icons/allroles.png" alt="All" className="h-3.5 w-3.5" />
          </button>
          {LOL_ROLE_ORDER.map((role) => (
            <button
              key={role}
              onClick={() => setRoleFilter(roleFilter === role ? "ALL" : role)}
              className={`px-3 py-1.5 rounded-lg text-xs font-heading font-bold uppercase tracking-wider transition-all ${roleFilter === role ? "bg-primary-500 text-white shadow-sm" : "bg-white dark:bg-navy-800 text-gray-500 dark:text-gray-400 border border-gray-200 dark:border-navy-600"}`}
            >
              <img src={LOL_ROLE_ICON_URLS[role]} alt={role} className="h-3.5 w-3.5" />
            </button>
          ))}
        </div>
        <span className="text-xs text-gray-400 dark:text-gray-500 font-heading uppercase tracking-wider">
          {filtered.length} {t("champions.results", "resultado(s)")}
        </span>
      </div>

      {/* Table */}
      <div className="overflow-x-auto rounded-xl border border-gray-200 dark:border-navy-600">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="bg-gray-50 dark:bg-navy-800 border-b border-gray-200 dark:border-navy-600 text-xs">
              <th className="py-3 px-4 w-14" />
              <th
                className="py-3 px-4 font-heading font-bold uppercase tracking-wider text-gray-500 dark:text-gray-400 cursor-pointer select-none hover:text-gray-800 dark:hover:text-gray-300 transition-colors"
                onClick={toggleSort}
              >
                <span className="inline-flex items-center gap-1">
                  {t("champions.name", "Campeón")}
                  <ArrowUpDown className="w-3 h-3 opacity-40" />
                </span>
              </th>
              <th className="py-3 px-4 font-heading font-bold uppercase tracking-wider text-gray-500 dark:text-gray-400">
                {t("champions.roles", "Roles")}
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-100 dark:divide-navy-600">
            {filtered.map((champion) => {
              const roles = parseRoles(champion.roles_json);
              return (
                <tr
                  key={champion.id}
                  onClick={() => handleClick(champion.champion_key)}
                  className="hover:bg-gray-50 dark:hover:bg-navy-700/50 cursor-pointer transition-colors group"
                >
                  <td className="py-2.5 px-4">
                    <img
                      src={championTileUrl(champion.champion_key)}
                      alt={champion.name}
                      className="w-10 h-10 rounded-lg object-cover bg-navy-800"
                      loading="lazy"
                    />
                  </td>
                  <td className="py-2.5 px-4">
                    <span className="font-semibold text-sm text-gray-800 dark:text-gray-200 group-hover:text-primary-600 dark:group-hover:text-primary-400 transition-colors">
                      {champion.name}
                    </span>
                  </td>
                  <td className="py-2.5 px-4">
                    <div className="flex gap-1">
                      {roles.map((role) => {
                        const normalized = role === "Bot" ? "ADC" : role.toUpperCase() as DraftRole;
                        const iconUrl = LOL_ROLE_ICON_URLS[normalized];
                        if (!iconUrl) return null;
                        return (
                          <img
                            key={role}
                            src={iconUrl}
                            alt={role}
                            className="w-5 h-5 object-contain"
                            title={role}
                          />
                        );
                      })}
                    </div>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}
