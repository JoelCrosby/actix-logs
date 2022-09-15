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
mod entities;
mod jwt;
mod models;
mod routes;
mod security;

use actix_web::middleware;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use env_logger::Env;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn get_db_connection_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    return Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    let database_pool = get_db_connection_pool();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database_pool.clone()))
            .wrap(auth::middleware::Jwt)
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    // logs
                    .service(routes::logs::get)
                    .service(routes::logs::get_by_id)
                    .service(routes::logs::create)
                    // users
                    .service(routes::users::get)
                    .service(routes::users::get_by_id)
                    // auth
                    .service(routes::auth::register)
                    .service(routes::auth::login),
            )
    })
    .bind(app_url)?
    .run()
    .await
}
