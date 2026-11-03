import { describe, expect, it } from "vitest";
import {
  groupByModule,
  lessonStates,
  selectedOrders,
  selectionAfterToggle,
  selectionAfterToggleModule,
  type PickableLesson,
} from "./lessonPicker";

const lesson = (order: number, module: string, title = `L${order}`): PickableLesson => ({ order, module, title });
const lessons = [lesson(0, "Intro"), lesson(1, "Intro"), lesson(2, "Core"), lesson(3, "Core"), lesson(4, "")];

describe("groupByModule", () => {
  it("keeps the course order and starts a group when the module changes", () => {
    const groups = groupByModule(lessons);
    expect(groups.map((g) => [g.module, g.lessons.map((l) => l.order)])).toEqual([
      ["Intro", [0, 1]],
      ["Core", [2, 3]],
      ["", [4]],
    ]);
  });

  it("does not merge two separate modules that share a name", () => {
    const groups = groupByModule([lesson(0, "A"), lesson(1, "B"), lesson(2, "A")]);
    expect(groups.map((g) => g.lessons.length)).toEqual([1, 1, 1]);
  });

  it("is empty for no lessons", () => {
    expect(groupByModule([])).toEqual([]);
  });
});

describe("selection", () => {
  it("toggles one lesson on and off without touching the original set", () => {
    const empty = new Set<number>();
    const one = selectionAfterToggle(empty, 3);
    expect([...one]).toEqual([3]);
    expect(empty.size).toBe(0);
    expect(selectionAfterToggle(one, 3).size).toBe(0);
  });

  it("selects a whole module, and clears it when it is already all selected", () => {
    const [intro] = groupByModule(lessons);
    const all = selectionAfterToggleModule(new Set([2]), intro);
    expect([...all].sort()).toEqual([0, 1, 2]);
    const cleared = selectionAfterToggleModule(all, intro);
    expect([...cleared]).toEqual([2]);
  });

  it("returns the chosen positions in course order", () => {
    expect(selectedOrders(new Set([4, 0, 2]))).toEqual([0, 2, 4]);
  });
});

describe("lessonStates", () => {
  it("marks the lessons before the running one as done, the running one as current and the rest as pending", () => {
    const states = lessonStates([5, 1, 3], 1, true);
    expect(states.get(1)).toBe("done");
    expect(states.get(3)).toBe("current");
    expect(states.get(5)).toBe("pending");
  });

  it("has no current lesson once nothing is running", () => {
    const states = lessonStates([1, 3], 2, false);
    expect([...states.values()]).toEqual(["done", "done"]);
    expect([...lessonStates([1, 3], 0, false).values()]).toEqual(["pending", "pending"]);
  });
});
