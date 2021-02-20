use crate::handlers::{users, create_user, login, get_user_by_id};
use crate::server_error::{ServerError};
use crate::user::CreateUserRequest;
use crate::login::LoginRequest;
use crate::Pool;

use anyhow::Result;
use actix_web::{Error, web, get, post, HttpResponse, Responder};

#[get("/logs")]
pub async fn get_logs() -> impl Responder {
  HttpResponse::Ok().body("GET /api/logs")
}

#[get("/logs/{id}")]
pub async fn get_log(web::Path(id): web::Path<u32>) -> impl Responder {
  HttpResponse::Ok().body(format!("GET /api/logs/{}", id))
}

#[get("/users")]
pub async fn get_users(
  pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
  Ok(web::block(move || users(pool))
    .await
    .map(|result| HttpResponse::Ok().json(result))
    .map_err(|_| HttpResponse::InternalServerError()
      .json(ServerError {
        message: "CreateUser Failed."
      })
    )?
  )
}

#[get("/users/{id}")]
pub async fn get_user(
  pool: web::Data<Pool>,
  id: web::Path<u32>
) -> Result<HttpResponse, Error> {

  Ok(web::block(move || get_user_by_id(pool, id))
  .await
  .map(|result| HttpResponse::Created().json(result))
  .map_err(|_| HttpResponse::InternalServerError()
    .json(ServerError {
      message: "CreateUser Failed."
    })
  )?
)
}

#[post("/users")]
pub async fn post_user(
  pool: web::Data<Pool>,
  request: web::Json<CreateUserRequest>
) -> Result<HttpResponse, Error> {

  Ok(web::block(move || create_user(pool, request))
      .await
      .map(|result| HttpResponse::Created().json(result))
      .map_err(|_| HttpResponse::InternalServerError()
        .json(ServerError {
          message: "CreateUser Failed."
        })
      )?
  )
}


#[post("/login")]
pub async fn user_login(
  pool: web::Data<Pool>,
  request: web::Json<LoginRequest>
) -> Result<HttpResponse, Error> {

  Ok(web::block(move || login(pool, request))
      .await
      .map(|result| HttpResponse::Ok().json(result))
      .map_err(|_| HttpResponse::InternalServerError()
        .json(ServerError {
          message: "Failed to authenticate user."
        })
      )?
  )
}

