# Implementation Plan

## Assumptions

- This repository owns the Tauri plugin crate and the generic JavaScript NPM package named `tauri-plugin-system-symbols`.
- `tauri-plugin-system-symbols-react` lives in a separate Git repository at `/Users/insd47/Developer/insd/tauri-plugin-system-symbols-react`.
- The plugin targets Tauri v2 and follows the current permission/capability model.
- Symbol geometry is returned as SVG path data plus a fixed `viewBox`; frontend packages own rendering and styling.

## Current API

Rust plugin commands:

- `get_fluent_icons(icons)`
- `get_sf_symbols(symbols)`

JavaScript API:

- `getFluentIcon(icon)`
- `getSfSymbol(symbol)`
- `getCachedFluentIcon(icon)`
- `getCachedSfSymbol(symbol)`
- `preloadFluentIcons(...icons)`
- `preloadSfSymbols(...symbols)`

React API:

- `<SfSymbol symbol="square.and.arrow.up" />`
- `<FluentIcon icon={"\uE8BB"} />`
- `<SystemSymbol symbol="square.and.arrow.up" icon={"\uE8BB"} />`

## Backend Design

`src/platform` is the only place allowed to know platform APIs.

- Windows:
  - Use DirectWrite system font collection.
  - Resolve `Segoe Fluent Icons` first, then `Segoe MDL2 Assets`.
  - Accept a single glyph character, `U+E921`, `0xE921`, or raw hex such as `E921`.
  - Return one or more SVG paths in a `Symbol`.
- macOS:
  - Keep the command and platform boundary in place.
  - Resolve SF Symbols by symbol name.
  - Implement vector extraction behind `platform/macos.rs` without changing the JS or React API.
- Wrong platform:
  - Return a typed Rust backend error.

## Shape

The public return type is multi-path from the start:

```ts
interface Symbol {
  viewBox: string
  paths: Path[]
}
```

Windows glyphs usually return one path. SF Symbols can later return layered
paths without changing the frontend API.

## Performance Plan

- Keep one command per symbol family instead of a generic auto resolver.
- Cache on both sides:
  - Rust cache avoids repeated font lookup/outline extraction.
  - JS cache avoids repeated Tauri IPC calls from component rerenders.
- Add backend-level font face caching when extraction work expands beyond the current scaffold.
- Avoid shipping full icon maps until name-to-codepoint lookup becomes necessary.

## React Package Strategy

The React package is intentionally thin:

- It depends on `tauri-plugin-system-symbols` for symbol loading.
- It depends on `@tauri-apps/plugin-os` for `platform()` in `SystemSymbol`.
- It does not call Tauri commands directly.
- It extends `SVGProps<SVGSVGElement>` and uses `cn` for class composition.

## Migration Plan For window-controls

Once this plugin has stable Windows extraction:

1. Move shared Segoe path extraction behavior into this crate as the canonical implementation.
2. Add a small adapter in `tauri-plugin-window-controls` that requests caption glyphs through `tauri-plugin-system-symbols`.
3. Keep `window-controls` caption behavior, snap overlay, and window commands inside `window-controls`.
4. Release a minor version of `window-controls` that depends on this plugin internally or exposes an opt-in feature first.
5. Remove duplicated DirectWrite glyph code after the adapter is verified on Windows 10 and Windows 11.

## Verification Criteria

- `cargo fmt --check` passes.
- `cargo check` passes on the host platform.
- `cargo test` passes on the host platform.
- `npm run check` passes for the core JavaScript package.
- `npm run build` passes for the core JavaScript package.
- React package `npm run check` and `npm run build` pass.
- Later Windows validation must include real glyph path extraction for `U+E921`, `U+E922`, `U+E923`, and `U+E8BB`.

## References

- [Tauri plugin development](https://v2.tauri.app/develop/plugins/)
- [Tauri permissions](https://v2.tauri.app/security/permissions/)
