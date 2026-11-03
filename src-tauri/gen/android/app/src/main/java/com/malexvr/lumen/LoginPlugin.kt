package com.malexvr.lumen

import android.app.Activity
import android.content.Intent
import android.webkit.CookieManager
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class UrlArgs {
  lateinit var url: String
}

// Drives LoginActivity from Rust: open it, ask whether it is still open, read
// the cookies of a URL (HttpOnly ones included) and close it wiping them.
@TauriPlugin
class LoginPlugin(private val activity: Activity) : Plugin(activity) {
  @Command
  fun open(invoke: Invoke) {
    try {
      val url = invoke.parseArgs(UrlArgs::class.java).url
      activity.startActivity(Intent(activity, LoginActivity::class.java).putExtra(LoginActivity.URL, url))
      invoke.resolve()
    } catch (e: Exception) {
      invoke.reject(e.message)
    }
  }

  @Command
  fun isOpen(invoke: Invoke) {
    invoke.resolve(JSObject().put("open", LoginActivity.current != null))
  }

  @Command
  fun cookies(invoke: Invoke) {
    try {
      val url = invoke.parseArgs(UrlArgs::class.java).url
      activity.runOnUiThread {
        // getCookie answers null while the jar is empty.
        val header = CookieManager.getInstance().getCookie(url) ?: ""
        invoke.resolve(JSObject().put("header", header))
      }
    } catch (e: Exception) {
      invoke.reject(e.message)
    }
  }

  @Command
  fun finish(invoke: Invoke) {
    activity.runOnUiThread {
      LoginActivity.current?.finish()
      val manager = CookieManager.getInstance()
      manager.removeAllCookies(null)
      manager.flush()
      invoke.resolve()
    }
  }
}
