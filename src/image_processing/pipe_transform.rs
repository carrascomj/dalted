use crate::image_processing::matrices::{KERNEL, MATRICES};
use base64::{decode, encode};
use image::io::Reader;
use image::{DynamicImage, Rgb};
use imageproc::map::map_colors;
use std::error::Error;
use std::io::Cursor;

/// Transform an image by applyng 5 matrix transformation that correspond to different types of
/// color blidness.
/// The return type is a Result so that the errors can be communicated to the client.
/// # References
/// - Vietnol el al, 1999 http://vision.psychol.cam.ac.uk/jdmollon/papers/colourmaps.pdf
/// - Explanatory post https://ixora.io/projects/colorblindness/color-blindness-simulation-research/
pub fn pipe_matrix_multiplication(raw_data: String) -> Result<Vec<String>, Box<dyn Error>> {
    let reader = Reader::new(Cursor::new(decode(raw_data)?))
        .with_guessed_format()
        .expect("hhh.");
    let img = reader.decode()?;

    let mut transformed: Vec<String> = vec![];
    for matrix in MATRICES.iter() {
        transformed.push(color_filter(&img, matrix)?)
    }
    Ok(transformed)
}

/// Tranform RGB values in linear space [0, 1] with a matrix and return normal RGB values [0, 255]
fn color_filter(img: &DynamicImage, filter: &KERNEL) -> Result<String, Box<dyn Error>> {
    let mut image_png = Vec::<u8>::new();
    DynamicImage::ImageRgb8(map_colors(img, |p| {
        let r = remove_gamma(&(p[0] as f32));
        let g = remove_gamma(&(p[1] as f32));
        let b = remove_gamma(&(p[2] as f32));
        Rgb([
            gamma_correction(filter[0] * r + filter[1] * g + filter[2] * b),
            gamma_correction(filter[3] * r + filter[4] * g + filter[5] * b),
            gamma_correction(filter[6] * r + filter[7] * g + filter[8] * b),
        ])
    }))
    .write_to(&mut image_png, image::ImageOutputFormat::Png)?;
    Ok(encode(image_png).to_string())
}

/// Transform RGB in [0, 255] to linear RGB [0, 1]
fn remove_gamma(rgb_a: &f32) -> f32 {
    if rgb_a > &0.04045 {
        (rgb_a / 269.025 + 0.0521327).powf(2.4)
    } else {
        rgb_a / 19.7368421
    }
}

/// Transform linear RGB [0, 1] back to RGB in [0, 255]
fn gamma_correction(rgb_linear: f32) -> u8 {
    let res;
    if rgb_linear > 0.0031308 {
        res = 269.025 * rgb_linear.powf(0.41666) - 14.025
    } else {
        res = 3294.6 * rgb_linear
    }
    // treat overflow of "very white" colors
    if res > 255.0 {
        255
    } else {
        res as u8
    }
}