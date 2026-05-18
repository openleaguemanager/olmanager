import { describe, it, expect } from "vitest";
import {
  resolveChampionTile,
  resolveChampionSplash,
  ddragonTileUrl,
  ddragonSplashUrl,
} from "./championImages";

describe("resolveChampionTile", () => {
  it("returns /champion-tiles/{key}.webp for a simple key", () => {
    expect(resolveChampionTile("Aatrox")).toBe("/champion-tiles/Aatrox.webp");
  });

  it("normalizes the key before building the path", () => {
    expect(resolveChampionTile("FiddleSticks")).toBe("/champion-tiles/Fiddlesticks.webp");
    expect(resolveChampionTile("Wukong")).toBe("/champion-tiles/MonkeyKing.webp");
  });

  it("returns null for null input", () => {
    expect(resolveChampionTile(null)).toBeNull();
  });

  it("returns null for undefined input", () => {
    expect(resolveChampionTile(undefined)).toBeNull();
  });

  it("returns null for empty string", () => {
    expect(resolveChampionTile("")).toBeNull();
  });
});

describe("resolveChampionSplash", () => {
  it("returns /champion-splash/{key}.webp for a simple key", () => {
    expect(resolveChampionSplash("Ahri")).toBe("/champion-splash/Ahri.webp");
  });

  it("normalizes the key before building the path", () => {
    expect(resolveChampionSplash("FiddleSticks")).toBe("/champion-splash/Fiddlesticks.webp");
  });

  it("returns null for null input", () => {
    expect(resolveChampionSplash(null)).toBeNull();
  });

  it("returns null for undefined input", () => {
    expect(resolveChampionSplash(undefined)).toBeNull();
  });

  it("returns null for empty string", () => {
    expect(resolveChampionSplash("")).toBeNull();
  });
});

describe("ddragonTileUrl", () => {
  it("returns DDragon tile URL for a champion key", () => {
    expect(ddragonTileUrl("Aatrox")).toBe(
      "https://ddragon.leagueoflegends.com/cdn/img/champion/tiles/Aatrox_0.jpg",
    );
  });

  it("returns DDragon tile URL using the normalized key", () => {
    expect(ddragonTileUrl("FiddleSticks")).toBe(
      "https://ddragon.leagueoflegends.com/cdn/img/champion/tiles/Fiddlesticks_0.jpg",
    );
  });

  it("returns null for null input", () => {
    expect(ddragonTileUrl(null)).toBeNull();
  });

  it("returns null for undefined input", () => {
    expect(ddragonTileUrl(undefined)).toBeNull();
  });

  it("returns null for empty string", () => {
    expect(ddragonTileUrl("")).toBeNull();
  });
});

describe("ddragonSplashUrl", () => {
  it("returns DDragon splash URL for a champion key", () => {
    expect(ddragonSplashUrl("Ahri")).toBe(
      "https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Ahri_0.jpg",
    );
  });

  it("returns DDragon splash URL using the normalized key", () => {
    expect(ddragonSplashUrl("Wukong")).toBe(
      "https://ddragon.leagueoflegends.com/cdn/img/champion/splash/MonkeyKing_0.jpg",
    );
  });

  it("returns null for null input", () => {
    expect(ddragonSplashUrl(null)).toBeNull();
  });

  it("returns null for undefined input", () => {
    expect(ddragonSplashUrl(undefined)).toBeNull();
  });

  it("returns null for empty string", () => {
    expect(ddragonSplashUrl("")).toBeNull();
  });
});
