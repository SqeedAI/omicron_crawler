use crate::driver::driver_capabilities::{Chrome, Firefox};
use crate::driver::driver_pool::{_get_driver_session_pool, GET_DRIVER_SESSION_POOL};
use crate::driver::driver_service::{chrome_driver_service, gecko_driver_service};
use crate::utils::browser_from_env;

pub mod driver_capabilities;
pub mod driver_pool;
pub mod driver_service;
pub mod driver_session;
pub mod traits;

pub async fn init_chrome() {
    chrome_driver_service().await;
    _get_driver_session_pool::<Chrome>().await;
    unsafe {
        GET_DRIVER_SESSION_POOL = || Box::pin(async { _get_driver_session_pool::<Chrome>().await });
    }
}

pub async fn init_firefox() {
    gecko_driver_service().await;
    _get_driver_session_pool::<Firefox>().await;
    unsafe {
        GET_DRIVER_SESSION_POOL = || Box::pin(async { _get_driver_session_pool::<Firefox>().await });
    }
}

pub async fn init() {
    match browser_from_env().as_str() {
        "chrome" => init_chrome().await,
        "firefox" => init_firefox().await,
        _ => {
            fatal_assert!("Unsupported browser {}", browser_from_env());
        }
    }
}
