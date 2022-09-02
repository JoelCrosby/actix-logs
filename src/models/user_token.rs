use crate::constants;
use crate::entities::user::LoginInfo;

use chrono::Utc;
use jsonwebtoken::encode;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub uid: String,
    pub login_session: String,
}

impl UserToken {
    pub fn create_jwt(login: &LoginInfo) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000;
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(12))
            .expect("valid timestamp")
            .timestamp();

        let payload = UserToken {
            iat: now,
            exp: expiration,
            uid: login.email.clone(),
            login_session: login.login_session.clone(),
        };

        encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(constants::JWT_SECRET),
        )
    }
}
