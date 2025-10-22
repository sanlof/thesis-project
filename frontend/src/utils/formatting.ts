/**
 * src/utils/formatting.ts
 * ---------------------------------------------------------
 * Utility functions for consistent display formatting
 * of names, flags, and dates throughout the app.
 */

/**
 * Formats a full name, handling null or empty values.
 *
 * @param fullName - Full name string or null
 * @returns Formatted name or placeholder
 *
 * @example
 * formatFullName("anna andersson"); // "Anna Andersson"
 * formatFullName(null); // "Unknown"
 */
export function formatFullName(fullName: string | null): string {
  if (!fullName) return "Unknown";
  return fullName
    .split(" ")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join(" ");
}

/**
 * Formats flag status into readable text.
 *
 * @param flag - Boolean or null flag value
 * @returns Readable flag label
 *
 * @example
 * formatFlagStatus(true); // "Flagged"
 * formatFlagStatus(false); // "Not Flagged"
 */
export function formatFlagStatus(flag: boolean | null): string {
  if (flag === true) return "Flagged";
  if (flag === false) return "Not Flagged";
  return "Unknown";
}

/**
 * Formats an ISO date string into readable format.
 *
 * @param dateString - ISO date string (e.g., "2025-10-21T14:30:00Z")
 * @returns Readable date (e.g., "21 Oct 2025, 14:30")
 *
 * @example
 * formatDate("2025-10-21T14:30:00Z"); // "21 Oct 2025, 14:30"
 */
export function formatDate(dateString: string): string {
  if (!dateString) return "—";
  const date = new Date(dateString);
  return date.toLocaleString("en-GB", {
    day: "2-digit",
    month: "short",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

/**
 * Truncates a long text string with ellipsis.
 *
 * @param text - The text to truncate
 * @param maxLength - Maximum length before truncating
 * @returns Truncated string
 *
 * @example
 * truncateText("This is a long sentence", 10); // "This is a..."
 */
export function truncateText(text: string, maxLength: number): string {
  if (!text) return "";
  return text.length > maxLength ? text.substring(0, maxLength) + "..." : text;
}
