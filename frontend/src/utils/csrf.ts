/**
 * CSRF Token Utility Functions
 *
 * Provides functions to extract CSRF tokens from cookies and
 * attach them to HTTP requests for protection against CSRF attacks.
 */

/**
 * Extract CSRF token from cookies
 *
 * @returns The CSRF token string or null if not found
 */
export function getCsrfToken(): string | null {
  const cookies = document.cookie.split(";");

  for (const cookie of cookies) {
    const [name, value] = cookie.trim().split("=");
    if (name === "csrf_token") {
      const token = decodeURIComponent(value);
      console.log(
        "[CSRF] Token found in cookie:",
        token.substring(0, 10) + "..."
      );
      return token;
    }
  }

  console.log("[CSRF] No token in cookies. All cookies:", document.cookie);
  return null;
}

/**
 * Ensure CSRF token is available by making a GET request if needed
 *
 * @param baseUrl - The base URL to fetch from (e.g., '/api/police' or '/api/hospital')
 * @returns Promise that resolves when token is available
 */
async function ensureCsrfToken(baseUrl: string): Promise<void> {
  const token = getCsrfToken();

  if (token) {
    console.log("[CSRF] Token already available");
    return;
  }

  console.log("[CSRF] No token found, fetching from", baseUrl);

  // Try multiple endpoints to get a CSRF token
  const endpoints = [
    `${baseUrl}/health`,
    `${baseUrl}/suspects`,
    `${baseUrl}/patients`,
  ];

  for (const endpoint of endpoints) {
    try {
      console.log(`[CSRF] Trying ${endpoint}...`);

      const response = await fetch(endpoint, {
        method: "GET",
        credentials: "include", // Important: receive cookies
        headers: {
          Accept: "application/json",
        },
      });

      console.log(`[CSRF] Response from ${endpoint}:`, response.status);
      console.log(
        `[CSRF] Response headers:`,
        Object.fromEntries(response.headers.entries())
      );

      if (!response.ok) {
        console.warn(`[CSRF] Request failed: ${response.status}`);
        continue;
      }

      // Consume the response body
      await response.text();

      // Wait for cookie to be set
      await new Promise((resolve) => setTimeout(resolve, 200));

      const newToken = getCsrfToken();
      if (newToken) {
        console.log("[CSRF] Token successfully obtained from", endpoint);
        return;
      } else {
        console.warn(`[CSRF] No token after fetching ${endpoint}`);
      }
    } catch (error) {
      console.error(`[CSRF] Failed to fetch from ${endpoint}:`, error);
    }
  }

  console.error("[CSRF] Failed to obtain token from any endpoint");
  throw new Error("Failed to obtain CSRF token");
}

/**
 * Create fetch headers with CSRF token
 *
 * @param additionalHeaders - Any additional headers to include
 * @returns Headers object with CSRF token and content type
 */
export function createCsrfHeaders(
  additionalHeaders: Record<string, string> = {}
): Record<string, string> {
  const token = getCsrfToken();

  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    ...additionalHeaders,
  };

  if (token) {
    headers["X-CSRF-Token"] = token;
  }

  return headers;
}

/**
 * Fetch wrapper that automatically includes CSRF token
 *
 * @param url - The URL to fetch
 * @param options - Fetch options (method, body, etc.)
 * @returns Promise resolving to the Response
 */
export async function fetchWithCsrf(
  url: string,
  options: RequestInit = {}
): Promise<Response> {
  const method = options.method?.toUpperCase() || "GET";

  // Only add CSRF token for state-changing methods
  if (["POST", "PUT", "DELETE", "PATCH"].includes(method)) {
    // Determine which backend this request is for
    const baseUrl = url.includes("/api/police")
      ? "/api/police"
      : url.includes("/api/hospital")
      ? "/api/hospital"
      : "/api/police"; // default

    console.log(`[CSRF] ${method} request to ${url}`);

    // Ensure we have a CSRF token before making the request
    try {
      await ensureCsrfToken(baseUrl);
    } catch (error) {
      console.error("[CSRF] Failed to ensure token:", error);
      throw new Error("Failed to obtain CSRF token - please refresh the page");
    }

    const token = getCsrfToken();

    if (!token) {
      console.error("[CSRF] Token still not available after fetch attempts");
      throw new Error(
        "CSRF token required but not available - please refresh the page"
      );
    }

    // Add CSRF token to headers
    options.headers = {
      ...options.headers,
      "X-CSRF-Token": token,
    };

    console.log(
      `[CSRF] Adding token to ${method} request:`,
      token.substring(0, 10) + "..."
    );
  }

  // Ensure credentials are included to receive/send cookies
  options.credentials = "include";

  const response = await fetch(url, options);

  // Handle CSRF validation failure
  if (response.status === 403) {
    try {
      const error = await response.json();
      if (error.code && error.code.startsWith("CSRF_")) {
        console.error("[CSRF] Validation failed:", error.error, error.code);

        // Clear the invalid token
        document.cookie =
          "csrf_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";

        // Try to refresh the token
        const baseUrl = url.includes("/api/police")
          ? "/api/police"
          : "/api/hospital";

        try {
          await ensureCsrfToken(baseUrl);
        } catch (refreshError) {
          console.error("[CSRF] Failed to refresh token:", refreshError);
        }

        // Throw error with code for caller to handle
        const csrfError = new Error(error.error) as Error & { code: string };
        csrfError.code = error.code;
        throw csrfError;
      }
    } catch (jsonError) {
      // If it's not a JSON response or not a CSRF error, just return the response
      if (!(jsonError instanceof Error && "code" in jsonError)) {
        return response;
      }
      throw jsonError;
    }
  }

  return response;
}

/**
 * Check if an error is a CSRF error
 *
 * @param error - The error to check
 * @returns True if the error is a CSRF validation error
 */
export function isCsrfError(error: unknown): boolean {
  return (
    error instanceof Error &&
    "code" in error &&
    typeof (error as any).code === "string" &&
    (error as any).code.startsWith("CSRF_")
  );
}
