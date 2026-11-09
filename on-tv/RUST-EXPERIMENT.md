# Couch TV OnTV — Rust experiment (isolated)

This branch is **not** a full Rust rewrite and does **not** claim lower RAM yet.
Its first milestone proves Android/JNI/native ABI packaging while preserving the
existing app's UI, ADB, pairing, requests and settings.

## What changed

- A small Rust `cdylib` at `on-tv/rust-core` with tests for the original static-file MIME table.
- An optional Kotlin/JNI bridge: if `libcouchtv_rust.so` is absent or fails to link,
  the exact original Kotlin MIME table remains the fallback.
- Only `Server.mime()` calls this bridge; the NanoHTTPD server and WebView remain.
- A dedicated GitHub Actions workflow for `rust-experimental` builds ARM64 + ARMv7
  libraries, runs Rust tests and packages an experimental APK.
- The **experimental** APK uses `dev.couchtv.ontv.rustexp`, label
  **Couch TV Rust Test**, and default HTTP port **5401**, to coexist with the
  normal OnTV package `dev.couchtv.ontv` on port 5400.
- A normal Gradle build, without `-PrustExperiment=true` and without .so files,
  retains its original application ID, label, port and Kotlin fallback.

## Build artifact

Open **Actions → OnTV Rust experimental APK → latest successful run** in this fork
and download `couch-tv-ontv-rust-experimental`. Unzip the artifact and install
`app-release.apk` on the TV.

The workflow is triggered by pushes to `rust-experimental` (if GitHub Actions
is enabled for this fork). Its APK is not published as a GitHub Release.

## Basic device verification (Android TV 11)

Do not switch the experimental app into the default HOME role yet. If you have
both installed, avoid running them at the same time during measurements.

```sh
adb install -r app-release.apk
adb shell am force-stop dev.couchtv.ontv
adb shell monkey -p dev.couchtv.ontv.rustexp 1
adb logcat -d -s couchtv.rust:I | tail -30
adb shell dumpsys meminfo dev.couchtv.ontv.rustexp
```

A log entry `Rust native core loaded` proves the native bridge was loaded;
it does *not* prove lower RAM or faster rendering. Open the TV remote from
`http://<tv-ip>:5401` after the experimental app is running.

For comparison, force-stop one app before launching the other and repeat the
**same** UI/phone remote workload and idle interval. Save Total PSS, Native Heap,
Dalvik Heap, CPU and cold-start time. Repeat several times to distinguish signal
from background variability. Use the original `dev.couchtv.ontv` as baseline.

## Important limitations

- Rust only handles MIME mapping in the initial milestone: negligible expected
  RAM savings, and JNI call overhead may outweigh the pure-function benefit.
- No TV hardware test or verified APK build is claimed until GitHub Actions is
  green **and** the APK is tested on the target stick.
- Migrating the HTTP/WebSocket server or the WebView UI is a separate step,
  gated on parity tests and real device memory measurements.
- Neither launchers nor setup apps are disabled by this experiment.
- Rust native libraries are staged into `jniLibs` by the workflow; do not
  commit these generated binaries to the repository.
