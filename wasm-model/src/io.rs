use crate::log::log;
use crate::utils::{model_multiplier, model_url};
use crate::yolov8_model::YoloV8;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestCache, RequestInit, RequestMode, Response};

#[derive(Serialize, Deserialize)]
pub struct ModelData {
    pub weights: Vec<u8>,
    pub model_size: String,
}

pub static LAZY_MODEL: Lazy<Mutex<Option<YoloV8>>> = Lazy::new(|| Mutex::new(None));

pub async fn download_binary(model_size: &str) -> ModelData {
    let window = web_sys::window().ok_or("window").unwrap();
    let mut opts = RequestInit::new();
    let opts = opts
        .method("GET")
        .mode(RequestMode::Cors)
        .cache(RequestCache::NoCache);

    let url = model_url(model_size);
    let request = Request::new_with_str_and_init(&url, opts).unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

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
pub async fn get_model(model_size: &str) {
    let model_data = download_binary(model_size).await;
    // let device = Device::Cpu;
    let device = Device::new_cuda(0).unwrap_or(Device::Cpu);
    let vb =
        VarBuilder::from_buffered_safetensors(model_data.weights, DType::F32, &device).unwrap();
    let model = YoloV8::load(vb, model_multiplier(model_size), 80).unwrap();
    log(&format!("Model loaded: {:?}", model));
    log(&format!("Model size: {:?}", model_size));
    *LAZY_MODEL.lock().unwrap() = Some(model);
}
