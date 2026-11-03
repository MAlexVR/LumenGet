package com.malexvr.lumen

import android.app.Activity
import android.content.ActivityNotFoundException
import android.content.Intent
import android.webkit.MimeTypeMap
import androidx.core.content.FileProvider
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import java.io.File

@InvokeArg
class OpenFileArgs {
  lateinit var path: String
}

// Opens a library file with whichever app can show it. The Rust side has
// already confined `path` to the library, which the file provider shares.
@TauriPlugin
class FilesPlugin(private val activity: Activity) : Plugin(activity) {
  @Command
  fun openFile(invoke: Invoke) {
    try {
      val file = File(invoke.parseArgs(OpenFileArgs::class.java).path)
      val uri = FileProvider.getUriForFile(activity, "${activity.packageName}.fileprovider", file)
      val mime = MimeTypeMap.getSingleton().getMimeTypeFromExtension(file.extension.lowercase()) ?: "*/*"
      val intent = Intent(Intent.ACTION_VIEW)
        .setDataAndType(uri, mime)
        .addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
      activity.startActivity(intent)
      invoke.resolve()
    } catch (e: ActivityNotFoundException) {
      invoke.reject("no app can open this file")
    } catch (e: Exception) {
      invoke.reject(e.message)
    }
  }
}
