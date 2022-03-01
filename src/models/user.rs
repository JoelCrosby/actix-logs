use crate::login::LoginRequest;
use crate::models::schema::users::{self, dsl::*};
use crate::models::user_token::UserToken;
use crate::security::{gen_password_hash, verify_password};
use crate::Pool;
use uuid::Uuid;

use actix_web::web;
use anyhow::Error;
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{dsl::insert_into, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable)]
pub struct UserEntity {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub login_session: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct CreateUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub created_at: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfo {
    pub email: String,
    pub login_session: String,
}

impl UserEntity {
    pub fn create_user(
        pool: web::Data<Pool>,
        user: web::Json<CreateUserRequest>,
    ) -> Result<User, Error> {
        let db_connection = pool.get()?;

        let user_password_hash = gen_password_hash(&user.password);

        let new_user = CreateUser {
            email: &user.email,
            password: &user_password_hash,
        };

        let data = insert_into(users)
            .values(&new_user)
            .get_result::<UserEntity>(&db_connection)?;

        let result = User {
            id: data.id,
            email: data.email,
            created_at: data.created_at.to_string(),
        };

        Ok(result)
    }

    pub fn users(pool: web::Data<Pool>) -> Result<Vec<User>, Error> {
        let db_connection = pool.get()?;

        let data = users.load::<UserEntity>(&db_connection)?;

        let result = data
            .into_iter()
            .map(|row| User {
                id: row.id,
                email: row.email,
                created_at: row.created_at.to_string(),
            })
            .collect();

        Ok(result)
    }

    pub fn login(pool: web::Data<Pool>, request: web::Json<LoginRequest>) -> Result<String> {
        let db_connection = pool.get()?;

        let pwd_hash: String = users
            .filter(email.eq(&request.email))
            .select(password)
            .first(&db_connection)?;

        let authenticated = verify_password(&request.password, &pwd_hash);

        if !authenticated {
            return Err(anyhow!("unauthorized"));
        }

        let login_session_str = UserEntity::generate_login_session();

        let login = LoginInfo {
            email: request.email.to_owned(),
            login_session: login_session_str,
        };

        let token = UserToken::create_jwt(&login)?;

        Ok(token)
    }

    pub fn is_valid_login_session(pool: &web::Data<Pool>, user_token: &UserToken) -> Result<bool> {
        let db_connection = pool.get()?;

        let result = users
            .filter(email.eq(&user_token.uid))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<UserEntity>(&db_connection)
            .is_ok();

        Ok(result)
    }

    pub fn get_user_by_id(pool: web::Data<Pool>, entity_id: web::Path<i32>) -> Result<User, Error> {
        let db_connection = pool.get()?;

        let data: UserEntity = users.find(&entity_id.into_inner()).first(&db_connection)?;

        let result = User {
            id: data.id,
            email: data.email,
            created_at: data.created_at.to_string(),
        };

        Ok(result)
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}
