# tauri-plugin-system-symbols

Resolve platform system symbols into SVG path data from a Tauri app.

This plugin is designed for apps that want native-looking symbols without
bundling icon fonts or SVG assets.

## Packages

- Rust crate: [`tauri-plugin-system-symbols`](https://github.com/insd47/tauri-plugin-system-symbols)
- JavaScript package: [`tauri-plugin-system-symbols`](https://github.com/insd47/tauri-plugin-system-symbols)

## Status

- Windows: `Segoe Fluent Icons` is resolved first, with `Segoe MDL2 Assets` as the fallback.
- macOS: copied `SF Symbols` characters are resolved through CoreText and converted from `CGPath` to SVG path data.
- Other platforms: unsupported.

Calling the resolver on an unsupported platform rejects with a Rust backend error.

On macOS, pass the character produced by SF Symbols' `Copy Symbol` command,
not the symbol name. For example, `􂰵` is the copied character for
`square.and.arrow.down.badge.clock`.

## Install

Install the Rust plugin in your Tauri app:

```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-system-symbols = "0.3"
```

Install the JavaScript package:

```sh
pnpm add tauri-plugin-system-symbols
```

Register the plugin:

```rust
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_system_symbols::init());
    ...
}
```

Add the default permission to your capability file:

```json
{
  "permissions": ["system-symbols:default"]
}
```

## JavaScript API

```ts
import { getCachedSymbol, getSymbol } from 'tauri-plugin-system-symbols';

const close = getCachedSymbol('\uE8BB', 10) ?? (await getSymbol('\uE8BB', 10));
const history = await getSymbol('􂰵', 16); // square.and.arrow.down.badge.clock
```

Tauri IPC is asynchronous, so first-time symbol resolution returns a `Promise<Path[]>`.
Resolved symbols are cached by symbol and size.

## Rust API

Other Rust crates can call the native resolvers directly without registering
this crate as a Tauri plugin:

```rust
use tauri_plugin_system_symbols;

let close = get_symbol("\u{E8BB}", 10.0)?;
let history = get_symbol("􂰵", 16.0)?; // square.and.arrow.down.badge.clock
```

This is the recommended integration path when another Tauri plugin only needs
the generated SVG path data internally. Symbols are resolved according to the
current operating system.

## Types

```ts
interface Path {
  d: string;
  fillRule?: 'nonzero' | 'evenodd';
  opacity?: number;
}
```

## React

There is no separate React package. If you want a component, keep one in your app:

```sh
pnpm add @tauri-apps/plugin-os
```

```tsx
import { platform } from '@tauri-apps/plugin-os';
import { type ComponentProps, useLayoutEffect, useState } from 'react';
import { getCachedSymbol, getSymbol } from 'tauri-plugin-system-symbols';

/**
 * Renders a platform system symbol as an SVG.
 *
 * @param macos SF Symbols name used when `platform()` is `"macos"`.
 * @param windows Segoe glyph used when `platform()` is `"windows"`.
 * @param size Target icon size in CSS pixels. Defaults to `16`.
 * @param props
 */
export default function Symbol({ macos, windows, size, ...props }: Props) {
  const key = platform() === 'macos' ? macos : windows;
  const [current, setCurrent] = useState(() => ({ key, size, path: getCachedSymbol(key, size) }));

  useLayoutEffect(() => {
    if (current.key === key && current.size === size && current.path) return;
    let active = true;

    getSymbol(key, size).then((path) => {
      if (active) setCurrent({ key, size, path });
    });

    return () => {
      active = false;
    };
  }, [key, size]);

  return (
    <svg
      fill="currentColor"
      height={size}
      viewBox={`0 0 ${size} ${size}`}
      width={size}
      data-symbol={key}
      {...props}
    >
      {current.path?.map((props, index) => (
        <path key={index} {...props} />
      ))}
    </svg>
  );
}

interface Props extends ComponentProps<'svg'> {
  macos: string;
  windows: string;
  size: number;
}
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
