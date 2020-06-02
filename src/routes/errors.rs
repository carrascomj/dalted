use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{web, Result};
use tera::Tera;

enum Wreck {
    On418,
    On404,
}

impl Wreck {
    fn err_string(&self) -> &str {
        match *self {
            Wreck::On418 => "MAY be short and stdout",
            Wreck::On404 => "Page not found",
        }
    }

    fn err_file(&self) -> &str {
        match *self {
            Wreck::On404 => "not-found.html",
            Wreck::On418 => "im-a-teapot.html",
        }
    }
}

// Custom error handlers, to return HTML responses when an error occurs.
pub fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Custom error handlers, to return HTML responses when a brew operation occurs.
pub fn error_handle_tea() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::IM_A_TEAPOT, im_a_teapot)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, Wreck::On404);
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Error handler for a 418 Page i'm a teapot.
pub fn im_a_teapot<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, Wreck::On418);
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: Wreck) -> Response<Body> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());
    match tera {
        Some(tera) => {
            let mut context = tera::Context::new();
            context.insert("path", res.request().path());
            let body = tera.render(error.err_file(), &context);

            match body {
                Ok(body) => Response::build(res.status())
                    .content_type("text/html")
                    .body(body),
                Err(_) => fallback(error.err_string()),
            }
        }
        None => fallback(error.err_string()),
    }
}
