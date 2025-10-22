/**
 * src/utils/validation.ts
 * ---------------------------------------------------------
 * Utility functions for validating and formatting
 * Swedish personal identity numbers (personnummer).
 *
 * Format: YYYYMMDD-XXXX
 * Example: 19900101-1234
 */

/**
 * Validates Swedish personal ID format (YYYYMMDD-XXXX).
 *
 * @param personalId - The personal ID to validate.
 * @returns True if valid, false otherwise.
 *
 * @example
 * validateSwedishPersonalId("19900101-1234"); // true
 * validateSwedishPersonalId("19901301-1234"); // false (invalid month)
 */
export function validateSwedishPersonalId(personalId: string): boolean {
  const regex = /^\d{8}-\d{4}$/;
  if (!regex.test(personalId)) return false;

  const datePart = personalId.substring(0, 8);
  const year = parseInt(datePart.substring(0, 4), 10);
  const month = parseInt(datePart.substring(4, 6), 10);
  const day = parseInt(datePart.substring(6, 8), 10);

  if (month < 1 || month > 12) return false;

  const daysInMonth = new Date(year, month, 0).getDate();
  if (day < 1 || day > daysInMonth) return false;

  return true;
}

/**
 * Auto-formats a personal ID string as user types.
 *
 * Adds a hyphen after 8 digits and limits total length.
 *
 * @param input - Unformatted input string
 * @returns Formatted string in YYYYMMDD-XXXX form
 *
 * @example
 * formatPersonalIdInput("199001011234"); // "19900101-1234"
 */
export function formatPersonalIdInput(input: string): string {
  // Remove all non-digit characters
  const digits = input.replace(/\D/g, "");

  // Insert hyphen after 8 digits if enough length
  if (digits.length > 8) {
    return `${digits.substring(0, 8)}-${digits.substring(8, 12)}`.substring(
      0,
      13
    );
  }

  return digits;
}

/**
 * Returns a user-friendly validation error message.
 *
 * @param personalId - The personal ID to validate
 * @returns Error message string, or null if valid
 *
 * @example
 * getValidationError("19900101-1234"); // null
 * getValidationError("199001011234"); // "Invalid format"
 */
export function getValidationError(personalId: string): string | null {
  if (!personalId) return "Personal ID is required.";
  if (!/^\d{8}-\d{4}$/.test(personalId))
    return "Invalid format (use YYYYMMDD-XXXX).";
  if (!validateSwedishPersonalId(personalId))
    return "Invalid date in personal ID.";
  return null;
}
