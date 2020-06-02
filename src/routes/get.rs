use actix_web::http::StatusCode;
use actix_web::{error, get, Error, HttpResponse, Result};

#[get("/")]
pub async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/index.html")))
}

#[get("/robots.txt")]
pub async fn robots() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/plain")
        .body(include_str!("../../static/robots.txt")))
}

#[get("/teapot")]
pub async fn tea() -> Result<&'static str, Error> {
    Err(error::ErrorImATeapot("I'm a teapot!"))
}
