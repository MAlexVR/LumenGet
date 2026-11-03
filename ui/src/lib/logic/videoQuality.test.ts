import { describe, expect, it } from "vitest";
import { DEFAULT_VIDEO_QUALITY, VIDEO_QUALITIES, loadVideoQuality, saveVideoQuality } from "./videoQuality";

const store = (initial: Record<string, string> = {}) => {
  const data = { ...initial };
  return {
    data,
    getItem: (k: string) => data[k] ?? null,
    setItem: (k: string, v: string) => {
      data[k] = v;
    },
  };
};

describe("video quality", () => {
  it("offers the best available and the usual heights, 720p by default", () => {
    expect(VIDEO_QUALITIES).toEqual(["best", "1080", "720", "480", "360"]);
    expect(DEFAULT_VIDEO_QUALITY).toBe("720");
  });

  it("remembers the choice", () => {
    const s = store();
    saveVideoQuality(s, "480");
    expect(loadVideoQuality(s)).toBe("480");
  });

  it("falls back to the default for an unknown or missing value or no storage", () => {
    expect(loadVideoQuality(store({ "lumen.videoQuality": "999" }))).toBe("720");
    expect(loadVideoQuality(store())).toBe("720");
    expect(loadVideoQuality(null)).toBe("720");
  });

  it("does not throw when storage is blocked", () => {
    const blocked = {
      getItem: () => {
        throw new Error("blocked");
      },
      setItem: () => {
        throw new Error("blocked");
      },
    };
    expect(loadVideoQuality(blocked)).toBe("720");
    expect(() => saveVideoQuality(blocked, "360")).not.toThrow();
  });
});
