import type { Symbol } from './types';

const resolved = new Map<string, Symbol>();
const pending = new Map<string, Promise<Symbol>>();

export function get(key: string): Symbol | undefined {
  return resolved.get(key);
}

export function load(key: string, create: () => Promise<Symbol>): Promise<Symbol> {
  const cached = resolved.get(key);
  if (cached) return Promise.resolve(cached);

  const inflight = pending.get(key);
  if (inflight) return inflight;

  const request = create().then(
    (symbol) => {
      pending.delete(key);
      resolved.set(key, symbol);
      return symbol;
    },
    (error) => {
      pending.delete(key);
      throw error;
    },
  );

  pending.set(key, request);
  return request;
}

export function clear(): void {
  resolved.clear();
  pending.clear();
}
