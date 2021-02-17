use crate::Pool;
use crate::user::{User, UserEntity, CreateUser};
use crate::log_entry::{LogEntry, LogEntryEntity, CreateLogEntry};


use anyhow::Result;
use diesel::RunQueryDsl;
use diesel::dsl::{insert_into};
use actix_web::{web, get, post, HttpResponse, Responder};

#[get("/logs")]
pub async fn get_logs() -> impl Responder {
  HttpResponse::Ok().body("GET /api/logs")
}

#[get("/logs/{id}")]
pub async fn get_log(web::Path(id): web::Path<u32>) -> impl Responder {
  HttpResponse::Ok().body(format!("GET /api/logs/{}", id))
}

#[get("/users")]
pub async fn get_users() -> impl Responder {
  HttpResponse::Ok().body("GET /api/logs")
}

#[get("/users/{id}")]
pub async fn get_user(web::Path(id): web::Path<u32>) -> impl Responder {
  HttpResponse::Ok().body(format!("GET /api/users/{}", id))
}

#[post("/users")]
pub async fn post_user(
  pool: web::Data<Pool>,
  user: web::Json<CreateUser<'_>>
) -> impl Responder {

  let res = web::block(move || create_user(pool, user)).await;

  
  HttpResponse::Ok()
}

fn create_user(
  pool: web::Data<Pool>,
  user: web::Json<CreateUser>
) -> Result<User, diesel::result::Error> {
  let db_connection = pool.get().unwrap();


}
