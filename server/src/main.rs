#![feature(once_cell)]
#![feature(async_closure)]

mod config;
pub mod errors;
mod models;
mod repositories;
mod services;
mod telemetry;

use crate::{config::CONFIG, services::api_services, telemetry::init_telemetry};
use actix_web::{web::Data, App, HttpServer};
use deadpool_postgres::Runtime::Tokio1;
use std::io;
use tokio_postgres::NoTls;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> io::Result<()> {
  init_telemetry();

  let pool = (*CONFIG)
    .pg
    .create_pool(Some(Tokio1), NoTls)
    .expect("Error while creating DB pool");

  let app = move || {
    App::new()
      .wrap(TracingLogger::default())
      .app_data(Data::new(pool.clone()))
      .service(api_services())
  };

  let server_addr = (*CONFIG).server_addr();
  println!("Server starting at http://{}", &server_addr);
  HttpServer::new(app).bind(&server_addr)?.run().await?;

  println!("Server stopped");

  Ok(())
}
