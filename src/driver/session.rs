use crate::driver::traits::BrowserConfig;
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

pub struct DriverSession {
    pub driver: WebDriver,
}

impl DriverSession {
    pub async fn new<T>(host: &str, port: &str, user_dir_param: &str, binary_path_param: Option<&str>) -> Self
    where
        T: BrowserConfig,
    {
        let caps = T::new(user_dir_param, binary_path_param);
        let driver = fatal_unwrap_e!(
            WebDriver::new(format!("http://{}:{}/", host, port), caps).await,
            "Failed to create session: {}"
        );
        Self { driver }
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
        match driver.quit().await {
            Ok(_) => {
                info!("Quitting session");
                Ok(())
            }
            Err(e) => {
                error!("Failed to quit the WebDriver: {}", e);
                Err(e)
            }
        }
    }
}
