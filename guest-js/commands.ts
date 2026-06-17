import { invoke } from '@tauri-apps/api/core'

import type { Symbol } from './types'

const COMMAND_PREFIX = 'plugin:system-symbols|'

export function fluent(icons: string[]): Promise<Symbol[]> {
  return invoke<Symbol[]>(`${COMMAND_PREFIX}get_fluent_icons`, {
    icons
  })
}

export function sf(symbols: string[]): Promise<Symbol[]> {
  return invoke<Symbol[]>(`${COMMAND_PREFIX}get_sf_symbols`, {
    symbols
  })
}
