use crate::entities::user::CreateUserRequest;
use crate::entities::user::UserEntity;
use crate::models::login::LoginRequest;
use crate::Pool;

use actix_web::{post, web, Error, HttpResponse};

#[post("/register")]
pub async fn register(
    pool: web::Data<Pool>,
    request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || UserEntity::create(pool, request))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(result))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<Pool>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || UserEntity::login(pool, request))
        .await?
        .map_err(|err| actix_web::error::ErrorUnauthorized(err))?;

    Ok(HttpResponse::Ok().json(result))
}
