use serde::{Serialize, Deserialize};
use super::schema::users;

#[derive(Debug, Queryable)]
pub struct UserEntity {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
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
