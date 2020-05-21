use actix_web::{
    web::{get, scope},
    HttpResponse, Scope,
};

async fn private() -> HttpResponse {
    HttpResponse::Ok().body("Hello World!")
}

pub fn register_private() -> Scope {
    scope("/private").route("/", get().to(private))
}
