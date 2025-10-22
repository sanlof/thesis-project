/**
 * Base API Service Configuration
 *
 * This module provides the core Axios configuration and utilities used by both
 * the police and hospital API services. It includes error handling, request/response
 * interceptors, and environment-based configuration.
 *
 * @module services/api
 */

import axios, {
  AxiosInstance,
  AxiosError,
  AxiosRequestConfig,
  AxiosResponse,
} from "axios";
import type { ApiError } from "../types";

// ============================================================================
// Environment Variables and Configuration
// ============================================================================

/**
 * Police system API base URL
 *
 * In development, this typically points to the Vite proxy (/api/police)
 * which forwards requests to http://localhost:8000
 *
 * In production, this should be the full URL of the police backend service
 */
export const POLICE_API_URL =
  import.meta.env.VITE_POLICE_API_URL || "http://localhost:8000";

/**
 * Hospital system API base URL
 *
 * In development, this typically points to the Vite proxy (/api/hospital)
 * which forwards requests to http://localhost:8001
 *
 * In production, this should be the full URL of the hospital backend service
 */
export const HOSPITAL_API_URL =
  import.meta.env.VITE_HOSPITAL_API_URL || "http://localhost:8001";

/**
 * Polling interval for data refresh (in milliseconds)
 *
 * Used by components that need to periodically fetch updated data
 * Default: 5000ms (5 seconds)
 */
export const POLL_INTERVAL = parseInt(
  import.meta.env.VITE_POLL_INTERVAL || "5000",
  10
);

/**
 * API request timeout (in milliseconds)
 *
 * Requests that take longer than this will be aborted
 * Default: 10000ms (10 seconds)
 */
export const API_TIMEOUT = parseInt(
  import.meta.env.VITE_API_TIMEOUT || "10000",
  10
);

/**
 * Debug mode flag
 *
 * When enabled, logs detailed information about API requests and responses
 * Should be disabled in production
 */
export const DEBUG_MODE =
  import.meta.env.VITE_DEBUG_MODE === "true" || import.meta.env.DEV;

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Standardizes API errors into a consistent format
 *
 * This function handles various error scenarios:
 * - Network errors (no response from server)
 * - HTTP errors (4xx, 5xx status codes)
 * - Timeout errors
 * - Request setup errors
 * - Unknown errors
 *
 * @param error - The error object from Axios or other sources
 * @returns Standardized ApiError object
 *
 * @example
 * try {
 *   const response = await api.get('/suspects/999');
 * } catch (error) {
 *   const apiError = handleApiError(error);
 *   console.error(apiError.error); // "Suspect not found"
 * }
 */
export function handleApiError(error: unknown): ApiError {
  // Default error message
  const defaultError: ApiError = {
    error: "An unexpected error occurred. Please try again.",
  };

  // Check if this is an Axios error
  if (!axios.isAxiosError(error)) {
    // Not an Axios error, might be a JavaScript error
    if (error instanceof Error) {
      return { error: error.message };
    }
    return defaultError;
  }

  const axiosError = error as AxiosError<ApiError>;

  // Case 1: Response received but with error status (4xx, 5xx)
  if (axiosError.response) {
    const { status, data } = axiosError.response;

    if (DEBUG_MODE) {
      console.error("API Error Response:", {
        status,
        data,
        url: axiosError.config?.url,
      });
    }

    // Backend returned an error response with our ApiError structure
    if (data && typeof data === "object" && "error" in data) {
      return { error: data.error };
    }

    // Backend returned an error but not in our expected format
    switch (status) {
      case 400:
        return { error: "Invalid request. Please check your input." };
      case 401:
        return { error: "Unauthorized. Please log in." };
      case 403:
        return {
          error:
            "Forbidden. You do not have permission to access this resource.",
        };
      case 404:
        return { error: "Resource not found." };
      case 409:
        return { error: "Conflict. This resource already exists." };
      case 500:
        return { error: "Internal server error. Please try again later." };
      case 502:
        return { error: "Bad gateway. The server is temporarily unavailable." };
      case 503:
        return { error: "Service unavailable. Please try again later." };
      default:
        return { error: `Request failed with status ${status}` };
    }
  }

  // Case 2: Request was made but no response received (network error)
  if (axiosError.request) {
    if (DEBUG_MODE) {
      console.error("Network Error:", {
        message: axiosError.message,
        url: axiosError.config?.url,
      });
    }

    // Check if it's a timeout
    if (axiosError.code === "ECONNABORTED") {
      return {
        error: "Request timeout. The server took too long to respond.",
      };
    }

    // Check for network errors
    if (axiosError.message.includes("Network Error")) {
      return {
        error: "Network error. Please check your internet connection.",
      };
    }

    return {
      error:
        "Unable to reach the server. Please check if the backend is running.",
    };
  }

  // Case 3: Error setting up the request
  if (DEBUG_MODE) {
    console.error("Request Setup Error:", axiosError.message);
  }

  return {
    error: `Request setup error: ${axiosError.message}`,
  };
}

