use crate::env::Browser::{Chrome, Firefox};
use crate::linkedin::web_driver::profiles::Profile;
use log::Level;
use serde::de::Unexpected::Str;
use std::fmt::{Display, Formatter};
use thirtyfour::session;
use tokio::sync::OnceCell;

#[derive(Clone, Copy)]
enum Browser {
    Chrome,
    Firefox,
}

impl Display for Browser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Chrome => write!(f, "Chrome"),
            Firefox => write!(f, "Firefox"),
        }
    }
}
pub struct Env {
    pub log_level: log::LevelFilter,
    pub port: u16,
    pub host: String,
    pub driver_path: String,
    pub browser_binary_path: Option<String>,
    pub profile_path: String,
    pub browser: Browser,
    pub driver_host: String,
    pub driver_port: u16,
    pub driver_session_count: u16,
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
    pub linkedin_username: String,
    pub linkedin_password: String,
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

pub fn env_linkedin_username() -> String {
    std::env::var("LINKEDIN_USERNAME").unwrap_or_else(|_| "".to_string())
}
pub fn env_linkedin_password() -> String {
    std::env::var("LINKEDIN_PASSWORD").unwrap_or_else(|_| "".to_string())
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
pub fn env_browser() -> Browser {
    match std::env::var("BROWSER") {
        Ok(browser) => match browser.as_str() {
            "chrome" => Chrome,
            "firefox" => Firefox,
            _ => {
                warn!("Invalid browser type {}, defaulting to chrome", browser);
                Chrome
            }
        },
        Err(_) => Chrome,
    }
}
pub fn env_chrome_profile_path() -> String {
    std::env::var("CHOMRE_PROFILE_PATH").unwrap_or_else(|_| "./user_data/".to_string())
}

pub fn driver_path(browser: Browser) -> String {
    match browser {
        Chrome => env_chrome_driver_path(),
        Firefox => env_gecko_driver_path(),
    }
}

pub fn env_browser_binary_path(browser: Browser) -> Option<String> {
    match browser {
        Chrome => None,
        Firefox => Some(std::env::var("BROWSER_BINARY_PATH").unwrap_or_else(|_| "./profile/".to_string())),
    }
}

pub fn profile_path(browser: Browser) -> String {
    match browser {
        Chrome => std::env::var("CHROME_PROFILE_PATH").unwrap_or_else(|_| "./user_data/".to_string()),
        Firefox => std::env::var("FIREFOX_PROFILE_PATH").unwrap_or_else(|_| "./user_data/".to_string()),
    }
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
    let browser = env_browser();
    let driver_path = driver_path(browser);
    let browser_binary_path = env_browser_binary_path(browser);
    let profile_path = profile_path(browser);
    ENV.get_or_init(|| async {
        Env {
            log_level: env_log_level(),
            port: env_port(),
            host: env_host(),
            driver_path,
            browser_binary_path,
            profile_path,
            browser: env_browser(),
            driver_port: env_driver_port(),
            driver_host: env_driver_host(),
            driver_session_count: env_driver_session_count(),
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
            linkedin_username: env_linkedin_username(),
            linkedin_password: env_linkedin_password(),
        }
    })
    .await
}
