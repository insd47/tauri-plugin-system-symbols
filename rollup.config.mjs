import typescript from '@rollup/plugin-typescript'

export default {
  input: 'guest-js/index.ts',
  external: ['@tauri-apps/api/core'],
  output: [
    {
      file: 'dist-js/index.js',
      format: 'esm',
      sourcemap: true
    },
    {
      file: 'dist-js/index.cjs',
      format: 'cjs',
      exports: 'named',
      sourcemap: true
    }
  ],
  plugins: [
    typescript({
      tsconfig: './tsconfig.json'
    })
  ]
}
