use serde::{ Serialize, Deserialize };
use actix_web::{Responder, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    pub payload: T
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String
}