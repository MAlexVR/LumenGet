// Maximum video height of course downloads, chosen by the user and remembered.
// Browser storage is only a convenience: any failure falls back to 720p, the
// height the backend starts with.

export const VIDEO_QUALITIES = ["best", "1080", "720", "480", "360"] as const;
export type VideoQuality = (typeof VIDEO_QUALITIES)[number];
export const DEFAULT_VIDEO_QUALITY: VideoQuality = "720";
const KEY = "lumen.videoQuality";

/** Height cap sent to the backend; `null` means the best available. */
export function maxHeight(quality: VideoQuality): number | null {
  return quality === "best" ? null : Number(quality);
}

export function loadVideoQuality(storage: Pick<Storage, "getItem"> | null): VideoQuality {
  try {
    const stored = storage?.getItem(KEY);
    return (VIDEO_QUALITIES as readonly string[]).includes(stored ?? "") ? (stored as VideoQuality) : DEFAULT_VIDEO_QUALITY;
  } catch {
    return DEFAULT_VIDEO_QUALITY;
  }
}

export function saveVideoQuality(storage: Pick<Storage, "setItem"> | null, quality: VideoQuality): void {
  try {
    storage?.setItem(KEY, quality);
  } catch {
    /* storage blocked: the choice lasts until the app closes */
  }
}
