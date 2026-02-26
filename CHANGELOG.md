## 0.2.2

#### Fixes

- Fixed typos in error messages ("Plase" → "Please", "manualy" → "manually")

#### Improvements

- Cached distribution metadata to avoid redundant network requests during install
- Added comments for version threshold constants
- Enhanced README with usage examples, version detection, supported platforms table, and proto version requirement
- Added GitHub Actions release workflow
- Removed redundant `build-wasm.sh` script

## 0.2.1

#### Fixes

- Fixed Linux download URL: use `.tar.xz` extension instead of `.zip`

## 0.2.0

#### Features

- Filter available versions by platform and architecture compatibility
- Support legacy versions with `v` prefix (< 1.17.0) on compatible platforms
- Show descriptive error when installing an unsupported version for the current OS/arch

#### Tests

- Added platform validation tests (macOS ARM64, Linux non-x64, Windows non-x64, unknown OS)
- Added version range and alias resolution tests
- Added download URL generation tests for all supported platforms

## 0.1.0

- Initial release
