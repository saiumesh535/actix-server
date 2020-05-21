use actix_web::{
    web::{get, post, scope},
    Scope,
};

mod json_examples;

pub fn json_routes() -> Scope {
    scope("/json")
        .route("/json_body", post().to(json_examples::json_body))
        .route(
            "/insert_json_body",
            post().to(json_examples::insert_json_body),
        )
        .route(
            "/read_json_body",
            get().to(json_examples::read_json_from_db),
        )
}
