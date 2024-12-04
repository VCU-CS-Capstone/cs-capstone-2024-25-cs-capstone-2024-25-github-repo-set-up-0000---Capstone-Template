import React from "react";
import Spectrogram from "./components/Spectrogram";
import "./index.css";

function App() {
  return (
    <div>
      <h1 style={{ textAlign: "center" }}>Spectrogram Viewer</h1>
      <Spectrogram />
    </div>
  );
}

export default App;
