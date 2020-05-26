use actix_web::{
    error,
    web::{post, scope, Json},
    HttpRequest, HttpResponse, Scope,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct User {
    #[validate(email)]
    email: String,
}

// error handler for json body
pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;
    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

async fn user_validation(body: Json<User>) -> HttpResponse {
    let user = body.0;
    let validation: Result<(), validator::ValidationErrors> = user.validate();
    if validation.is_err() {
        let mut error_string = String::new();
        for (key, _) in validation.err().unwrap().field_errors() {
            error_string.push_str(key)
        }
        error_string.push_str(" are invalid");
        return HttpResponse::BadRequest().body(error_string);
    };
    HttpResponse::Ok().body(user.email)
}

pub fn register_validation_routes() -> Scope {
    scope("/validation").route("/user", post().to(user_validation))
}
