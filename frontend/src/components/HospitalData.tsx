import { Patient } from "../types";
import { usePolling } from "../hooks/usePolling";

function HospitalData() {
  const fetchPatients = async (): Promise<Patient[]> => {
    const response = await fetch("/api/hospital/patients");

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return response.json();
  };

  const {
    data: patients,
    loading,
    error,
    isRefreshing,
  } = usePolling<Patient[]>(fetchPatients, {
    enabled: true,
    interval: 3000,
    pauseOnInactive: true,
  });

  if (loading) {
    return <div>Loading hospital data...</div>;
  }

  if (error) {
    return <div>Error loading hospital data: {error}</div>;
  }

  if (!patients) {
    return <div>No data available</div>;
  }

  return (
    <div>
      <h2>
        Hospital System - Patients
        {isRefreshing && <span> (refreshing...)</span>}
      </h2>
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
