// Size of the text of a lesson's content (description, notes, reading), chosen
// by the reader and remembered. Browser storage is only a convenience: any
// failure falls back to the default.

/** Pixel sizes the reader can step through. */
export const TEXT_SIZES = [13, 15, 17, 19, 22] as const;
export const DEFAULT_TEXT_SIZE = 15;
const KEY = "lumen.textSize";

function nearestIndex(px: number): number {
  let best = 0;
  TEXT_SIZES.forEach((size, i) => {
    // `<=` so a tie goes to the larger size.
    if (Math.abs(size - px) <= Math.abs(TEXT_SIZES[best] - px)) best = i;
  });
  return best;
}

export function stepTextSize(current: number, direction: 1 | -1): number {
  const next = Math.max(0, Math.min(TEXT_SIZES.length - 1, nearestIndex(current) + direction));
  return TEXT_SIZES[next];
}

export function loadTextSize(storage: Pick<Storage, "getItem"> | null): number {
  try {
    const stored = Number(storage?.getItem(KEY));
    return (TEXT_SIZES as readonly number[]).includes(stored) ? stored : DEFAULT_TEXT_SIZE;
  } catch {
    return DEFAULT_TEXT_SIZE;
  }
}

export function saveTextSize(storage: Pick<Storage, "setItem"> | null, px: number): void {
  try {
    storage?.setItem(KEY, String(px));
  } catch {
    /* storage blocked: the size lasts until the app closes */
  }
}
