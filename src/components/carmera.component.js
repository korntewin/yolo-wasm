import React from "react";
import Webcam from "react-webcam";


const videoConstraints = {
  width: 1280,
  height: 720,
  facingMode: "user"
};

const WebcamCapture = () => (
  <Webcam
    audio={false}
    height={720}
    screenshotFormat="image/jpeg"
    width={1280}
    videoConstraints={videoConstraints}
  >
    {({ getScreenshot }) => (
      <button
        onClick={() => {
          const imageSrc = getScreenshot()
          const img = new Image();
          console.log(imageSrc);
        }}
      >
        Capture photo
      </button>
    )}
  </Webcam>
);


export default WebcamCapture;