import { useEffect, useRef, useState } from "react";

interface UsePollingOptions {
  enabled?: boolean;
  interval?: number;
  pauseOnInactive?: boolean;
}

export function usePolling<T>(
  fetchFn: () => Promise<T>,
  options: UsePollingOptions = {}
) {
  const { enabled = true, interval = 3000, pauseOnInactive = true } = options;

  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [isRefreshing, setIsRefreshing] = useState<boolean>(false);

  const intervalRef = useRef<number | null>(null);
  const isFirstFetch = useRef<boolean>(true);
  const isPageVisible = useRef<boolean>(true);

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
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "Unknown error";
      setError(errorMessage);
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

    intervalRef.current = globalThis.setInterval(() => {
      if (!pauseOnInactive || isPageVisible.current) {
        void fetchData(true);
      }
    }, interval);
  };

  const stopPolling = () => {
    if (intervalRef.current !== null) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  };

  useEffect(() => {
    const handleVisibilityChange = () => {
      isPageVisible.current = !document.hidden;

      if (pauseOnInactive) {
        if (document.hidden) {
          stopPolling();
        } else {
          void fetchData(true);
          startPolling();
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
  }, [pauseOnInactive]);

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
    refetch: () => fetchData(true),
  };
}
