mod log;
mod utils;
mod io;
mod coco_classes;
pub mod yolov8_model;

pub use yolov8_model::YoloV8;
pub use io::download_binary;
use log::log;
use candle_core::Module;

use crate::io::LAZY_MODEL;
use crate::utils::{transform_image, identify_bboxes};

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
pub fn test_gen_img(img: String) {
    let _img = transform_image(img).unwrap();
    log("Finished transform image");
}


#[wasm_bindgen]
pub fn test_lazy_model(img: String) {
    // log(&format!("Before tranforming image: {:?}", img));
    let img = transform_image(img).unwrap();
    // log(&format!("Finished tranformed image: {:?}", img));
    let maybe_model = LAZY_MODEL.lock().unwrap();
    log(&format!("Finished locking the model"));

    if let Some(ref model) = *maybe_model {
        log(&format!("Before model forwarding"));
        let tensor = model.forward(&img).unwrap().squeeze(0);
        log(&format!("After model forwarding"));
        log(&format!("Tensor: {:?}", tensor));
    } else {
        log("Model not loaded");
    }
}

#[wasm_bindgen]
pub fn test_identify_bboxes(img: String, conf_threshold: f32, iou_threshold: f32) {
    let img = transform_image(img).unwrap();
    let maybe_model = LAZY_MODEL.lock().unwrap();
    log(&format!("Finished locking the model"));

    if let Some(ref model) = *maybe_model {
        log(&format!("Before model forwarding"));
        let pred = model.forward(&img).unwrap().squeeze(0).unwrap();
        let bboxes = identify_bboxes(&pred, conf_threshold, iou_threshold);
        log(&format!("After model forwarding"));
        log(&format!("Tensor: {:?}", bboxes));
    } else {
        log("Model not loaded");
    }
}