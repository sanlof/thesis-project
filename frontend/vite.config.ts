import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    port: 3000,
    proxy: {
      "/api/police": {
        target: "http://localhost:8000",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api\/police/, ""),
        // Important: preserve cookies for CSRF tokens
        configure: (proxy, _options) => {
          proxy.on("proxyReq", (proxyReq, req, _res) => {
            // Forward cookies from browser to backend
            if (req.headers.cookie) {
              proxyReq.setHeader("Cookie", req.headers.cookie);
            }
          });
          proxy.on("proxyRes", (proxyRes, _req, _res) => {
            // Forward Set-Cookie headers from backend to browser
            const setCookieHeaders = proxyRes.headers["set-cookie"];
            if (setCookieHeaders) {
              proxyRes.headers["set-cookie"] = setCookieHeaders;
            }
          });
        },
      },
      "/api/hospital": {
        target: "http://localhost:8001",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api\/hospital/, ""),
        // Important: preserve cookies for CSRF tokens
        configure: (proxy, _options) => {
          proxy.on("proxyReq", (proxyReq, req, _res) => {
            // Forward cookies from browser to backend
            if (req.headers.cookie) {
              proxyReq.setHeader("Cookie", req.headers.cookie);
            }
          });
          proxy.on("proxyRes", (proxyRes, _req, _res) => {
            // Forward Set-Cookie headers from backend to browser
            const setCookieHeaders = proxyRes.headers["set-cookie"];
            if (setCookieHeaders) {
              proxyRes.headers["set-cookie"] = setCookieHeaders;
            }
          });
        },
      },
    },
  },
});
