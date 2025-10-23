import { useEffect, useState } from "react";
import { Patient } from "../types";

function HospitalData() {
  const [patients, setPatients] = useState<Patient[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchPatients = async () => {
      try {
        setLoading(true);
        setError(null);

        const response = await fetch("/api/hospital/patients");

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data: Patient[] = await response.json();
        setPatients(data);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : "Unknown error";
        setError(errorMessage);
      } finally {
        setLoading(false);
      }
    };

    fetchPatients();
  }, []);

  if (loading) {
    return <div>Loading hospital data...</div>;
  }

  if (error) {
    return <div>Error loading hospital data: {error}</div>;
  }

  return (
    <div>
      <h2>Hospital System - Patients</h2>
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
          {patients.map((patient) => (
            <tr key={patient.id}>
              <td>{patient.id}</td>
              <td>{patient.full_name ?? "N/A"}</td>
              <td>{patient.personal_id ?? "N/A"}</td>
              <td>
                {patient.flag === true
                  ? "Yes"
                  : patient.flag === false
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

export default HospitalData;
