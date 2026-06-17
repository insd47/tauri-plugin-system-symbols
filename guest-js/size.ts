export function assert(value: number): void {
  if (!Number.isFinite(value) || value <= 0) {
    throw new Error('size must be a positive finite number')
  }
}

export function key(symbol: string, value: number): string {
  return `${value}:${symbol}`
}
