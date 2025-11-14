import { useEffect, useState } from "react";
import { Suspect, FlagUpdateRequest } from "../types";

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

        console.log("[PoliceData] Fetching suspects...");

        const response = await fetch("/api/police/suspects", {
          credentials: "include",
          headers: {
            Accept: "application/json",
          },
        });

        console.log("[PoliceData] Response status:", response.status);

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data: Suspect[] = await response.json();
        setSuspects(data);

        console.log("[PoliceData] Suspects loaded:", data.length);
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

      const response = await fetch("/api/police/suspects/flag", {
        method: "POST",
        credentials: "include",
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
      setToggleError(
        `Failed to toggle flag for ${suspect.full_name}: ${errorMessage}`
      );
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
        <div style={{ color: "red", marginBottom: "10px" }}>{toggleError}</div>
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
        database triggers.
      </p>
    </div>
  );
}

export default PoliceData;
