import { describe, expect, it } from "vitest";
import { displayedPosition, muteTarget, nextRepeat, ownsKeys, percentToVolume, progressFraction, trackLabel, volumeToPercent } from "./music";
import type { NowPlaying } from "../types";

const song = (over: Partial<NowPlaying> = {}): NowPlaying => ({
  title: "La Larga Marcha",
  artist: "Warcry",
  artUrl: "https://i.scdn.co/image/abc",
  positionSecs: 37,
  lengthSecs: 263,
  shuffle: false,
  repeat: "none",
  playing: true,
  volume: 0.5,
  canPrevious: true,
  canNext: true,
  ...over,
});

describe("trackLabel", () => {
  it("joins the title and the artist", () => {
    expect(trackLabel(song())).toEqual({ title: "La Larga Marcha", artist: "Warcry" });
  });

  it("falls back to the player's name when it reports no title", () => {
    expect(trackLabel(song({ title: "  ", artist: "" }))).toEqual({ title: "Spotify", artist: "" });
  });

  it("keeps the artist out when there is none", () => {
    expect(trackLabel(song({ artist: "" })).artist).toBe("");
  });
});

describe("volume", () => {
  it("shows the volume as a whole percentage", () => {
    expect(volumeToPercent(0.6286869)).toBe(63);
    expect(volumeToPercent(0)).toBe(0);
    expect(volumeToPercent(1)).toBe(100);
  });

  it("shows no volume when the player does not expose one", () => {
    expect(volumeToPercent(null)).toBe(0);
  });

  it("sends the slider position as 0 to 1, in hundredths", () => {
    expect(percentToVolume(63)).toBe(0.63);
    expect(percentToVolume(0)).toBe(0);
    expect(percentToVolume(100)).toBe(1);
  });

  it("never sends a volume outside 0 to 1", () => {
    expect(percentToVolume(-5)).toBe(0);
    expect(percentToVolume(250)).toBe(1);
    expect(percentToVolume(Number.NaN)).toBe(0);
  });
});

describe("ownsKeys", () => {
  const inside = { closest: (sel: string) => (sel === "[data-music]" ? {} : null) };
  const outside = { closest: () => null };

  it("claims every key typed inside the music control", () => {
    expect(ownsKeys(inside)).toBe(true);
  });

  it("leaves the keys of the rest of the player alone", () => {
    expect(ownsKeys(outside)).toBe(false);
    expect(ownsKeys(null)).toBe(false);
  });
});

describe("nextRepeat", () => {
  it("cycles off, the whole list, the song, off", () => {
    expect(nextRepeat("none")).toBe("playlist");
    expect(nextRepeat("playlist")).toBe("track");
    expect(nextRepeat("track")).toBe("none");
  });
});

describe("displayedPosition", () => {
  it("moves on with the clock while the song plays", () => {
    expect(displayedPosition(37, 263, true, 1.5)).toBe(38.5);
  });

  it("stays put while it is paused", () => {
    expect(displayedPosition(37, 263, false, 9)).toBe(37);
  });

  it("never passes the end of the song", () => {
    expect(displayedPosition(262, 263, true, 30)).toBe(263);
  });

  it("goes on counting when the player gave no length", () => {
    expect(displayedPosition(10, null, true, 2)).toBe(12);
  });

  it("shows zero when the player gave no position", () => {
    expect(displayedPosition(null, 263, true, 2)).toBe(0);
  });
});

describe("progressFraction", () => {
  it("is how far through the song it is, from 0 to 1", () => {
    expect(progressFraction(131.5, 263)).toBe(0.5);
    expect(progressFraction(0, 263)).toBe(0);
  });

  it("is empty when the length is unknown or zero", () => {
    expect(progressFraction(10, null)).toBe(0);
    expect(progressFraction(10, 0)).toBe(0);
  });

  it("never leaves the bar", () => {
    expect(progressFraction(300, 263)).toBe(1);
    expect(progressFraction(-4, 263)).toBe(0);
  });
});

describe("muteTarget", () => {
  it("mutes a playing volume", () => {
    expect(muteTarget(0.6, 0.6)).toBe(0);
  });

  it("brings back the volume from before the mute", () => {
    expect(muteTarget(0, 0.4)).toBe(0.4);
  });

  it("falls back to half when there is nothing to bring back", () => {
    expect(muteTarget(0, 0)).toBe(0.5);
    expect(muteTarget(null, 0)).toBe(0.5);
  });
});
