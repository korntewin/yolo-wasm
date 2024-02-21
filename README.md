# 🚀 Overview

>🔥 Simple Web application for YOLOv8 Object Detection using **React + Rust's WebAssembly**!  

The web application is run & rendered purely on client-side 🖥️ without transferring any data out to the public internet, so no need to worry! 

The YOLOv8 model is backed by Rust programming language compiled into WebAssembly 🎉 which is imported and modified from 🕯️[candle](https://github.com/huggingface/candle) repository. Below is the example of webapp:

<img src="./imgs/demo.png" width="950" height="480">

_Note that, the web app seems to have the most performance on firefox._

# Get started
1. cd into project directory root and run `npm install` to install all of the required React pacakge
2. run `npm run build:wasm` to build webassembly artifact  
3. run `npm run start` to test locally 
4. run `npm run build` to build the deployable artifact
