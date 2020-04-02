use crate::image_proocessing::pipe_transform::pipe_matrix_multiplication;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct Image {
    // need to inspect what the AJAX post image looks like
    file_type: String,
    image: String,
    message: String,
}

#[derive(Serialize)]
pub struct Images {
    // need to inspect what the AJAX post image looks like
    file_type: String,
    images: Vec<String>,
    message: String,
}

#[post("/img_upload", data = "<image>")]
pub fn upload(image: Json<Image>) -> Json<Images> {
    // here the back end should do some matrix multiplications over the pixels
    Json(Images {
        file_type: image.0.file_type,
        images: pipe_matrix_multiplication(image.0.image),
        message: format!("Passed message: {}", image.0.message),
    })
}
