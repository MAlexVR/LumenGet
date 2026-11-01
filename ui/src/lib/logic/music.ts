// Pure rules of the player's music control. The control only shows and
// drives what the owner's Spotify plays; none of this touches the workout.

import type { NowPlaying, RepeatMode } from "../types";

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

const REPEAT_CYCLE: readonly RepeatMode[] = ["none", "playlist", "track"];

/** The repeat button walks off, the whole list, the song, off. */
export function nextRepeat(mode: RepeatMode): RepeatMode {
  return REPEAT_CYCLE[(REPEAT_CYCLE.indexOf(mode) + 1) % REPEAT_CYCLE.length];
}

/**
 * Where the song is shown to be: the last position the player gave plus the
 * time since, while it plays, so the bar moves between two polls.
 */
export function displayedPosition(
  position: number | null,
  length: number | null,
  playing: boolean,
  elapsedSecs: number,
): number {
  const now = (position ?? 0) + (playing && position !== null ? Math.max(0, elapsedSecs) : 0);
  return length === null ? now : Math.min(now, length);
}

/** How far through the song it is, 0 to 1. */
export function progressFraction(position: number, length: number | null): number {
  if (length === null || length <= 0) return 0;
  return Math.min(1, Math.max(0, position / length));
}

/** The volume the speaker button sets: silence, or what it was before the silence. */
export function muteTarget(volume: number | null, before: number): number {
  if (volume !== null && volume > 0) return 0;
  return before > 0 ? before : 0.5;
}
