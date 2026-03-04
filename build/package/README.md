# Unified Packaging Scripts

This directory contains a unified entry script and per-platform packaging scripts.

## Entry Script

```bash
./build/package/package.sh --platform <macos|windows|android|all> [options]
```

Options:

- `--profile release|debug` (default from `config.env`, recommended `release`)
- `--version <value>`
- `--ci-mode 0|1`
- `--config <path>`

Artifacts are written to:

- `dist/macos/<debug|release>/`
- `dist/windows/<debug|release>/`
- `dist/android/<debug|release>/`

## Configuration

Edit `build/package/config.env`.

### Common fields

- `APP_NAME`: app display name
- `APP_SLUG`: artifact name prefix
- `BIN_NAME`: rust executable name
- `VERSION`: package version
- `BUNDLE_ID`: reverse-domain id
- `PROFILE`: `release` or `debug`
- `OUTPUT_DIR`: output root (default: `dist`)
- `ICON_SOURCE`: single source icon, e.g. `build/icon_1024x1024.png`
- `CI_MODE`: `0` (local) / `1` (CI)

### macOS-only fields

- `MACOS_APP_NAME`
- `MACOS_DISPLAY_NAME`
- `MACOS_DEPLOYMENT_TARGET`
- `MACOS_ARCHS`
- `MACOS_TEMPLATE_APP_DIR`
- `MACOS_SIGN_ENABLED`
- `MACOS_CODESIGN_IDENTITY`

### Windows-only fields

- `WINDOWS_TARGET`
- `WINDOWS_PACKAGE_FORMAT`
- `WINDOWS_ICON_PATH` (optional)

### Android-only fields

- `ANDROID_TARGETS`
- `ANDROID_PACKAGE_FORMAT` (only for artifact pickup naming/selection)
- `ANDROID_SDK_ROOT` / `ANDROID_NDK_ROOT` (optional in file, can come from environment)

Android metadata source of truth (Mode A):

- `package`, `application.label`, `sdk`, `uses_permission` are read from `Cargo.toml`
- Paths:
  - `[package.metadata.android]`
  - `[package.metadata.android.application]`
  - `[package.metadata.android.sdk]`

### Android signing configuration

Signing keys are managed automatically:

- **Debug builds**: Auto-generated on first build (`build/android/keystore/debug.keystore`)
- **Release builds**: Run `./scripts/setup_android_signing.sh` and choose option 2 or 3
  - The script will prompt for passwords and automatically configure `keystore.env`
  - No manual editing required

See [Android Signing Guide](../../docs/ANDROID_SIGNING_GUIDE.md) for details.

## Icon strategy

Use one source icon (`ICON_SOURCE`). Platform scripts convert it automatically:

- macOS: `.icns`
- Windows: `.ico`
- Android: `mipmap` icons

