import PoliceData from "./components/PoliceData";
import HospitalData from "./components/HospitalData";

function App() {
  return (
    <div>
      <h1>Police and Hospital Data System</h1>

      <h2>Police System - Suspects</h2>
      <PoliceData />

      <hr />

      <h2>Hospital System - Patients</h2>
      <HospitalData />
    </div>
  );
}

export default App;
