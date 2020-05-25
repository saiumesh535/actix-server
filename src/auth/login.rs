use crate::types::token::UserLoginToken;
use crate::utils::password_hash::verify_password;
use crate::utils::redis_utils::{set_redis, RedisClient};
use crate::utils::response::{ErrorResponse, LoginResponse};
use crate::utils::uuid_utils::get_uuid;
use actix_web::web::Data;
use actix_web::{web::Json, HttpResponse, Responder};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::to_value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

pub async fn user_login(
    body: Json<UserLogin>,
    db: Data<Pool>,
    redis: Data<RedisClient>,
) -> impl Responder {
    let user = body.0;
    let query = r#"SELECT id, email, password FROM users WHERE email = $1"#;
    let pool = db.get().await.unwrap();
    let rows = pool.query(query, &[&user.email]).await.unwrap();
    if rows.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: String::from("check username and password"),
        });
    }
    let hash: String = rows.get(0).unwrap().get("password");
    let is_password_matches = verify_password(&hash, &user.password);
    if !is_password_matches {
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: String::from("check username and password"),
        });
    }
    let token = get_uuid();
    let token_payload = to_value(UserLoginToken {
        id: rows.get(0).unwrap().get("id"),
        email: rows.get(0).unwrap().get("email"),
    })
    .unwrap()
    .to_string();
    set_redis::<String>(redis, &token, token_payload, None);
    HttpResponse::Ok().json(LoginResponse { token })
}
