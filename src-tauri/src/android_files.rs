//! Opening a library file with another app. Desktop uses the opener plugin;
//! on Android that plugin only understands URLs, so a small plugin of ours
//! (`FilesPlugin.kt`) hands the file to the system through a file provider.

use std::path::Path;

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{AppHandle, Runtime};

#[cfg(target_os = "android")]
struct FilesHandle<R: Runtime>(tauri::plugin::PluginHandle<R>);

#[cfg(target_os = "android")]
#[derive(serde::Serialize)]
struct OpenFile<'a> {
    path: &'a str,
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    #[allow(unused_mut)]
    let mut builder = Builder::new("lumen-files");
    #[cfg(target_os = "android")]
    {
        use tauri::Manager;
        builder = builder.setup(|app, api| {
            let handle = api.register_android_plugin("com.malexvr.lumen", "FilesPlugin")?;
            app.manage(FilesHandle(handle));
            Ok(())
        });
    }
    builder.build()
}

#[cfg(not(target_os = "android"))]
pub fn open_file<R: Runtime>(app: &AppHandle<R>, path: &Path) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_path(path.to_string_lossy(), None::<&str>)
        .map_err(|e| e.to_string())
}

#[cfg(target_os = "android")]
pub fn open_file<R: Runtime>(app: &AppHandle<R>, path: &Path) -> Result<(), String> {
    use tauri::Manager;
    let path = path.to_string_lossy();
    app.state::<FilesHandle<R>>()
        .0
        .run_mobile_plugin::<()>("openFile", OpenFile { path: &path })
        .map_err(|e| e.to_string())
}
