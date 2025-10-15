import React, { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [patients, setPatients] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch("http://localhost:8001/api/patients")
      .then((res) => res.json())
      .then((data) => {
        setPatients(data);
        setLoading(false);
      })
      .catch((err) => console.error(err));
  }, []);

  return (
    <div className="App">
      <header>
        <h1>🏥 Hospital System</h1>
      </header>
      <main>
        <h2>Patients</h2>
        {loading ? (
          <p>Loading...</p>
        ) : (
          <div className="patients-list">
            {patients.map((p: any) => (
              <div key={p.id} className="patient-card">
                <strong>{p.name}</strong>
                <span>ID: {p.patient_id}</span>
              </div>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
