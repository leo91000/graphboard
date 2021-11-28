#![feature(once_cell)]
#![feature(async_closure)]

mod config;
pub mod errors;
mod models;
mod repositories;
mod services;

use crate::config::Config;
use crate::services::api_services;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::Runtime::Tokio1;
use tokio_postgres::NoTls;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Hello Graphboard API !")
}

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(Some(Tokio1), NoTls).unwrap();

    let app = move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(ping)
            .service(api_services())
    };

    let server_addr = config.server_addr();
    println!("Server starting at http://{}", &server_addr);
    HttpServer::new(app)
        .bind(&server_addr)
        .expect(format!("Cannot bind address {}", &server_addr).as_str())
        .run()
        .await
        .unwrap();

    println!("Server stopped");
}
