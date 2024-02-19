use crate::log::log;
use candle_core::{Device, Tensor, DType};
use image::{DynamicImage, ImageBuffer, Rgba};

// pub const YOLOV8_X_MODEL_URL: &str =
//     "https://huggingface.co/lmz/candle-yolo-v8/resolve/main/yolov8{}.safetensors?download=true";
pub static MODEL_SIZE: &str = "n";


pub fn transform_image(img: Vec<u8>, width: u32, height: u32) -> Option<Tensor> {
    let device = Device::new_cuda(0).unwrap_or(Device::Cpu);
    log(&format!("Device: {:?}", device));
    let dynimg =
        ImageBuffer::<Rgba<u8>, _>::from_vec(width, height, img).map(DynamicImage::ImageRgba8);

    if let Some(original_image) = dynimg {
        let (width, height) = {
            let w = original_image.width() as usize;
            let h = original_image.height() as usize;
            if w < h {
                let w = w * 640 / h;
                // Sizes have to be divisible by 32.
                (w / 32 * 32, 640)
            } else {
                let h = h * 640 / w;
                (640, h / 32 * 32)
            }
        };

        let image_t = {
            let img = original_image.resize_exact(
                width as u32,
                height as u32,
                image::imageops::FilterType::CatmullRom,
            );
            let data = img.to_rgb8().into_raw();
            Tensor::from_vec(
                data,
                (img.height() as usize, img.width() as usize, 3),
                &device,
            )
            .unwrap()
            .permute((2, 0, 1))
            .unwrap()
        };

        let image_t = (image_t.unsqueeze(0).unwrap().to_dtype(DType::F32).unwrap() * (1. / 255.)).unwrap();

        return Some(image_t);
    }
    None
}
