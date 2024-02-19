mod coco_classes;
mod io;
mod log;
mod utils;
pub mod yolov8_model;

use candle_core::Module;
pub use io::download_binary;
use log::log;
pub use yolov8_model::YoloV8;

use crate::io::LAZY_MODEL;
use crate::utils::{
    annotate_images, get_dyn_image, identify_bboxes, img_to_base64, transform_image,
};

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
pub fn test_gen_img(img: String, shrink_width: f32, shrink_height: f32) {
    let _img = transform_image(img, shrink_width, shrink_height).unwrap();
    log("Finished transform image");
}

#[wasm_bindgen]
pub fn test_lazy_model(img: String, shrink_width: f32, shrink_height: f32) {
    // log(&format!("Before tranforming image: {:?}", img));
    let img = transform_image(img, shrink_width, shrink_height).unwrap();
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
pub fn test_identify_bboxes(img: String, shrink_width: f32, shrink_height: f32, conf_threshold: f32, iou_threshold: f32) {
    let img = transform_image(img, shrink_width, shrink_height).unwrap();
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

#[wasm_bindgen]
pub fn test_annotate_images(img: String, shrink_width: f32, shrink_height: f32, conf_threshold: f32, iou_threshold: f32) {
    let orig_img = get_dyn_image(&img).unwrap();
    let img = transform_image(img, shrink_width, shrink_height).unwrap();
    let maybe_model = LAZY_MODEL.lock().unwrap();
    log(&format!("Finished locking the model"));

    if let Some(ref model) = *maybe_model {
        log(&format!("Before model forwarding"));
        let pred = model.forward(&img).unwrap().squeeze(0).unwrap();
        let bboxes = identify_bboxes(&pred, conf_threshold, iou_threshold);
        let annotated_img =
            annotate_images(orig_img, shrink_width, shrink_height, &bboxes.unwrap());
        log(&format!("After annotate image"));
    } else {
        log("Model not loaded");
    }
}

#[wasm_bindgen]
pub fn js_annotate_images(img: String, shrink_width: f32, shrink_height: f32, conf_threshold: f32, iou_threshold: f32) -> String {
    let orig_img = get_dyn_image(&img).unwrap();
    let img = transform_image(img, shrink_width, shrink_height).unwrap();
    let maybe_model = LAZY_MODEL.lock().unwrap();
    log(&format!("Finished locking the model"));

    if let Some(ref model) = *maybe_model {
        log(&format!("Before model forwarding"));
        let pred = model.forward(&img).unwrap().squeeze(0).unwrap();
        let bboxes = identify_bboxes(&pred, conf_threshold, iou_threshold);
        let annotated_img =
            annotate_images(orig_img, shrink_width, shrink_height, &bboxes.unwrap()).unwrap();
        log(&format!("After annotate image"));
        return img_to_base64(annotated_img).unwrap();
    } else {
        log("Model not loaded");
        return "".to_string();
    }
}
