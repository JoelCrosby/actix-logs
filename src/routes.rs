use crate::login::LoginRequest;
use crate::models::user::CreateUserRequest;
use crate::models::user::UserEntity;
use crate::Pool;

use actix_web::{get, post, web, Error, HttpResponse, Responder};

#[get("/logs")]
pub async fn get_logs() -> impl Responder {
  HttpResponse::Ok().body("GET /api/logs")
}

#[get("/logs/{id}")]
pub async fn get_log(id: web::Path<u32>) -> impl Responder {
  HttpResponse::Ok().body(format!("GET /api/logs/{}", id))
}

#[get("/users")]
pub async fn get_users(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
  let result = web::block(move || UserEntity::users(pool))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(result))
}

#[get("/users/{id}")]
pub async fn get_user(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
  let result = web::block(move || UserEntity::get_user_by_id(pool, id))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(result))
}

#[post("/users")]
pub async fn post_user(
  pool: web::Data<Pool>,
  request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, Error> {
  let result = web::block(move || UserEntity::create_user(pool, request))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Created().json(result))
}

#[post("/login")]
pub async fn user_login(
  pool: web::Data<Pool>,
  request: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
  let result = web::block(move || UserEntity::login(pool, request))
    .await?
    .map_err(|err| actix_web::error::ErrorUnauthorized(err))?;

  Ok(HttpResponse::Ok().json(result))
}
