use crate::selenium::SeleniumLinkedin;
use crate::utils::generate_random_string;
use std::env::current_dir;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::Once;
use std::time::Duration;
use std::{fs, mem};
use thirtyfour::error::{WebDriverError, WebDriverResult};
use thirtyfour::{BrowserCapabilitiesHelper, By, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver, WebElement};
use tokio::sync::oneshot;
use undetected_chromedriver::chrome;

pub struct WebDriverExt {
    child: Child,
    pub port: String,
    pub driver: WebDriver,
}

impl WebDriverExt {
    pub async fn new(port: String, chromedriver_path: &str) -> Self {
        patch_cdc(chromedriver_path);
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
                let initial_args = get_undetected_chromedriver_args();
                for arg in initial_args.iter() {
                    fatal_unwrap_e!(caps.add_arg(*arg), "Failed to add arg {}");
                }
                fatal_unwrap_e!(caps.add_experimental_option("excludeSwitches", ["enable-automation"]), "Failed to add experimental excludeSwitches option {}");

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

pub fn patch_cdc(chromedriver_path: &str) {
    const CDC_SIZE: usize = 22;
    let mut driver_binary = fatal_unwrap_e!(fs::read(chromedriver_path), "Failed to read chromedriver binary {}");
    let pattern = b"cdc_";
    let new_cdc = generate_random_string(CDC_SIZE);
    let mut matches = Vec::with_capacity(3);
    for (index, window) in driver_binary.windows(pattern.len()).enumerate() {
        if window == pattern {
            matches.push(index);
        }
    }
    if matches.len() == 0 {
        info!("no cdc matches found, no need to patch!");
        return;
    }

    let first_match = unsafe { String::from_raw_parts(driver_binary.as_mut_ptr().add(matches[0]), CDC_SIZE, CDC_SIZE) };
    info!("Replacing {} with {}", first_match, new_cdc);
    mem::forget(first_match);

    for index in matches {
        let mut cdc_str = unsafe { String::from_raw_parts(driver_binary.as_mut_ptr().add(index), CDC_SIZE, CDC_SIZE) };
        cdc_str.replace_range(0..CDC_SIZE, &new_cdc);
        mem::forget(cdc_str);
    }
    fs::write(chromedriver_path, driver_binary).unwrap();
}

impl Drop for WebDriverExt {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
