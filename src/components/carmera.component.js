import React, { useEffect } from "react";
import Webcam from "react-webcam";

const RENDER_INTERVAL = 500;
const WIDTH = 256;
const HEIGHT = 256;

const videoConstraints = {
  width: WIDTH,
  height: HEIGHT,
  facingMode: "user"
};

const WebcamCapture = ({ webcamRef, setFrame, setScreen }) => {

  // The function for capturing the video frame
  const handleUserMedia = () => {

    if (webcamRef.current === null || webcamRef.current.stream === undefined) {
      return;
    }

    const screenshot = webcamRef.current.getScreenshot();
    setFrame(screenshot);
  }

  // Capture video frame as stream on interval = 1 sec
  let intervalId = null;
  const handleUserMediaWithInterval = () => {
    intervalId = setInterval(handleUserMedia, RENDER_INTERVAL);
  }

  useEffect(() => {
    handleUserMediaWithInterval();
    return () => {
      clearInterval(intervalId);
    }
  }, [intervalId]);


  return <Webcam
    ref={webcamRef}
    audio={false}
    height={HEIGHT}
    screenshotFormat="image/jpeg"
    width={WIDTH}
    videoConstraints={videoConstraints}
    onUserMedia={handleUserMedia}
  >
    {({ getScreenshot }) => (
      <button
        onClick={() => {
          const imageSrc = getScreenshot();
          setScreen(imageSrc);
        }}
      >
        Capture photo
      </button>
    )}
  </Webcam>
};


export default WebcamCapture;
export { WIDTH, HEIGHT };