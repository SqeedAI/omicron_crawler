use log::Level;
use serde::de::Unexpected::Str;
use std::fmt::{Display, Formatter};
use thirtyfour::session;
use tokio::sync::OnceCell;
pub struct Env {
    pub log_level: log::LevelFilter,
    pub port: u16,
    pub host: String,
    pub azure_search_uri: String,
    pub azure_search_dequeue_api: String,
    pub azure_profile_uri: String,
    pub azure_profile_dequeue_api: String,
    pub azure_profile_queue_api: String,
    pub azure_manager_bus_uri: String,
    pub azure_manager_bus_api: String,
    pub azure_sas_key_name: String,
    pub azure_manager_bus_key: String,
    pub azure_sas_profile_key: String,
    pub azure_sas_search_key: String,
    pub manager_search_api: String,
    pub manager_profile_api: String,
}

static ENV: OnceCell<Env> = OnceCell::const_new();

pub fn env_log_level() -> log::LevelFilter {
    match std::env::var("LOG_LEVEL") {
        Ok(level) => match level.as_str() {
            "TRACE" => log::LevelFilter::Trace,
            "DEBUG" => log::LevelFilter::Debug,
            "INFO" => log::LevelFilter::Info,
            "WARN" => log::LevelFilter::Warn,
            "ERROR" => log::LevelFilter::Error,
            _ => {
                warn!("Invalid log level {}, defaulting to info", level);
                log::LevelFilter::Info
            }
        },
        Err(_) => log::LevelFilter::Info,
    }
}
pub fn env_azure_search_uri() -> String {
    std::env::var("AZURE_SEARCH_URI").unwrap_or_else(|_| "".to_string())
}
pub fn env_azure_search_dequeue_api() -> String {
    std::env::var("AZURE_SEARCH_DEQUEUE_API").unwrap_or_else(|_| "".to_string())
}
pub fn env_azure_profile_uri() -> String {
    std::env::var("AZURE_PROFILE_URI").unwrap_or_else(|_| "".to_string())
}
pub fn env_azure_profile_dequeue_api() -> String {
    std::env::var("AZURE_PROFILE_DEQUEUE_API").unwrap_or_else(|_| "".to_string())
}
pub fn env_azure_profile_queue_api() -> String {
    std::env::var("AZURE_PROFILE_QUEUE_API").unwrap_or_else(|_| "".to_string())
}

pub fn env_azure_manager_bus_uri() -> String {
    std::env::var("AZURE_MANAGER_BUS_URI").unwrap_or_else(|_| "".to_string())
}

pub fn env_azure_manager_bus_api() -> String {
    std::env::var("AZURE_MANAGER_BUS_API").unwrap_or_else(|_| "".to_string())
}

pub fn env_azure_sas_key_name() -> String {
    std::env::var("AZURE_SAS_KEY_NAME").unwrap_or_else(|_| "".to_string())
}

pub fn env_azure_manager_bus_key() -> String {
    std::env::var("AZURE_MANAGER_BUS_KEY").unwrap_or_else(|_| "".to_string())
}

pub fn env_azure_sas_profile_key() -> String {
    std::env::var("AZURE_SAS_PROFILE_KEY").unwrap_or_else(|_| "".to_string())
}

pub fn env_azure_sas_search_key() -> String {
    std::env::var("AZURE_SAS_SEARCH_KEY").unwrap_or_else(|_| "".to_string())
}

pub fn env_host() -> String {
    std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
}

pub fn env_manager_search_api() -> String {
    std::env::var("MANAGER_SEARCH_API").unwrap_or_else(|_| "".to_string())
}

pub fn env_manager_profile_api() -> String {
    std::env::var("MANAGER_PROFILE_API").unwrap_or_else(|_| "".to_string())
}

pub fn env_port() -> u16 {
    std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(8080)
}

pub fn env_chrome_driver_path() -> String {
    std::env::var("CHROME_DRIVER_PATH").unwrap_or_else(|_| "./drivers/chromedriver.exe".to_string())
}

pub fn env_gecko_driver_path() -> String {
    std::env::var("GECKO_DRIVER_PATH").unwrap_or_else(|_| "./drivers/geckodriver.exe".to_string())
}

pub fn env_driver_host() -> String {
    std::env::var("DRIVER_HOST").unwrap_or_else(|_| "localhost".to_string())
}

pub fn env_driver_port() -> u16 {
    fatal_unwrap_e!(
        std::env::var("DRIVER_PORT").unwrap_or_else(|_| "9515".to_string()).parse(),
        "Failed to parse DRIVER_PORT {}"
    )
}
pub fn env_driver_session_count() -> u16 {
    let sessions = std::env::var("DRIVER_SESSION_COUNT").unwrap_or_else(|_| "1".to_string());
    fatal_unwrap_e!(sessions.parse(), "Failed to parse DRIVER_SESSION_COUNT {}")
}
pub fn load_env() {
    #[cfg(target_os = "windows")]
    {
        if let Err(e) = dotenvy::from_filename(".env-windows") {
            warn!("Failed to load .env file, will use defaults!{}", e);
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Err(e) = dotenvy::from_filename(".env-linux") {
            warn!("Failed to load .env file, will use defaults!{}", e);
        }
    }
}

pub async fn get_env() -> &'static Env {
    ENV.get_or_init(|| async {
        Env {
            log_level: env_log_level(),
            port: env_port(),
            host: env_host(),
            azure_search_uri: env_azure_search_uri(),
            azure_search_dequeue_api: env_azure_search_dequeue_api(),
            azure_profile_uri: env_azure_profile_uri(),
            azure_profile_dequeue_api: env_azure_profile_dequeue_api(),
            azure_profile_queue_api: env_azure_profile_queue_api(),
            azure_manager_bus_uri: env_azure_manager_bus_uri(),
            azure_manager_bus_api: env_azure_manager_bus_api(),
            azure_sas_key_name: env_azure_sas_key_name(),
            azure_manager_bus_key: env_azure_manager_bus_key(),
            azure_sas_profile_key: env_azure_sas_profile_key(),
            azure_sas_search_key: env_azure_sas_search_key(),
            manager_search_api: env_manager_search_api(),
            manager_profile_api: env_manager_profile_api(),
        }
    })
    .await
}
