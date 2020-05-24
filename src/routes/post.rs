use crate::image_processing::pipe_matrix_multiplication;
use actix_web::{web, Error, HttpResponse};
use base64::decode;
use futures::StreamExt;
use image::io::Reader;
use std::io::Cursor;

#[derive(Serialize)]
pub struct Images {
    file_type: String,
    images: Vec<String>,
    message: String,
}

/// Receive an image and respond with a vector of 5 transformed images
pub async fn upload(mut stream: web::Payload) -> Result<HttpResponse, Error> {
    // decode raw stream
    let mut bytes = web::BytesMut::new();
    while let Some(item) = stream.next().await {
        bytes.extend_from_slice(&item?);
    }
    // for some reason, base64 decoding fails for certain PNG images
    let image = web::block(move || {
        let reader = Reader::new(Cursor::new(decode(&bytes).unwrap())).with_guessed_format()?;
        reader.decode()
    })
    .await?;
    // backend logic here
    let images = web::block(move || pipe_matrix_multiplication(&image)).await?;
    Ok(HttpResponse::Ok().json(Images {
        file_type: String::from("oklahoma"),
        images,
        message: String::from("good_morning"),
    }))
}
