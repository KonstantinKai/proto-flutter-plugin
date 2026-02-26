# Flutter plugin

[![Release](https://github.com/KonstantinKai/proto-flutter-plugin/actions/workflows/release.yml/badge.svg)](https://github.com/KonstantinKai/proto-flutter-plugin/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A community [WASM plugin](https://moonrepo.dev/docs/proto/wasm-plugin) for [proto](https://github.com/moonrepo/proto) that manages [Flutter](https://flutter.dev/) SDK versions.

Requires [proto](https://github.com/moonrepo/proto) >= 0.47.0

## Installation

```sh
proto plugin add flutter "github://KonstantinKai/proto-flutter-plugin"
proto install flutter
```

Or add manually to `.prototools`:

```toml
[plugins.tools]
flutter = "github://KonstantinKai/proto-flutter-plugin"
```

## Usage

```sh
# Install Flutter
proto install flutter 3.29

# Use Flutter
proto run flutter -- --version

# List available versions
proto versions flutter

# Pin a version in the current directory
proto pin flutter 3.29
```

## Version Detection

The plugin automatically detects Flutter versions from:

- `pubspec.yaml` / `pubspec.yml` — reads `environment.flutter` field (supports version constraints)

## Configuration

Configure in `.prototools` under `[tools.flutter]`:

```toml
[tools.flutter]
# Custom base URL for Flutter SDK archives (default: official Google storage)
base-url = "https://storage.googleapis.com/flutter_infra_release/releases"
```

## Supported Platforms

| Platform | Architecture | Notes |
|----------|-------------|-------|
| Linux | x64 | All versions |
| macOS | x64 | All versions |
| macOS | arm64 | Stable >= 3.0.0, beta >= 2.12.0-4.1.pre |
| Windows | x64 | All versions |

## Notes

- Supports version aliases: `stable`, `beta`, `latest`
- Does not support channel switching via `flutter channel` — use `proto install flutter beta` instead
- Only includes stable and beta channel versions with non-zero MAJOR part
- Respects platform and architecture compatibility when listing versions

## Hooks

Flutter plugin does not support hooks.

## Contributing

Build the plugin:

```sh
cargo build --target wasm32-wasip1
```
