# tauri-plugin-system-symbols

Resolve platform system symbols into SVG path data from a Tauri app.

This plugin is designed for apps that want native-looking symbols without
bundling icon fonts or SVG assets.

## Packages

- Rust crate: [`tauri-plugin-system-symbols`](https://github.com/insd47/tauri-plugin-system-symbols)
- JavaScript package: [`tauri-plugin-system-symbols`](https://github.com/insd47/tauri-plugin-system-symbols)
- React package: [`tauri-plugin-system-symbols-react`](https://github.com/insd47/tauri-plugin-system-symbols-react)

## Status

- Windows: `Segoe Fluent Icons` is resolved first, with `Segoe MDL2 Assets` as the fallback.
- macOS: `SF Symbols` command/API boundaries are in place, but vector extraction is still pending.
- Other platforms: unsupported.

Calling a platform-specific API on the wrong platform rejects with a Rust backend error.

## Install

Install the Rust plugin in your Tauri app:

```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-system-symbols = "0.1"
```

Install the JavaScript package:

```sh
pnpm add tauri-plugin-system-symbols
```

Register the plugin:

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

## JavaScript API

```ts
import {
  getCachedFluentIcon,
  getCachedSfSymbol,
  getFluentIcon,
  getSfSymbol,
  preloadFluentIcons,
  preloadSfSymbols,
} from 'tauri-plugin-system-symbols'

await preloadFluentIcons('\uE8BB', '\uE921')
await preloadSfSymbols('square.and.arrow.up')

const close = getCachedFluentIcon('\uE8BB') ?? await getFluentIcon('\uE8BB')
const share = getCachedSfSymbol('square.and.arrow.up') ?? await getSfSymbol('square.and.arrow.up')
```

Tauri IPC is asynchronous, so first-time symbol resolution returns a `Promise<Symbol>`.
Use `preloadFluentIcons(...icons)` and `preloadSfSymbols(...symbols)` to warm the cache before rendering.

## Types

```ts
interface Symbol {
  viewBox: string
  paths: Path[]
}

interface Path {
  d: string
  fillRule?: 'nonzero' | 'evenodd'
  opacity?: number
}
```

## React

React components are provided by
[`tauri-plugin-system-symbols-react`](https://github.com/insd47/tauri-plugin-system-symbols-react).

```sh
pnpm add tauri-plugin-system-symbols-react @tauri-apps/plugin-os
```

## Development

```sh
pnpm install
pnpm run check
pnpm run build
cargo check
cargo test
```

## License

MIT
