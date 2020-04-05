use crate::image_proocessing::matrices::{KERNEL, MATRICES};
use base64::{decode, encode};
use image::io::Reader;
use image::{DynamicImage, Rgb};
use imageproc::map::map_colors;
use std::io::Cursor;

pub fn pipe_matrix_multiplication(raw_data: String) -> Vec<String> {
    // let mut reader = Reader::new(Cursor::new(decode(raw_data).unwrap()));
    // reader.set_format(ImageFormat::Png);
    let reader = Reader::new(Cursor::new(decode(raw_data).unwrap()))
        .with_guessed_format()
        .expect("hhh.");
    let img = reader.decode().unwrap();

    let mut transformed: Vec<String> = vec![];
    for matrix in MATRICES.iter() {
        transformed.push(color_filter(&img, matrix))
    }
    transformed
}

fn color_filter(img: &DynamicImage, filter: &KERNEL) -> String {
    let mut image_png = Vec::<u8>::new();
    DynamicImage::ImageRgb8(map_colors(img, |p| {
        let r = p[0] as f32;
        let g = p[1] as f32;
        let b = p[2] as f32;
        Rgb([
            (filter[0] * r) as u8 + (filter[1] + g) as u8 + (filter[2] * b) as u8,
            (filter[3] * r) as u8 + (filter[4] + g) as u8 + (filter[5] * b) as u8,
            (filter[6] * r) as u8 + (filter[7] + g) as u8 + (filter[8] * b) as u8,
        ])
    }))
    .write_to(&mut image_png, image::ImageOutputFormat::Png).unwrap();
    encode(image_png).to_string()
}
