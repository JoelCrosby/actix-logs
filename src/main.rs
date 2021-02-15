mod routes;

use crate::routes::*;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(
            // prefixes all resources and routes attached to it...
            web::scope("/api")
                .service(get_logs)
                .service(get_log)
        )
    })
    .bind("127.0.0.1:6600")?
    .run()
    .await
}