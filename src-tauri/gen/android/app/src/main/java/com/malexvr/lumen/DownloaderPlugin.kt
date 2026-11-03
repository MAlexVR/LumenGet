package com.malexvr.lumen

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import com.yausername.youtubedl_android.YoutubeDL

// Reports and refreshes the yt-dlp that ships with the app. The work runs off
// the calling thread: an update downloads a release from GitHub.
@TauriPlugin
class DownloaderPlugin(private val activity: Activity) : Plugin(activity) {
  private val context get() = activity.applicationContext

  @Command
  fun version(invoke: Invoke) {
    Thread {
      try {
        invoke.resolve(JSObject().put("version", YoutubeDL.getInstance().version(context)))
      } catch (e: Exception) {
        invoke.reject(e.message)
      }
    }.start()
  }

  @Command
  fun update(invoke: Invoke) {
    Thread {
      try {
        val status = YoutubeDL.getInstance().updateYoutubeDL(context, YoutubeDL.UpdateChannel.STABLE)
        val updated = status == YoutubeDL.UpdateStatus.DONE
        invoke.resolve(
          JSObject()
            .put("status", if (updated) "updated" else "current")
            .put("version", YoutubeDL.getInstance().version(context)),
        )
      } catch (e: Exception) {
        invoke.reject(e.message ?: "the update failed")
      }
    }.start()
  }
}
