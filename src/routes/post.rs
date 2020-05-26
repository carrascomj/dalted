use crate::image_processing::{decode_raw_image, pipe_matrix_multiplication};
use actix_web::{error, web, Error, HttpResponse};
use futures::StreamExt;

const MAX_SIZE: usize = 3_145_728;

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
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        if (bytes.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("Image was too big!"));
        }

        bytes.extend_from_slice(&chunk);
    }
    let image = web::block(move || decode_raw_image(&bytes.freeze())).await?;
    // backend logic here
    let images = web::block(move || pipe_matrix_multiplication(&image)).await?;
    Ok(HttpResponse::Ok().json(Images {
        file_type: String::from("image/png"),
        images,
        message: String::from("OK!"),
    }))
}
