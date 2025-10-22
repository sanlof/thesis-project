/**
 * src/hooks/usePoliceData.ts
 * ---------------------------------------------------------
 * Custom React hook for managing Police System data.
 *
 * Handles fetching, CRUD operations, and flag updates
 * for suspects, including automatic polling and error handling.
 *
 * Uses: src/services/policeApi.ts and src/services/api.ts
 */

import { useState, useEffect, useCallback } from "react";
import {
  getAllSuspects,
  createSuspect as createSuspectApi,
  updateSuspect as updateSuspectApi,
  deleteSuspect as deleteSuspectApi,
  updateSuspectFlag as updateFlagApi,
} from "../services/policeApi";
import { POLL_INTERVAL } from "../services/api";
import type {
  Suspect,
  CreateSuspect,
  UpdateSuspect,
  ApiError,
} from "../services/types";

/**
 * Hook for managing suspects from the Police System.
 *
 * Provides:
 * - suspects: Suspect[]
 * - loading, error states
 * - CRUD operations
 * - Automatic polling refresh
 */
export function usePoliceData() {
  const [suspects, setSuspects] = useState<Suspect[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * Fetch all suspects from the backend.
   */
  const fetchSuspects = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await getAllSuspects();
      setSuspects(data);
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.error || "Failed to load suspects.");
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Create a new suspect.
   */
  const createSuspect = useCallback(
    async (data: CreateSuspect) => {
      try {
        setLoading(true);
        await createSuspectApi(data);
        await fetchSuspects(); // Refresh data
      } catch (err) {
        const apiError = err as ApiError;
        setError(apiError.error || "Failed to create suspect.");
      } finally {
        setLoading(false);
      }
    },
    [fetchSuspects]
  );

  /**
   * Update an existing suspect by ID.
   */
  const updateSuspect = useCallback(
    async (id: number, data: UpdateSuspect) => {
      try {
        setLoading(true);
        await updateSuspectApi(id, data);
        await fetchSuspects(); // Refresh data
      } catch (err) {
        const apiError = err as ApiError;
        setError(apiError.error || "Failed to update suspect.");
      } finally {
        setLoading(false);
      }
    },
    [fetchSuspects]
  );

  /**
   * Delete a suspect by ID.
   */
  const deleteSuspect = useCallback(
    async (id: number) => {
      try {
        setLoading(true);
        await deleteSuspectApi(id);
        await fetchSuspects(); // Refresh data
      } catch (err) {
        const apiError = err as ApiError;
        setError(apiError.error || "Failed to delete suspect.");
      } finally {
        setLoading(false);
      }
    },
    [fetchSuspects]
  );

  /**
   * Update a suspect's flag by personal ID.
   */
  const updateFlag = useCallback(
    async (personalId: string, flag: boolean) => {
      try {
        setLoading(true);
        await updateFlagApi(personalId, flag);
        await fetchSuspects(); // Refresh data
      } catch (err) {
        const apiError = err as ApiError;
        setError(apiError.error || "Failed to update flag status.");
      } finally {
        setLoading(false);
      }
    },
    [fetchSuspects]
  );

  /**
   * Automatically poll suspects at defined intervals.
   * Cleans up on unmount to prevent memory leaks.
   */
  useEffect(() => {
    fetchSuspects();
    const interval = setInterval(fetchSuspects, POLL_INTERVAL);
    return () => clearInterval(interval);
  }, [fetchSuspects]);

  return {
    suspects,
    loading,
    error,
    refetch: fetchSuspects,
    createSuspect,
    updateSuspect,
    deleteSuspect,
    updateFlag,
  };
}
