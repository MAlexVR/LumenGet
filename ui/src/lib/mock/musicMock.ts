// Music control for browser previews. `?music=1` plays a made-up song; without
// it Spotify counts as closed, so the player shows no music control.
import type { Api } from "../api";
import type { NowPlaying } from "../types";

// A cover drawn in the preview itself: nothing is fetched.
const COVER = `data:image/svg+xml,${encodeURIComponent(
  '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><defs><linearGradient id="g" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#2b3a55"/><stop offset="1" stop-color="#c9a24a"/></linearGradient></defs><rect width="64" height="64" fill="url(#g)"/><circle cx="32" cy="32" r="12" fill="none" stroke="#fff" stroke-opacity=".7" stroke-width="3"/></svg>',
)}`;

export function createMusicMock(on: string | null): Pick<
  Api,
  "mediaNowPlaying" | "mediaPlayPause" | "mediaNext" | "mediaPrevious" | "mediaSetVolume" | "mediaSetShuffle" | "mediaSetRepeat"
> {
  let now: NowPlaying | null = on
    ? {
        title: "Steady Pulse",
        artist: "Lumen Ensemble",
        artUrl: COVER,
        playing: true,
        positionSecs: 37,
        lengthSecs: 263,
        shuffle: false,
        repeat: "none",
        volume: 0.6,
        canPrevious: true,
        canNext: true,
      }
    : null;
  const update = (patch: Partial<NowPlaying>) => {
    if (now) now = { ...now, ...patch };
  };
  return {
    mediaNowPlaying: async () => (now ? { ...now } : null),
    mediaPlayPause: async () => update({ playing: !now?.playing }),
    mediaNext: async () => update({ title: "Second Wind", positionSecs: 0 }),
    mediaPrevious: async () => update({ title: "Steady Pulse", positionSecs: 0 }),
    mediaSetVolume: async (volume) => update({ volume }),
    mediaSetShuffle: async (shuffle) => update({ shuffle }),
    mediaSetRepeat: async (repeat) => update({ repeat }),
  };
}
