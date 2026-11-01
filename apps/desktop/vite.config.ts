import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import tailwind from '@tailwindcss/vite'
export default defineConfig(({ mode }) => ({
  plugins: [react(), tailwind()],
  clearScreen: false,
  define: { __TEST_BACKEND__: JSON.stringify(mode === 'test') },
  server: { host: '127.0.0.1', port: 1420, strictPort: true },
  build: { target: 'es2022' },
  test: {
    environment: 'jsdom',
    setupFiles: ['./test/setup.ts'],
    include: ['test/**/*.test.tsx'],
    globals: true,
    testTimeout: 15000,
  },
}))
