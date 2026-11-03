package com.malexvr.lumen

import android.os.Bundle
import android.util.Log
import androidx.activity.enableEdgeToEdge
import com.yausername.ffmpeg.FFmpeg
import com.yausername.youtubedl_android.YoutubeDL

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    prepareDownloader()
  }

  // Unpacks Python, yt-dlp and ffmpeg into the app's no_backup folder (the
  // Rust side runs them from there). The first start takes a few seconds; later
  // starts find them already in place.
  private fun prepareDownloader() {
    val context = applicationContext
    Thread {
      try {
        YoutubeDL.getInstance().init(context)
        FFmpeg.getInstance().init(context)
      } catch (e: Exception) {
        Log.e("LumenGet", "cannot prepare the downloader", e)
      }
    }.start()
  }
}
