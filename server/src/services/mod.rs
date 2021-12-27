use crate::services::job_service::jobs_service;
use actix_web::{web, Scope};

mod job_service;

pub fn api_services() -> Scope {
  web::scope("/api").service(jobs_service())
}
