import { describe, expect, it } from "vitest";
import { ownsKeys, percentToVolume, trackLabel, volumeToPercent } from "./music";
import type { NowPlaying } from "../types";

const song = (over: Partial<NowPlaying> = {}): NowPlaying => ({
  title: "La Larga Marcha",
  artist: "Warcry",
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
