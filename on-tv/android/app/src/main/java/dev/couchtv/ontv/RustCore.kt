package dev.couchtv.ontv

import android.util.Log

/**
 * Optional native core for the Rust experiment.
 *
 * The ordinary Couch TV build has no .so and keeps running unchanged. The
 * experimental APK includes native libraries for arm64-v8a and armeabi-v7a.
 * On load/link failure, retain the original Kotlin behavior.
 */
object RustCore {
    private const val TAG = "couchtv.rust"
    private var usable: Boolean = runCatching {
        System.loadLibrary("couchtv_rust")
        Log.i(TAG, "Rust native core loaded")
        true
    }.getOrElse {
        Log.i(TAG, "Rust native core not present; using Kotlin fallback")
        false
    }

    val isLoaded: Boolean get() = usable

    private external fun nativeMime(path: String): String?

    fun mime(path: String): String {
        if (usable) {
            try {
                nativeMime(path)?.let { return it }
            } catch (e: LinkageError) {
                usable = false
                Log.w(TAG, "Native MIME unavailable; switching to Kotlin", e)
            }
        }
        // Original Server.kt mapping. Do not change until native parity tests pass.
        return when (path.substringAfterLast('.').lowercase()) {
            "html" -> "text/html; charset=utf-8"
            "js", "mjs" -> "application/javascript"
            "css" -> "text/css"
            "json" -> "application/json"
            "webmanifest" -> "application/manifest+json"
            "png" -> "image/png"
            "jpg", "jpeg" -> "image/jpeg"
            "webp" -> "image/webp"
            "svg" -> "image/svg+xml"
            "woff2" -> "font/woff2"
            "ico" -> "image/x-icon"
            "txt" -> "text/plain"
            else -> "application/octet-stream"
        }
    }
}