/**
 * Constructs a full URL by combining base URL and path
 *
 * Handles trailing slashes and leading slashes properly to avoid
 * malformed URLs like "http://localhost:8000//suspects"
 *
 * @param base - Base URL (e.g., "http://localhost:8000")
 * @param path - API path (e.g., "/suspects" or "suspects")
 * @returns Full URL with proper formatting
 *
 * @example
 * buildUrl("http://localhost:8000", "/suspects")
 * // Returns: "http://localhost:8000/suspects"
 *
 * buildUrl("http://localhost:8000/", "suspects")
 * // Returns: "http://localhost:8000/suspects"
 */
export function buildUrl(base: string, path: string): string {
  // Remove trailing slash from base
  const cleanBase = base.endsWith("/") ? base.slice(0, -1) : base;

  // Ensure path starts with slash
  const cleanPath = path.startsWith("/") ? path : `/${path}`;

  return `${cleanBase}${cleanPath}`;
}

/**
 * Formats a URL with query parameters
 *
 * @param url - Base URL
 * @param params - Query parameters as key-value pairs
 * @returns URL with appended query string
 *
 * @example
 * formatUrlWithParams("/suspects", { flag: "true", limit: "10" })
 * // Returns: "/suspects?flag=true&limit=10"
 */
export function formatUrlWithParams(
  url: string,
  params?: Record<string, string | number | boolean>
): string {
  if (!params || Object.keys(params).length === 0) {
    return url;
  }

  const queryString = Object.entries(params)
    .map(
      ([key, value]) =>
        `${encodeURIComponent(key)}=${encodeURIComponent(value)}`
    )
    .join("&");

  return `${url}?${queryString}`;
}

// ============================================================================
// Axios Instance Creation
// ============================================================================

/**
 * Creates a configured Axios instance
 *
 * This factory function creates an Axios instance with all necessary configuration:
 * - Default headers
 * - Timeout settings
 * - Request interceptors for logging
 * - Response interceptors for error handling
 *
 * @param baseURL - The base URL for this instance (police or hospital API)
 * @returns Configured Axios instance
 */
function createApiInstance(baseURL: string): AxiosInstance {
  // Create Axios instance with base configuration
  const instance = axios.create({
    baseURL,
    timeout: API_TIMEOUT,
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
    // Include credentials if needed for CORS (future authentication)
    withCredentials: false,
  });

  // ============================================================================
  // Request Interceptor
  // ============================================================================

  /**
   * Request interceptor for debugging and request modification
   *
   * In development mode, this logs all outgoing requests for debugging.
   * In production, this can be used to add authentication tokens, etc.
   */
  instance.interceptors.request.use(
    (config: AxiosRequestConfig) => {
      // Log request in debug mode
      if (DEBUG_MODE) {
        console.log("🚀 API Request:", {
          method: config.method?.toUpperCase(),
          url: config.url,
          baseURL: config.baseURL,
          data: config.data,
          params: config.params,
        });
      }

      // Future: Add authentication token here
      // const token = getAuthToken();
      // if (token && config.headers) {
      //   config.headers.Authorization = `Bearer ${token}`;
      // }

      return config;
    },
    (error: AxiosError) => {
      // Request setup failed
      if (DEBUG_MODE) {
        console.error("❌ Request Error:", error);
      }
      return Promise.reject(error);
    }
  );

  // ============================================================================
  // Response Interceptor
  // ============================================================================

  /**
   * Response interceptor for data extraction and error handling
   *
   * Success case:
   * - Extracts data from response.data, making API calls cleaner
   * - Logs response in debug mode
   *
   * Error case:
   * - Standardizes errors using handleApiError()
   * - Logs errors in debug mode
   * - Always rejects with a properly formatted ApiError
   */
  instance.interceptors.response.use(
    (response: AxiosResponse) => {
      // Log successful response in debug mode
      if (DEBUG_MODE) {
        console.log("✅ API Response:", {
          status: response.status,
          url: response.config.url,
          data: response.data,
        });
      }

      // Return just the data, not the full Axios response
      // This allows: const suspects = await api.get('/suspects')
      // Instead of: const suspects = (await api.get('/suspects')).data
      return response.data;
    },
    (error: AxiosError) => {
      // Log error in debug mode
      if (DEBUG_MODE) {
        console.error("❌ API Response Error:", {
          message: error.message,
          status: error.response?.status,
          url: error.config?.url,
          data: error.response?.data,
        });
      }

      // Standardize the error and reject
      const apiError = handleApiError(error);
      return Promise.reject(apiError);
    }
  );

  return instance;
}

