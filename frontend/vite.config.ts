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

          proxy.on("proxyRes", (proxyRes, req, res) => {
            const setCookieHeaders = proxyRes.headers["set-cookie"];

            if (setCookieHeaders) {
              console.log(
                `[Police->Proxy] Original Set-Cookie:`,
                setCookieHeaders
              );

              // Aggressively rewrite cookies for maximum compatibility
              const rewrittenCookies = setCookieHeaders.map((cookie) => {
                // Start fresh - just keep name and value
                const match = cookie.match(/^([^=]+)=([^;]+)/);
                if (!match) return cookie;

                const [, name, value] = match;

                // Rebuild cookie with minimal attributes for development
                let rewritten = `${name}=${value}`;
                rewritten += "; Path=/";
                rewritten += "; HttpOnly";
                // No SameSite, no Secure, no Domain - maximum compatibility

                return rewritten;
              });

              res.setHeader("Set-Cookie", rewrittenCookies);
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
            if (req.headers.cookie) {
              proxyReq.setHeader("Cookie", req.headers.cookie);
            }
            console.log(`[Proxy->Hospital] ${req.method} ${req.url}`);
          });

          proxy.on("proxyRes", (proxyRes, req, res) => {
            const setCookieHeaders = proxyRes.headers["set-cookie"];

            if (setCookieHeaders) {
              console.log(
                `[Hospital->Proxy] Original Set-Cookie:`,
                setCookieHeaders
              );

              const rewrittenCookies = setCookieHeaders.map((cookie) => {
                // Start fresh - just keep name and value
                const match = cookie.match(/^([^=]+)=([^;]+)/);
                if (!match) return cookie;

                const [, name, value] = match;

                // Rebuild cookie with minimal attributes
                let rewritten = `${name}=${value}`;
                rewritten += "; Path=/";
                rewritten += "; HttpOnly";

                return rewritten;
              });

              res.setHeader("Set-Cookie", rewrittenCookies);
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
