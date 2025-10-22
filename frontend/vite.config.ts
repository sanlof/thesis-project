import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  
  // Development server configuration
  server: {
    port: 5173,
    strictPort: true,
    host: true,
    
    // Proxy configuration for backend API requests
    // This allows the frontend to make requests to /api/* which will be forwarded
    // to the appropriate backend service, avoiding CORS issues during development
    proxy: {
      // Police system API
      '/api/police': {
        target: 'http://localhost:8000',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api\/police/, ''),
        secure: false,
      },
      
      // Hospital system API
      '/api/hospital': {
        target: 'http://localhost:8001',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api\/hospital/, ''),
        secure: false,
      },
    },
  },
  
  // Build configuration
  build: {
    outDir: 'dist',
    sourcemap: true,
    // Optimize chunk size
    rollupOptions: {
      output: {
        manualChunks: {
          'react-vendor': ['react', 'react-dom'],
        },
      },
    },
  },
  
  // Path resolution
  resolve: {
    alias: {
      '@': '/src',
    },
  },
})
