// pub mod services;
//
// use crate::services::hello;
// use crate::services::request::{profiles, search};
// use actix_web::{App, HttpServer};
// use omicron_crawler::logger::Logger;
// use std::any::Any;
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     dotenvy::dotenv().expect("Failed to load .env file");
//     Logger::init(log_level_from_env());
//     chrome_driver_service().await;
//     let (host, port) = omicron_crawler::utils::host_data_from_env();
//     let result = HttpServer::new(|| App::new().service(hello).service(search).service(profiles))
//         .bind((host, port))?
//         .system_exit()
//         .run()
//         .await;
//     result
// }

fn main() {}
