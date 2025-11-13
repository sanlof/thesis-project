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
        configure: (proxy, _options) => {
          proxy.on("proxyReq", (proxyReq, req, _res) => {
            // Forward all cookies from browser to backend
            if (req.headers.cookie) {
              proxyReq.setHeader("Cookie", req.headers.cookie);
            }
            console.log(`[Proxy->Police] ${req.method} ${req.url}`);
          });

          proxy.on("proxyRes", (proxyRes, _req, _res) => {
            // Forward and rewrite Set-Cookie headers from backend to browser
            const setCookieHeaders = proxyRes.headers["set-cookie"];
            if (setCookieHeaders) {
              console.log(
                `[Police->Proxy] Original Set-Cookie:`,
                setCookieHeaders
              );

              // Rewrite cookies to work with Vite dev server
              const rewrittenCookies = setCookieHeaders.map((cookie) => {
                // Remove Domain attribute (let browser use current domain)
                let rewritten = cookie.replace(/;\s*Domain=[^;]+/gi, "");
                // Ensure path is set to root
                if (!rewritten.includes("Path=")) {
                  rewritten += "; Path=/";
                }
                // Remove Secure flag if present (we're on HTTP in dev)
                rewritten = rewritten.replace(/;\s*Secure/gi, "");
                return rewritten;
              });

              proxyRes.headers["set-cookie"] = rewrittenCookies;
              console.log(
                `[Police->Proxy] Rewritten Set-Cookie:`,
                rewrittenCookies
              );
            }
          });
        },
      },
      "/api/hospital": {
        target: "http://localhost:8001",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api\/hospital/, ""),
        configure: (proxy, _options) => {
          proxy.on("proxyReq", (proxyReq, req, _res) => {
            // Forward all cookies from browser to backend
            if (req.headers.cookie) {
              proxyReq.setHeader("Cookie", req.headers.cookie);
            }
            console.log(`[Proxy->Hospital] ${req.method} ${req.url}`);
          });

          proxy.on("proxyRes", (proxyRes, _req, _res) => {
            // Forward and rewrite Set-Cookie headers from backend to browser
            const setCookieHeaders = proxyRes.headers["set-cookie"];
            if (setCookieHeaders) {
              console.log(
                `[Hospital->Proxy] Original Set-Cookie:`,
                setCookieHeaders
              );

              // Rewrite cookies to work with Vite dev server
              const rewrittenCookies = setCookieHeaders.map((cookie) => {
                // Remove Domain attribute (let browser use current domain)
                let rewritten = cookie.replace(/;\s*Domain=[^;]+/gi, "");
                // Ensure path is set to root
                if (!rewritten.includes("Path=")) {
                  rewritten += "; Path=/";
                }
                // Remove Secure flag if present (we're on HTTP in dev)
                rewritten = rewritten.replace(/;\s*Secure/gi, "");
                return rewritten;
              });

              proxyRes.headers["set-cookie"] = rewrittenCookies;
              console.log(
                `[Hospital->Proxy] Rewritten Set-Cookie:`,
                rewrittenCookies
              );
            }
          });
        },
      },
    },
  },
});
