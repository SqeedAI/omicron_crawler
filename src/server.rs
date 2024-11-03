pub mod services;

use crate::services::hello;
use crate::services::request::{profiles, search};
use actix_web::{App, HttpServer};
use omicron_crawler::logger::Logger;
use omicron_crawler::utils::log_level_from_env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Logger::init(log_level_from_env());
    let (host, port) = omicron_crawler::utils::host_data_from_env();
    HttpServer::new(|| App::new().service(hello).service(search).service(profiles))
        .bind((host, port))?
        .run()
        .await
}
