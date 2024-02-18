import React, { useEffect } from "react";
import Webcam from "react-webcam";

const RENDER_INTERVAL = 1000;


const videoConstraints = {
  width: 1280,
  height: 720,
  facingMode: "user"
};

const WebcamCapture = ({webcamRef, setFrame}) => {

  // The function for capturing the video frame
  const handleUserMedia = () => {

    if (webcamRef.current === null || webcamRef.current.stream === undefined) {
      return;
    }

    const stream = webcamRef.current.stream;
    const videoTrack = stream.getVideoTracks()[0];
    const imageCapture = new ImageCapture(videoTrack);

    imageCapture.grabFrame().then(imageBitmap => {
      const canvas = document.createElement('canvas');
      const context = canvas.getContext("2d");
      context.drawImage(imageBitmap, 0, 0);
      const imageData = context.getImageData(0, 0, imageBitmap.width, imageBitmap.height);
      const data = new Uint8Array(imageData.data.buffer);
      setFrame(data);
      console.log(data);
    })
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
    height={720}
    screenshotFormat="image/jpeg"
    width={1280}
    videoConstraints={videoConstraints}
    onUserMedia={handleUserMedia}
  />
};


export default WebcamCapture;