// frontend/src/types.ts
/**
 * Type definitions for Police and Hospital System API
 *
 * These types match the backend Rust data models:
 * - backend/police-system/src/models/suspect.rs
 * - backend/hospital-system/src/models/patient.rs
 */

/**
 * Suspect from the Police System
 *
 * Represents a person in the police database.
 * All fields except id can be null.
 */
export interface Suspect {
  id: number;
  full_name: string | null;
  personal_id: string | null; // Swedish format: YYYYMMDD-XXXX
  flag: boolean | null;
}

/**
 * Patient from the Hospital System
 *
 * Represents a person in the hospital database.
 * All fields except id can be null.
 */
export interface Patient {
  id: number;
  full_name: string | null;
  personal_id: string | null; // Swedish format: YYYYMMDD-XXXX
  flag: boolean | null;
}

/**
 * Create Suspect payload (for POST requests to police system)
 * Used when creating a new suspect record.
 */
export interface CreateSuspect {
  full_name: string;
  personal_id: string; // Required: Swedish format YYYYMMDD-XXXX
  flag?: boolean; // Optional: defaults to false
}

/**
 * Create Patient payload (for POST requests to hospital system)
 * Used when creating a new patient record.
 */
export interface CreatePatient {
  full_name: string;
  personal_id: string; // Required: Swedish format YYYYMMDD-XXXX
  flag?: boolean; // Optional: defaults to false
}

/**
 * Update Suspect payload (for PUT requests to police system)
 * All fields except personal_id are optional.
 */
export interface UpdateSuspect {
  personal_id: string; // Required: identifies which suspect to update
  full_name?: string;
  flag?: boolean;
}

/**
 * Update Patient payload (for PUT requests to hospital system)
 * All fields except personal_id are optional.
 */
export interface UpdatePatient {
  personal_id: string; // Required: identifies which patient to update
  full_name?: string;
  flag?: boolean;
}

/**
 * Flag update payload (for POST /suspects/flag)
 *
 * SECURITY: Both personal_id and flag are now in the request body
 * to prevent exposure of sensitive data in URL paths and server logs.
 */
export interface FlagUpdateRequest {
  personal_id: string; // Swedish format: YYYYMMDD-XXXX
  flag: boolean;
}

/**
 * Health check response from both systems
 */
export interface HealthResponse {
  status: "healthy" | "unhealthy";
  service: "police-system" | "hospital-system";
}

/**
 * Generic error response from the API
 */
export interface ApiError {
  error: string;
  personal_id?: string; // Optional: included in some error responses
}
