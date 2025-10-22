/**
 * src/hooks/useHospitalData.ts
 * ---------------------------------------------------------
 * Custom React hook for managing Hospital System data.
 *
 * Handles fetching, CRUD operations, and flagged patient
 * monitoring with automatic polling and error handling.
 *
 * Uses: src/services/hospitalApi.ts and src/services/api.ts
 */

import { useState, useEffect, useCallback } from "react";
import {
  getAllPatients,
  getFlaggedPatients,
  createPatient as createPatientApi,
  updatePatient as updatePatientApi,
  deletePatient as deletePatientApi,
} from "../services/hospitalApi";
import { POLL_INTERVAL } from "../services/api";
import type {
  Patient,
  CreatePatient,
  UpdatePatient,
  ApiError,
} from "../services/types";

/**
 * Hook for managing hospital system data, including
 * flagged patients tracked separately for monitoring sync.
 */
export function useHospitalData() {
  const [patients, setPatients] = useState<Patient[]>([]);
  const [flaggedPatients, setFlaggedPatients] = useState<Patient[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [flaggedLoading, setFlaggedLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * Fetch all patients.
   */
  const fetchPatients = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await getAllPatients();
      setPatients(data);
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.error || "Failed to load patients.");
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Fetch all flagged patients.
   */
  const fetchFlaggedPatients = useCallback(async () => {
    try {
      setFlaggedLoading(true);
      const data = await getFlaggedPatients();
      setFlaggedPatients(data);
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.error || "Failed to load flagged patients.");
    } finally {
      setFlaggedLoading(false);
    }
  }, []);

  /**
   * Create a new patient record.
   */
  const createPatient = useCallback(
    async (data: CreatePatient) => {
      try {
        setLoading(true);
        await createPatientApi(data);
        await fetchPatients();
        await fetchFlaggedPatients();
      } catch (err) {
        const apiError = err as ApiError;
        setError(apiError.error || "Failed to create patient.");
      } finally {
        setLoading(false);
      }
    },
    [fetchPatients, fetchFlaggedPatients]
  );

  /**
   * Update an existing patient by ID.
   */
  const updatePatient = useCallback(
    async (id: number, data: UpdatePatient) => {
      try {
        setLoading(true);
        await updatePatientApi(id, data);
        await fetchPatients();
        await fetchFlaggedPatients();
      } catch (err) {
        const apiError = err as ApiError;
        setError(apiError.error || "Failed to update patient.");
      } finally {
        setLoading(false);
      }
    },
    [fetchPatients, fetchFlaggedPatients]
  );

  /**
   * Delete a patient by ID.
   */
  const deletePatient = useCallback(
    async (id: number) => {
      try {
        setLoading(true);
        await deletePatientApi(id);
        await fetchPatients();
        await fetchFlaggedPatients();
      } catch (err) {
        const apiError = err as ApiError;
        setError(apiError.error || "Failed to delete patient.");
      } finally {
        setLoading(false);
      }
    },
    [fetchPatients, fetchFlaggedPatients]
  );

  /**
   * Automatic polling for regular and flagged data.
   * Two intervals ensure flagged data can refresh more often if needed.
   */
  useEffect(() => {
    fetchPatients();
    fetchFlaggedPatients();

    const patientInterval = setInterval(fetchPatients, POLL_INTERVAL);
    const flaggedInterval = setInterval(fetchFlaggedPatients, POLL_INTERVAL);

    // Cleanup to prevent memory leaks
    return () => {
      clearInterval(patientInterval);
      clearInterval(flaggedInterval);
    };
  }, [fetchPatients, fetchFlaggedPatients]);

  return {
    patients,
    flaggedPatients,
    loading,
    flaggedLoading,
    error,
    refetch: fetchPatients,
    refetchFlagged: fetchFlaggedPatients,
    createPatient,
    updatePatient,
    deletePatient,
  };
}
