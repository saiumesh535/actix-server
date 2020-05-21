use crate::utils::errors;
use actix_web::{
    web::{get, scope, Path},
    Error, HttpResponse, Scope,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    #[serde(rename(deserialize = "fileName"))]
    file_name: String,
}

async fn do_something(info: Path<Info>) -> Result<HttpResponse, Error> {
    let contents = std::fs::read_to_string(&info.file_name)?;
    Ok(HttpResponse::Ok().body(contents))
}

async fn with_custom_message(info: Path<Info>) -> Result<HttpResponse, Error> {
    let lol = std::fs::read_to_string(&info.file_name).map_err(|_e| {
        errors::CustomError::NoFileFound(format!("no file found with name {}", &info.file_name))
    })?;
    Ok(HttpResponse::Ok().body(lol))
}

pub fn register_error_handlers() -> Scope {
    scope("/error")
        .route("/default/{fileName}", get().to(do_something))
        .route("/custom/{fileName}", get().to(with_custom_message))
}
