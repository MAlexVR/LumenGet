import { describe, expect, it } from "vitest";
import { countStatuses, filterAndOrder, type CourseFilter } from "./courseFilter";
import type { LocalStatus } from "./localStatus";

const course = (key: string, status: LocalStatus) => ({ key, status });
const items = [
  course("a", "complete"),
  course("b", "none"),
  course("c", "partial"),
  course("d", "complete"),
  course("e", "partial"),
  course("f", "none"),
];
const keys = (list: { key: string }[]) => list.map((i) => i.key);

describe("countStatuses", () => {
  it("counts every state and the total", () => {
    expect(countStatuses(items.map((i) => i.status))).toEqual({ all: 6, incomplete: 2, none: 2, complete: 2 });
  });

  it("is all zeros for no courses", () => {
    expect(countStatuses([])).toEqual({ all: 0, incomplete: 0, none: 0, complete: 0 });
  });
});

describe("filterAndOrder", () => {
  it("lists the incomplete courses first, then the ones not downloaded, then the complete ones", () => {
    expect(keys(filterAndOrder(items, "all"))).toEqual(["c", "e", "b", "f", "a", "d"]);
  });

  it("keeps only the courses of one state, in their original order", () => {
    const only = (f: CourseFilter) => keys(filterAndOrder(items, f));
    expect(only("incomplete")).toEqual(["c", "e"]);
    expect(only("none")).toEqual(["b", "f"]);
    expect(only("complete")).toEqual(["a", "d"]);
  });

  it("does not change the list it receives", () => {
    const copy = [...items];
    filterAndOrder(items, "all");
    expect(items).toEqual(copy);
  });
});
