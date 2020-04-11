use crate::image_processing::pipe_matrix_multiplication;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct Image {
    file_type: String,
    image: String,
    message: String,
}

#[derive(Serialize)]
pub struct Images {
    file_type: String,
    images: Vec<String>,
    message: String,
}

/// Receive an image and respond with a vector of 5 transformed images
#[post("/img_upload", data = "<image>")]
pub fn upload(image: Json<Image>) -> Result<Json<Images>, status::Custom<String>> {
    match pipe_matrix_multiplication(image.0.image) {
        Ok(images) => Ok(Json(Images {
            file_type: image.0.file_type,
            images,
            message: format!("Passed message: {}", image.0.message),
        })),
        Err(_) => Err(status::Custom(
            Status::NotAcceptable,
            String::from("Image couldn't be parsed!"),
        )),
    }
}
