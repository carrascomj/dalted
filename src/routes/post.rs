use crate::image_processing::{decode_raw_image, pipe_matrix_multiplication};
use rocket::data::{Data, ToByteUnit};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json, Serialize};
use rocket::tokio::task::spawn_blocking;

const MAX_SIZE: usize = 3;

#[derive(Serialize)]
pub struct Images {
    file_type: String,
    images: Vec<String>,
    message: String,
}

/// Receive an image and respond with a vector of 5 transformed images
#[post("/img_upload", data = "<bytes>")]
pub async fn upload<'a>(bytes: Data<'_>) -> Result<Json<Images>, status::Custom<String>> {
    // decode raw stream
    let bytes = bytes
        .open(MAX_SIZE.mebibytes())
        .into_bytes()
        .await
        .map_err(|e| status::Custom(Status::NotAcceptable, e.to_string()))?;
    let images = spawn_blocking(move || {
        pipe_matrix_multiplication(&decode_raw_image(bytes.into_inner().as_slice())?)
    })
    .await
    // not sure about how to do this properly
    .map_err(|e| status::Custom(Status::NotAcceptable, e.to_string()))?
    .map_err(|e| status::Custom(Status::NotAcceptable, e.to_string()))?;
    Ok(Json(Images {
        file_type: String::from("image/png"),
        images,
        message: String::from("OK!"),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::io::Reader;
    use rocket::serde::json::Json;

    #[test]
    fn test_post() {
        let image = Reader::open("tests/stickblind.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let images = pipe_matrix_multiplication(&image).unwrap();
        Json(Images {
            file_type: String::from("image/png"),
            images,
            message: String::from("OK!"),
        });
    }
}
