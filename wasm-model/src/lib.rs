mod log;
mod utils;
mod io;
pub mod yolov8_model;

use image::{DynamicImage, ImageBuffer, Rgba};
use log::log;
pub use yolov8_model::YoloV8;
use candle_core::Module;
pub use io::download_binary;
use crate::io::LAZY_MODEL;
use crate::utils::transform_image;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn sum_vec(img: Vec<u8>) -> i32 {
    img.iter().map(|&x| x as i32).sum()
}


#[wasm_bindgen]
pub fn test_gen_img(img: Vec<u8>, width: u32, height: u32) {
    let dynimg =
        ImageBuffer::<Rgba<u8>, _>::from_vec(width, height, img).map(DynamicImage::ImageRgba8);
    if let Some(dynimg) = dynimg {
        log(&format!("Frame data size: {:?}", dynimg));
    } else {
        log("Failed to create image");
    }
}


#[wasm_bindgen]
pub fn test_lazy_model(img: Vec<u8>, width: u32, height: u32) {
    let img = transform_image(img, width, height).unwrap();
    log(&format!("Finished tranformed image: {:?}", img));
    let maybe_model = LAZY_MODEL.lock().unwrap();
    log(&format!("Finished locking the model"));

    if let Some(ref model) = *maybe_model {
        log(&format!("Before model forwarding"));
        let tensor = model.forward(&img).unwrap().squeeze(0);
        log(&format!("Tensor: {:?}", tensor));
    } else {
        log("Model not loaded");
    }
}