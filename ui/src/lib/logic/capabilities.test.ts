import { describe, expect, it } from "vitest";
import { capabilitiesFor } from "./capabilities";

describe("capabilitiesFor", () => {
  it("keeps every desktop feature on linux, windows and macos", () => {
    for (const os of ["linux", "windows", "macos"]) {
      expect(capabilitiesFor(os)).toEqual({
        android: false,
        changeLibraryFolder: true,
        openFolders: true,
        selfUpdate: true,
        downloaderUpdate: false,
        musicControl: true,
      });
    }
  });

  it("hides what Android cannot do", () => {
    expect(capabilitiesFor("android")).toEqual({
      android: true,
      changeLibraryFolder: false,
      openFolders: false,
      selfUpdate: false,
      downloaderUpdate: true,
      musicControl: false,
    });
  });

  it("treats an unknown system as desktop, so nothing disappears by accident", () => {
    expect(capabilitiesFor("").android).toBe(false);
    expect(capabilitiesFor("freebsd").selfUpdate).toBe(true);
  });
});
