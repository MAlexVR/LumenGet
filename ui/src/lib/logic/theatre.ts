// Pure rules of the player's theatre mode, kept apart so they are testable.

/** The side menu steps aside only while a video plays in theatre mode. */
export function railCollapsed(routeName: string, theatre: boolean): boolean {
  return routeName === "watch" && theatre;
}

const DATE_PREFIX = /^\d{4}-\d{2}-\d{2}\s*[-–—]\s*/;

/** A Harbiz workout folder is named `<date> - <title>`; the header shows only the title. */
export function withoutDatePrefix(title: string): string {
  const rest = title.replace(DATE_PREFIX, "").trim();
  return rest || title;
}
