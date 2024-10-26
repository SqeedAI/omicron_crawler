use crate::selenium::SeleniumLinkedin;
use std::env::current_dir;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use thirtyfour::error::{WebDriverError, WebDriverResult};
use thirtyfour::{By, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver, WebElement};
use tokio::sync::oneshot;
use undetected_chromedriver::chrome;

pub struct WebDriverExt {
    child: Child,
    pub port: String,
    pub driver: WebDriver,
}

impl WebDriverExt {
    pub async fn new(port: String, chromedriver_path: &str) -> Self {
        let mut cmd = Command::new(chromedriver_path);
        cmd.arg(format!("--port={}", port));
        cmd.stdout(Stdio::piped());

        let mut child: Child = fatal_unwrap_e!(cmd.spawn(), "Failed to start chromedriver {}");
        let stdout = child.stdout.take().expect("Failed to get stdout");

        let (tx, rx) = oneshot::channel();
        let port_clone = port.clone();

        tokio::spawn(async move {
            let expected_output = format!("ChromeDriver was started successfully on port {}", port_clone);

            let mut reader = BufReader::new(stdout);
            let mut out_str = String::new();
            fatal_unwrap_e!(reader.read_line(&mut out_str), "Failed to read line {}");

            loop {
                println!("{}", out_str);
                if out_str.contains(&expected_output) {
                    fatal_unwrap_e!(tx.send(()), "Failed to notify on success! {:?}");
                    break;
                }
                out_str.clear();
                fatal_unwrap_e!(reader.read_line(&mut out_str), "Failed to read line {}");
            }
        });

        // Wait for driver to start
        tokio::select! {
            _ = rx => {
                let mut caps = DesiredCapabilities::chrome();
                let mut curent_dir = current_dir().unwrap();
                curent_dir.push("user_data");

                let user_data_dir = curent_dir.to_str().unwrap();
                let arg = format!("user-data-dir={}", user_data_dir);
                caps.add_arg(arg.as_str()).unwrap();
                let driver = fatal_unwrap_e!(WebDriver::new("http://localhost:8888", caps).await, "Failed to create driver {}");
                Self{
                    child,
                    port,
                    driver
                }
            },
        }
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

impl Drop for WebDriverExt {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
