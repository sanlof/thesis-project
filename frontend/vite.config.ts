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
            if (req.headers.cookie) {
              console.log(
                `[Proxy->Police] Forwarding cookies:`,
                req.headers.cookie
              );
            }
          });

          proxy.on("proxyRes", (proxyRes, req, res) => {
            const setCookieHeaders = proxyRes.headers["set-cookie"];

            if (setCookieHeaders) {
              console.log(
                `[Police->Proxy] Original Set-Cookie:`,
                setCookieHeaders
              );

              // Rewrite cookies to work with HTTP proxy
              const rewrittenCookies = setCookieHeaders.map((cookie) => {
                let rewritten = cookie;

                // Remove Domain attribute
                rewritten = rewritten.replace(/;\s*Domain=[^;]*/gi, "");

                // Remove Secure flag (HTTP mode)
                rewritten = rewritten.replace(/;\s*Secure\b/gi, "");

                // Change SameSite to None for proxy compatibility
                // Without Secure flag, SameSite=None won't work in some browsers,
                // so we just remove SameSite entirely for HTTP development
                rewritten = rewritten.replace(/;\s*SameSite=[^;]*/gi, "");

                // Ensure Path is set to root
                if (!rewritten.includes("Path=")) {
                  rewritten += "; Path=/";
                }

                return rewritten;
              });

              // Set the rewritten cookies
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
            // Forward all cookies from browser to backend
            if (req.headers.cookie) {
              proxyReq.setHeader("Cookie", req.headers.cookie);
            }
            console.log(`[Proxy->Hospital] ${req.method} ${req.url}`);
            if (req.headers.cookie) {
              console.log(
                `[Proxy->Hospital] Forwarding cookies:`,
                req.headers.cookie
              );
            }
          });

          proxy.on("proxyRes", (proxyRes, req, res) => {
            const setCookieHeaders = proxyRes.headers["set-cookie"];

            if (setCookieHeaders) {
              console.log(
                `[Hospital->Proxy] Original Set-Cookie:`,
                setCookieHeaders
              );

              // Rewrite cookies to work with HTTP proxy
              const rewrittenCookies = setCookieHeaders.map((cookie) => {
                let rewritten = cookie;

                // Remove Domain attribute
                rewritten = rewritten.replace(/;\s*Domain=[^;]*/gi, "");

                // Remove Secure flag (HTTP mode)
                rewritten = rewritten.replace(/;\s*Secure\b/gi, "");

                // Remove SameSite entirely for HTTP development
                rewritten = rewritten.replace(/;\s*SameSite=[^;]*/gi, "");

                // Ensure Path is set to root
                if (!rewritten.includes("Path=")) {
                  rewritten += "; Path=/";
                }

                return rewritten;
              });

              // Set the rewritten cookies
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
