#!/usr/bin/env bash
set -euo pipefail

PLATFORM_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../lib/common.sh
source "$PLATFORM_DIR/../lib/common.sh"

to_windows_path_for_powershell() {
  local input_path="$1"
  if command -v cygpath >/dev/null 2>&1; then
    cygpath -w "$input_path"
  else
    printf '%s' "$input_path"
  fi
}

generate_windows_icon() {
  local icon_source="$1"
  local icon_out="$2"

  if command -v magick >/dev/null 2>&1; then
    magick "$icon_source" -define icon:auto-resize=256,128,64,48,32,16 "$icon_out"
    return
  fi

  if command -v convert >/dev/null 2>&1; then
    convert "$icon_source" -define icon:auto-resize=256,128,64,48,32,16 "$icon_out"
    return
  fi

  die "Cannot generate .ico, install ImageMagick (magick/convert) or set WINDOWS_ICON_PATH"
}

package_windows() {
  require_cmd cargo
  require_cmd rustup

  local version_no_v profile_dir_name output_platform_dir
  version_no_v="$(normalize_version "$VERSION")"
  profile_dir_name="$(profile_dir "$PROFILE")"
  output_platform_dir="$(artifact_dir "$OUTPUT_DIR" "windows" "$PROFILE")"
  ensure_dir "$output_platform_dir"

  local build_icon_dir build_icon_path
  build_icon_dir="$PROJECT_ROOT/target/package/windows"
  build_icon_path="$build_icon_dir/icon.ico"
  ensure_dir "$build_icon_dir"

  if [[ -n "$WINDOWS_ICON_PATH" ]]; then
    local icon_abs
    icon_abs="$(join_path "$PROJECT_ROOT" "$WINDOWS_ICON_PATH")"
    [[ -f "$icon_abs" ]] || die "WINDOWS_ICON_PATH not found: $icon_abs"
    cp "$icon_abs" "$build_icon_path"
  else
    local icon_source_abs
    icon_source_abs="$(join_path "$PROJECT_ROOT" "$ICON_SOURCE")"
    [[ -f "$icon_source_abs" ]] || die "ICON_SOURCE not found: $icon_source_abs"
    generate_windows_icon "$icon_source_abs" "$build_icon_path"
  fi

  local target="$WINDOWS_TARGET"
  rustup target add "$target"

  log "Build Windows target: $target ($PROFILE)"
  if [[ "$PROFILE" == "release" ]]; then
    APP_ICON_ICO="$build_icon_path" cargo build --target "$target" --release
  else
    APP_ICON_ICO="$build_icon_path" cargo build --target "$target"
  fi

  local built_exe="$PROJECT_ROOT/target/$target/$profile_dir_name/$BIN_NAME.exe"
  [[ -f "$built_exe" ]] || die "Built executable not found: $built_exe"

  local stage_dir="$output_platform_dir/stage"
  rm -rf "$stage_dir"
  mkdir -p "$stage_dir"
  cp "$built_exe" "$stage_dir/$BIN_NAME.exe"
  if [[ -d "$PROJECT_ROOT/assets" ]]; then
    cp -R "$PROJECT_ROOT/assets" "$stage_dir/assets"
  fi

  local zip_name zip_path
  zip_name="${APP_SLUG}_${version_no_v}_windows.zip"
  zip_path="$output_platform_dir/$zip_name"
  rm -f "$zip_path"

  log "Create zip package: $zip_path"
  if command -v zip >/dev/null 2>&1; then
    (cd "$stage_dir" && zip -qr "$zip_path" .)
  elif command -v powershell >/dev/null 2>&1; then
    local ps_stage_dir ps_zip_path
    ps_stage_dir="$(to_windows_path_for_powershell "$stage_dir")"
    ps_zip_path="$(to_windows_path_for_powershell "$zip_path")"
    powershell -NoProfile -Command \
      "Compress-Archive -Path '$ps_stage_dir\\*' -DestinationPath '$ps_zip_path' -Force -ErrorAction Stop" >/dev/null
  else
    die "Cannot create zip: install zip or powershell"
  fi

  [[ -f "$zip_path" ]] || die "Zip file not created: $zip_path"
  log "Windows done: $zip_path"
}

