use actix_web::{web, get, HttpResponse, Responder};

#[get("/logs")]
pub async fn get_logs() -> impl Responder {
  HttpResponse::Ok().body("GET /api/logs")
}

#[get("/logs/{id}")]
pub async fn get_log(web::Path(id): web::Path<u32>) -> impl Responder {
  HttpResponse::Ok().body(format!("GET /api/logs/{}", id))
}