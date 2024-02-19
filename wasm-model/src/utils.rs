use base64::{engine::general_purpose, Engine};
use std::io::Cursor;
use crate::log::log;
use crate::io::LAZY_MODEL;
use candle_core::{Device, Tensor, DType, Error, IndexOp};
use candle_nn::Module;
use crate::yolov8_model::Bbox;
use crate::coco_classes::NAMES;

pub static MODEL_SIZE: &str = "n";


pub fn transform_image(img: String) -> Option<Tensor> {
    let device = Device::new_cuda(0).unwrap_or(Device::Cpu);
    log(&format!("Device: {:?}", device));
    // let dynimg =
    //     ImageBuffer::<Rgba<u8>, _>::from_vec(width, height, img).map(DynamicImage::ImageRgba8);
    let base64_data = img.split(",").collect::<Vec<&str>>()[1];
    let data = general_purpose::STANDARD.decode(base64_data).unwrap();
    let dynimg = image::io::Reader::new(Cursor::new(data)).with_guessed_format().unwrap().decode();

    if let Ok(original_image) = dynimg {
        let (width, height) = {
            let w = original_image.width() as usize;
            let h = original_image.height() as usize;
            // Both w and h must be divisible by 32!
            (w, h)
            // if w < h {
            //     let w = w * 640 / h;
            //     // Sizes have to be divisible by 32.
            //     (w / 32 * 32, 640)
            // } else {
            //     let h = h * 640 / w;
            //     (640, h / 32 * 32)
            // }
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


pub fn annotate_image(img: String) -> Result<Tensor, Error> {
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
        return tensor
    }

    Err(Error::Msg("Model not found".to_string()))

}


pub fn identify_bboxes(pred: &Tensor, conf_threshold: f32, iou_threshold: f32) -> Result<Vec<Vec<Bbox>>, Error> {
    let (pred_size, npreds) = pred.dims2()?;
    let nclasses = pred_size - 4;
    let conf_threshold = conf_threshold.clamp(0.0, 1.0);
    let iou_threshold = iou_threshold.clamp(0.0, 1.0);

    let mut bboxes: Vec<Vec<Bbox>> = (0..nclasses).map(|_| vec![]).collect();

    for index in 0..npreds {
        let pred = Vec::<f32>::try_from(pred.i((.., index))?)?;
        let (class_idx, &max_conf)= pred[4..].iter().enumerate().max_by(|(_, x),  (_, y)| x.total_cmp(y)).unwrap();
        if max_conf > 0. && max_conf > conf_threshold {
            let bbox = Bbox {
                xmin: pred[0] - pred[2] / 2.,
                ymin: pred[1] - pred[3] / 2.,
                xmax: pred[0] + pred[2] / 2.,
                ymax: pred[1] + pred[3] / 2.,
                confidence: max_conf,
                keypoints: vec![],
            };
            log(&format!("Found {}", NAMES[class_idx]));
            bboxes[class_idx].push(bbox);
        } 
    }

    non_maximum_suppression(&mut bboxes, iou_threshold);

    Ok(bboxes)

}

fn non_maximum_suppression(bboxes: &mut [Vec<Bbox>], threshold: f32) {
    // Perform non-maximum suppression.
    for bboxes_for_class in bboxes.iter_mut() {
        bboxes_for_class.sort_by(|b1, b2| b2.confidence.partial_cmp(&b1.confidence).unwrap());
        let mut current_index = 0;
        for index in 0..bboxes_for_class.len() {
            let mut drop = false;
            for prev_index in 0..current_index {
                let iou = iou(&bboxes_for_class[prev_index], &bboxes_for_class[index]);
                if iou > threshold {
                    drop = true;
                    break;
                }
            }
            if !drop {
                bboxes_for_class.swap(current_index, index);
                current_index += 1;
            }
        }
        bboxes_for_class.truncate(current_index);
    }
}

fn iou(b1: &Bbox, b2: &Bbox) -> f32 {
    let b1_area = (b1.xmax - b1.xmin + 1.) * (b1.ymax - b1.ymin + 1.);
    let b2_area = (b2.xmax - b2.xmin + 1.) * (b2.ymax - b2.ymin + 1.);
    let i_xmin = b1.xmin.max(b2.xmin);
    let i_xmax = b1.xmax.min(b2.xmax);
    let i_ymin = b1.ymin.max(b2.ymin);
    let i_ymax = b1.ymax.min(b2.ymax);
    let i_area = (i_xmax - i_xmin + 1.).max(0.) * (i_ymax - i_ymin + 1.).max(0.);
    i_area / (b1_area + b2_area - i_area)
}