#[macro_use]
extern crate rocket;
#[warn(unused_extern_crates)]
mod image_processing;
mod routes;
use crate::routes::{errors, get, post};

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get::index, get::robots, get::tea, post::upload])
        .mount("/static", FileServer::from("static/"))
        .register(
            "/",
            catchers![errors::catch_not_found, errors::catch_teapot],
        )
        .attach(Template::fairing())
}
