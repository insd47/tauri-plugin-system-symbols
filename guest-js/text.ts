export function assert(value: string, name: string): void {
  if (value.trim().length === 0) {
    throw new Error(`${name} must not be empty`)
  }
}

export function unique(values: string[]): string[] {
  return [...new Set(values)]
}
