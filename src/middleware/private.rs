use std::task::{Context, Poll};

use crate::utils::redis_utils::{get_redis, RedisClient};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, Ready};
use r2d2_redis::redis::RedisResult;

pub struct CheckToken;

impl<S, B> Transform<S> for CheckToken
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckTokenMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckTokenMiddleware { service })
    }
}

pub struct CheckTokenMiddleware<S> {
    service: S,
}

pub fn verify_token(req: &ServiceRequest, token: &str) -> RedisResult<String> {
    let redis: Data<RedisClient> = req.app_data().unwrap();
    get_redis(redis, token)
}

impl<S, B> Service for CheckTokenMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization");
        if token == None {
            return Either::Right(ok(req.into_response(
                HttpResponse::Unauthorized()
                    .body("Token is missing") // you can swap body with json
                    .into_body(),
            )));
        }
        let is_valid_token = verify_token(&req, token.unwrap().to_str().unwrap());
        if is_valid_token.is_err() {
            Either::Right(ok(req.into_response(
                HttpResponse::Unauthorized()
                    .body("Check token") // you can swap body with json
                    .into_body(),
            )))
        } else {
            Either::Left(self.service.call(req))
        }
    }
}
