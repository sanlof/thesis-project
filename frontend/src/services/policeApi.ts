/**
 * src/services/policeApi.ts
 * ---------------------------------------------------------
 * Police System API Service
 *
 * Provides typed functions for all Police backend operations,
 * including suspect management and cross-system (shared) queries.
 *
 * Uses the shared Axios instance from api.ts for consistent
 * configuration, error handling, and logging.
 */

import { api, buildUrl, handleApiError, POLICE_API_URL } from "./api";
import type {
  Suspect,
  Patient,
  CreateSuspect,
  UpdateSuspect,
  ApiError,
} from "./types";

const BASE_URL = POLICE_API_URL;

// -----------------------------------------------------------------------------
// Suspect CRUD Operations
// -----------------------------------------------------------------------------

/**
 * Fetch all suspects from the police system.
 * @returns Promise<Suspect[]>
 */
export async function getAllSuspects(): Promise<Suspect[]> {
  try {
    const url = buildUrl(BASE_URL, "/suspects");
    return await api.get(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Fetch a suspect by database ID.
 * @param id - Database ID of the suspect
 * @returns Promise<Suspect>
 */
export async function getSuspectById(id: number): Promise<Suspect> {
  try {
    const url = buildUrl(BASE_URL, `/suspects/${id}`);
    return await api.get(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Fetch a suspect by Swedish personal ID.
 * @param personalId - Personal ID (YYYYMMDD-XXXX)
 * @returns Promise<Suspect>
 */
export async function getSuspectByPersonalId(
  personalId: string
): Promise<Suspect> {
  try {
    const url = buildUrl(BASE_URL, `/suspects/personal/${personalId}`);
    return await api.get(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Create a new suspect.
 * @param data - Suspect creation data
 * @returns Promise<Suspect>
 */
export async function createSuspect(data: CreateSuspect): Promise<Suspect> {
  try {
    const url = buildUrl(BASE_URL, "/suspects");
    return await api.post(url, data);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Update an existing suspect by ID.
 * @param id - Suspect ID
 * @param data - Partial update data
 * @returns Promise<Suspect>
 */
export async function updateSuspect(
  id: number,
  data: UpdateSuspect
): Promise<Suspect> {
  try {
    const url = buildUrl(BASE_URL, `/suspects/${id}`);
    return await api.put(url, data);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Delete a suspect by ID.
 * @param id - Suspect ID
 * @returns Promise<void>
 */
export async function deleteSuspect(id: number): Promise<void> {
  try {
    const url = buildUrl(BASE_URL, `/suspects/${id}`);
    await api.delete(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Update the flag status of a suspect by personal ID.
 * @param personalId - Personal ID (YYYYMMDD-XXXX)
 * @param flag - New flag status (true = flagged)
 * @returns Promise<Suspect>
 */
export async function updateSuspectFlag(
  personalId: string,
  flag: boolean
): Promise<Suspect> {
  try {
    const url = buildUrl(BASE_URL, `/suspects/${personalId}/flag`);
    return await api.put(url, { flag });
  } catch (error) {
    throw handleApiError(error);
  }
}

// -----------------------------------------------------------------------------
// Shared API (for hospital system queries)
// -----------------------------------------------------------------------------

/**
 * Fetch all patients from the shared API (hospital querying police data).
 * @returns Promise<Patient[]>
 */
export async function getAllPatients(): Promise<Patient[]> {
  try {
    const url = buildUrl(BASE_URL, "/api/shared/suspects");
    return await api.get(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Query a patient (hospital person) by personal ID via shared API.
 * Used by hospital system to check if a patient exists in police records.
 * @param personalId - Swedish personal ID (YYYYMMDD-XXXX)
 * @returns Promise<Patient | null>
 */
export async function queryPatient(
  personalId: string
): Promise<Patient | null> {
  try {
    const url = buildUrl(BASE_URL, `/api/shared/suspects/${personalId}`);
    return await api.get(url);
  } catch (error: any) {
    const apiError = handleApiError(error);

    // Gracefully handle 404 as "not found" instead of throwing
    if ((error as ApiError)?.error?.includes("404")) {
      return null;
    }

    throw apiError;
  }
}

// -----------------------------------------------------------------------------
// Exports
// -----------------------------------------------------------------------------

export default {
  getAllSuspects,
  getSuspectById,
  getSuspectByPersonalId,
  createSuspect,
  updateSuspect,
  deleteSuspect,
  updateSuspectFlag,
  getAllPatients,
  queryPatient,
};
