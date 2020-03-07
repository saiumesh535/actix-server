use actix_web::{web::{ scope, post }, Scope};

pub mod login;
pub mod register;

pub fn auth_routes() -> Scope {
    scope("/auth")
        .route("/login", post().to(login::user_login))
        .route("/register", post().to(register::register_user))
}
