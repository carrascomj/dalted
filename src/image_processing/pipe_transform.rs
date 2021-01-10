use super::matrices::MATRICES;
use super::svg::decode_svg;
use base64::encode;
use image::{DynamicImage, Rgba};
use imageproc::map::map_colors;
use rayon::prelude::*;
use std::ops::Mul;
use ultraviolet::{Mat3, Vec3};

/// Decode incoming raw bytes image into a DynamicImage object.
/// It works for all format supported by `image`.
pub fn decode_raw_image(
    bytes: &[u8],
) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
    Ok(match image::load_from_memory(bytes) {
        Ok(img) => img,
        // try rendering a possible SVG
        Err(_) => image::load_from_memory_with_format(
            decode_svg(bytes)?.as_slice(),
            // we know that the bytes from decode_svg always correspond to PNG
            image::ImageFormat::Png,
        )?,
    })
}

/// Transform an image by applying 5 matrix transformation that correspond to different types of
/// color blindness.
/// The return type is a Result so that the errors can be communicated to the client.
/// # References
/// - Vietnol el al, 1999 http://vision.psychol.cam.ac.uk/jdmollon/papers/colourmaps.pdf
/// - Explanatory post https://ixora.io/projects/colorblindness/color-blindness-simulation-research/
pub fn pipe_matrix_multiplication(
    img: &DynamicImage,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(MATRICES
        .par_iter()
        .map(|mat| color_filter(&img, Mat3::from(*mat)).expect("works"))
        .collect::<Vec<String>>())
}

/// Tranform RGB values in linear space [0, 1] with a matrix and return normal RGB values [0, 255]
///
/// Here, `T` is a Vector of length 3 or a Matrix of dimension 3x3, which implement this custom
/// trait with the functions to do linear tranformations over the colors and apply functions to
/// them.
fn color_filter<T>(
    img: &DynamicImage,
    matrix: T,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
where
    T: Mul<Vec3, Output = Vec3> + Copy,
{
    let mut image_png = Vec::<u8>::new();
    DynamicImage::ImageRgba8(map_colors(img, |p| {
        if p[3] == 0 {
            // transformation is meaningless when opacity is 0
            p
        } else {
            let pix = [p.0[0] as f32, p.0[1] as f32, p.0[2] as f32];
            let v = (matrix * Vec3::from(pix).map(remove_gamma)).map(gamma_correction);
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
    // first, treat overflow out of the linear range
    if rgb_linear >= 1.0 {
        255.0
    } else if rgb_linear <= 0.0 {
        0.0
    } else if rgb_linear > 0.003_130_8 {
        269.025 * rgb_linear.powf(0.41666) - 14.025
    } else {
        3294.6 * rgb_linear
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use image::io::Reader;
    use ultraviolet::Mat3;

    #[test]
    fn color_jpg() {
        let image = Reader::open("tests/stickblind.jpg")
            .unwrap()
            .decode()
            .unwrap();
        color_filter(&image, Mat3::from(MATRICES[0])).unwrap();
    }

    #[test]
    fn color_png() {
        let image = Reader::open("tests/stickblind.png")
            .unwrap()
            .decode()
            .unwrap();
        color_filter(&image, Mat3::from(MATRICES[0])).unwrap();
    }
}
