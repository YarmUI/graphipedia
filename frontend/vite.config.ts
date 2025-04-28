import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    manifest: true, // ← これを追加
  },
  server: {
    proxy: {
      '/api/search': {
        target: 'http://127.0.0.1:3000',
        changeOrigin: true,
      },
      '/api/graph_search': {
        target: 'http://127.0.0.1:3000',
        changeOrigin: true,
      }
    }
  }
})
