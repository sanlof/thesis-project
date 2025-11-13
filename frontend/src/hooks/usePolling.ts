import { useEffect, useRef, useState } from "react";

interface UsePollingOptions {
  enabled?: boolean;
  interval?: number;
  pauseOnInactive?: boolean;
  maxBackoffInterval?: number;
}

export function usePolling<T>(
  fetchFn: () => Promise<T>,
  options: UsePollingOptions = {}
) {
  const {
    enabled = true,
    interval = 3000,
    pauseOnInactive = true,
    maxBackoffInterval = 60000,
  } = options;

  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [isRefreshing, setIsRefreshing] = useState<boolean>(false);
  const [consecutiveErrors, setConsecutiveErrors] = useState<number>(0);

  const intervalRef = useRef<number | null>(null);
  const isFirstFetch = useRef<boolean>(true);
  const isPageVisible = useRef<boolean>(true);

  // Calculate current polling interval based on consecutive errors
  const getCurrentInterval = (): number => {
    if (consecutiveErrors === 0) {
      return interval;
    }

    // Exponential backoff: baseInterval * 2^errorCount
    const backoffInterval = interval * Math.pow(2, consecutiveErrors);

    // Cap at maxBackoffInterval
    return Math.min(backoffInterval, maxBackoffInterval);
  };

  const fetchData = async (showRefreshIndicator = false) => {
    try {
      if (showRefreshIndicator) {
        setIsRefreshing(true);
      } else {
        setLoading(true);
      }
      setError(null);

      const result = await fetchFn();
      setData(result);

      // Reset error count on successful fetch
      setConsecutiveErrors(0);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "Unknown error";
      setError(errorMessage);

      // Increment consecutive error count
      setConsecutiveErrors((prev) => prev + 1);
    } finally {
      setLoading(false);
      setIsRefreshing(false);
      isFirstFetch.current = false;
    }
  };

  const startPolling = () => {
    if (intervalRef.current !== null) {
      return;
    }

    const currentInterval = getCurrentInterval();

    intervalRef.current = globalThis.setInterval(() => {
      if (!pauseOnInactive || isPageVisible.current) {
        void fetchData(true);
      }
    }, currentInterval);
  };

  const stopPolling = () => {
    if (intervalRef.current !== null) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  };

  const restartPolling = () => {
    stopPolling();
    startPolling();
  };

  // Handle visibility changes
  useEffect(() => {
    const handleVisibilityChange = () => {
      isPageVisible.current = !document.hidden;

      if (pauseOnInactive) {
        if (document.hidden) {
          stopPolling();
        } else {
          void fetchData(true);
          restartPolling();
        }
      }
    };

    if (pauseOnInactive) {
      document.addEventListener("visibilitychange", handleVisibilityChange);
    }

    return () => {
      if (pauseOnInactive) {
        document.removeEventListener(
          "visibilitychange",
          handleVisibilityChange
        );
      }
    };
  }, [pauseOnInactive, consecutiveErrors]); // Added consecutiveErrors dependency

  // Restart polling when error count changes (for dynamic interval)
  useEffect(() => {
    if (!enabled || isFirstFetch.current) {
      return;
    }

    // Restart polling with new interval based on error count
    restartPolling();
  }, [consecutiveErrors]);

  // Main effect for initial fetch and polling setup
  useEffect(() => {
    if (!enabled) {
      stopPolling();
      return;
    }

    void fetchData(false);
    startPolling();

    return () => {
      stopPolling();
    };
  }, [enabled, interval]);

  return {
    data,
    loading,
    error,
    isRefreshing,
    consecutiveErrors,
    currentInterval: getCurrentInterval(),
    refetch: () => fetchData(true),
  };
}
