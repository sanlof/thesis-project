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
        changeOrigin: false, // Changed to false - keep origin as localhost:3000
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

              // Manually set cookies in the response to browser
              // This ensures cookies are properly set for localhost:3000
              const rewrittenCookies = setCookieHeaders.map((cookie) => {
                // Parse and rewrite the cookie
                let rewritten = cookie;

                // Remove any Domain attribute
                rewritten = rewritten.replace(/;\s*Domain=[^;]*/gi, "");

                // Remove Secure flag (we're on HTTP)
                rewritten = rewritten.replace(/;\s*Secure/gi, "");

                // Change SameSite to Lax or None for cross-origin
                rewritten = rewritten.replace(
                  /;\s*SameSite=[^;]*/gi,
                  "; SameSite=Lax"
                );

                // Ensure Path is set to root
                if (!rewritten.includes("Path=")) {
                  rewritten += "; Path=/";
                } else {
                  rewritten = rewritten.replace(/;\s*Path=[^;]*/gi, "; Path=/");
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
        changeOrigin: false, // Changed to false - keep origin as localhost:3000
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

              // Manually set cookies in the response to browser
              const rewrittenCookies = setCookieHeaders.map((cookie) => {
                // Parse and rewrite the cookie
                let rewritten = cookie;

                // Remove any Domain attribute
                rewritten = rewritten.replace(/;\s*Domain=[^;]*/gi, "");

                // Remove Secure flag (we're on HTTP)
                rewritten = rewritten.replace(/;\s*Secure/gi, "");

                // Change SameSite to Lax
                rewritten = rewritten.replace(
                  /;\s*SameSite=[^;]*/gi,
                  "; SameSite=Lax"
                );

                // Ensure Path is set to root
                if (!rewritten.includes("Path=")) {
                  rewritten += "; Path=/";
                } else {
                  rewritten = rewritten.replace(/;\s*Path=[^;]*/gi, "; Path=/");
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
