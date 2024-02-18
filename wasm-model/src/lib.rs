pub mod yolov8_model;
mod log;

use log::log;
use image::{ImageBuffer, Rgba};
pub use yolov8_model::YoloV8;

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
pub fn gen_img(img: Vec<u8>, width: u32, height: u32) {
    let dynimg = ImageBuffer::<Rgba<u8>, _>::from_vec(width, height, img);
    if let Some(dynimg) = dynimg {
        log(&format!("Frame data size: {:?}", dynimg.len()));
    } else {
        log("Failed to create image");
    }
}
