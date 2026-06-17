import type { Symbol } from './types';

import * as cache from './cache';
import * as commands from './commands';
import * as key from './key';
import * as text from './text';

/**
 * Resolves a Windows Segoe Fluent Icons glyph to SVG path data.
 *
 * `Segoe Fluent Icons` is preferred by the Rust backend, with
 * `Segoe MDL2 Assets` used as the fallback font. Calling this on a non-Windows
 * platform rejects with the backend platform error.
 *
 * @param icon Glyph text or codepoint, for example `"\uE8BB"` or `"U+E8BB"`.
 * @returns SVG path data and viewBox metadata for the icon.
 */
export function getFluentIcon(icon: string): Promise<Symbol> {
  text.assert(icon, 'icon');
  return cache.load(key.fluent(icon), async () => {
    const [symbol] = await commands.fluent([icon]);
    return symbol;
  });
}

/**
 * Returns a previously resolved Windows Fluent icon without starting IPC.
 *
 * @param icon Glyph text or codepoint used to resolve the icon.
 * @returns Cached SVG data, or `undefined` when the icon has not been loaded.
 */
export function getCachedFluentIcon(icon: string): Symbol | undefined {
  text.assert(icon, 'icon');
  return cache.get(key.fluent(icon));
}

/**
 * Preloads Windows Fluent icons in one backend request.
 *
 * @param icons Glyph texts or codepoints to preload.
 */
export async function preloadFluentIcons(...icons: string[]): Promise<void> {
  const unique = text.unique(icons);
  unique.forEach((icon) => text.assert(icon, 'icon'));

  const missing = unique.filter((icon) => !cache.get(key.fluent(icon)));
  if (missing.length === 0) {
    return;
  }

  const symbols = await commands.fluent(missing);
  cache.save(
    symbols,
    missing.map((icon) => key.fluent(icon)),
  );
}

/**
 * Resolves a macOS SF Symbol name to SVG path data.
 *
 * Calling this on a non-macOS platform rejects with the backend platform error.
 *
 * @param symbol SF Symbols name, for example `"square.and.arrow.up"`.
 * @returns SVG path data and viewBox metadata for the symbol.
 */
export function getSfSymbol(symbol: string): Promise<Symbol> {
  text.assert(symbol, 'symbol');
  return cache.load(key.sf(symbol), async () => {
    const [icon] = await commands.sf([symbol]);
    return icon;
  });
}

/**
 * Returns a previously resolved SF Symbol without starting IPC.
 *
 * @param symbol SF Symbols name used to resolve the symbol.
 * @returns Cached SVG data, or `undefined` when the symbol has not been loaded.
 */
export function getCachedSfSymbol(symbol: string): Symbol | undefined {
  text.assert(symbol, 'symbol');
  return cache.get(key.sf(symbol));
}

/**
 * Preloads macOS SF Symbols in one backend request.
 *
 * @param symbols SF Symbols names to preload.
 */
export async function preloadSfSymbols(...symbols: string[]): Promise<void> {
  const unique = text.unique(symbols);
  unique.forEach((symbol) => text.assert(symbol, 'symbol'));

  const missing = unique.filter((symbol) => !cache.get(key.sf(symbol)));
  if (missing.length === 0) {
    return;
  }

  const icons = await commands.sf(missing);
  cache.save(
    icons,
    missing.map((symbol) => key.sf(symbol)),
  );
}

/**
 * Clears the in-memory JavaScript symbol cache.
 *
 * This only affects frontend-side caching. The Rust backend keeps its own
 * process-local cache.
 */
export function clearSymbolCache(): void {
  cache.clear();
}

export type { Path, Symbol } from './types';
