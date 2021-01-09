use crate::image_processing::{decode_raw_image, pipe_matrix_multiplication};
use actix_web::{web, Error, HttpResponse};

#[derive(Serialize)]
pub struct Images {
    file_type: String,
    images: Vec<String>,
    message: String,
}

/// Receive an image and respond with a vector of 5 transformed images
pub async fn upload(bytes: web::Bytes) -> Result<HttpResponse, Error> {
    // decode raw stream
    let image = web::block(move || decode_raw_image(&bytes)).await?;
    // backend logic here
    let images = web::block(move || pipe_matrix_multiplication(&image)).await?;
    Ok(HttpResponse::Ok().json(Images {
        file_type: String::from("image/png"),
        images,
        message: String::from("OK!"),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::HttpResponse;
    use image::io::Reader;

    #[test]
    fn test_post() {
        let image = Reader::open("tests/stickblind.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let images = pipe_matrix_multiplication(&image).unwrap();
        HttpResponse::Ok().json(Images {
            file_type: String::from("image/png"),
            images,
            message: String::from("OK!"),
        });
    }
}
