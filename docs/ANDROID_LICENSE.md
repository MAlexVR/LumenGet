# Android build: licensing

The Windows and Linux builds of LumenGet are MIT, as described in [`LICENSE`](../LICENSE). They are not affected by anything below.

The **Android build is distributed under the GNU General Public License v3.0** (GPL-3.0), because it bundles GPL-3.0 components that the Android app needs to run yt-dlp:

| Component | Used for | Source |
| --- | --- | --- |
| `youtubedl-android` library 0.18.1 | unpacks and runs Python and yt-dlp | https://github.com/junkfood02/youtubedl-android |
| `youtubedl-android` ffmpeg 0.18.1 | merges and converts downloaded video | https://github.com/junkfood02/youtubedl-android |
| Python, QuickJS (inside the library) | run yt-dlp | linked from the project above |
| yt-dlp | resolves and downloads video | https://github.com/yt-dlp/yt-dlp (Unlicense) |

The source of LumenGet is this repository (https://github.com/MAlexVR/LumenGet); the source of the bundled components is at the links above, at the stated versions. Anyone receiving the Android build may use, study, change and redistribute it under the GPL-3.0 terms.

This note is not legal advice.
