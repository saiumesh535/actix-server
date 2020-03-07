
use dotenv;
use actix_web::{App, HttpServer };
use deadpool_postgres::{Config};
use tokio_postgres::{NoTls};
use crate::utils::redis_utils::connect_redis;

// json - postgres example
mod Json;

// user authentication
mod auth;

mod models;
mod utils;
mod types;

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
            .data(pool.clone())
            .data(redis_client.clone())
            .service(auth::auth_routes())
            .service(Json::json_routes())
    }).bind("127.0.0.1:8000")?.run().await
}
