package com.malexvr.lumen

import android.app.Activity
import android.os.Bundle
import android.util.Log
import android.view.View
import android.view.ViewGroup
import android.webkit.ConsoleMessage
import android.webkit.CookieManager
import android.webkit.WebChromeClient
import android.webkit.WebResourceError
import android.webkit.WebResourceRequest
import android.webkit.WebView
import android.webkit.WebViewClient
import android.widget.Button
import android.widget.LinearLayout
import android.widget.ProgressBar
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat

// The sign-in screen of a course platform. Android has a single Tauri window, so
// the login runs here and the Rust side polls the cookies through LoginPlugin
// until the credential shows up (or the user closes this screen).
class LoginActivity : Activity() {
  companion object {
    const val URL = "url"
    private const val TAG = "LumenGet"

    @Volatile
    var current: LoginActivity? = null

    // True from the request to open the screen until it is gone: the screen takes
    // a moment to be created, and Rust asks right away whether it is still open.
    @Volatile
    var active = false
  }

  private var web: WebView? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    val url = intent.getStringExtra(URL)
    if (url == null) {
      active = false
      finish()
      return
    }
    val webView = WebView(this)
    web = webView
    webView.settings.javaScriptEnabled = true
    webView.settings.domStorageEnabled = true
    CookieManager.getInstance().setAcceptThirdPartyCookies(webView, true)
    webView.webViewClient = object : WebViewClient() {
      override fun onPageFinished(view: WebView, url: String) {
        Log.d(TAG, "login page loaded: ${url.substringBefore('?')}")
      }

      override fun onReceivedError(view: WebView, request: WebResourceRequest, error: WebResourceError) {
        if (request.isForMainFrame) Log.e(TAG, "login page error ${error.errorCode}: ${error.description}")
      }
    }
    val progress = ProgressBar(this, null, android.R.attr.progressBarStyleHorizontal).apply {
      max = 100
      visibility = View.GONE
    }
    webView.webChromeClient = object : WebChromeClient() {
      override fun onProgressChanged(view: WebView, newProgress: Int) {
        progress.progress = newProgress
        progress.visibility = if (newProgress in 1..99) View.VISIBLE else View.GONE
      }

      override fun onConsoleMessage(message: ConsoleMessage): Boolean {
        if (BuildConfig.DEBUG) {
          Log.d(TAG, "console: ${message.message()} (${message.sourceId().substringBefore('?')}:${message.lineNumber()})")
        }
        return true
      }
    }

    val close = Button(this).apply {
      text = getString(R.string.login_close)
      setOnClickListener { finish() }
    }
    // A slow network can leave the page blank: let the user try again.
    val reload = Button(this).apply {
      text = getString(R.string.login_reload)
      setOnClickListener { webView.reload() }
    }
    val bar = LinearLayout(this).apply {
      orientation = LinearLayout.HORIZONTAL
      addView(close, LinearLayout.LayoutParams(ViewGroup.LayoutParams.WRAP_CONTENT, ViewGroup.LayoutParams.WRAP_CONTENT))
      addView(reload, LinearLayout.LayoutParams(ViewGroup.LayoutParams.WRAP_CONTENT, ViewGroup.LayoutParams.WRAP_CONTENT))
    }
    val root = LinearLayout(this).apply {
      orientation = LinearLayout.VERTICAL
      addView(bar, LinearLayout.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.WRAP_CONTENT))
      addView(progress, LinearLayout.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.WRAP_CONTENT))
      addView(webView, LinearLayout.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, 0, 1f))
    }
    // Targeting a recent SDK draws edge to edge: keep clear of the system bars.
    ViewCompat.setOnApplyWindowInsetsListener(root) { view, insets ->
      val bars = insets.getInsets(WindowInsetsCompat.Type.systemBars() or WindowInsetsCompat.Type.displayCutout())
      view.setPadding(bars.left, bars.top, bars.right, bars.bottom)
      insets
    }
    setContentView(root)
    current = this
    webView.loadUrl(url)
  }

  @Deprecated("Deprecated in Java")
  override fun onBackPressed() {
    val webView = web
    if (webView != null && webView.canGoBack()) webView.goBack() else super.onBackPressed()
  }

  override fun onDestroy() {
    if (current === this) {
      current = null
      active = false
    }
    web?.destroy()
    web = null
    super.onDestroy()
  }
}
