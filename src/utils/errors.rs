use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use std::io;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("unable to find file `{0}`")]
    Disconnect(#[from] io::Error),
    #[error("`{0}`")]
    NoFileFound(String),
}


/// Actix web uses `ResponseError` for conversion of errors to a response
impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::Disconnect(error) => {
                HttpResponse::BadRequest().body(error.to_string())
            }
            CustomError::NoFileFound(error) => {
                HttpResponse::BadRequest().body(error)
            }
            _ => {
                println!("do some stuff related to CustomFour error");
                HttpResponse::InternalServerError().into()
            }
        }
    }
}