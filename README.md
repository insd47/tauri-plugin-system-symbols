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
- macOS: `SF Symbols` are resolved through AppKit and converted from `NSBezierPath` to SVG path data.
- Other platforms: unsupported.

Calling the resolver on an unsupported platform rejects with a Rust backend error.

The macOS backend uses AppKit's public `NSImage(systemSymbolName:)` resolver,
then reads the returned symbol representation through the undocumented
`NSSymbolImageRep.outlinePath` selector. This is intentional for native SF
Symbols path extraction, but it is not a public Apple API contract and should
not be treated as App Store-safe.

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
import { getCachedSymbol, getSymbol } from 'tauri-plugin-system-symbols'

const close = getCachedSymbol('\uE8BB', 10) ?? await getSymbol('\uE8BB', 10)
const share = await getSymbol('square.and.arrow.up', 16)
```

Tauri IPC is asynchronous, so first-time symbol resolution returns a `Promise<Symbol>`.
Resolved symbols are cached by symbol and size.

## Rust API

Other Rust crates can call the native resolvers directly without registering
this crate as a Tauri plugin:

```rust
let close = tauri_plugin_system_symbols::get_symbol("\u{E8BB}", 10.0)?;
let share = tauri_plugin_system_symbols::get_symbol("square.and.arrow.up", 16.0)?;
```

This is the recommended integration path when another Tauri plugin only needs
the generated SVG path data internally. Symbols are resolved according to the
current operating system.

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

## Publishing

Publishing is handled by [`.github/workflows/publish.yml`](./.github/workflows/publish.yml).

Create a tag to publish both packages:

```sh
git tag v0.1.0
git push origin v0.1.0
```

The workflow sets `package.json` and `Cargo.toml` versions from the tag, then publishes:

- npm with `npm publish --provenance --access public`
- crates.io with `rust-lang/crates-io-auth-action`

Before the first trusted publish, configure trusted publishing in both registries:

- npm: trust `insd47/tauri-plugin-system-symbols` and workflow `.github/workflows/publish.yml`
- crates.io: trust `insd47/tauri-plugin-system-symbols` and workflow `.github/workflows/publish.yml`

## License

MIT
