import { invoke } from '@tauri-apps/api/core'

const COMMAND_PREFIX = 'plugin:system-symbols|'

export type SymbolFamily =
  | 'auto'
  | 'sfSymbols'
  | 'segoeFluentIcons'
  | 'segoeMdl2Assets'

export interface SymbolRequest {
  family?: SymbolFamily
  symbol: string
}

export interface SvgSymbol {
  family: SymbolFamily
  symbol: string
  path: string
  viewBox: string
}

const cache = new Map<string, Promise<SvgSymbol>>()

export async function getSymbol(request: SymbolRequest): Promise<SvgSymbol> {
  const normalized = normalizeRequest(request)
  const key = cacheKey(normalized)
  const cached = cache.get(key)
  if (cached) {
    return cached
  }

  const pending = invoke<SvgSymbol>(`${COMMAND_PREFIX}get_symbol`, {
    request: normalized
  }).catch((error) => {
    cache.delete(key)
    throw error
  })

  cache.set(key, pending)
  return pending
}

export async function getSymbols(
  requests: SymbolRequest[]
): Promise<SvgSymbol[]> {
  const normalized = requests.map(normalizeRequest)
  const missing: Required<SymbolRequest>[] = []

  for (const request of normalized) {
    if (!cache.has(cacheKey(request))) {
      missing.push(request)
    }
  }

  if (missing.length > 0) {
    const uniqueMissing = uniqueRequests(missing)
    const pending = invoke<SvgSymbol[]>(`${COMMAND_PREFIX}get_symbols`, {
      requests: uniqueMissing
    })

    uniqueMissing.forEach((request, index) => {
      const key = cacheKey(request)
      cache.set(
        key,
        pending.then((symbols) => symbols[index])
      )
    })

    try {
      await pending
    } catch (error) {
      for (const request of uniqueMissing) {
        cache.delete(cacheKey(request))
      }
      throw error
    }
  }

  return Promise.all(normalized.map((request) => getSymbol(request)))
}

export function clearSymbolCache(): void {
  cache.clear()
}

function normalizeRequest(request: SymbolRequest): Required<SymbolRequest> {
  if (request.symbol.trim().length === 0) {
    throw new Error('symbol must not be empty')
  }

  return {
    family: request.family ?? 'auto',
    symbol: request.symbol
  }
}

function uniqueRequests(
  requests: Required<SymbolRequest>[]
): Required<SymbolRequest>[] {
  const seen = new Set<string>()
  const unique: Required<SymbolRequest>[] = []

  for (const request of requests) {
    const key = cacheKey(request)
    if (seen.has(key)) {
      continue
    }

    seen.add(key)
    unique.push(request)
  }

  return unique
}

function cacheKey(request: Required<SymbolRequest>): string {
  return `${request.family}:${request.symbol}`
}
