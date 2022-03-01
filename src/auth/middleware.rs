use crate::jwt;
use crate::Pool;
use actix_web::body::EitherBody;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::{
        header::{HeaderName, HeaderValue},
        Method,
    },
    web::Data,
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

use crate::{constants, models::response::ResponseBody};

pub struct Jwt;

impl<S, B> Transform<S, ServiceRequest> for Jwt
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddleware { service }))
    }
}

pub struct JwtMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut request: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;

        // Bypass some account routes

        request.headers_mut().append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );

        loop {
            if Method::OPTIONS == *request.method() {
                authenticate_pass = true;
                break;
            }

            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if request.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }

            if authenticate_pass {
                break;
            }

            let pool = match request.app_data::<Data<Pool>>() {
                Some(pool) => pool,
                None => break,
            };

            info!("connecting to database...");

            let authen_header = match request.headers().get(constants::AUTHORIZATION) {
                Some(pool) => pool,
                None => break,
            };

            info!("parsing authorization header...");

            let authen_str = match authen_header.to_str() {
                Ok(pool) => pool,
                Err(_) => break,
            };

            if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                info!("parsing token...");

                let token = authen_str[6..authen_str.len()].trim();
                let token_data = match jwt::decode_jwt(&token.to_string()) {
                    Ok(pool) => pool,
                    Err(_) => break,
                };

                info!("decoding token...");

                if jwt::verify_jwt(&token_data, pool).is_ok() {
                    info!("valid token");
                    authenticate_pass = true;
                } else {
                    panic!("invalid token");
                }
            }

            break;
        }

        if authenticate_pass {
            let res = self.service.call(request);
            return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
        }

        let (request, _pl) = request.into_parts();
        let response = HttpResponse::Unauthorized()
            .json(ResponseBody::new(
                constants::MESSAGE_INVALID_TOKEN,
                constants::EMPTY,
            ))
            .map_into_right_body();

        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
    }
}
