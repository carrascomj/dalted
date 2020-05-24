use actix_web::{get, HttpResponse, Result};
use actix_web::http::{StatusCode};

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
