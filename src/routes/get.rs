use rocket::response::NamedFile;
use std::io;

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/robots.txt")]
pub fn robots() -> io::Result<NamedFile> {
    NamedFile::open("static/robots.txt")
}
