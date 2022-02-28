// use crate::{error::Error};

// use chrono::Utc;
// use serde::{Serialize, Deserialize};
// use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
// use anyhow::Result;

// const BEARER: &str = "Bearer ";
// const JWT_SECRET: &[u8] = b"secret";


// /// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     sub: String,
//     exp: usize,
// }


// pub fn create_jwt(uid: &str) -> Result<String, Error> {
//     let expiration = Utc::now()
//         .checked_add_signed(chrono::Duration::seconds(60))
//         .expect("valid timestamp")
//         .timestamp();

//     let claims = Claims {
//         sub: uid.to_owned(),
//         exp: expiration as usize,
//     };

//     let header = Header::new(Algorithm::HS512);

//     encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
//         .map_err(|_| Error::JWTTokenCreationError)
// }

// // pub fn decode_jwt(token: &String) -> bool {
// //     let token = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
// // }

// // #[derive(Serialize)]
// // pub struct LoginResponse {
// //     pub token: String,
// // }

// // use std::{future::{Future, Ready}, pin::Pin};
// // use std::task::{Context, Poll};

// // use actix_service::{Service, Transform};
// // use actix_web::{dev::ServiceRequest, dev::ServiceResponse};


// // pub struct JwtAuth;

// // impl<S, B> Transform<S> for JwtAuth
// // where
// //     S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
// //     S::Future: 'static,
// //     B: 'static,
// // {
// //     type Request = ServiceRequest;
// //     type Response = ServiceResponse<B>;
// //     type Error = Error;
// //     type InitError = ();
// //     type Transform = JwtAuthiddleware<S>;
// //     type Future = Ready<Result<Self::Transform, Self::InitError>>;

// //     fn new_transform(&self, service: S) -> Self::Future {
// //         ok(JwtAuthiddleware { service })
// //     }
// // }

// // pub struct JwtAuthiddleware<S> {
// //     service: S,
// // }

// // impl<S, B> Service for JwtAuthiddleware<S>
// // where
// //     S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
// //     S::Future: 'static,
// //     B: 'static,
// // {
// //     type Request = ServiceRequest;
// //     type Response = ServiceResponse<B>;
// //     type Error = Error;
// //     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

// //     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
// //         self.service.poll_ready(cx)
// //     }

// //     fn call(&mut self, req: ServiceRequest) -> Self::Future {
// //         println!("Hi from start. You requested: {}", req.path());

// //         self.service.call(req);
// //     }
// // }
