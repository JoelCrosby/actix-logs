use serde::{Serialize, Deserialize};
use super::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserEntity {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]
pub struct CreateUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub created_at: String,
}
