import logo from './logo.svg';
import { useState, useRef } from 'react';
import './App.css';

import WebcamCapture from './components/camera.component';
import InferenceWebcam from './components/inference.component';
import FlexContainer from './components/flexcontainer.component';
import Layout from './layouts/base.layout';

import init from "wasm-model";

// Initialize the wasm module
await init();


function App() {

  const webcamRef = useRef(null);
  const [frame, setFrame] = useState(null);
  const [annotatedImgSrc, setAnnotatedImgSrc] = useState(null);


  return (
    <div className="App">
      <Layout>
        <h2> 🔥Object Detection with YOLOv8 </h2>
        <FlexContainer>
          <div>
            <h4> Original Video Stream </h4>
            <WebcamCapture webcamRef={webcamRef} setFrame={setFrame}/>
          </div>
          <div>
            <InferenceWebcam frame={frame} setAnnotatedImgSrc={setAnnotatedImgSrc} />
            {annotatedImgSrc && <img src={annotatedImgSrc} alt="annotated" width="auto"/>}
          </div>
        </FlexContainer>
      </Layout>
    </div>
  );
}

export default App;
