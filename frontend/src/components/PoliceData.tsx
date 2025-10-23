import { useEffect, useState } from "react";
import { Suspect } from "../types";

function PoliceData() {
  const [suspects, setSuspects] = useState<Suspect[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchSuspects = async () => {
      try {
        setLoading(true);
        setError(null);

        const response = await fetch("/api/police/suspects");

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

    fetchSuspects();
  }, []);

  if (loading) {
    return <div>Loading police data...</div>;
  }

  if (error) {
    return <div>Error loading police data: {error}</div>;
  }

  return (
    <div>
      <h2>Police System - Suspects</h2>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Full Name</th>
            <th>Personal ID</th>
            <th>Flag</th>
          </tr>
        </thead>
        <tbody>
          {suspects.map((suspect) => (
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
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default PoliceData;
