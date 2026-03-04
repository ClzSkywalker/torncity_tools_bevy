#!/usr/bin/env bash
set -euo pipefail

PLATFORM_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../lib/common.sh
source "$PLATFORM_DIR/../lib/common.sh"

prepare_macos_icns() {
  local icon_source="$1"
  local icns_out="$2"
  local tmp_iconset
  tmp_iconset="$(mktemp -d)/AppIcon.iconset"
  mkdir -p "$tmp_iconset"

  require_cmd sips
  require_cmd iconutil

  sips -z 16 16   "$icon_source" --out "$tmp_iconset/icon_16x16.png" >/dev/null
  sips -z 32 32   "$icon_source" --out "$tmp_iconset/icon_16x16@2x.png" >/dev/null
  sips -z 32 32   "$icon_source" --out "$tmp_iconset/icon_32x32.png" >/dev/null
  sips -z 64 64   "$icon_source" --out "$tmp_iconset/icon_32x32@2x.png" >/dev/null
  sips -z 128 128 "$icon_source" --out "$tmp_iconset/icon_128x128.png" >/dev/null
  sips -z 256 256 "$icon_source" --out "$tmp_iconset/icon_128x128@2x.png" >/dev/null
  sips -z 256 256 "$icon_source" --out "$tmp_iconset/icon_256x256.png" >/dev/null
  sips -z 512 512 "$icon_source" --out "$tmp_iconset/icon_256x256@2x.png" >/dev/null
  sips -z 512 512 "$icon_source" --out "$tmp_iconset/icon_512x512.png" >/dev/null
  cp "$icon_source" "$tmp_iconset/icon_512x512@2x.png"

  iconutil -c icns "$tmp_iconset" -o "$icns_out"
}

package_macos() {
  is_macos || die "macOS packaging must run on macOS."
  require_cmd cargo
  require_cmd rustup
  require_cmd lipo
  require_cmd hdiutil
  require_cmd xcrun

  local version_no_v profile_dir_name output_platform_dir
  version_no_v="$(normalize_version "$VERSION")"
  profile_dir_name="$(profile_dir "$PROFILE")"
  output_platform_dir="$(artifact_dir "$OUTPUT_DIR" "macos" "$PROFILE")"
  ensure_dir "$output_platform_dir"

  local icon_source_abs template_app_abs stage_dir app_dir plist_path
  icon_source_abs="$(join_path "$PROJECT_ROOT" "$ICON_SOURCE")"
  template_app_abs="$(join_path "$PROJECT_ROOT" "$MACOS_TEMPLATE_APP_DIR")"
  stage_dir="$output_platform_dir/stage"
  app_dir="$stage_dir/${MACOS_APP_NAME}.app"

  [[ -f "$icon_source_abs" ]] || die "ICON_SOURCE not found: $icon_source_abs"
  [[ -d "$template_app_abs" ]] || die "MACOS_TEMPLATE_APP_DIR not found: $template_app_abs"

  IFS=',' read -r -a archs <<< "$MACOS_ARCHS"
  [[ "${#archs[@]}" -ge 1 ]] || die "MACOS_ARCHS is empty"

  log "Install rust targets: ${archs[*]}"
  rustup target add "${archs[@]}"

  local arch target_bin universal_bin
  for arch in "${archs[@]}"; do
    log "Build macOS target: $arch ($PROFILE)"
    if [[ "$PROFILE" == "release" ]]; then
      SDKROOT="$(xcrun -sdk macosx --show-sdk-path)" \
        MACOSX_DEPLOYMENT_TARGET="$MACOS_DEPLOYMENT_TARGET" \
        cargo build --target "$arch" --release
    else
      SDKROOT="$(xcrun -sdk macosx --show-sdk-path)" \
        MACOSX_DEPLOYMENT_TARGET="$MACOS_DEPLOYMENT_TARGET" \
        cargo build --target "$arch"
    fi
  done

  universal_bin="$output_platform_dir/$BIN_NAME"
  local lipo_inputs=()
  for arch in "${archs[@]}"; do
    target_bin="$PROJECT_ROOT/target/$arch/$profile_dir_name/$BIN_NAME"
    [[ -f "$target_bin" ]] || die "Built binary not found: $target_bin"
    lipo_inputs+=("$target_bin")
  done

  log "Create universal binary"
  lipo -create -output "$universal_bin" "${lipo_inputs[@]}"

  log "Assemble .app bundle"
  rm -rf "$stage_dir"
  mkdir -p "$stage_dir"
  cp -R "$template_app_abs" "$app_dir"
  mkdir -p "$app_dir/Contents/MacOS" "$app_dir/Contents/Resources"
  cp "$universal_bin" "$app_dir/Contents/MacOS/$BIN_NAME"
  chmod +x "$app_dir/Contents/MacOS/$BIN_NAME"

  prepare_macos_icns "$icon_source_abs" "$app_dir/Contents/Resources/AppIcon.icns"

  plist_path="$app_dir/Contents/Info.plist"
  /usr/libexec/PlistBuddy -c "Set :CFBundleDisplayName $MACOS_DISPLAY_NAME" "$plist_path" || true
  /usr/libexec/PlistBuddy -c "Set :CFBundleName $MACOS_DISPLAY_NAME" "$plist_path" || true
  /usr/libexec/PlistBuddy -c "Set :CFBundleExecutable $BIN_NAME" "$plist_path" || true
  /usr/libexec/PlistBuddy -c "Set :CFBundleIdentifier $BUNDLE_ID" "$plist_path" || true
  /usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $version_no_v" "$plist_path" || true

  if [[ -d "$PROJECT_ROOT/assets" ]]; then
    cp -R "$PROJECT_ROOT/assets" "$app_dir/Contents/MacOS/assets"
  fi

  if [[ "$MACOS_SIGN_ENABLED" == "1" ]]; then
    log "Codesign app"
    codesign --force --deep --sign "$MACOS_CODESIGN_IDENTITY" "$app_dir"
  fi

  ln -sfn /Applications "$stage_dir/Applications"
  local dmg_name dmg_path
  dmg_name="${APP_SLUG}_${version_no_v}_macOS.dmg"
  dmg_path="$output_platform_dir/$dmg_name"
  rm -f "$dmg_path"
  log "Create DMG: $dmg_path"
  hdiutil create -fs HFS+ -volname "$MACOS_APP_NAME" -srcfolder "$stage_dir" "$dmg_path" >/dev/null

  log "macOS done: $dmg_path"
}

