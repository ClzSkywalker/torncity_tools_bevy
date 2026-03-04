#!/usr/bin/env bash
set -euo pipefail

PACKAGE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/common.sh
source "$PACKAGE_DIR/lib/common.sh"
# shellcheck source=platform/macos.sh
source "$PACKAGE_DIR/platform/macos.sh"
# shellcheck source=platform/windows.sh
source "$PACKAGE_DIR/platform/windows.sh"
# shellcheck source=platform/android.sh
source "$PACKAGE_DIR/platform/android.sh"

CONFIG_FILE="$PACKAGE_DIR/config.env"
PLATFORM=""
PROFILE_OVERRIDE=""
VERSION_OVERRIDE=""
CI_MODE_OVERRIDE=""

usage() {
  cat <<'EOF'
Usage:
  ./build/package/package.sh --platform <macos|windows|android|all> [options]

Options:
  --platform <name>   Target platform: macos|windows|android|all
  --profile <name>    Build profile: release|debug
  --version <value>   Version (v-prefix allowed)
  --ci-mode <0|1>     Override CI mode
  --config <path>     Config file path (default: build/package/config.env)
  -h, --help          Show help
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --platform)
      PLATFORM="${2:-}"
      shift 2
      ;;
    --profile)
      PROFILE_OVERRIDE="${2:-}"
      shift 2
      ;;
    --version)
      VERSION_OVERRIDE="${2:-}"
      shift 2
      ;;
    --ci-mode)
      CI_MODE_OVERRIDE="${2:-}"
      shift 2
      ;;
    --config)
      CONFIG_FILE="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      die "Unknown argument: $1"
      ;;
  esac
done

[[ -n "$PLATFORM" ]] || { usage; die "--platform is required"; }
[[ -f "$CONFIG_FILE" ]] || die "Config file not found: $CONFIG_FILE"

# shellcheck disable=SC1090
source "$CONFIG_FILE"

PROFILE="${PROFILE_OVERRIDE:-$PROFILE}"
VERSION="${VERSION_OVERRIDE:-$VERSION}"
CI_MODE="${CI_MODE_OVERRIDE:-$CI_MODE}"

[[ "$PROFILE" == "release" || "$PROFILE" == "debug" ]] || die "PROFILE must be release|debug"
[[ "$CI_MODE" == "0" || "$CI_MODE" == "1" ]] || die "CI_MODE must be 0|1"
[[ -n "${APP_NAME:-}" ]] || die "APP_NAME is required"
[[ -n "${APP_SLUG:-}" ]] || die "APP_SLUG is required"
[[ -n "${BIN_NAME:-}" ]] || die "BIN_NAME is required"
[[ -n "${VERSION:-}" ]] || die "VERSION is required"
[[ -n "${BUNDLE_ID:-}" ]] || die "BUNDLE_ID is required"
[[ -n "${OUTPUT_DIR:-}" ]] || die "OUTPUT_DIR is required"
[[ -n "${ICON_SOURCE:-}" ]] || die "ICON_SOURCE is required"

OUTPUT_DIR="$(join_path "$PROJECT_ROOT" "$OUTPUT_DIR")"
ensure_dir "$OUTPUT_DIR"

log "Project root: $PROJECT_ROOT"
log "Platform: $PLATFORM"
log "Profile: $PROFILE"
log "Version: $(normalize_version "$VERSION")"
log "Output root: $OUTPUT_DIR"
log "CI mode: $CI_MODE"

case "$PLATFORM" in
  macos)
    package_macos
    ;;
  windows)
    package_windows
    ;;
  android)
    package_android
    ;;
  all)
    package_macos
    package_windows
    package_android
    ;;
  *)
    die "Invalid platform: $PLATFORM"
    ;;
esac

log "Packaging complete."

