import React, { useRef, useEffect } from "react";
import Webcam from "react-webcam";


const videoConstraints = {
  width: 1280,
  height: 720,
  facingMode: "user"
};

const WebcamCapture = () => {

  // The function for capturing the video frame
  const webcamRef = useRef(null);
  const handleUserMedia = () => {
    const stream = webcamRef.current.stream;
    const videoTrack = stream.getVideoTracks()[0];
    const imageCapture = new ImageCapture(videoTrack);

    imageCapture.grabFrame().then(imageBitmap => {
      const canvas = document.createElement('canvas');
      const context = canvas.getContext("2d");
      context.drawImage(imageBitmap, 0, 0);
      const imageData = context.getImageData(0, 0, imageBitmap.width, imageBitmap.height);
      const data = new Uint8Array(imageData.data.buffer);
      console.log(data);
    })
  }

  // Capture video frame as stream on interval = 1 sec
  let intervalId = null;
  const handleUserMediaWithInterval = () => {
    intervalId = setInterval(handleUserMedia, 500);
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