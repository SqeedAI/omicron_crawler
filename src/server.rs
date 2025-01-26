pub mod services;

use crate::services::hello;
use crate::services::request::{message, profiles, search};
use actix_web::{App, HttpServer};
use omicron_crawler::driver::service::GeckoDriverService;
use omicron_crawler::driver::session_manager::SessionManager;
use omicron_crawler::env::{get_env, load_env};
use omicron_crawler::logger::Logger;
use tokio::sync::OnceCell;

static DRIVER_SESSION_MANAGER: OnceCell<SessionManager<GeckoDriverService>> = OnceCell::const_new();

pub async fn get_driver_session_manager() -> &'static SessionManager<GeckoDriverService> {
    DRIVER_SESSION_MANAGER
        .get_or_init(|| async {
            let env = get_env().await;
            SessionManager::<GeckoDriverService>::new(
                env.driver_host.as_str(),
                env.driver_port,
                env.driver_session_count,
                env.driver_path.as_str(),
                env.profile_path.as_str(),
                env.browser_binary_path.as_deref(),
            )
            .await
        })
        .await
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    load_env();
    let env = get_env().await;
    Logger::init(env.log_level);
    get_driver_session_manager().await;

    let result = HttpServer::new(|| App::new().service(hello).service(search).service(profiles).service(message))
        .bind((env.host.as_str(), env.port))?
        .system_exit()
        .run()
        .await;
    result
}
