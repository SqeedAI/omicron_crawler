use crate::linkedin::crawler::Crawler;
use crate::utils::generate_random_string;
use fs_extra::dir::CopyOptions;
use std::env::current_dir;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Once;
use std::time::Duration;
use std::{fs, future, mem};
use thirtyfour::error::{WebDriverError, WebDriverResult};
use thirtyfour::{BrowserCapabilitiesHelper, By, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver, WebElement};
use tokio::fs::DirEntry;
use tokio::runtime::Runtime;
use tokio::sync::{futures, oneshot};
use uuid::Uuid;

pub struct DriverSession {
    pub port: String,
    pub driver: WebDriver,
    user_dir: PathBuf,
}

//OPTIMIZE Consider driver pool for multiple requests

impl DriverSession {
    pub async fn new(host: String, port: String) -> Self {
        let mut caps = DesiredCapabilities::chrome();
        let user_dir = create_user_dir();
        let initial_args = get_undetected_chromedriver_args();
        for arg in initial_args.iter() {
            fatal_unwrap_e!(caps.add_arg(*arg), "Failed to add arg {}");
        }
        fatal_unwrap_e!(
            caps.add_experimental_option("excludeSwitches", ["enable-automation"]),
            "Failed to add experimental excludeSwitches option {}"
        );
        info!("User dir: {}", user_dir.to_str().unwrap());

        let arg = format!("user-data-dir={}", user_dir.to_str().unwrap());
        caps.add_arg(arg.as_str()).unwrap();
        let driver = fatal_unwrap_e!(
            WebDriver::new(format!("http://{}:{}/", host, port), caps).await,
            "Failed to create session: {}"
        );
        Self { port, driver, user_dir }
    }
    pub async fn find_until_loaded(&self, by: By, timeout: Duration) -> WebDriverResult<WebElement> {
        let driver = &self.driver;
        let start = tokio::time::Instant::now();
        while start.elapsed() < timeout {
            match driver.find(by.clone()).await {
                Ok(element) => return Ok(element),
                Err(_) => {
                    tokio::time::sleep(Duration::from_millis(250)).await;
                }
            }
        }

        Err(WebDriverError::Timeout("element not found. Timed out!".to_string()))
    }
    pub async fn quit(&self) -> WebDriverResult<()> {
        let driver = unsafe { std::ptr::read(&self.driver) };
        let result = driver.quit().await;

        if let Err(e) = fs_extra::dir::remove(self.user_dir.clone()) {
            error!("Failed to remove tmp user data {}", e);
        }
        result
    }
}

// OPTIMIZE By using driver pool, we avoid copying the user data to the tmp folder
pub fn create_user_dir() -> PathBuf {
    let mut target_dir = current_dir().unwrap();
    //OPTIMIZE Done during runtime. This is not necessary. Can be done during compile time
    target_dir.push("tmp");
    if !target_dir.exists() {
        fs::create_dir_all(target_dir.clone()).unwrap();
    }

    let uuid = Uuid::new_v4().to_string();

    let mut user_data_dir = current_dir().unwrap();
    user_data_dir.push("user_data");
    target_dir.push(uuid);
    if !target_dir.exists() {
        info!("Creating {}", target_dir.to_str().unwrap());
        fs::create_dir_all(target_dir.clone()).unwrap();
    }
    let copy_options = CopyOptions::new().content_only(true);
    let target_dir_buff = target_dir.clone();
    info!(
        "Copying user data from {} to {}",
        user_data_dir.to_str().unwrap(),
        target_dir.to_str().unwrap()
    );
    fs_extra::dir::copy(user_data_dir.clone(), target_dir, &copy_options).unwrap();
    target_dir_buff
}

pub fn get_undetected_chromedriver_args() -> Vec<&'static str> {
    vec![
        "--disable-background-timer-throttling",
        "--disable-backgrounding-occluded-windows",
        "--disable-logging",
        "--no-sandbox",
        "--headless",    // Add headless mode
        "--disable-gpu", // Recommended for headless
        "--disable-blink-features=AutomationControlled",
        "--disable-infobars",
        "--disable-notifications",
        "--disable-popup-blocking",
        "--disable-extensions",
        "--disable-dev-shm-usage",
        "--no-sandbox",
        "--window-size=1920,1080",
        "--start-maximized",
        "--ignore-certificate-errors",
        "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    ]
}
