use crate::utils::email_sender;
use actix_web::{
    web::{get, route, scope, Path},
    HttpResponse, Scope,
};
use std::fs::read_to_string;

async fn send_email(info: Path<String>) -> HttpResponse {
    let content = read_to_string("./email-templates/welcome.html").unwrap();
    email_sender::send_email(info.to_string(), content);
    HttpResponse::Ok().body(info.to_string())
}

pub fn register_email_routes() -> Scope {
    scope("/email").route("/{email}", get().to(send_email))
}
