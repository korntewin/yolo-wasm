import logo from './logo.svg';
import { useState, useRef } from 'react';
import './App.css';

import WebcamCapture from './components/carmera.component';
import InferenceWebcam from './components/inference.component';
import ListWebcam from './components/lscamera.component';

import init, { add } from "wasm-model";

// Initialize the wasm module
await init();


function App() {

  const webcamRef = useRef(null);
  const [frame, setFrame] = useState(null);


  return (
    <div className="App">
      <header className="App-header">
        <p> 1 + 5 = {add(1, 5)} </p>
      <WebcamCapture webcamRef={webcamRef} setFrame={setFrame} />
      <InferenceWebcam frame={frame} />
      {/* <ListWebcam /> */}
      </header>
    </div>
  );
}

export default App;
