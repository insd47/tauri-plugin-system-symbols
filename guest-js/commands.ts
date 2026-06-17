import { invoke } from '@tauri-apps/api/core';
import type { Symbol } from './types';

export function symbol(symbol: string, size: number): Promise<Symbol> {
  return invoke<Symbol>(`plugin:system-symbols|get_symbol`, {
    symbol,
    size,
  });
}
