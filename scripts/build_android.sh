#!/usr/bin/env bash
set -euo pipefail

MODE="${1:-release}"
if [[ "$MODE" != "release" && "$MODE" != "debug" ]]; then
  echo "Usage: $0 [release|debug]"
  exit 1
fi

# Android build script for this Bevy project.
# Detected local defaults on this machine:
# - Android SDK: /Users/sky/Library/Android/sdk
# - JDK: 17
# - NDKs: 28.2.13676358, 29.0.13113456

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

SDK_DEFAULT="$HOME/Library/Android/sdk"
export ANDROID_HOME="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-$SDK_DEFAULT}}"
unset ANDROID_SDK_ROOT

if [[ ! -d "$ANDROID_HOME" ]]; then
  echo "ERROR: Android SDK not found: $ANDROID_HOME"
  echo "Please install Android SDK (platform-tools, build-tools, platforms) first."
  exit 1
fi

if [[ -z "${ANDROID_NDK_ROOT:-}" ]]; then
  if [[ -d "$ANDROID_HOME/ndk" ]]; then
    ANDROID_NDK_ROOT="$(
      python3 - "$ANDROID_HOME/ndk" <<'PY'
import os
import sys
from pathlib import Path

root = Path(sys.argv[1])
candidates = [p for p in root.iterdir() if p.is_dir()]
if not candidates:
    print("")
    sys.exit(0)

def parse_ver(text: str):
    parts = []
    for seg in text.split("."):
        if seg.isdigit():
            parts.append(int(seg))
        else:
            parts.append(seg)
    return tuple(parts)

best = sorted(candidates, key=lambda p: parse_ver(p.name))[-1]
print(str(best))
PY
    )"
    export ANDROID_NDK_ROOT
  fi
fi

if [[ -z "${ANDROID_NDK_ROOT:-}" || ! -d "$ANDROID_NDK_ROOT" ]]; then
  echo "ERROR: Android NDK not found."
  echo "Set ANDROID_NDK_ROOT manually, e.g."
  echo "  export ANDROID_NDK_ROOT=\"$ANDROID_HOME/ndk/29.0.13113456\""
  exit 1
fi

if [[ -z "${JAVA_HOME:-}" ]]; then
  if /usr/libexec/java_home -v 17 >/dev/null 2>&1; then
    export JAVA_HOME="$(/usr/libexec/java_home -v 17)"
  else
    export JAVA_HOME="$(/usr/libexec/java_home)"
  fi
fi

export PATH="$ANDROID_HOME/platform-tools:$JAVA_HOME/bin:$PATH"

if ! command -v rustup >/dev/null 2>&1; then
  echo "ERROR: rustup not found. Install Rust toolchain first."
  exit 1
fi

if ! rustup target list --installed | rg -x "aarch64-linux-android" >/dev/null; then
  echo "Installing rust target: aarch64-linux-android"
  rustup target add aarch64-linux-android
fi

if ! command -v cargo-apk >/dev/null 2>&1; then
  echo "Installing cargo-apk"
  cargo install cargo-apk
fi

# cargo-apk requires explicit signing config for release builds.
# For local testing, reuse the default Android debug keystore.
if [[ -z "${CARGO_APK_RELEASE_KEYSTORE:-}" ]]; then
  export CARGO_APK_RELEASE_KEYSTORE="$HOME/.android/debug.keystore"
fi
if [[ -z "${CARGO_APK_RELEASE_KEYSTORE_PASSWORD:-}" ]]; then
  export CARGO_APK_RELEASE_KEYSTORE_PASSWORD="android"
fi

if [[ ! -f "$CARGO_APK_RELEASE_KEYSTORE" ]]; then
  if ! command -v keytool >/dev/null 2>&1; then
    echo "ERROR: keytool not found and release keystore is missing."
    echo "Install JDK tools or set CARGO_APK_RELEASE_KEYSTORE manually."
    exit 1
  fi

  mkdir -p "$(dirname "$CARGO_APK_RELEASE_KEYSTORE")"
  keytool -genkeypair -v \
    -keystore "$CARGO_APK_RELEASE_KEYSTORE" \
    -storepass "$CARGO_APK_RELEASE_KEYSTORE_PASSWORD" \
    -alias androiddebugkey \
    -keypass android \
    -keyalg RSA \
    -keysize 2048 \
    -validity 10000 \
    -dname "CN=Android Debug,O=Android,C=US" >/dev/null
fi

echo "Using ANDROID_HOME=$ANDROID_HOME"
echo "Using ANDROID_NDK_ROOT=$ANDROID_NDK_ROOT"
echo "Using JAVA_HOME=$JAVA_HOME"

if [[ "$MODE" == "release" ]]; then
  echo "Using CARGO_APK_RELEASE_KEYSTORE=$CARGO_APK_RELEASE_KEYSTORE"
  echo "Building APK (release)..."
  cargo apk build --release --lib
else
  echo "Building APK (debug)..."
  cargo apk build --lib
fi

echo "Done. APK artifacts in: $ROOT_DIR/target/debug/apk/torncity_tools_bevy.apk"
echo "Install to device:"
echo "  adb install -r $ROOT_DIR/target/debug/apk/torncity_tools_bevy.apk"
