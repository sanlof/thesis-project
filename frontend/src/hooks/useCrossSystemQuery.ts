/**
 * src/hooks/useCrossSystemQuery.ts
 * ---------------------------------------------------------
 * Custom React hook for performing cross-system queries.
 *
 * Enables:
 *  - Police → Hospital queries (patients)
 *  - Hospital → Police queries (suspects)
 *
 * Provides unified error, loading, and result management.
 *
 * Uses: src/services/policeApi.ts, src/services/hospitalApi.ts
 */

import { useState, useCallback } from "react";
import { queryPatient } from "../services/policeApi";
import { querySuspect } from "../services/hospitalApi";
import type { Patient, Suspect, ApiError } from "../services/types";

/**
 * Hook providing cross-system query functions with loading and error states.
 *
 * Example usage:
 * const {
 *   queryPatientFromPolice,
 *   querySuspectFromHospital,
 *   policeQueryLoading,
 *   hospitalQueryLoading,
 *   policeQueryError,
 *   hospitalQueryError,
 *   clearErrors,
 * } = useCrossSystemQuery();
 */
export function useCrossSystemQuery() {
  // State for query results
  const [policeQueryResult, setPoliceQueryResult] = useState<Patient | null>(
    null
  );
  const [hospitalQueryResult, setHospitalQueryResult] =
    useState<Suspect | null>(null);

  // Loading states
  const [policeQueryLoading, setPoliceQueryLoading] = useState<boolean>(false);
  const [hospitalQueryLoading, setHospitalQueryLoading] =
    useState<boolean>(false);

  // Error states
  const [policeQueryError, setPoliceQueryError] = useState<string | null>(null);
  const [hospitalQueryError, setHospitalQueryError] = useState<string | null>(
    null
  );

  /**
   * Query a hospital patient record via the police system API.
   * (Police → Hospital)
   */
  const queryPatientFromPolice = useCallback(
    async (personalId: string): Promise<Patient | null> => {
      setPoliceQueryLoading(true);
      setPoliceQueryError(null);

      try {
        const data = await queryPatient(personalId);
        setPoliceQueryResult(data);
        return data;
      } catch (err) {
        const apiError = err as ApiError;
        setPoliceQueryError(
          apiError.error || "Failed to query hospital system."
        );
        setPoliceQueryResult(null);
        return null;
      } finally {
        setPoliceQueryLoading(false);
      }
    },
    []
  );

  /**
   * Query a suspect record via the hospital system API.
   * (Hospital → Police)
   */
  const querySuspectFromHospital = useCallback(
    async (personalId: string): Promise<Suspect | null> => {
      setHospitalQueryLoading(true);
      setHospitalQueryError(null);

      try {
        const data = await querySuspect(personalId);
        setHospitalQueryResult(data);
        return data;
      } catch (err) {
        const apiError = err as ApiError;
        setHospitalQueryError(
          apiError.error || "Failed to query police system."
        );
        setHospitalQueryResult(null);
        return null;
      } finally {
        setHospitalQueryLoading(false);
      }
    },
    []
  );

  /**
   * Clears all stored query results and errors.
   */
  const clearErrors = useCallback(() => {
    setPoliceQueryError(null);
    setHospitalQueryError(null);
  }, []);

  return {
    // Query functions
    queryPatientFromPolice,
    querySuspectFromHospital,

    // Results
    policeQueryResult,
    hospitalQueryResult,

    // Loading states
    policeQueryLoading,
    hospitalQueryLoading,

    // Error states
    policeQueryError,
    hospitalQueryError,

    // Utilities
    clearErrors,
  };
}
