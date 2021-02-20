use crate::login::LoginRequest;
use crate::user::{User, UserEntity, CreateUser};
use crate::user::CreateUserRequest;
use crate::Pool;
use crate::security::{gen_password_hash, verify_password};

use diesel::{RunQueryDsl, dsl::insert_into};
use anyhow::Result;
use diesel::prelude::*;
use actix_web::web;

pub fn create_user(
    pool: web::Data<Pool>,
    user: web::Json<CreateUserRequest>
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();

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

    return Ok(result);
}

pub fn users(
    pool: web::Data<Pool>
) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();

    let data = users.load::<UserEntity>(&db_connection)?;

    let result = data.into_iter().map(|row| User { 
        id: row.id,
        email: row.email,
        created_at: row.created_at.to_string(),
    })
     .collect();

    return Ok(result);
}

pub fn login(
    pool: web::Data<Pool>,
    request: web::Json<LoginRequest>
) -> Result<bool, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();

    let pwd_hash: String = users
        .filter(email.eq(&request.email))
        .select(password)
        .first(&db_connection)?;

    let authenticated = verify_password(&request.password, &pwd_hash);
    
    return Ok(authenticated);
}

pub fn get_user_by_id(
    pool: web::Data<Pool>,
    entity_id: web::Path<u32>
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();

    let data: UserEntity = users
        .find(entity_id)
        .first(&db_connection)?;

    let result = User {
        id: data.id,
        email: data.email,
        created_at: data.created_at.to_string(),
    };

    return Ok(result);
}
