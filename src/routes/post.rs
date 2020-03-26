use rocket::request::LenientForm;

#[derive(FromForm)]
pub struct Image {
    // need to inspect what the AJAX post image looks like
    complete: bool,
}

#[post("/img_upload", data="<image>")]
pub fn upload(image: LenientForm<Image>) -> String {
    // here the back end should do some matrix multiplications over the pixels
    String::from("Hello... world?")
}
