#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

extern crate tera;

mod routes;
use crate::routes::{errors, get, post, static_files};

// tera
use rocket_contrib::templates::Template;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                static_files::file,
                get::index,
                get::single_page_app,
                post::upload,
            ],
        )
        .register(catchers![errors::not_found])
}

fn main() {
    rocket().launch();
}
