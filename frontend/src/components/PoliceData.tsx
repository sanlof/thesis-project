import { useEffect, useState } from "react";
import { Suspect, FlagUpdateRequest } from "../types";
import { fetchWithCsrf, isCsrfError } from "../utils/csrf";

function PoliceData() {
  const [suspects, setSuspects] = useState<Suspect[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [togglingId, setTogglingId] = useState<string | null>(null);
  const [toggleError, setToggleError] = useState<string | null>(null);

  useEffect(() => {
    const fetchSuspects = async () => {
      try {
        setLoading(true);
        setError(null);

        // GET requests automatically receive CSRF token via cookie
        const response = await fetch("/api/police/suspects", {
          credentials: "include", // Important: include cookies
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data: Suspect[] = await response.json();
        setSuspects(data);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : "Unknown error";
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

      // Use fetchWithCsrf for automatic CSRF token handling
      const response = await fetchWithCsrf("/api/police/suspects/flag", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(flagUpdate),
      });

      if (!response.ok) {
        throw new Error(`Failed to update flag: ${response.status}`);
      }

      const updatedSuspect: Suspect = await response.json();

      // Update local state with the new suspect data
      setSuspects((prevSuspects) =>
        prevSuspects.map((s) =>
          s.personal_id === suspect.personal_id ? updatedSuspect : s
        )
      );

      setToggleError(null);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "Unknown error";

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
