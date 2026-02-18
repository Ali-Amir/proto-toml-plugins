# buf-proto-plugin

A WASM plugin for [moonrepo/proto](https://github.com/moonrepo/proto) that installs [buf](https://github.com/bufbuild/buf) (Protocol Buffers CLI).

## Installation

```
proto add-plugin buf "source:https://github.com/Ali-Amir/proto-toml-plugins/releases/download/buf_proto_plugin/v0.1.0/buf_proto_plugin.wasm"
proto install buf
```

Refer to https://moonrepo.dev/docs/proto/tools#third-party for more information.

## Why WASM instead of TOML?

There is already a TOML plugin for buf at [stk0vrfl0w/proto-toml-plugins](https://github.com/stk0vrfl0w/proto-toml-plugins/blob/main/plugins/buf.toml). However, TOML plugins apply architecture name mappings globally across all platforms. The buf project uses inconsistent architecture naming in their releases:

- macOS: `buf-Darwin-arm64`
- Linux: `buf-Linux-aarch64`
- Windows: `buf-Windows-arm64.exe`

The TOML plugin maps `aarch64` to `arm64` for all platforms, which breaks Linux arm64 downloads. This WASM plugin handles the per-platform arch mapping correctly.

## Building from source

```
cargo build --target wasm32-wasip1 --release
```

The built `.wasm` file will be in `target/wasm32-wasip1/release/buf_proto_plugin.wasm`.
