import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import tailwindcss from '@tailwindcss/vite';
import eslint from '@nabla/vite-plugin-eslint';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

// https://vite.dev/config/
export default defineConfig({
  server: {
    fs: {
      allow: [
        // le dossier courant
        '.',
        // ton dossier wasm-api
        '../wasm-api/pkg',
      ],
    },
  },
  plugins: [
    react(),
    wasm(),
    topLevelAwait(),
    tailwindcss(),
    eslint({
      formatter: 'stylish',
      eslintOptions: {
        warnIgnored: true,
      },
    }),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
});
