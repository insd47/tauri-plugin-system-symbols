import { Path } from './types';

const resolved = new Map<string, Path[]>();
const pending = new Map<string, Promise<Path[]>>();

export function get(key: string): Path[] | undefined {
  return resolved.get(key);
}

export function load(key: string, create: () => Promise<Path[]>): Promise<Path[]> {
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
