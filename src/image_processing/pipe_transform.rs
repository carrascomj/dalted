use super::matrices::{Kernel, Matops3, Vec3, MATRICES};
use base64::{decode, encode};
use image::io::Reader;
use image::{DynamicImage, Rgba};
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
    let reader = Reader::new(Cursor::new(decode(raw_data)?)).with_guessed_format()?;
    let img = reader.decode()?;

    let mut transformed: Vec<String> = vec![];
    for matrix in MATRICES.iter() {
        transformed.push(color_filter(&img, Kernel::<f32>::new(*matrix))?);
    }
    Ok(transformed)
}

/// Tranform RGB values in linear space [0, 1] with a matrix and return normal RGB values [0, 255]
pub fn color_filter(img: &DynamicImage, matrix: Kernel<f32>) -> Result<String, Box<dyn Error>> {
    let mut image_png = Vec::<u8>::new();
    DynamicImage::ImageRgba8(map_colors(img, |p| {
        if p[3] == 0 {
            // transformation is meaningless when opacity is 100%
            p
        } else {
            let v = matrix
                .vecmul(Vec3::<f32>::from(p.0).apply(remove_gamma))
                .apply(gamma_correction)
                .cont();
            Rgba([v[0] as u8, v[1] as u8, v[2] as u8, p[3]])
        }
    }))
    // faster among tested (Png, Jpeg, Gif) is Bmp but images were too large
    .write_to(&mut image_png, image::ImageOutputFormat::Png)?;
    Ok(encode(image_png))
}

/// Transform RGB in [0, 255] to linear RGB [0, 1]
fn remove_gamma(rgb_a: f32) -> f32 {
    if rgb_a > 0.04045 {
        (rgb_a / 269.025 + 0.052_132_7).powf(2.4)
    } else {
        rgb_a / 19.73684
    }
}

/// Transform linear RGB [0, 1] back to RGB in [0, 255]
fn gamma_correction(rgb_linear: f32) -> f32 {
    let res;
    if rgb_linear > 0.003_130_8 {
        res = 269.025 * rgb_linear.powf(0.41666) - 14.025
    } else {
        res = 3294.6 * rgb_linear
    }
    // treat overflow of "very white" colors
    if res > 255.0 {
        255.0
    } else {
        res
    }
}

#[cfg(test)]
mod tests {

    extern crate test;
    use super::*;
    use crate::image_processing::matrices::Kernel;
    use test::Bencher;

    #[bench]
    fn color_jpg(b: &mut Bencher) {
        let image = Reader::open("tests/stickblind.jpg")
            .unwrap()
            .decode()
            .unwrap();
        b.iter(|| {
            color_filter(&image, Kernel::<f32>::new(MATRICES[0])).unwrap();
        });
    }

    #[bench]
    fn color_png(b: &mut Bencher) {
        let image = Reader::open("tests/stickblind.png")
            .unwrap()
            .decode()
            .unwrap();
        b.iter(|| {
            color_filter(&image, Kernel::<f32>::new(MATRICES[0])).unwrap();
        });
    }
}
