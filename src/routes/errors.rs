use rocket::{http::RawStr, Request, serde::Serialize};
use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct CatchPathMap<'a> {
    path: &'a RawStr,
}

#[catch(404)]
pub fn catch_not_found(req: &Request) -> Template {
    let map = CatchPathMap {
        path: req.uri().path().raw(),
    };
    Template::render("not-found", &map)
}

#[catch(418)]
pub fn catch_teapot(req: &Request) -> Template {
    let map = CatchPathMap {
        path: req.uri().path().raw(),
    };
    Template::render("im-a-teapot", &map)
}
