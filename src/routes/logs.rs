use crate::entities::log_entry::{CreateLogEntryRequest, LogEntryEntity};
use crate::Pool;

use actix_web::{get, post, web, Error, HttpResponse};

#[get("/logs")]
pub async fn get(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || LogEntryEntity::get(pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/logs/{id}")]
pub async fn get_by_id(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let result = web::block(move || LogEntryEntity::get_by_id(pool, id))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/logs")]
pub async fn create(
    pool: web::Data<Pool>,
    request: web::Json<CreateLogEntryRequest>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || LogEntryEntity::create(pool, request))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(result))
}
