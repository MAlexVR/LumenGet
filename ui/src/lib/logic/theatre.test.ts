import { describe, expect, it } from "vitest";
import { railCollapsed, withoutDatePrefix } from "./theatre";

describe("railCollapsed", () => {
  it("collapses the menu only while watching in theatre mode", () => {
    expect(railCollapsed("watch", true)).toBe(true);
  });

  it("keeps the menu in every other case", () => {
    expect(railCollapsed("watch", false)).toBe(false);
    expect(railCollapsed("library", true)).toBe(false);
    expect(railCollapsed("home", false)).toBe(false);
  });
});

describe("withoutDatePrefix", () => {
  it("drops the date a Harbiz workout folder starts with", () => {
    expect(withoutDatePrefix("2026-10-31 - Transferencia Flex a patadas")).toBe("Transferencia Flex a patadas");
  });

  it("accepts the other dash styles", () => {
    expect(withoutDatePrefix("2026-10-31 – Core")).toBe("Core");
    expect(withoutDatePrefix("2026-10-31 — Core")).toBe("Core");
  });

  it("leaves any other title untouched", () => {
    expect(withoutDatePrefix("Introducción")).toBe("Introducción");
    expect(withoutDatePrefix("1. Intro - parte 2")).toBe("1. Intro - parte 2");
  });

  it("keeps a title that is only a date", () => {
    expect(withoutDatePrefix("2026-10-31")).toBe("2026-10-31");
  });
});
