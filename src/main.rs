
use dotenv;
use actix_web::{App, HttpServer };
use deadpool_postgres::{Config};
use tokio_postgres::{NoTls};

// json - postgres example
mod Json;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // loading .env file
    dotenv::dotenv().ok();

    // creating postgres pool connection
    let cfg = Config::from_env("PG").unwrap();
    let pool = cfg.create_pool(NoTls).unwrap();
    pool.get().await.unwrap();

    // actix server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(Json::json_routes())
    }).bind("127.0.0.1:8000")?.run().await
}
