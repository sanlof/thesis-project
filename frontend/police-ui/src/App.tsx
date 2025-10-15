import React, { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [cases, setCases] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch("http://localhost:8000/api/cases")
      .then((res) => res.json())
      .then((data) => {
        setCases(data);
        setLoading(false);
      })
      .catch((err) => console.error(err));
  }, []);

  return (
    <div className="App">
      <header>
        <h1>🚔 Police System</h1>
      </header>
      <main>
        <h2>Active Cases</h2>
        {loading ? (
          <p>Loading...</p>
        ) : (
          <div className="cases-list">
            {cases.map((c: any) => (
              <div key={c.id} className="case-card">
                <strong>{c.case_number}</strong>
                <span>Status: {c.status}</span>
              </div>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
