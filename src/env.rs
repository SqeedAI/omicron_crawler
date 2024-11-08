use crate::env::Browser::{Chrome, Firefox};
use crate::linkedin::profiles::Profile;
use log::Level;
use serde::de::Unexpected::Str;
use std::cell::OnceCell;
use std::fmt::{Display, Formatter};
use thirtyfour::session;

#[derive(Clone, Copy)]
enum Browser {
    Chrome,
    Firefox,
}

impl Display for Browser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Browser::Chrome => write!(f, "Chrome"),
            Browser::Firefox => write!(f, "Firefox"),
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
    pub driver_session_count: u16,
}

static ENV: OnceCell<Env> = OnceCell::new();

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
pub fn env_host() -> (String, u16) {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(8080);
    (host, port)
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
    let sessions = std::env::var("DRIVER_SESSION_COUNT");
    fatal_unwrap_e!(sessions.parse(), "Failed to parse DRIVER_SESSION_COUNT {}");
}
// OPTIMIZE Create an env struct that is initialized at the start of the program.
// The struct shall contain correct types, not strings. Browser type should be an enum.
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
pub fn env_firefox_profile_path() -> String {
    std::env::var("FIREFOX_PROFILE_PATH").unwrap_or_else(|_| "./profile/".to_string())
}

pub fn env_chrome_profile_path() -> String {
    std::env::var("CHOMRE_PROFILE_PATH").unwrap_or_else(|_| "./user_data/".to_string())
}

pub fn env_firefox_binary_path() -> String {
    std::env::var("FIREFOX_BINARY_PATH").unwrap_or_else(|_| "C:\\Program Files\\Mozilla Firefox\\firefox.exe".to_string())
}

pub fn driver_path(browser: Browser) -> String {
    match browser {
        Chrome => env_chrome_driver_path(),
        Firefox => env_gecko_driver_path(),
    }
}

pub fn browser_binary_path(browser: Browser) -> Option<String> {
    match browser {
        Chrome => Some(env_firefox_binary_path()),
        Firefox => None,
    }
}

pub fn profile_path(browser: Browser) -> String {
    match browser {
        Chrome => env_chrome_profile_path(),
        Firefox => env_firefox_profile_path(),
    }
}

pub fn get_env() -> &'static Env {
    let browser = env_browser();
    let driver_path = driver_path(browser);
    let browser_binary_path = browser_binary_path(browser);
    let profile_path = profile_path(browser);
    ENV.get_or_init(|| Env {
        log_level: env_log_level(),
        port: env_driver_port(),
        host: env_driver_host(),
        driver_path,
        browser_binary_path,
        profile_path,
        browser: env_browser(),
        driver_host: env_driver_host(),
        driver_session_count: env_driver_session_count(),
    })
}
