// Music control for browser previews. `?music=1` plays a made-up song; without
// it Spotify counts as closed, so the player shows no music control.
import type { Api } from "../api";
import type { NowPlaying } from "../types";

export function createMusicMock(on: string | null): Pick<
  Api,
  "mediaNowPlaying" | "mediaPlayPause" | "mediaNext" | "mediaPrevious" | "mediaSetVolume"
> {
  let now: NowPlaying | null = on
    ? { title: "Steady Pulse", artist: "Lumen Ensemble", playing: true, volume: 0.6, canPrevious: true, canNext: true }
    : null;
  const update = (patch: Partial<NowPlaying>) => {
    if (now) now = { ...now, ...patch };
  };
  return {
    mediaNowPlaying: async () => (now ? { ...now } : null),
    mediaPlayPause: async () => update({ playing: !now?.playing }),
    mediaNext: async () => update({ title: "Second Wind", artist: "Lumen Ensemble" }),
    mediaPrevious: async () => update({ title: "Steady Pulse", artist: "Lumen Ensemble" }),
    mediaSetVolume: async (volume) => update({ volume }),
  };
}
