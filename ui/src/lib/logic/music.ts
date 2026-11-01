// Pure rules of the player's music control. The control only shows and
// drives what the owner's Spotify plays; none of this touches the workout.

import type { NowPlaying } from "../types";

/** How often the control asks Spotify what it is playing. */
export const POLL_MS = 2000;

/** Marks the control so the player's key map leaves its keys alone. */
const MUSIC_ATTR = "data-music";
export const musicAttr = MUSIC_ATTR;

const PLAYER_NAME = "Spotify";

export function trackLabel(now: NowPlaying): { title: string; artist: string } {
  return { title: now.title.trim() || PLAYER_NAME, artist: now.artist.trim() };
}

/** The slider works in whole percentages. */
export function volumeToPercent(volume: number | null): number {
  return volume === null ? 0 : Math.round(volume * 100);
}

/** What the player is asked for: 0 to 1, in hundredths, never out of range. */
export function percentToVolume(percent: number): number {
  if (!Number.isFinite(percent)) return 0;
  return Math.min(100, Math.max(0, Math.round(percent))) / 100;
}

/** True for keys typed inside the control: Space or an arrow there must not pause or seek the video. */
export function ownsKeys(target: { closest(selector: string): unknown } | null): boolean {
  return Boolean(target?.closest(`[${MUSIC_ATTR}]`));
}
