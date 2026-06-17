export interface Path {
  d: string;
  fillRule?: 'nonzero' | 'evenodd';
  opacity?: number;
}

export interface Symbol {
  viewBox: string;
  paths: Path[];
}
