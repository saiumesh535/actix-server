use actix_web::web::Data;
use r2d2::Pool;
use r2d2_redis::redis::{cmd, RedisResult, ToRedisArgs};
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::env::var;

pub type RedisClient = Pool<RedisConnectionManager>;

pub fn connect_redis() -> RedisClient {
    let manager = RedisConnectionManager::new(var("REDIS_URL").unwrap()).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    pool
}

pub fn set_redis<T: ToRedisArgs>(client: Data<RedisClient>, key: &String, payload: T) {
    let mut pool = client.get().unwrap();
    cmd("SET").arg(key).arg(payload).execute(&mut *pool)
}

pub fn get_redis(client: Data<RedisClient>, token: &str) -> RedisResult<String> {
    let mut pool = client.get().unwrap();
    cmd("GET").arg(token).query(&mut *pool)
}
