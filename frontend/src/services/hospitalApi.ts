/**
 * src/services/hospitalApi.ts
 * ---------------------------------------------------------
 * Hospital System API Service
 *
 * Provides typed functions for all Hospital backend operations,
 * including patient management and shared endpoints used by
 * the Police System for cross-system queries.
 *
 * Relies on the shared Axios instance and helpers from api.ts.
 */

import { api, buildUrl, handleApiError, HOSPITAL_API_URL } from "./api";
import type {
  Patient,
  Suspect,
  CreatePatient,
  UpdatePatient,
  ApiError,
} from "./types";

const BASE_URL = HOSPITAL_API_URL;

// -----------------------------------------------------------------------------
// Patient CRUD Operations
// -----------------------------------------------------------------------------

/**
 * Fetch all patients from the hospital system.
 * @returns Promise<Patient[]>
 */
export async function getAllPatients(): Promise<Patient[]> {
  try {
    const url = buildUrl(BASE_URL, "/patients");
    return await api.get(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Fetch a patient by database ID.
 * @param id - Database ID of the patient
 * @returns Promise<Patient>
 */
export async function getPatientById(id: number): Promise<Patient> {
  try {
    const url = buildUrl(BASE_URL, `/patients/${id}`);
    return await api.get(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Fetch a patient by Swedish personal ID.
 * @param personalId - Personal ID (YYYYMMDD-XXXX)
 * @returns Promise<Patient>
 */
export async function getPatientByPersonalId(
  personalId: string
): Promise<Patient> {
  try {
    const url = buildUrl(BASE_URL, `/patients/personal/${personalId}`);
    return await api.get(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Fetch all flagged patients (flag = true).
 * @returns Promise<Patient[]>
 */
export async function getFlaggedPatients(): Promise<Patient[]> {
  try {
    const url = buildUrl(BASE_URL, "/patients/flagged");
    return await api.get(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Create a new patient record.
 * @param data - Patient creation data
 * @returns Promise<Patient>
 */
export async function createPatient(data: CreatePatient): Promise<Patient> {
  try {
    const url = buildUrl(BASE_URL, "/patients");
    return await api.post(url, data);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Update an existing patient by ID.
 * @param id - Patient ID
 * @param data - Partial update data
 * @returns Promise<Patient>
 */
export async function updatePatient(
  id: number,
  data: UpdatePatient
): Promise<Patient> {
  try {
    const url = buildUrl(BASE_URL, `/patients/${id}`);
    return await api.put(url, data);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Delete a patient by ID.
 * @param id - Patient ID
 * @returns Promise<void>
 */
export async function deletePatient(id: number): Promise<void> {
  try {
    const url = buildUrl(BASE_URL, `/patients/${id}`);
    await api.delete(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

// -----------------------------------------------------------------------------
// Shared API (for police system queries)
// -----------------------------------------------------------------------------

/**
 * Get all patients via shared API (for police system use).
 * @returns Promise<Patient[]>
 */
export async function getAllSharedPatients(): Promise<Patient[]> {
  try {
    const url = buildUrl(BASE_URL, "/api/shared/patients");
    return await api.get(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Get all flagged patients via shared API (for police system use).
 * @returns Promise<Patient[]>
 */
export async function getSharedFlaggedPatients(): Promise<Patient[]> {
  try {
    const url = buildUrl(BASE_URL, "/api/shared/patients/flagged");
    return await api.get(url);
  } catch (error) {
    throw handleApiError(error);
  }
}

/**
 * Query a suspect by personal ID (police querying hospital data).
 * Returns null if not found (404).
 * @param personalId - Swedish personal ID (YYYYMMDD-XXXX)
 * @returns Promise<Suspect | null>
 */
export async function querySuspect(
  personalId: string
): Promise<Suspect | null> {
  try {
    const url = buildUrl(BASE_URL, `/api/shared/patients/${personalId}`);
    return await api.get(url);
  } catch (error: any) {
    const apiError = handleApiError(error);

    // Gracefully handle 404 responses
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
  getAllPatients,
  getPatientById,
  getPatientByPersonalId,
  getFlaggedPatients,
  createPatient,
  updatePatient,
  deletePatient,
  getAllSharedPatients,
  getSharedFlaggedPatients,
  querySuspect,
};
