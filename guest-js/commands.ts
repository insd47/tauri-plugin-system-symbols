import { invoke } from '@tauri-apps/api/core';
import { Path } from './types';

export function symbol(symbol: string, size: number): Promise<Path[]> {
  return invoke<Path[]>(`plugin:system-symbols|get_symbol`, {
    symbol,
    size,
  });
}
