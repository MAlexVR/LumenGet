//! Keeping the Android build's bundled yt-dlp current. Sites change often and
//! an old yt-dlp stops resolving videos, so the Settings screen can fetch the
//! latest release. The Kotlin side (`DownloaderPlugin.kt`) does the work with
//! the `youtubedl-android` library; desktop installs use their own yt-dlp.

use serde::{Deserialize, Serialize};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{AppHandle, Runtime};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DownloaderCode {
    /// This system has no bundled downloader to update.
    Unsupported,
    /// The update could not be fetched or installed.
    Failed,
}

/// Serialized to the UI as `{ "code": "...", "message": "..." }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, thiserror::Error)]
#[error("{message}")]
pub struct DownloaderError {
    pub code: DownloaderCode,
    pub message: String,
}

impl DownloaderError {
    #[cfg_attr(target_os = "android", allow(dead_code))]
    fn unsupported() -> Self {
        Self {
            code: DownloaderCode::Unsupported,
            message: "the downloader is updated with the system's own yt-dlp".to_string(),
        }
    }

    #[cfg_attr(not(target_os = "android"), allow(dead_code))]
    fn failed(message: impl Into<String>) -> Self {
        Self {
            code: DownloaderCode::Failed,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloaderInfo {
    /// Installed yt-dlp release, e.g. `2026.10.01`; absent while it unpacks.
    pub version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateStatus {
    Updated,
    Current,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateResult {
    pub status: UpdateStatus,
    pub version: Option<String>,
}

#[cfg(target_os = "android")]
struct DownloaderHandle<R: Runtime>(tauri::plugin::PluginHandle<R>);

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    #[allow(unused_mut)]
    let mut builder = Builder::new("lumen-downloader");
    #[cfg(target_os = "android")]
    {
        use tauri::Manager;
        builder = builder.setup(|app, api| {
            let handle = api.register_android_plugin("com.malexvr.lumen", "DownloaderPlugin")?;
            app.manage(DownloaderHandle(handle));
            Ok(())
        });
    }
    builder.build()
}

#[cfg(not(target_os = "android"))]
pub async fn version<R: Runtime>(_app: AppHandle<R>) -> Result<DownloaderInfo, DownloaderError> {
    Err(DownloaderError::unsupported())
}

#[cfg(not(target_os = "android"))]
pub async fn update<R: Runtime>(_app: AppHandle<R>) -> Result<UpdateResult, DownloaderError> {
    Err(DownloaderError::unsupported())
}

#[cfg(target_os = "android")]
async fn call<R: Runtime, T: serde::de::DeserializeOwned + Send + 'static>(
    app: AppHandle<R>,
    command: &'static str,
) -> Result<T, DownloaderError> {
    use tauri::Manager;
    // The call waits for the Kotlin side, which may download for a while.
    tauri::async_runtime::spawn_blocking(move || {
        app.state::<DownloaderHandle<R>>()
            .0
            .run_mobile_plugin::<T>(command, ())
            .map_err(|e| DownloaderError::failed(e.to_string()))
    })
    .await
    .map_err(|e| DownloaderError::failed(e.to_string()))?
}

#[cfg(target_os = "android")]
pub async fn version<R: Runtime>(app: AppHandle<R>) -> Result<DownloaderInfo, DownloaderError> {
    call(app, "version").await
}

#[cfg(target_os = "android")]
pub async fn update<R: Runtime>(app: AppHandle<R>) -> Result<UpdateResult, DownloaderError> {
    call(app, "update").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn the_update_answer_of_the_android_side_is_understood() {
        let updated: UpdateResult =
            serde_json::from_value(json!({"status": "updated", "version": "2026.10.01"})).unwrap();
        assert_eq!(
            updated,
            UpdateResult {
                status: UpdateStatus::Updated,
                version: Some("2026.10.01".to_string())
            }
        );
        let current: UpdateResult = serde_json::from_value(json!({"status": "current"})).unwrap();
        assert_eq!(current.status, UpdateStatus::Current);
        assert_eq!(current.version, None);
    }

    #[test]
    fn the_ui_gets_camel_case_json_and_a_code_on_errors() {
        assert_eq!(
            serde_json::to_value(DownloaderInfo {
                version: Some("1".to_string())
            })
            .unwrap(),
            json!({"version": "1"})
        );
        assert_eq!(
            serde_json::to_value(DownloaderError::unsupported()).unwrap()["code"],
            "unsupported"
        );
        assert_eq!(
            serde_json::to_value(DownloaderError::failed("offline")).unwrap(),
            json!({"code": "failed", "message": "offline"})
        );
    }
}
