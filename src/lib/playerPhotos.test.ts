import { describe, expect, it } from "vitest";

import { resolvePlayerPhoto } from "./playerPhotos";

describe("resolvePlayerPhoto", () => {
  it("prefers imported match-name photos over generated player-id guesses", () => {
    expect(resolvePlayerPhoto("player-randomid", "Selenex", null)).toBe(
      "/player-photos/3d26f35c.webp",
    );
  });

  it("keeps explicit profile image urls as the highest priority", () => {
    expect(resolvePlayerPhoto("player-randomid", "Selenex", "/custom/photo.webp")).toBe(
      "/custom/photo.webp",
    );
  });
});
