use crate::services::job_service::jobs_service;
use actix_web::{get, web, HttpResponse, Responder, Scope};

mod job_service;

#[get("/ping")]
async fn ping() -> impl Responder {
  HttpResponse::Ok().body("Hello Graphboard API !")
}

pub fn api_services() -> Scope {
  web::scope("/api").service(ping).service(jobs_service())
}
