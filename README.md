# tauri-plugin-system-symbols

Tauri plugin for resolving platform system symbols into SVG path data.

## Packages

- Rust crate: `tauri-plugin-system-symbols`
- Generic JavaScript package: `tauri-plugin-system-symbols`
- React package: `tauri-plugin-system-symbols-react`

The React package is developed in a separate repository at
`/Users/insd47/Developer/insd/tauri-plugin-system-symbols-react`.

## Install

```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-system-symbols = "0.1"
```

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_system_symbols::init());
```

Add the default permission to your capability file:

```json
{
  "permissions": ["system-symbols:default"]
}
```

## JavaScript Usage

```ts
import { getFluentIcon, getSfSymbol } from 'tauri-plugin-system-symbols'

const close = await getFluentIcon('\uE8BB')
const share = await getSfSymbol('square.and.arrow.up')
```

`getFluentIcon` is Windows-only. The backend resolves `Segoe Fluent Icons`
first and falls back to `Segoe MDL2 Assets`.

`getSfSymbol` is macOS-only. Calling either API on the wrong platform rejects
with a Rust backend error.

Tauri IPC is asynchronous, so the JavaScript API returns `Promise<Symbol>`.
Repeated requests are cached in both JavaScript and Rust.

## Current Backend Status

- Windows: DirectWrite-based Segoe Fluent Icons / Segoe MDL2 Assets path extraction is implemented behind `cfg(windows)` and needs Windows host validation.
- macOS: `get_sf_symbols` command and platform boundary exist; SF Symbols vector extraction is still pending.
- Other platforms: unsupported.

## References

- [Tauri plugin development](https://v2.tauri.app/develop/plugins/)
- [Tauri permissions](https://v2.tauri.app/security/permissions/)
