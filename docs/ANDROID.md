# LumenGet on Android

Tablet first (landscape and portrait), 64-bit ARM, Android 7.0 (API 24) or newer. The Android build is distributed under the GPL-3.0, see [`ANDROID_LICENSE.md`](ANDROID_LICENSE.md). Windows and Linux are unaffected.

## How it differs from the desktop app

- **Library:** lives in the app's own storage (`files/Lumen`). There is no folder picker, and the file manager buttons are hidden.
- **Downloads:** yt-dlp runs through a Python and an ffmpeg built for Android (the `youtubedl-android` libraries). `MainActivity` unpacks them on the first start, and the Rust side launches them (`crates/lumen-download/src/android.rs`). Settings → Downloader updates yt-dlp from its latest release.
- **Attachments:** opened with the system's apps through a file provider (`FilesPlugin.kt`).
- **Not available:** self-update of the app (install a new APK instead), Spotify music control, and, for now, nothing else is known to be missing.

## Build

Needs the Android SDK (platform 34+), NDK `27.0.12077973`, JDK 17+ and the Rust target:

```bash
rustup target add aarch64-linux-android
export ANDROID_HOME=~/Android/Sdk NDK_HOME=$ANDROID_HOME/ndk/27.0.12077973
pnpm tauri android build --debug --apk --target aarch64   # signed with the debug key
pnpm tauri android build --apk --target aarch64           # release: needs the signing key below
```

The APK lands in `src-tauri/gen/android/app/build/outputs/apk/universal/{debug,release}/`. If Gradle fails right after a network error, run `./gradlew --refresh-dependencies` inside `src-tauri/gen/android` once.

## Signing the release APK

Updates only install over the same signing key, so keep one key for the life of the app and back it up.

```bash
keytool -genkeypair -keystore lumenget-release.jks -storetype PKCS12 -alias lumenget \
  -keyalg RSA -keysize 4096 -validity 10000
```

Locally, put `src-tauri/gen/android/keystore.properties` next to the Gradle project (it is git-ignored):

```properties
storeFile=/absolute/path/lumenget-release.jks
storePassword=...
keyAlias=lumenget
keyPassword=...
```

In GitHub (Settings → Secrets and variables → Actions) the release workflow reads `ANDROID_KEYSTORE_BASE64` (`base64 -w0 lumenget-release.jks`), `ANDROID_KEYSTORE_PASSWORD`, `ANDROID_KEY_ALIAS` and `ANDROID_KEY_PASSWORD`. Without them the Android job fails by itself and the Windows and Linux installers still publish.

## Install and debug

```bash
adb install -r LumenGet_<version>_android_arm64.apk
adb logcat -s LumenGet RustStdoutStderr
```

Or copy the APK to the device and open it (allow installing from that source).
