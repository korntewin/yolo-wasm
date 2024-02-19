use crate::log::log;
use crate::utils::YOLOV8_X_MODEL_URL;
use crate::yolov8_model::{YoloV8, Multiples};

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use serde::{Deserialize, Serialize};
use web_sys::{Request, RequestCache, RequestInit, RequestMode, Response};
use candle_core::{Device, DType};
use candle_nn::VarBuilder;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct ModelData {
    pub weights: Vec<u8>,
    pub model_size: String,
}

unsafe impl Send for YoloV8 {}
unsafe impl Sync for YoloV8 {}

pub static LAZY_MODEL: Lazy<Mutex<Option<YoloV8>>> = Lazy::new(|| Mutex::new(None));


pub async fn download_binary() -> ModelData {
    let window = web_sys::window().ok_or("window").unwrap();
    let mut opts = RequestInit::new();
    let opts = opts
        .method("GET")
        .mode(RequestMode::Cors)
        .cache(RequestCache::NoCache);

    let request = Request::new_with_str_and_init(YOLOV8_X_MODEL_URL, opts).unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let data = JsFuture::from(resp.blob().unwrap()).await.unwrap();
    let blob = web_sys::Blob::from(data);
    let array_buffer = JsFuture::from(blob.array_buffer()).await.unwrap();
    let data = js_sys::Uint8Array::new(&array_buffer).to_vec();
    log(&format!("Downloaded model with size: {}", data.len()));
    ModelData {
        weights: data,
        model_size: "yolov8x".to_string(),
    }
} 

#[wasm_bindgen]
pub async fn get_model() {
    let model_data = download_binary().await;
    let device = Device::Cpu;
    let vb = unsafe { VarBuilder::from_buffered_safetensors(model_data.weights, DType::F32, &device).unwrap() };
    let model = YoloV8::load(vb, Multiples::x(), 80).unwrap();
    log(&format!("Model loaded: {:?}", model));
    *LAZY_MODEL.lock().unwrap() = Some(model);
}