import { describe, expect, it } from "vitest";
import { videoTap } from "./touch";

describe("videoTap", () => {
  it("a touch on a video whose controls are hidden only reveals them", () => {
    expect(videoTap("touch", true)).toBe("reveal");
  });

  it("a touch while the controls are visible toggles playback", () => {
    expect(videoTap("touch", false)).toBe("toggle");
  });

  it("a mouse or pen click always toggles playback, hidden controls or not", () => {
    for (const pointer of ["mouse", "pen", ""]) {
      expect(videoTap(pointer, true)).toBe("toggle");
      expect(videoTap(pointer, false)).toBe("toggle");
    }
  });
});
