# Implementation Plan

## Assumptions

- This repository owns the Tauri plugin crate and the generic JavaScript NPM package named `tauri-plugin-system-symbols`.
- `tauri-plugin-system-symbols-react` should be a separate repository. It will depend on the generic JavaScript package and expose React components/hooks only.
- The plugin targets Tauri v2 and follows the current permission/capability model.
- Symbol geometry is returned as SVG path data plus a fixed `viewBox`; frontend packages own rendering and styling.

## Current Boilerplate

1. Rust plugin entrypoint registers `system-symbols` with two commands:
   - `get_symbol`
   - `get_symbols`
2. `build.rs` generates Tauri permission metadata for those commands.
3. `permissions/default.toml` exposes `system-symbols:default`.
4. `guest-js/index.ts` exposes framework-neutral bindings with in-memory caching.
5. Windows Segoe extraction is shaped after `insd47/tauri-plugin-window-controls`:
   - font family resolution
   - DirectWrite outline extraction
   - SVG path normalization

## Backend Design

`src/platform` is the only place allowed to know platform APIs.

- Windows:
  - Use DirectWrite system font collection.
  - Resolve `Segoe Fluent Icons` first, then `Segoe MDL2 Assets` for `family: "auto"`.
  - Accept a single glyph character, `U+E921`, `0xE921`, or raw hex such as `E921`.
  - Cache resolved `SymbolRequest -> SvgSymbol` at the Tauri state layer.
- macOS:
  - Resolve SF Symbols by symbol name.
  - Prefer system APIs that produce vector data directly.
  - Keep symbol configuration explicit later: weight, scale, variable value, and rendering mode should not be added until needed by a caller.
- Unsupported platforms:
  - Return a typed plugin error instead of silently substituting bundled icons.

## Performance Plan

- Keep `get_symbols` as the preferred public path for UI startup and icon-heavy screens.
- Keep `get_symbol` as a convenience wrapper for low-volume calls.
- Cache on both sides:
  - Rust cache avoids repeated font lookup/outline extraction.
  - JS cache avoids repeated Tauri IPC calls from component rerenders.
- Add backend-level font face caching when real Windows/macOS extraction work expands beyond the current scaffold.
- Do not ship full icon maps in the first version. Add optional generated metadata only if name-to-codepoint lookup becomes necessary.

## React Package Strategy

Create `insd47/tauri-plugin-system-symbols-react` separately.

Initial API should be thin:

- `<SystemSymbol symbol="xmark" />`
- `useSystemSymbol(request)`
- `SystemSymbolsProvider` only if shared configuration becomes necessary.

The React package should not call Tauri commands directly. It should depend on
`tauri-plugin-system-symbols`, so non-React consumers and React consumers share
the same cache and request semantics.

## Migration Plan For window-controls

Once this plugin has stable Windows extraction:

1. Move shared Segoe path extraction behavior into this crate as the canonical implementation.
2. Add a small adapter in `tauri-plugin-window-controls` that requests caption glyphs through `tauri-plugin-system-symbols`.
3. Keep `window-controls` caption behavior, snap overlay, and window commands inside `window-controls`.
4. Release a minor version of `window-controls` that depends on this plugin internally or exposes an opt-in feature first.
5. Remove duplicated DirectWrite glyph code after the adapter is verified on Windows 10 and Windows 11.

## Verification Criteria

- `cargo check` passes on the host platform.
- `npm run check` passes for guest bindings.
- `npm run build` produces `dist-js/index.js`, `dist-js/index.cjs`, and `dist-js/index.d.ts`.
- Later Windows validation must include real glyph path extraction for `U+E921`, `U+E922`, `U+E923`, and `U+E8BB`.

## References

- [Tauri plugin development](https://v2.tauri.app/develop/plugins/)
- [Tauri permissions](https://v2.tauri.app/security/permissions/)
