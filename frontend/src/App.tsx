import PoliceData from "./components/PoliceData";
import HospitalData from "./components/HospitalData";
import "./styles.css";

function App() {
  return (
    <div className="container">
      <header>
        <h1>Police & Hospital Data System</h1>
        <p>Real-time monitoring of suspects and patients</p>
      </header>

      <section>
        <PoliceData />
      </section>

      <section>
        <HospitalData />
      </section>
    </div>
  );
}

export default App;
