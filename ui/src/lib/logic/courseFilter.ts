// Which courses to show in a platform's list: the incomplete ones come first so
// they can be completed, and each state can be listed on its own.
import type { LocalStatus } from "./localStatus";

export type CourseFilter = "all" | "incomplete" | "none" | "complete";

export interface StatusCounts {
  all: number;
  incomplete: number;
  none: number;
  complete: number;
}

/** A course that has some of its content downloaded but not all of it. */
const FILTER_STATUS: Record<Exclude<CourseFilter, "all">, LocalStatus> = {
  incomplete: "partial",
  none: "none",
  complete: "complete",
};

const ORDER: Record<LocalStatus, number> = { partial: 0, none: 1, complete: 2 };

export function countStatuses(statuses: readonly LocalStatus[]): StatusCounts {
  const counts: StatusCounts = { all: statuses.length, incomplete: 0, none: 0, complete: 0 };
  for (const status of statuses) {
    if (status === "partial") counts.incomplete += 1;
    else counts[status] += 1;
  }
  return counts;
}

export function filterAndOrder<T extends { status: LocalStatus }>(items: readonly T[], filter: CourseFilter): T[] {
  if (filter !== "all") return items.filter((item) => item.status === FILTER_STATUS[filter]);
  // Array.prototype.sort is stable: courses keep their order within a state.
  return [...items].sort((a, b) => ORDER[a.status] - ORDER[b.status]);
}
