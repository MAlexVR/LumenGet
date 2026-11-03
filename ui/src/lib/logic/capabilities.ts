// What the running system lets the app do, from the OS name the backend
// reports. Unknown systems count as desktop so no feature vanishes by mistake.

export interface Capabilities {
  android: boolean;
  /** Pick another library folder (Android keeps it in the app's own storage). */
  changeLibraryFolder: boolean;
  /** Download and install a new version from inside the app. */
  selfUpdate: boolean;
  /** Control the Spotify player through MPRIS (Linux desktop only for now). */
  musicControl: boolean;
}

export function capabilitiesFor(os: string): Capabilities {
  const android = os === "android";
  return {
    android,
    changeLibraryFolder: !android,
    selfUpdate: !android,
    musicControl: !android,
  };
}
