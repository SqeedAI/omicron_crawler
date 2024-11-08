use crate::driver::browser_config::{Chrome, Firefox};
use crate::driver::driver_pool::{create_driver_session_pool, DriverSessionPool};
use crate::driver::driver_service::{chrome_driver_service, gecko_driver_service};
use crate::utils::browser_from_env;

pub mod browser_config;
pub mod driver_pool;
pub mod driver_service;
pub mod driver_session;
pub mod traits;

pub async fn init_chrome() -> &'static DriverSessionPool {
    chrome_driver_service().await;
    create_driver_session_pool::<Chrome>().await
}

pub async fn init_firefox() -> &'static DriverSessionPool {
    gecko_driver_service().await;
    create_driver_session_pool::<Firefox>().await
}

pub async fn init() -> &'static DriverSessionPool {
    match browser_from_env().as_str() {
        "chrome" => init_chrome().await,
        "firefox" => init_firefox().await,
        _ => {
            fatal_assert!("Unsupported browser {}", browser_from_env());
        }
    }
}
