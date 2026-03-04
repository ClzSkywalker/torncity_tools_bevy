#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"

log() {
  printf '[%s] %s\n' "$(date +'%H:%M:%S')" "$*"
}

die() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 1
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "Missing command: $1"
}

is_macos() {
  [[ "$(uname -s)" == "Darwin" ]]
}

is_windows() {
  case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*) return 0 ;;
    *) return 1 ;;
  esac
}

normalize_version() {
  local input="$1"
  if [[ "$input" == v* ]]; then
    printf '%s' "${input#v}"
  else
    printf '%s' "$input"
  fi
}

profile_dir() {
  local profile="$1"
  case "$profile" in
    release) printf 'release' ;;
    debug) printf 'debug' ;;
    *) die "Invalid PROFILE: $profile (expected: release|debug)" ;;
  esac
}

release_flag() {
  local profile="$1"
  case "$profile" in
    release) printf '%s' '--release' ;;
    debug) printf '%s' '' ;;
    *) die "Invalid PROFILE: $profile (expected: release|debug)" ;;
  esac
}

ensure_dir() {
  mkdir -p "$1"
}

join_path() {
  local left="$1"
  local right="$2"
  if [[ "$right" = /* ]]; then
    printf '%s' "$right"
  else
    printf '%s/%s' "$left" "$right"
  fi
}

artifact_dir() {
  local output_dir="$1"
  local platform="$2"
  local profile="$3"
  local profile_dir_name
  profile_dir_name="$(profile_dir "$profile")"
  printf '%s/%s/%s' "$output_dir" "$platform" "$profile_dir_name"
}

