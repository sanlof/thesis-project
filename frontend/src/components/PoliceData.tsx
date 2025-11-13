import { useEffect, useState } from "react";
import { Suspect, FlagUpdateRequest } from "../types";
import { fetchWithCsrf, isCsrfError, getCsrfToken } from "../utils/csrf";

function PoliceData() {
  const [suspects, setSuspects] = useState<Suspect[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [togglingId, setTogglingId] = useState<string | null>(null);
  const [toggleError, setToggleError] = useState<string | null>(null);
  const [csrfReady, setCsrfReady] = useState<boolean>(false);

  useEffect(() => {
    const fetchSuspects = async () => {
      try {
        setLoading(true);
        setError(null);

        console.log("[PoliceData] Fetching suspects...");

        // GET requests automatically receive CSRF token via cookie
        const response = await fetch("/api/police/suspects", {
          credentials: "include", // Important: include cookies
          headers: {
            Accept: "application/json",
          },
        });

        console.log("[PoliceData] Response status:", response.status);
        console.log(
          "[PoliceData] Response headers:",
          Object.fromEntries(response.headers.entries())
        );

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data: Suspect[] = await response.json();
        setSuspects(data);

        console.log("[PoliceData] Suspects loaded:", data.length);

        // Wait a moment for cookie to be set
        await new Promise((resolve) => setTimeout(resolve, 300));

        // Check if we received a CSRF token
        const token = getCsrfToken();
        if (token) {
          console.log("[PoliceData] CSRF token available after initial fetch");
          setCsrfReady(true);
        } else {
          console.warn("[PoliceData] No CSRF token after initial fetch");
          console.warn("[PoliceData] Current cookies:", document.cookie);
          // Still mark as ready - token will be fetched on demand if needed
          setCsrfReady(true);
        }
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : "Unknown error";
        console.error("[PoliceData] Error fetching suspects:", errorMessage);
        setError(errorMessage);
      } finally {
        setLoading(false);
      }
    };

    void fetchSuspects();
  }, []);

  const toggleFlag = async (suspect: Suspect) => {
    if (!suspect.personal_id) {
      setToggleError("Cannot toggle flag: personal_id is missing");
      return;
    }

    const newFlagValue = !suspect.flag;

    try {
      setTogglingId(suspect.personal_id);
      setToggleError(null);

      const flagUpdate: FlagUpdateRequest = {
        personal_id: suspect.personal_id,
        flag: newFlagValue,
      };

      console.log(
        "[PoliceData] Attempting flag toggle for:",
        suspect.personal_id
      );
      console.log(
        "[PoliceData] Current cookies before toggle:",
        document.cookie
      );

      // Use fetchWithCsrf for automatic CSRF token handling
      const response = await fetchWithCsrf("/api/police/suspects/flag", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(flagUpdate),
      });

      console.log("[PoliceData] Toggle response status:", response.status);

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(
          `Failed to update flag: ${response.status} - ${
            errorData.error || response.statusText
          }`
        );
      }

      const updatedSuspect: Suspect = await response.json();

      // Update local state with the new suspect data
      setSuspects((prevSuspects) =>
        prevSuspects.map((s) =>
          s.personal_id === suspect.personal_id ? updatedSuspect : s
        )
      );

      console.log("[PoliceData] Flag toggle successful");
      setToggleError(null);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "Unknown error";

      console.error("[PoliceData] Flag toggle failed:", errorMessage);

      // Handle CSRF errors specially
      if (isCsrfError(err)) {
        setToggleError(
          `CSRF validation failed. Please refresh the page and try again.`
        );
      } else {
        setToggleError(
          `Failed to toggle flag for ${suspect.full_name}: ${errorMessage}`
        );
      }
    } finally {
      setTogglingId(null);
    }
  };

  if (loading) {
    return <div>Loading police data...</div>;
  }

  if (error) {
    return <div>Error loading police data: {error}</div>;
  }

  return (
    <div>
      <h2>Police System - Suspects</h2>
      {!csrfReady && (
        <div
          style={{ color: "orange", marginBottom: "10px", fontSize: "0.9em" }}
        >
          ⚠️ CSRF protection initializing...
        </div>
      )}
      {toggleError && (
        <div style={{ color: "red", marginBottom: "10px" }}>
          {toggleError}
          {toggleError.includes("CSRF") && (
            <button
              onClick={() => window.location.reload()}
              style={{ marginLeft: "10px" }}
            >
              Refresh Page
            </button>
          )}
        </div>
      )}
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Full Name</th>
            <th>Personal ID</th>
            <th>Flag</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {suspects.map((suspect) => {
            const isToggling = togglingId === suspect.personal_id;
            const canToggle = suspect.personal_id !== null;

            return (
              <tr key={suspect.id}>
                <td>{suspect.id}</td>
                <td>{suspect.full_name ?? "N/A"}</td>
                <td>{suspect.personal_id ?? "N/A"}</td>
                <td>
                  {suspect.flag === true
                    ? "Yes"
                    : suspect.flag === false
                    ? "No"
                    : "N/A"}
                </td>
                <td>
                  <button
                    onClick={() => toggleFlag(suspect)}
                    disabled={!canToggle || isToggling}
                    style={{
                      padding: "5px 10px",
                      cursor:
                        canToggle && !isToggling ? "pointer" : "not-allowed",
                      opacity: canToggle && !isToggling ? 1 : 0.5,
                    }}
                  >
                    {isToggling
                      ? "Updating..."
                      : suspect.flag
                      ? "Unflag"
                      : "Flag"}
                  </button>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
      <p style={{ marginTop: "10px", fontSize: "0.9em", color: "#666" }}>
        Note: Flag changes automatically sync to the hospital system via
        database triggers. CSRF protection is active for all flag updates.
      </p>
    </div>
  );
}

export default PoliceData;
