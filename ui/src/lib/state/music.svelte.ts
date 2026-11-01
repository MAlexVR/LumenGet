// What Spotify plays, shared by the player's music control. It polls only
// while the control is on screen and never touches the workout state.

import { POLL_MS } from "../logic/music";
import { app } from "./app.svelte";
import type { NowPlaying } from "../types";

export const music = $state<{ now: NowPlaying | null; readAt: number }>({ now: null, readAt: 0 });

async function refresh(): Promise<void> {
  try {
    music.now = (await app.api?.mediaNowPlaying()) ?? null;
    music.readAt = performance.now();
  } catch {
    // An unreachable bus is the same as no Spotify: the control hides.
    music.now = null;
  }
}

/** Starts watching Spotify; returns the function that stops it. */
export function watchMusic(): () => void {
  void refresh();
  const timer = setInterval(() => {
    if (!document.hidden) void refresh();
  }, POLL_MS);
  return () => {
    clearInterval(timer);
    music.now = null;
  };
}

/** Runs a command, then reads what changed so the control never shows stale state. */
export async function command(run: () => Promise<void>): Promise<void> {
  try {
    await run();
  } catch {
    /* the refresh below shows whether Spotify is still there */
  }
  await refresh();
}

export function setVolumeLocally(volume: number): void {
  if (music.now) music.now = { ...music.now, volume };
}

/** Shows a jump at once, before the next reading of Spotify confirms it. */
export function setPositionLocally(seconds: number): void {
  if (music.now) {
    music.now = { ...music.now, positionSecs: seconds };
    music.readAt = performance.now();
  }
}
