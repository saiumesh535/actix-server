use crate::models::auth::register::UserRegister;
use actix_web::{web::Json, HttpResponse, Responder};
use actix_web::web::Data;
use deadpool_postgres::Pool;
use crate::utils::password_hash::get_password_hash;
use crate::utils::response::{ErrorResponse, SuccessResponse};

pub async fn register_user(body: Json<UserRegister>, db: Data<Pool>) -> impl Responder {
    let user = body.0;
    let pool = db.get().await.unwrap();
    let password = get_password_hash(&user.password).hash;
    let query = r#"INSERT INTO users (email, username, password) VALUES ($1, $2, $3)"#;
    let rows = pool.query(query, &[&user.email, &user.username, &password]).await;
    match rows {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse{
            payload: String::from("successfully registered")
        }),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse{
            message: String::from(format!("{}", err))
        })
    }
}
