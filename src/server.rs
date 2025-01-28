pub mod services;

use crate::services::hello;
use crate::services::request::{profiles, search};
use actix_web::{App, HttpServer};
use omicron_crawler::env::{get_env, load_env};
use omicron_crawler::linkedin::api::crawler::LinkedinSessionManager;
use omicron_crawler::linkedin::api::rate_limits::RateLimiter;
use omicron_crawler::logger::Logger;
use tokio::sync::OnceCell;
static CRAWLER: OnceCell<LinkedinSessionManager> = OnceCell::const_new();

pub async fn get_crawler() -> &'static LinkedinSessionManager {
    CRAWLER
        .get_or_init(|| async {
            let username = get_env().await.linkedin_username.as_str();
            let password = get_env().await.linkedin_password.as_str();
            let mut crawler = LinkedinSessionManager::new(RateLimiter::new(100, 800), username, password).await;
            crawler
        })
        .await
}
#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    load_env();
    let env = get_env().await;
    Logger::init(env.log_level);
    get_crawler().await;
    let result = HttpServer::new(|| App::new().service(hello).service(search).service(profiles))
        .bind((env.host.as_str(), env.port))?
        .system_exit()
        .run()
        .await;
    result
}
