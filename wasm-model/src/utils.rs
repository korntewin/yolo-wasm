use crate::coco_classes::NAMES as COCOE_CLASS_NAMES;
use crate::log::log;
use crate::yolov8_model::Bbox;
use base64::{engine::general_purpose, Engine};
use candle_core::{DType, Device, IndexOp, Result, Tensor};
use image::DynamicImage;
use image::ImageOutputFormat;
use image::ImageResult;
use std::io::Cursor;

pub static MODEL_SIZE: &str = "n";
pub static LEGEND_SIZE: u32 = 14;
// Optimal width and height to maximize model speed
pub static OPTIMAL_WIDTH: f32 = 256.;
pub static OPTIMAL_HEIGHT: f32 = 256.;

pub fn get_dyn_image(img: &str) -> ImageResult<DynamicImage> {
    let base64_data = img.split(",").collect::<Vec<&str>>()[1];
    let data = general_purpose::STANDARD.decode(base64_data).unwrap();
    let dynimg = image::io::Reader::new(Cursor::new(data))
        .with_guessed_format()
        .unwrap()
        .decode();
    dynimg
}

pub fn transform_image(img: String) -> Option<Tensor> {
    let device = Device::new_cuda(0).unwrap_or(Device::Cpu);
    log(&format!("Device: {:?}", device));
    // let dynimg =
    //     ImageBuffer::<Rgba<u8>, _>::from_vec(width, height, img).map(DynamicImage::ImageRgba8);
    let dynimg = get_dyn_image(&img);

    if let Ok(original_image) = dynimg {
        let (width, height) = {
            // Both w and h must be divisible by 32!
            (OPTIMAL_WIDTH, OPTIMAL_HEIGHT)
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

        let image_t =
            (image_t.unsqueeze(0).unwrap().to_dtype(DType::F32).unwrap() * (1. / 255.)).unwrap();

        return Some(image_t);
    }
    None
}

pub fn identify_bboxes(
    pred: &Tensor,
    conf_threshold: f32,
    iou_threshold: f32,
) -> Result<Vec<Vec<Bbox>>> {
    let (pred_size, npreds) = pred.dims2()?;
    let nclasses = pred_size - 4;
    let conf_threshold = conf_threshold.clamp(0.0, 1.0);
    let iou_threshold = iou_threshold.clamp(0.0, 1.0);

    let mut bboxes: Vec<Vec<Bbox>> = (0..nclasses).map(|_| vec![]).collect();

    for index in 0..npreds {
        let pred = Vec::<f32>::try_from(pred.i((.., index))?)?;
        let (class_idx, &max_conf) = pred[4..]
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.total_cmp(y))
            .unwrap();
        if max_conf > 0. && max_conf > conf_threshold {
            let bbox = Bbox {
                xmin: pred[0] - pred[2] / 2.,
                ymin: pred[1] - pred[3] / 2.,
                xmax: pred[0] + pred[2] / 2.,
                ymax: pred[1] + pred[3] / 2.,
                confidence: max_conf,
                keypoints: vec![],
            };
            log(&format!("Found {}", COCOE_CLASS_NAMES[class_idx]));
            bboxes[class_idx].push(bbox);
        }
    }

    non_maximum_suppression(&mut bboxes, iou_threshold);

    Ok(bboxes)
}

pub fn annotate_images(
    orig_img: DynamicImage,
    w: f32,
    h: f32,
    bboxes: &[Vec<Bbox>],
) -> Result<DynamicImage> {
    // Annotate the original image and print boxes information.
    let (initial_h, initial_w) = (orig_img.height(), orig_img.width());
    let w_ratio = initial_w as f32 / w as f32;
    let h_ratio = initial_h as f32 / h as f32;
    let mut img = orig_img.to_rgb8();
    let font = Vec::from(include_bytes!("roboto-mono-stripped.ttf") as &[u8]);
    let font = rusttype::Font::try_from_vec(font);
    for (class_index, bboxes_for_class) in bboxes.iter().enumerate() {
        for b in bboxes_for_class.iter() {
            log(&format!("{}: {:?}", COCOE_CLASS_NAMES[class_index], b));
            let xmin = (b.xmin * w_ratio) as i32;
            let ymin = (b.ymin * h_ratio) as i32;
            let dx = (b.xmax - b.xmin) * w_ratio;
            let dy = (b.ymax - b.ymin) * h_ratio;
            if dx >= 0. && dy >= 0. {
                imageproc::drawing::draw_hollow_rect_mut(
                    &mut img,
                    imageproc::rect::Rect::at(xmin, ymin).of_size(dx as u32, dy as u32),
                    image::Rgb([255, 0, 0]),
                );
            }
            if LEGEND_SIZE > 0 {
                if let Some(font) = font.as_ref() {
                    imageproc::drawing::draw_filled_rect_mut(
                        &mut img,
                        imageproc::rect::Rect::at(xmin, ymin).of_size(dx as u32, LEGEND_SIZE),
                        image::Rgb([170, 0, 0]),
                    );
                    let legend = format!(
                        "{}   {:.0}%",
                        COCOE_CLASS_NAMES[class_index],
                        100. * b.confidence
                    );
                    imageproc::drawing::draw_text_mut(
                        &mut img,
                        image::Rgb([255, 255, 255]),
                        xmin,
                        ymin,
                        rusttype::Scale::uniform(LEGEND_SIZE as f32 - 1.),
                        font,
                        &legend,
                    )
                }
            }
        }
    }

    Ok(DynamicImage::ImageRgb8(img))
}

pub fn img_to_base64(img: DynamicImage) -> ImageResult<String> {
    let mut image_data: Vec<u8> = vec![];
    let mut buf = Cursor::new(&mut image_data);
    img.write_to(&mut buf, ImageOutputFormat::Jpeg(100))?;
    let base64_data = general_purpose::STANDARD.encode(&image_data);
    Ok(format!("data:image/jpeg;base64,{}", base64_data))
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