// ============================================================================
// Pre-configured Axios Instances
// ============================================================================

/**
 * Axios instance for Police System API
 *
 * Pre-configured with the police API base URL and all interceptors.
 * Use this for all police-related API calls.
 *
 * @example
 * import { policeApi } from './api';
 *
 * const suspects = await policeApi.get('/suspects');
 * const newSuspect = await policeApi.post('/suspects', data);
 */
export const policeApi = createApiInstance(POLICE_API_URL);

/**
 * Axios instance for Hospital System API
 *
 * Pre-configured with the hospital API base URL and all interceptors.
 * Use this for all hospital-related API calls.
 *
 * @example
 * import { hospitalApi } from './api';
 *
 * const patients = await hospitalApi.get('/patients');
 * const newPatient = await hospitalApi.post('/patients', data);
 */
export const hospitalApi = createApiInstance(HOSPITAL_API_URL);

// ============================================================================
// Generic API Instance (optional - for flexibility)
// ============================================================================

/**
 * Generic Axios instance without a predefined base URL
 *
 * Use this for one-off requests to arbitrary URLs or when you need
 * more control over the request configuration.
 *
 * @example
 * import { api } from './api';
 *
 * const data = await api.get('https://external-api.com/data');
 */
export const api = createApiInstance("");

// ============================================================================
// Utility Functions for Common Patterns
// ============================================================================

/**
 * Retries a failed API request
 *
 * Useful for handling transient network errors or temporary server issues.
 *
 * @param fn - Async function that makes the API call
 * @param retries - Number of retry attempts (default: 3)
 * @param delay - Delay between retries in ms (default: 1000)
 * @returns Result of the API call
 *
 * @example
 * const suspects = await retryRequest(
 *   () => policeApi.get('/suspects'),
 *   3,
 *   1000
 * );
 */
export async function retryRequest<T>(
  fn: () => Promise<T>,
  retries: number = 3,
  delay: number = 1000
): Promise<T> {
  let lastError: ApiError | null = null;

  for (let i = 0; i < retries; i++) {
    try {
      return await fn();
    } catch (error) {
      lastError = handleApiError(error);

      if (DEBUG_MODE) {
        console.warn(
          `Retry attempt ${i + 1}/${retries} failed:`,
          lastError.error
        );
      }

      // Don't retry on client errors (4xx) - only on server/network errors
      if (error && typeof error === "object" && "response" in error) {
        const axiosError = error as AxiosError;
        const status = axiosError.response?.status;
        if (status && status >= 400 && status < 500) {
          // Client error - don't retry
          throw lastError;
        }
      }

      // Wait before retrying (except on last attempt)
      if (i < retries - 1) {
        await new Promise((resolve) => setTimeout(resolve, delay));
      }
    }
  }

  // All retries failed
  throw lastError || { error: "All retry attempts failed" };
}

/**
 * Checks if an API is healthy
 *
 * Makes a GET request to the /health endpoint to verify the service is running.
 *
 * @param apiInstance - Axios instance (policeApi or hospitalApi)
 * @returns True if healthy, false otherwise
 *
 * @example
 * const policeHealthy = await checkApiHealth(policeApi);
 * const hospitalHealthy = await checkApiHealth(hospitalApi);
 */
export async function checkApiHealth(
  apiInstance: AxiosInstance
): Promise<boolean> {
  try {
    const response = await apiInstance.get("/health");
    return response && response.status === "healthy";
  } catch (error) {
    if (DEBUG_MODE) {
      console.error("Health check failed:", handleApiError(error));
    }
    return false;
  }
}

// ============================================================================
// Type Exports for External Use
// ============================================================================

/**
 * Re-export Axios types for convenience
 *
 * This allows other modules to import common Axios types from this module
 * without needing to import from axios directly.
 */
export type {
  AxiosInstance,
  AxiosError,
  AxiosRequestConfig,
  AxiosResponse,
} from "axios";

// ============================================================================
// Summary of Exports
// ============================================================================

/**
 * This module exports:
 *
 * Constants:
 * - POLICE_API_URL - Police backend URL
 * - HOSPITAL_API_URL - Hospital backend URL
 * - POLL_INTERVAL - Data refresh interval
 * - API_TIMEOUT - Request timeout
 * - DEBUG_MODE - Debug logging flag
 *
 * Axios Instances:
 * - policeApi - Pre-configured for police system
 * - hospitalApi - Pre-configured for hospital system
 * - api - Generic instance
 *
 * Helper Functions:
 * - handleApiError() - Standardize errors
 * - buildUrl() - Construct URLs
 * - formatUrlWithParams() - Add query parameters
 * - retryRequest() - Retry failed requests
 * - checkApiHealth() - Verify service health
 *
 * Types:
 * - AxiosInstance, AxiosError, AxiosRequestConfig, AxiosResponse
 */
