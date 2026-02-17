# proto-toml-plugins

TOML plugins for moonrepo/proto

## About

Just a small collection of TOML plugins for the [moonrepo/proto](https://github.com/moonrepo/proto) tool.
See the [Moonrepo Plugins](https://moonrepo.dev/docs/proto/plugins) pages for more information about how to
create your own TOML/WASM plugins.

## Installation Methods

The plugins can be installed via `proto` itself. Assuming you wanted to install the latest available `buf`:

```
proto add-plugin buf "source:https://raw.githubusercontent.com/Ali-Amir/proto-toml-plugins/main/plugins/buf.toml"
proto install buf
```

Refer to https://moonrepo.dev/docs/proto/tools#third-party for more information.

## Available plugins in this repo

* buf - Protocol Buffers (https://github.com/bufbuild/buf)
