use rocket::fs::{relative, NamedFile};
use rocket::{http::Status, response::status};
use std::path::Path;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("static")).join("index.html");

    NamedFile::open(path).await.ok()
}

#[get("/robots.txt")]
pub fn robots() -> status::Accepted<&'static str> {
    status::Accepted(Some(include_str!("../../static/robots.txt")))
}

#[get("/teapot")]
pub fn tea() -> Status {
    Status::ImATeapot
}
