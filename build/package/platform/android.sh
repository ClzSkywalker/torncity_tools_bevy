#!/usr/bin/env bash
set -euo pipefail

PLATFORM_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../lib/common.sh
source "$PLATFORM_DIR/../lib/common.sh"

# 配置 Android 签名密钥
configure_android_signing() {
  local profile="$1"
  local keystore_dir="$PROJECT_ROOT/build/android/keystore"
  local keystore_env="$keystore_dir/keystore.env"

  # 默认配置
  local debug_keystore="$keystore_dir/debug.keystore"
  local debug_password="android"
  local debug_alias="androiddebugkey"

  local release_keystore="$keystore_dir/release.keystore"
  local release_password=""
  local release_alias="torn_trade"

  # 加载配置文件(如果存在)
  if [[ -f "$keystore_env" ]]; then
    # shellcheck source=/dev/null
    source "$keystore_env"
    debug_keystore="${DEBUG_KEYSTORE_PATH:-$debug_keystore}"
    debug_password="${DEBUG_KEYSTORE_PASSWORD:-$debug_password}"
    debug_alias="${DEBUG_KEY_ALIAS:-$debug_alias}"
    release_keystore="${RELEASE_KEYSTORE_PATH:-$release_keystore}"
    release_password="${RELEASE_KEYSTORE_PASSWORD:-}"
    release_alias="${RELEASE_KEY_ALIAS:-$release_alias}"
  fi

  local keystore_path=""
  local keystore_password=""
  local key_alias=""

  if [[ "$profile" == "release" ]]; then
    # Release 模式配置
    keystore_path="$release_keystore"
    keystore_password="$release_password"
    key_alias="$release_alias"

    # 转换为绝对路径
    [[ "$keystore_path" = /* ]] || keystore_path="$PROJECT_ROOT/$keystore_path"

    # 检查 Release 密钥
    if [[ ! -f "$keystore_path" ]]; then
      die "Release keystore not found: $keystore_path
Please generate it first:
  ./scripts/setup_android_signing.sh
Or use debug profile instead."
    fi

    if [[ -z "$keystore_password" ]]; then
      die "Release keystore password not configured.
Please configure it:
  1. Copy template: cp $keystore_dir/keystore.env.template $keystore_dir/keystore.env
  2. Edit keystore.env and fill in actual passwords"
    fi
  else
    # Debug 模式配置
    keystore_path="$debug_keystore"
    keystore_password="$debug_password"
    key_alias="$debug_alias"

    # 转换为绝对路径
    [[ "$keystore_path" = /* ]] || keystore_path="$PROJECT_ROOT/$keystore_path"

    # 自动生成 Debug 密钥(如果不存在)
    if [[ ! -f "$keystore_path" ]]; then
      log "Generating debug keystore: $keystore_path"
      ensure_dir "$(dirname "$keystore_path")"
      keytool -genkeypair -v \
        -keystore "$keystore_path" \
        -storepass "$keystore_password" \
        -alias "$key_alias" \
        -keypass "$keystore_password" \
        -keyalg RSA \
        -keysize 2048 \
        -validity 10000 \
        -dname "CN=Android Debug,O=Android,C=US" >/dev/null
    fi
  fi

  # 导出环境变量供 cargo-apk 使用
  export CARGO_APK_RELEASE_KEYSTORE="$keystore_path"
  export CARGO_APK_RELEASE_KEYSTORE_PASSWORD="$keystore_password"

  log "Keystore: $keystore_path (alias: $key_alias)"
}

resize_png() {
  local src="$1"
  local size="$2"
  local out="$3"

  if command -v magick >/dev/null 2>&1; then
    magick "$src" -resize "${size}x${size}" "$out"
    return
  fi

  if command -v convert >/dev/null 2>&1; then
    convert "$src" -resize "${size}x${size}" "$out"
    return
  fi

  if is_macos && command -v sips >/dev/null 2>&1; then
    sips -z "$size" "$size" "$src" --out "$out" >/dev/null
    return
  fi

  die "Cannot resize png: install ImageMagick or use macOS sips"
}

prepare_android_icons() {
  local icon_source="$1"
  local res_dir="$PROJECT_ROOT/build/android/res"
  local mappings=(
    "mipmap-mdpi:48"
    "mipmap-hdpi:72"
    "mipmap-xhdpi:96"
    "mipmap-xxhdpi:144"
    "mipmap-xxxhdpi:192"
  )
  local entry density size out

  for entry in "${mappings[@]}"; do
    density="${entry%%:*}"
    size="${entry##*:}"
    ensure_dir "$res_dir/$density"
    out="$res_dir/$density/icon.png"
    resize_png "$icon_source" "$size" "$out"
  done
}

pick_latest_artifact() {
  local pattern="$1"
  local latest=""
  local latest_mtime=-1
  local f mtime
  local files=()
  shopt -s nullglob
  files=($pattern)
  shopt -u nullglob

  for f in "${files[@]}"; do
    if is_macos; then
      mtime="$(stat -f "%m" "$f")"
    else
      mtime="$(stat -c "%Y" "$f")"
    fi
    if [[ "$mtime" -gt "$latest_mtime" ]]; then
      latest_mtime="$mtime"
      latest="$f"
    fi
  done
  printf '%s' "$latest"
}

package_android() {
  require_cmd cargo
  require_cmd rustup
  require_cmd cargo-apk

  local version_no_v output_platform_dir
  version_no_v="$(normalize_version "$VERSION")"
  output_platform_dir="$(artifact_dir "$OUTPUT_DIR" "android" "$PROFILE")"
  ensure_dir "$output_platform_dir"

  local icon_source_abs
  icon_source_abs="$(join_path "$PROJECT_ROOT" "$ICON_SOURCE")"
  [[ -f "$icon_source_abs" ]] || die "ICON_SOURCE not found: $icon_source_abs"
  prepare_android_icons "$icon_source_abs"

  IFS=',' read -r -a targets <<< "$ANDROID_TARGETS"
  rustup target add "${targets[@]}"

  local sdk_root="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-}}"
  local ndk_root="${ANDROID_NDK_ROOT:-${ANDROID_NDK_LATEST_HOME:-}}"
  [[ -n "$sdk_root" ]] || die "ANDROID_HOME or ANDROID_SDK_ROOT is required"
  [[ -n "$ndk_root" ]] || die "ANDROID_NDK_ROOT or ANDROID_NDK_LATEST_HOME is required"

  # 配置签名
  configure_android_signing "$PROFILE"

  log "Build Android package ($ANDROID_PACKAGE_FORMAT, $PROFILE)"
  if [[ "$PROFILE" == "release" ]]; then
    ANDROID_HOME="$sdk_root" ANDROID_NDK_ROOT="$ndk_root" cargo apk build --release --lib
  else
    ANDROID_HOME="$sdk_root" ANDROID_NDK_ROOT="$ndk_root" cargo apk build --lib
  fi

  local built_artifact=""
  if [[ "$PROFILE" == "release" ]]; then
    built_artifact="$(pick_latest_artifact "$PROJECT_ROOT/target/release/apk/*.apk")"
    if [[ "$ANDROID_PACKAGE_FORMAT" == "aab" ]]; then
      built_artifact="$(pick_latest_artifact "$PROJECT_ROOT/target/release/apk/*.aab")"
    fi
  else
    built_artifact="$(pick_latest_artifact "$PROJECT_ROOT/target/debug/apk/*.apk")"
    if [[ "$ANDROID_PACKAGE_FORMAT" == "aab" ]]; then
      built_artifact="$(pick_latest_artifact "$PROJECT_ROOT/target/debug/apk/*.aab")"
    fi
  fi

  [[ -n "$built_artifact" ]] || die "Cannot find Android artifact in target/*/apk"

  local ext out_name out_path
  ext="${built_artifact##*.}"
  out_name="${APP_SLUG}_${version_no_v}_android.${ext}"
  out_path="$output_platform_dir/$out_name"
  cp "$built_artifact" "$out_path"

  log "Android done: $out_path"
}

