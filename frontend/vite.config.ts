import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api/search': {
        target: 'http://127.0.0.1:3000',
        changeOrigin: true,
      },
      '/api/search_graph': {
        target: 'http://127.0.0.1:3000',
        changeOrigin: true,
      }
    }
  }
})
