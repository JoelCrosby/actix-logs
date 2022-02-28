use crate::handlers::{users, create_user, login, get_user_by_id};
use crate::user::CreateUserRequest;
use crate::login::LoginRequest;
use crate::Pool;


use actix_web::{Error, web, get, post, HttpResponse, Responder};

#[get("/logs")]
pub async fn get_logs() -> impl Responder {
  HttpResponse::Ok().body("GET /api/logs")
}

#[get("/logs/{id}")]
pub async fn get_log(id: web::Path<u32>) -> impl Responder {
  HttpResponse::Ok().body(format!("GET /api/logs/{}", id))
}

#[get("/users")]
pub async fn get_users(
  pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
  let result = web::block(move || users(pool))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(result))
}

#[get("/users/{id}")]
pub async fn get_user(
  pool: web::Data<Pool>,
  id: web::Path<i32>
) -> Result<HttpResponse, Error> {

  let result = web::block(move || get_user_by_id(pool, id))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(result))
}

#[post("/users")]
pub async fn post_user(
  pool: web::Data<Pool>,
  request: web::Json<CreateUserRequest>
) -> Result<HttpResponse, Error> {

  let result = web::block(move || create_user(pool, request))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Created().json(result))
}


#[post("/login")]
pub async fn user_login(
  pool: web::Data<Pool>,
  request: web::Json<LoginRequest>
) -> Result<HttpResponse, Error> {

  let result = web::block(move || login(pool, request))
      .await
      .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(result.unwrap()))
}

