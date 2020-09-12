extern crate actix_web;
#[macro_use]
extern crate serde_derive;
#[warn(unused_extern_crates)]
mod image_processing;
mod routes;
use crate::routes::{errors, get, post};

// tera
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            // .wrap(middleware::Logger::default())
            .data(tera)
            .service(fs::Files::new("/static", "./static/"))
            .service(get::index)
            .service(get::robots)
            .service(get::tea)
            .service(web::resource("/img_upload").route(web::post().to(post::upload)))
            .service(web::scope("").wrap(errors::error_handlers()))
            .wrap(errors::error_handle_tea())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
