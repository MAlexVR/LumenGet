import { describe, expect, it } from "vitest";
import { DEFAULT_TEXT_SIZE, TEXT_SIZES, loadTextSize, saveTextSize, stepTextSize } from "./textSize";

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

describe("stepTextSize", () => {
  it("moves one step up or down the list", () => {
    expect(stepTextSize(15, 1)).toBe(17);
    expect(stepTextSize(15, -1)).toBe(13);
  });

  it("stops at the smallest and the largest size", () => {
    expect(stepTextSize(TEXT_SIZES[0], -1)).toBe(TEXT_SIZES[0]);
    expect(stepTextSize(TEXT_SIZES[TEXT_SIZES.length - 1], 1)).toBe(TEXT_SIZES[TEXT_SIZES.length - 1]);
  });

  it("snaps a value that is not in the list to the nearest step first", () => {
    expect(stepTextSize(16, 1)).toBe(19);
    expect(stepTextSize(99, -1)).toBe(19);
  });
});

describe("loadTextSize and saveTextSize", () => {
  it("falls back to the default when nothing usable is stored", () => {
    expect(loadTextSize(store())).toBe(DEFAULT_TEXT_SIZE);
    expect(loadTextSize(store({ "lumen.textSize": "huge" }))).toBe(DEFAULT_TEXT_SIZE);
    expect(loadTextSize(store({ "lumen.textSize": "400" }))).toBe(DEFAULT_TEXT_SIZE);
    expect(loadTextSize(null)).toBe(DEFAULT_TEXT_SIZE);
  });

  it("restores a saved size from the list", () => {
    const s = store();
    saveTextSize(s, 19);
    expect(loadTextSize(s)).toBe(19);
  });

  it("never throws when the storage is blocked", () => {
    const blocked = {
      getItem: () => {
        throw new Error("blocked");
      },
      setItem: () => {
        throw new Error("blocked");
      },
    };
    expect(loadTextSize(blocked)).toBe(DEFAULT_TEXT_SIZE);
    expect(() => saveTextSize(blocked, 17)).not.toThrow();
  });
});
