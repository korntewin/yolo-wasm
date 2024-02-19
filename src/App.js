import logo from './logo.svg';
import { useState, useRef } from 'react';
import './App.css';

import WebcamCapture from './components/carmera.component';
import InferenceWebcam from './components/inference.component';

import init from "wasm-model";

// Initialize the wasm module
await init();


function App() {

  const webcamRef = useRef(null);
  const [frame, setFrame] = useState(null);
  const [annotatedImgSrc, setAnnotatedImgSrc] = useState(null);


  return (
    <div className="App">
      <header className="App-header">
        <p> Object Detection with YOLOv8 </p>
      <WebcamCapture webcamRef={webcamRef} setFrame={setFrame}/>
      <InferenceWebcam frame={frame} setAnnotatedImgSrc={setAnnotatedImgSrc} />
      <img src={annotatedImgSrc} alt="annotated" />
      </header>
    </div>
  );
}

export default App;
