use crate::entities::user::UserEntity;
use crate::Pool;

use actix_web::{get, web, Error, HttpResponse};

#[get("/users")]
pub async fn get(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || UserEntity::get(pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/users/{id}")]
pub async fn get_by_id(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let result = web::block(move || UserEntity::get_by_id(pool, id))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}
