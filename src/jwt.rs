use crate::constants;
use crate::models::user::UserEntity;
use actix_web::web;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::Serialize;

use crate::models::user_token::UserToken;
use crate::Pool;

pub fn verify_jwt(
    token_data: &TokenData<UserToken>,
    pool: &web::Data<Pool>,
) -> Result<String, String> {
    if UserEntity::is_valid_login_session(pool, &token_data.claims).is_ok() {
        Ok(token_data.claims.uid.to_string())
    } else {
        Err(constants::MESSAGE_INVALID_TOKEN.to_string())
    }
}

pub fn decode_jwt(token: &String) -> Result<TokenData<UserToken>, jsonwebtoken::errors::Error> {
    return decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(constants::JWT_SECRET),
        &Validation::default(),
    );
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
