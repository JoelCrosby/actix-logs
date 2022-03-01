#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel;
extern crate argon2;
extern crate chrono;
extern crate dotenv;
extern crate uuid;

mod auth;
mod constants;
mod jwt;
mod login;
mod models;
mod routes;
mod security;
mod server_error;

use crate::routes::*;

use actix_web::middleware;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use env_logger::Env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn get_db_connection_pool() -> Pool {
    let database_url =
        std::env::var("DATABASE_URL").expect("Database connection string  not found.");

    return Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_pool = get_db_connection_pool();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database_pool.clone()))
            .wrap(auth::middleware::Jwt)
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(get_logs)
                    .service(get_log)
                    .service(get_user)
                    .service(get_users)
                    .service(post_user)
                    .service(user_login),
            )
    })
    .bind("127.0.0.1:6600")?
    .run()
    .await
}
