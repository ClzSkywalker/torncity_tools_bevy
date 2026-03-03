#!/usr/bin/env bash
set -euo pipefail

PACKAGE_NAME="${1:-com.smth.torncitytools}"
ACTIVITY_NAME="${2:-android.app.NativeActivity}"

if ! command -v adb >/dev/null 2>&1; then
  echo "ERROR: adb not found."
  exit 1
fi

# adb kill-server >/dev/null 2>&1 || true
# adb start-server >/dev/null

DEVICE_STATE="$(adb get-state 2>/dev/null || true)"
if [[ "$DEVICE_STATE" != "device" ]]; then
  echo "ERROR: no online Android device."
  echo "Run: adb devices -l"
  exit 1
fi

echo "Clearing old logs..."
adb logcat -c

echo "Stopping previous app process (if running)..."
adb shell am force-stop "$PACKAGE_NAME" >/dev/null 2>&1 || true

echo "Launching app: $PACKAGE_NAME/$ACTIVITY_NAME"
adb shell am start -n "$PACKAGE_NAME/$ACTIVITY_NAME" >/dev/null

# Wait a short time for startup crash to occur and be written to logcat.
sleep 2

echo
echo "Crash-related log lines:"
adb logcat -d | rg -n "AndroidRuntime|FATAL EXCEPTION|$PACKAGE_NAME|bevy|panic|SIGSEGV|SIGABRT|libbevy_demo" || true

echo
echo "Tip: if no useful lines appear, rerun with explicit package/activity:"
echo "  ./scripts/logcat_crash.sh com.smth.torncitytools android.app.NativeActivity"
