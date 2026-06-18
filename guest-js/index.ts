import type { Path } from './types';
import * as cache from './cache';
import * as commands from './commands';
import * as dimensions from './size';
import * as text from './text';

/**
 * Resolves one platform system symbol to SVG path data.
 *
 * On Windows, the value is resolved as a Segoe Fluent Icons glyph or
 * codepoint. On macOS, the value is resolved as a copied SF Symbol character.
 *
 * @param symbol Platform-specific symbol character, glyph, or codepoint.
 * @param size Target icon size in CSS pixels.
 * @returns SVG path data for the requested symbol.
 */
export function getSymbol(symbol: string, size: number): Promise<Path[]> {
  text.assert(symbol, 'symbol');
  dimensions.assert(size);

  return cache.load(dimensions.key(symbol, size), () => commands.symbol(symbol, size));
}

/**
 * Returns a previously resolved platform symbol without starting IPC.
 *
 * @param symbol Platform-specific symbol character, glyph, or codepoint.
 * @param size Target icon size in CSS pixels.
 * @returns Cached SVG path data, or `undefined` when the symbol has not been loaded.
 */
export function getCachedSymbol(symbol: string, size: number): Path[] | undefined {
  text.assert(symbol, 'symbol');
  dimensions.assert(size);
  return cache.get(dimensions.key(symbol, size));
}

/**
 * Clears the in-memory JavaScript symbol cache.
 */
export function clearSymbolCache(): void {
  cache.clear();
}

export type { Path } from './types';
