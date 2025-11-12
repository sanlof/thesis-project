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
      return decodeURIComponent(value);
    }
  }

  return null;
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
    const token = getCsrfToken();

    if (!token) {
      // Try to get a CSRF token by making a GET request first
      console.warn("No CSRF token found, attempting to fetch one...");

      // Extract base URL (without query params)
      const baseUrl = url.split("?")[0];
      const tokenUrl = baseUrl.includes("/api/police")
        ? "/api/police/health"
        : "/api/hospital/health";

      try {
        await fetch(tokenUrl, {
          method: "GET",
          credentials: "include",
        });

        // Try to get token again after GET request
        const newToken = getCsrfToken();
        if (newToken) {
          options.headers = {
            ...options.headers,
            "X-CSRF-Token": newToken,
          };
        }
      } catch (error) {
        console.error("Failed to fetch CSRF token:", error);
        throw new Error("CSRF token required but not available");
      }
    } else {
      options.headers = {
        ...options.headers,
        "X-CSRF-Token": token,
      };
    }
  }

  // Ensure credentials are included to receive/send cookies
  options.credentials = "include";

  const response = await fetch(url, options);

  // Handle CSRF validation failure
  if (response.status === 403) {
    try {
      const error = await response.json();
      if (error.code && error.code.startsWith("CSRF_")) {
        console.error("CSRF validation failed:", error.error);

        // Try to refresh the token by making a GET request
        const baseUrl = url.split("?")[0];
        const tokenUrl = baseUrl.includes("/api/police")
          ? "/api/police/health"
          : "/api/hospital/health";

        await fetch(tokenUrl, {
          method: "GET",
          credentials: "include",
        });

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
