pub mod services;

use crate::services::hello;
use crate::services::request::{profiles, search};
use actix_web::{App, HttpServer};
use log::error;
use log::info;
use omicron_crawler::azure::AzureClient;
use omicron_crawler::driver::service::GeckoDriverService;
use omicron_crawler::driver::session_manager::SessionManager;
use omicron_crawler::env::{get_env, load_env};
use omicron_crawler::fatal_assert;
use omicron_crawler::linkedin::api::LinkedinSession;
use omicron_crawler::logger::Logger;
use tokio::sync::OnceCell;

static LINKEDIN_SESSION: OnceCell<LinkedinSession> = OnceCell::const_new();
static AZURE_SESSION: OnceCell<AzureClient> = OnceCell::const_new();

pub async fn get_linkedin_session() -> &'static LinkedinSession {
    LINKEDIN_SESSION
        .get_or_init(|| async {
            let env = get_env().await;
            let mut linkedin_session = LinkedinSession::new();
            let username = env.linkedin_username.as_str();
            let password = env.linkedin_password.as_str();
            if !linkedin_session.is_auth() {
                info!("Not authenticated, trying to authenticate");
                match linkedin_session.authenticate(username, password).await {
                    Ok(_) => {
                        info!("Authenticated successfully");
                    }
                    Err(e) => {
                        fatal_assert!("Failed to authenticate {}", e);
                    }
                }
            }
            linkedin_session
        })
        .await
}
#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    load_env();
    let env = get_env().await;
    Logger::init(env.log_level);
    get_linkedin_session().await;

    let result = HttpServer::new(|| App::new().service(hello).service(search).service(profiles))
        .bind((env.host.as_str(), env.port))?
        .system_exit()
        .run()
        .await;
    result
}
