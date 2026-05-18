import { normalizeChampionKey } from "./championIds";

const DDragonCDN = "https://ddragon.leagueoflegends.com";
const TILE_PATH = "/cdn/img/champion/tiles";
const SPLASH_PATH = "/cdn/img/champion/splash";

/**
 * Build a tile URL for the given champion key.
 * Returns a local webp path if key is valid, null otherwise.
 *
 * The key is normalized via normalizeChampionKey before constructing the path,
 * so non-standard forms like "FiddleSticks" → "Fiddlesticks" are handled.
 *
 * @param key — Champion key string, null, or undefined
 * @returns "/champion-tiles/{normalizedKey}.webp" or null
 */
export function resolveChampionTile(
  key: string | null | undefined,
): string | null {
  if (!key) return null;
  const normalized = normalizeChampionKey(key);
  if (!normalized) return null;
  return `/champion-tiles/${normalized}.webp`;
}

/**
 * Build a splash URL for the given champion key.
 * Returns a local webp path if key is valid, null otherwise.
 *
 * @param key — Champion key string, null, or undefined
 * @returns "/champion-splash/{normalizedKey}.webp" or null
 */
export function resolveChampionSplash(
  key: string | null | undefined,
): string | null {
  if (!key) return null;
  const normalized = normalizeChampionKey(key);
  if (!normalized) return null;
  return `/champion-splash/${normalized}.webp`;
}

/**
 * Build a DDragon CDN tile URL for the given champion key.
 * Useful as an onError fallback for <img> elements.
 *
 * @param key — Champion key string, null, or undefined
 * @returns DDragon tile URL or null
 */
export function ddragonTileUrl(
  key: string | null | undefined,
): string | null {
  if (!key) return null;
  const normalized = normalizeChampionKey(key);
  if (!normalized) return null;
  return `${DDragonCDN}${TILE_PATH}/${normalized}_0.jpg`;
}

/**
 * Build a DDragon CDN splash URL for the given champion key.
 * Useful as an onError fallback for <img> elements.
 *
 * @param key — Champion key string, null, or undefined
 * @returns DDragon splash URL or null
 */
export function ddragonSplashUrl(
  key: string | null | undefined,
): string | null {
  if (!key) return null;
  const normalized = normalizeChampionKey(key);
  if (!normalized) return null;
  return `${DDragonCDN}${SPLASH_PATH}/${normalized}_0.jpg`;
}
