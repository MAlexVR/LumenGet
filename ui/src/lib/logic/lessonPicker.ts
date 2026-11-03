// Choosing lessons of a course before downloading them: grouping by module,
// the selection and what each chosen lesson is doing while the download runs.

/** A lesson as the backend lists it: `order` is its position in the course. */
export interface PickableLesson {
  order: number;
  module: string;
  title: string;
}

export interface ModuleGroup {
  module: string;
  lessons: PickableLesson[];
}

/** Consecutive lessons of the same module, in course order. */
export function groupByModule(lessons: readonly PickableLesson[]): ModuleGroup[] {
  const groups: ModuleGroup[] = [];
  for (const lesson of lessons) {
    const last = groups[groups.length - 1];
    if (last && last.module === lesson.module) last.lessons.push(lesson);
    else groups.push({ module: lesson.module, lessons: [lesson] });
  }
  return groups;
}

export function selectionAfterToggle(selection: ReadonlySet<number>, order: number): Set<number> {
  const next = new Set(selection);
  if (!next.delete(order)) next.add(order);
  return next;
}

/** Selects every lesson of the module, or clears them when all were chosen. */
export function selectionAfterToggleModule(selection: ReadonlySet<number>, group: ModuleGroup): Set<number> {
  const next = new Set(selection);
  const allChosen = group.lessons.every((l) => next.has(l.order));
  for (const lesson of group.lessons) {
    if (allChosen) next.delete(lesson.order);
    else next.add(lesson.order);
  }
  return next;
}

/** The chosen positions in course order, as the start command takes them. */
export function selectedOrders(selection: ReadonlySet<number>): number[] {
  return [...selection].sort((a, b) => a - b);
}

export type LessonState = "done" | "current" | "pending";

/**
 * What each chosen lesson is doing. Lessons download one after another in
 * course order, so with `completed` already finished the next one is the
 * current lesson while the download runs.
 */
export function lessonStates(chosen: readonly number[], completed: number, running: boolean): Map<number, LessonState> {
  const states = new Map<number, LessonState>();
  [...chosen]
    .sort((a, b) => a - b)
    .forEach((order, index) => {
      states.set(order, index < completed ? "done" : index === completed && running ? "current" : "pending");
    });
  return states;
}
