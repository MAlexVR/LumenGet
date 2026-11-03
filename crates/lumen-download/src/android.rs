//! yt-dlp and ffmpeg on Android. The desktop binaries cannot run there, so the
//! app bundles Python, QuickJS and ffmpeg built for Android (the
//! `youtubedl-android` libraries); the Kotlin side unpacks them on first start
//! and this module only knows where they end up and how to launch them. Pure
//! path logic, so it is tested on the host.

use std::path::{Path, PathBuf};

use crate::ytdlp::YtDlp;

/// Directory of the app's native libraries (`.../lib/arm64`), found from the
/// memory map of the running process: it is where our own library is loaded
/// from, and the only place Android lets an app execute files from.
pub fn native_lib_dir(proc_maps: &str, own_lib: &str) -> Option<PathBuf> {
    proc_maps
        .lines()
        .filter_map(|line| line.split_whitespace().last())
        .map(Path::new)
        .find(|path| path.file_name().is_some_and(|name| name == own_lib))
        .and_then(Path::parent)
        .map(Path::to_path_buf)
}

/// Where the bundled tools live for one installation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndroidRuntime {
    native_lib_dir: PathBuf,
    /// Unpacked by the Kotlin side under the app's `no_backup` folder.
    package_root: PathBuf,
    cache_dir: PathBuf,
}

impl AndroidRuntime {
    /// `app_dir` is the app's data directory (`/data/user/0/<id>`); the
    /// unpacked tools sit in its `no_backup/youtubedl-android` folder.
    pub fn new(native_lib_dir: PathBuf, app_dir: &Path, cache_dir: PathBuf) -> Self {
        Self {
            native_lib_dir,
            package_root: app_dir.join("no_backup").join("youtubedl-android"),
            cache_dir,
        }
    }

    /// The ffmpeg executable shipped in the native libraries.
    pub fn ffmpeg(&self) -> PathBuf {
        self.native_lib_dir.join("libffmpeg.so")
    }

    fn python_home(&self) -> PathBuf {
        self.package_root
            .join("packages")
            .join("python")
            .join("usr")
    }

    fn ffmpeg_libs(&self) -> PathBuf {
        self.package_root
            .join("packages")
            .join("ffmpeg")
            .join("usr")
            .join("lib")
    }

    /// yt-dlp run by the bundled Python, with the environment it expects.
    pub fn ytdlp(&self) -> YtDlp {
        let python_home = self.python_home();
        let library_path = format!(
            "{}:{}",
            self.ffmpeg_libs().display(),
            python_home.join("lib").display()
        );
        let prefix_args = vec![
            self.package_root
                .join("yt-dlp")
                .join("yt-dlp")
                .to_string_lossy()
                .into_owned(),
            "--ffmpeg-location".to_string(),
            self.ffmpeg().to_string_lossy().into_owned(),
            "--js-runtimes".to_string(),
            format!(
                "quickjs:{}",
                self.native_lib_dir.join("libqjs.so").display()
            ),
        ];
        let envs = vec![
            ("PYTHONHOME".to_string(), python_home.display().to_string()),
            ("LD_LIBRARY_PATH".to_string(), library_path),
            (
                "SSL_CERT_FILE".to_string(),
                python_home
                    .join("etc")
                    .join("tls")
                    .join("cert.pem")
                    .display()
                    .to_string(),
            ),
            ("TMPDIR".to_string(), self.cache_dir.display().to_string()),
            ("HOME".to_string(), self.cache_dir.display().to_string()),
        ];
        YtDlp::with_launcher(self.native_lib_dir.join("libpython.so"), prefix_args, envs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAPS: &str = "\
7a00000000-7a00001000 r--p 00000000 fd:01 1 /system/lib64/libc.so
7b00000000-7b00200000 r-xp 00000000 fd:01 2 /data/app/~~x/com.malexvr.lumen-y/lib/arm64/liblumen.so
7b00200000-7b00300000 r--p 00200000 fd:01 2 /data/app/~~x/com.malexvr.lumen-y/lib/arm64/liblumen.so
";

    fn runtime() -> AndroidRuntime {
        AndroidRuntime::new(
            PathBuf::from("/data/app/pkg/lib/arm64"),
            Path::new("/data/user/0/com.malexvr.lumen"),
            PathBuf::from("/data/user/0/com.malexvr.lumen/cache"),
        )
    }

    #[test]
    fn native_lib_dir_is_the_folder_our_library_is_loaded_from() {
        assert_eq!(
            native_lib_dir(MAPS, "liblumen.so"),
            Some(PathBuf::from("/data/app/~~x/com.malexvr.lumen-y/lib/arm64"))
        );
    }

    #[test]
    fn native_lib_dir_is_none_when_the_library_is_not_mapped() {
        assert_eq!(native_lib_dir(MAPS, "libother.so"), None);
        assert_eq!(native_lib_dir("", "liblumen.so"), None);
    }

    #[test]
    fn ffmpeg_is_the_native_library_executable() {
        assert_eq!(
            runtime().ffmpeg(),
            PathBuf::from("/data/app/pkg/lib/arm64/libffmpeg.so")
        );
    }

    #[test]
    fn ytdlp_runs_the_unpacked_script_with_the_bundled_python() {
        let ytdlp = runtime().ytdlp();
        assert_eq!(
            ytdlp.binary(),
            Path::new("/data/app/pkg/lib/arm64/libpython.so")
        );
        let root = "/data/user/0/com.malexvr.lumen/no_backup/youtubedl-android";
        assert_eq!(
            ytdlp.launch_args(vec!["-J".into()]),
            vec![
                format!("{root}/yt-dlp/yt-dlp"),
                "--ffmpeg-location".into(),
                "/data/app/pkg/lib/arm64/libffmpeg.so".into(),
                "--js-runtimes".into(),
                "quickjs:/data/app/pkg/lib/arm64/libqjs.so".into(),
                "-J".into(),
            ]
        );
    }

    #[test]
    fn ytdlp_environment_points_python_at_its_unpacked_files() {
        let envs = runtime().ytdlp().envs().to_vec();
        let root = "/data/user/0/com.malexvr.lumen/no_backup/youtubedl-android/packages";
        let get = |key: &str| envs.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        assert_eq!(get("PYTHONHOME"), Some(format!("{root}/python/usr")));
        assert_eq!(
            get("LD_LIBRARY_PATH"),
            Some(format!("{root}/ffmpeg/usr/lib:{root}/python/usr/lib"))
        );
        assert_eq!(
            get("SSL_CERT_FILE"),
            Some(format!("{root}/python/usr/etc/tls/cert.pem"))
        );
        assert_eq!(
            get("TMPDIR"),
            Some("/data/user/0/com.malexvr.lumen/cache".into())
        );
    }

    #[test]
    fn desktop_ytdlp_adds_nothing_to_its_arguments() {
        let plain = YtDlp::new("yt-dlp");
        assert_eq!(plain.launch_args(vec!["-J".into()]), vec!["-J".to_string()]);
        assert!(plain.envs().is_empty());
    }
}
