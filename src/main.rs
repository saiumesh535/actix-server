#[macro_use]
extern crate validator_derive;
extern crate validator;

use crate::utils::redis_utils::connect_redis;
use actix_files::Files;
use actix_web::{middleware::Compress, web, App, FromRequest, HttpServer};
use deadpool_postgres::Config;
use dotenv;
use tokio_postgres::NoTls;

// json - postgres example
mod Json;

// user authentication
mod auth;

mod models;
mod types;
mod utils;

// private routes
mod private;

// middleware
mod middleware;

// errors example
mod errors;

// downloads
mod downloads;

mod email;

mod validations;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // loading .env file
    dotenv::dotenv().ok();

    // creating postgres pool connection
    let cfg = Config::from_env("PG").unwrap();
    let pool = cfg.create_pool(NoTls).unwrap();
    pool.get().await.unwrap();

    // redis cache
    let redis_client = connect_redis();

    // actix server
    HttpServer::new(move || {
        App::new()
            // you can replace () this with any specific struct type
            .app_data(web::Json::<()>::configure(|cfg| {
                cfg.error_handler(validations::json_error_handler)
            }))
            .data(pool.clone())
            .data(redis_client.clone())
            .wrap(Compress::default())
            .service(auth::auth_routes())
            .service(Json::json_routes())
            .service(errors::register_error_handlers())
            .service(email::register_email_routes())
            .service(downloads::register_download_routes())
            .service(private::register_private().wrap(middleware::private::CheckToken))
            .service(validations::register_validation_routes())
            .service(Files::new("/", "static").index_file("index.html"))
    })
    .keep_alive(75)
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
