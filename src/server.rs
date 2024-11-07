pub mod services;

use crate::services::hello;
use crate::services::request::{profiles, search};
use actix_web::{App, HttpServer};
use omicron_crawler::driver_pool::driver_session_pool;
use omicron_crawler::driver_service::driver_service;
use omicron_crawler::logger::Logger;
use omicron_crawler::utils::log_level_from_env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");
    Logger::init(log_level_from_env());
    driver_service().await;
    driver_session_pool().await;
    let (host, port) = omicron_crawler::utils::host_data_from_env();
    let result = HttpServer::new(|| App::new().service(hello).service(search).service(profiles))
        .bind((host, port))?
        .run()
        .await;

    driver_session_pool().await.quit().await;
    result
}
