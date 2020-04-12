#![feature(proc_macro_hygiene, decl_macro)]
#![feature(test)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

extern crate tera;

mod image_processing;
mod routes;
use crate::routes::{errors, get, post, static_files};

// tera
use rocket_contrib::templates::Template;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![static_files::file, get::index, post::upload,])
        .register(catchers![errors::not_found])
}

fn main() {
    rocket().launch();
}
