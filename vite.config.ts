import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  server: {
    port: 5173,
    strictPort: true,
  },
  envPrefix: ['TAURI_'],
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    sourcemap: false,
  },
});