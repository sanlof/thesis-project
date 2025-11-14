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

        const response = await fetch("/api/police/suspects", {
          credentials: "include",
          headers: { Accept: "application/json" },
        });

        if (!response.ok)
          throw new Error(`HTTP error! status: ${response.status}`);

        const data: Suspect[] = await response.json();
        setSuspects(data);
      } catch (err) {
        const message = err instanceof Error ? err.message : "Unknown error";
        setError(message);
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

      const response = await fetch("/api/police/suspects/flag", {
        method: "POST",
        credentials: "include",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(flagUpdate),
      });

      if (!response.ok) {
        const errData = await response.json().catch(() => ({}));
        throw new Error(
          `Failed to update flag: ${response.status} - ${
            errData.error || response.statusText
          }`
        );
      }

      const updatedSuspect: Suspect = await response.json();
      setSuspects((prev) =>
        prev.map((s) =>
          s.personal_id === suspect.personal_id ? updatedSuspect : s
        )
      );
    } catch (err) {
      const message = err instanceof Error ? err.message : "Unknown error";
      setToggleError(
        `Failed to toggle flag for ${suspect.full_name}: ${message}`
      );
    } finally {
      setTogglingId(null);
    }
  };

  if (loading) return <div>Loading police data...</div>;
  if (error) return <div className="error">{error}</div>;

  return (
    <div className="card">
      <h2 className="header-police">Police System - Suspects</h2>
      {toggleError && <div className="error">{toggleError}</div>}

      <div style={{ overflowX: "auto" }}>
        <table className="table">
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
            {suspects.map((suspect, idx) => {
              const isToggling = togglingId === suspect.personal_id;
              const canToggle = suspect.personal_id !== null;

              return (
                <tr key={suspect.id}>
                  <td>{suspect.id}</td>
                  <td>{suspect.full_name ?? "N/A"}</td>
                  <td>{suspect.personal_id ?? "N/A"}</td>
                  <td>{suspect.flag ? "Yes" : "No"}</td>
                  <td>
                    <button
                      className={`button-flag ${
                        suspect.flag ? "unflag" : "flag"
                      } ${!canToggle ? "disabled" : ""}`}
                      onClick={() => toggleFlag(suspect)}
                      disabled={!canToggle || isToggling}
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
      </div>

      <p className="note">
        Note: Flag changes automatically sync to the hospital system via
        database triggers.
      </p>
    </div>
  );
}

export default PoliceData;
