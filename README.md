# tauri-plugin-system-symbols

Tauri plugin for resolving system icon glyphs into SVG path data.

## Packages

- Rust crate: `tauri-plugin-system-symbols`
- Generic JavaScript package: `tauri-plugin-system-symbols`
- Planned React package: `tauri-plugin-system-symbols-react`

The generic JavaScript package lives in this repository. The React package should
be developed in a separate repository so React release cadence, peer dependency
policy, and component APIs can evolve without coupling to the core Tauri plugin.

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
import { getSymbol, getSymbols } from 'tauri-plugin-system-symbols'

const close = await getSymbol({
  family: 'segoeFluentIcons',
  symbol: 'U+E8BB'
})

const icons = await getSymbols([
  { family: 'segoeFluentIcons', symbol: 'U+E921' },
  { family: 'segoeFluentIcons', symbol: 'U+E8BB' }
])
```

## Current Backend Status

- Windows: DirectWrite-based Segoe Fluent Icons / Segoe MDL2 Assets path extraction is implemented behind `cfg(windows)` and needs Windows host validation.
- macOS: SF Symbols backend is planned.
- Other platforms: unsupported until a platform backend is added.

Use `getSymbols` for batch requests. Both Rust and JavaScript layers cache
resolved symbols to reduce repeated Tauri command and font extraction overhead.

## References

- [Tauri plugin development](https://v2.tauri.app/develop/plugins/)
- [Tauri permissions](https://v2.tauri.app/security/permissions/)
