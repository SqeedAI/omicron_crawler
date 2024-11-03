use crate::linkedin::crawler::Crawler;
use crate::utils::generate_random_string;
use std::env::current_dir;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::process::{Child, Command, Stdio};
use std::sync::Once;
use std::time::Duration;
use std::{fs, future, mem};
use thirtyfour::error::{WebDriverError, WebDriverResult};
use thirtyfour::{BrowserCapabilitiesHelper, By, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver, WebElement};
use tokio::runtime::Runtime;
use tokio::sync::{futures, oneshot};

pub struct DriverSession {
    pub port: String,
    pub driver: WebDriver,
}

impl DriverSession {
    async fn cleanup(&self) {
        let driver = unsafe { std::ptr::read(&self.driver) };
        if let Err(e) = driver.quit().await {
            error!("Failed to quit the WebDriver: {}", e);
        }
    }
    pub async fn new(host: String, port: String) -> Self {
        let mut caps = DesiredCapabilities::chrome();
        let mut current_dir = current_dir().unwrap();
        current_dir.push("user_data");
        let initial_args = get_undetected_chromedriver_args();
        for arg in initial_args.iter() {
            fatal_unwrap_e!(caps.add_arg(*arg), "Failed to add arg {}");
        }
        fatal_unwrap_e!(
            caps.add_experimental_option("excludeSwitches", ["enable-automation"]),
            "Failed to add experimental excludeSwitches option {}"
        );

        let user_data_dir = current_dir.to_str().unwrap();
        let arg = format!("user-data-dir={}", user_data_dir);
        caps.add_arg(arg.as_str()).unwrap();
        let driver = fatal_unwrap_e!(
            WebDriver::new(format!("http://{}:{}/", host, port), caps).await,
            "Failed to create driver {}"
        );
        Self { port, driver }
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
}

impl Drop for DriverSession {
    fn drop(&mut self) {
        tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(async { self.cleanup().await }));
    }
}

pub fn get_undetected_chromedriver_args() -> Vec<&'static str> {
    vec![
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
