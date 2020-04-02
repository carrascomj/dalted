use base64::{decode, encode};
use image::imageops::filter3x3;
use image::io::Reader;
use std::io::Cursor;

type KERNEL = [f32; 9];
const MATRIX_BLUE: KERNEL = [1., 1., 1., 1., 1., 1., 1., 1., 1.];
const MATRIX_RED: KERNEL = [0., 0.4, 0.5, 0., 0.9, 0., 0.3, 0.3, 0.9];
const MATRICES: [KERNEL; 2] = [MATRIX_BLUE, MATRIX_RED];

pub fn pipe_matrix_multiplication(raw_data: String) -> Vec<String> {
    // let mut reader = Reader::new(Cursor::new(decode(raw_data).unwrap()));
    // reader.set_format(ImageFormat::Png);
    let reader = Reader::new(Cursor::new(decode(raw_data).unwrap()))
        .with_guessed_format()
        .expect("hhh.");
    let image = reader.decode().unwrap();

    let mut transformed: Vec<String> = vec![];
    for matrix in MATRICES.iter() {
        transformed.push(filter_stringed(&image, matrix));
    }
    transformed
}

fn filter_stringed(image: &image::DynamicImage, matrix: &KERNEL) -> String {
    encode(filter3x3(image, matrix).into_raw())
}
