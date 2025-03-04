# Flutter plugin

[Flutter](https://flutter.dev/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

Add the following to `.prototools`.

```toml
[plugins]
flutter = "github://KonstantinKai/proto-flutter-plugin"
```

Or

```sh
proto plugin add flutter github://KonstantinKai/proto-flutter-plugin
```

## Configuration

Flutter plugin can be configured with a `.prototools` file.

- `base-url` (string) - The base URL to download Flutter SDK archives and version fetching.

```toml
[tools.flutter]
base-url = "https://storage.googleapis.com/flutter_infra_release/releases" # default
```

## Notes

- Flutter plugin supports version aliases like `stable`, `beta`, `latest`
- Flutter plugin does not support built-in channel switching feature, `upgrade` and `downgrade` commands. It provides only versions for stable and beta channels with Non zero MAJOR part and respects arch and os compatibility.

## Hooks

Flutter plugin does not support hooks.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasip1
```
