// What the running system lets the app do, from the OS name the backend
// reports. Unknown systems count as desktop so no feature vanishes by mistake.

export interface Capabilities {
  android: boolean;
  /** Pick another library folder (Android keeps it in the app's own storage). */
  changeLibraryFolder: boolean;
  /** Show a library folder in the system file manager (Android has none to open). */
  openFolders: boolean;
  /** Download and install a new version from inside the app. */
  selfUpdate: boolean;
  /** Refresh the yt-dlp that ships inside the app (Android; desktop uses the system's). */
  downloaderUpdate: boolean;
  /** Control the Spotify player through MPRIS (Linux desktop only for now). */
  musicControl: boolean;
}

export function capabilitiesFor(os: string): Capabilities {
  const android = os === "android";
  return {
    android,
    changeLibraryFolder: !android,
    openFolders: !android,
    selfUpdate: !android,
    downloaderUpdate: android,
    musicControl: !android,
  };
}
