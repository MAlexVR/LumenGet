//! The sign-in window of a platform. Desktop opens a second Tauri window and
//! reads its cookie jar. Android has a single window, and the cookie call of
//! its webview crashes the app when the jar is empty, so there the sign-in runs
//! in its own screen (`LoginActivity.kt`) and the cookies come from Android's
//! `CookieManager` (`LoginPlugin.kt`).

/// `name=value` pairs of a `Cookie` header (what `CookieManager` returns).
/// Values may hold `=`; entries without a name are dropped.
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
pub fn parse_cookie_header(header: &str) -> Vec<(String, String)> {
    header
        .split(';')
        .filter_map(|entry| entry.trim().split_once('='))
        .filter(|(name, _)| !name.trim().is_empty())
        .map(|(name, value)| (name.trim().to_string(), value.trim().to_string()))
        .collect()
}

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{AppHandle, Runtime};
use url::Url;

#[cfg(not(target_os = "android"))]
mod window {
    use super::*;
    use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

    pub fn open<R: Runtime>(
        app: &AppHandle<R>,
        label: &str,
        title: &str,
        start: &Url,
    ) -> Result<(), String> {
        if let Some(existing) = app.get_webview_window(label) {
            let _ = existing.set_focus();
            return Ok(());
        }
        WebviewWindowBuilder::new(app, label, WebviewUrl::External(start.clone()))
            .title(title)
            .inner_size(520.0, 760.0)
            .build()
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    pub fn is_open<R: Runtime>(app: &AppHandle<R>, label: &str) -> bool {
        app.get_webview_window(label).is_some()
    }

    pub async fn cookies<R: Runtime>(
        app: &AppHandle<R>,
        label: &str,
        jar_url: &Url,
    ) -> Result<Vec<(String, String)>, String> {
        let Some(window) = app.get_webview_window(label) else {
            return Ok(Vec::new());
        };
        let url = jar_url.clone();
        // Reading cookies can deadlock on Windows outside a separate thread.
        let jar = tauri::async_runtime::spawn_blocking(move || window.cookies_for_url(url))
            .await
            .map_err(|e| e.to_string())?;
        Ok(jar
            .map(|cookies| {
                cookies
                    .iter()
                    .map(|c| (c.name().to_string(), c.value().to_string()))
                    .collect()
            })
            .unwrap_or_default())
    }

    /// Wipes the window's own browsing data and closes it.
    pub fn finish<R: Runtime>(app: &AppHandle<R>, label: &str) {
        if let Some(window) = app.get_webview_window(label) {
            let _ = window.clear_all_browsing_data();
            let _ = window.close();
        }
    }
}

#[cfg(target_os = "android")]
mod window {
    use super::*;
    use serde::{Deserialize, Serialize};
    use tauri::plugin::PluginHandle;
    use tauri::Manager;

    pub struct LoginHandle<R: Runtime>(pub PluginHandle<R>);

    #[derive(Serialize)]
    struct UrlArgs<'a> {
        url: &'a str,
    }

    #[derive(Deserialize)]
    struct OpenState {
        open: bool,
    }

    #[derive(Deserialize)]
    struct CookieJar {
        header: String,
    }

    pub fn open<R: Runtime>(
        app: &AppHandle<R>,
        _label: &str,
        _title: &str,
        start: &Url,
    ) -> Result<(), String> {
        app.state::<LoginHandle<R>>()
            .0
            .run_mobile_plugin::<()>(
                "open",
                UrlArgs {
                    url: start.as_str(),
                },
            )
            .map_err(|e| e.to_string())
    }

    pub fn is_open<R: Runtime>(app: &AppHandle<R>, _label: &str) -> bool {
        app.state::<LoginHandle<R>>()
            .0
            .run_mobile_plugin::<OpenState>("isOpen", ())
            .map(|state| state.open)
            .unwrap_or_else(|e| {
                eprintln!("login screen state unavailable: {e}");
                false
            })
    }

    pub async fn cookies<R: Runtime>(
        app: &AppHandle<R>,
        _label: &str,
        jar_url: &Url,
    ) -> Result<Vec<(String, String)>, String> {
        let app = app.clone();
        let url = jar_url.to_string();
        tauri::async_runtime::spawn_blocking(move || {
            app.state::<LoginHandle<R>>()
                .0
                .run_mobile_plugin::<CookieJar>("cookies", UrlArgs { url: &url })
                .map(|jar| super::parse_cookie_header(&jar.header))
                .map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| e.to_string())?
    }

    /// Closes the sign-in screen and wipes its cookies.
    pub fn finish<R: Runtime>(app: &AppHandle<R>, _label: &str) {
        let _ = app
            .state::<LoginHandle<R>>()
            .0
            .run_mobile_plugin::<()>("finish", ());
    }
}

pub use window::{cookies, finish, is_open, open};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    #[allow(unused_mut)]
    let mut builder = Builder::new("lumen-login");
    #[cfg(target_os = "android")]
    {
        use tauri::Manager;
        builder = builder.setup(|app, api| {
            let handle = api.register_android_plugin("com.malexvr.lumen", "LoginPlugin")?;
            app.manage(window::LoginHandle(handle));
            Ok(())
        });
    }
    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_cookie_header_becomes_name_value_pairs() {
        assert_eq!(
            parse_cookie_header("a=1; b=2"),
            vec![
                ("a".to_string(), "1".to_string()),
                ("b".to_string(), "2".to_string())
            ]
        );
    }

    #[test]
    fn values_keep_their_equals_signs_and_spaces_around_pairs_are_trimmed() {
        assert_eq!(
            parse_cookie_header("  token=abc==  ;auth = x "),
            vec![
                ("token".to_string(), "abc==".to_string()),
                ("auth".to_string(), "x".to_string())
            ]
        );
    }

    #[test]
    fn an_empty_jar_and_nameless_entries_give_nothing() {
        assert!(parse_cookie_header("").is_empty());
        assert!(parse_cookie_header(" ; ;").is_empty());
        assert!(parse_cookie_header("justtext").is_empty());
        assert!(parse_cookie_header("=value").is_empty());
    }
}
