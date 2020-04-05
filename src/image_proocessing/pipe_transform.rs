use crate::image_proocessing::matrices::MATRICES;
use base64::{decode, encode};
use image::io::Reader;
use std::io::Cursor;

pub fn pipe_matrix_multiplication(raw_data: String) -> Vec<String> {
    // let mut reader = Reader::new(Cursor::new(decode(raw_data).unwrap()));
    // reader.set_format(ImageFormat::Png);
    let reader = Reader::new(Cursor::new(decode(raw_data).unwrap()))
        .with_guessed_format()
        .expect("hhh.");
    let image = reader.decode().unwrap();

    let mut transformed: Vec<String> = vec![];
    for matrix in MATRICES.iter() {
        let mut image_png = Vec::<u8>::new();
        image
            .filter3x3(matrix)
            .write_to(&mut image_png, image::ImageOutputFormat::Png)
            .unwrap();
        transformed.push(encode(image_png).to_string())
    }
    transformed
}

// fn filter_stringed(image: &image::DynamicImage, matrix: &KERNEL) -> &image::DynamicImage {
//     filter3x3(image, matrix)
// }
