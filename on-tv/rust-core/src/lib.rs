//! Couch TV OnTV native-core proof of integration.
//!
//! Keep Android UI, ADB and token handling in the existing app for this stage.
//! A tiny, stateless function is the safest way to prove JNI, ABI packaging and
//! parity before migrating the stateful HTTP server.

use jni::objects::{JObject, JString};
use jni::sys::jstring;
use jni::JNIEnv;

/// Intentionally mirrors Server.kt's original MIME table.
/// Avoid allocations: JNI is already the most expensive part of this operation.
fn mime_for_path(path: &str) -> &'static str {
    let ext = path.rsplit('.').next().unwrap_or("");
    if ext.eq_ignore_ascii_case("html") {
        "text/html; charset=utf-8"
    } else if ext.eq_ignore_ascii_case("js") || ext.eq_ignore_ascii_case("mjs") {
        "application/javascript"
    } else if ext.eq_ignore_ascii_case("css") {
        "text/css"
    } else if ext.eq_ignore_ascii_case("json") {
        "application/json"
    } else if ext.eq_ignore_ascii_case("webmanifest") {
        "application/manifest+json"
    } else if ext.eq_ignore_ascii_case("png") {
        "image/png"
    } else if ext.eq_ignore_ascii_case("jpg") || ext.eq_ignore_ascii_case("jpeg") {
        "image/jpeg"
    } else if ext.eq_ignore_ascii_case("webp") {
        "image/webp"
    } else if ext.eq_ignore_ascii_case("svg") {
        "image/svg+xml"
    } else if ext.eq_ignore_ascii_case("woff2") {
        "font/woff2"
    } else if ext.eq_ignore_ascii_case("ico") {
        "image/x-icon"
    } else if ext.eq_ignore_ascii_case("txt") {
        "text/plain"
    } else {
        "application/octet-stream"
    }
}

/// Signature for the instance external function in dev.couchtv.ontv.RustCore.
/// No panics or exceptions are allowed across the JNI boundary.
#[no_mangle]
pub extern "system" fn Java_dev_couchtv_ontv_RustCore_nativeMime(
    mut env: JNIEnv<'_>,
    _this: JObject<'_>,
    path: JString<'_>,
) -> jstring {
    let path = match env.get_string(&path) {
        Ok(value) => value,
        Err(_) => return std::ptr::null_mut(),
    };
    let path = match path.to_str() {
        Ok(value) => value,
        Err(_) => return std::ptr::null_mut(),
    };
    match env.new_string(mime_for_path(path)) {
        Ok(value) => value.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[cfg(test)]
mod tests {
    use super::mime_for_path;

    #[test]
    fn matches_the_original_kotlin_mime_table() {
        let examples = [
            ("index.html", "text/html; charset=utf-8"),
            ("bundle.JS", "application/javascript"),
            ("script.mjs", "application/javascript"),
            ("app.css", "text/css"),
            ("data.json", "application/json"),
            ("manifest.webmanifest", "application/manifest+json"),
            ("photo.png", "image/png"),
            ("photo.jpg", "image/jpeg"),
            ("photo.jpeg", "image/jpeg"),
            ("photo.webp", "image/webp"),
            ("icon.svg", "image/svg+xml"),
            ("font.woff2", "font/woff2"),
            ("favicon.ico", "image/x-icon"),
            ("robots.txt", "text/plain"),
            ("custom.ext", "application/octet-stream"),
            ("no-extension", "application/octet-stream"),
        ];
        for (path, expected) in examples {
            assert_eq!(mime_for_path(path), expected, "{path}");
        }
    }
}
