use http::header::HeaderValue;
use std::env::current_dir;
use std::future::Future;
use std::io::{BufRead, BufReader, Read};
use std::pin::Pin;
use std::process::{Child, Command, Stdio};
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::common::config::WebDriverConfigBuilder;
use thirtyfour::{ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use tokio::sync::oneshot;

pub struct Selenium {
    child: Child,
    pub port: String,
    pub driver: WebDriver,
}

impl Selenium {
    pub async fn new(port: String) -> Self {
        let mut cmd = Command::new("./drivers/chromedriver.exe");
        cmd.arg(format!("--port={}", port));
        cmd.stdout(Stdio::piped());

        let mut child = fatal_unwrap_e!(cmd.spawn(), "Failed to start chromedriver {}");
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

        // Wait for the expected output or timeout
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
}

impl Drop for Selenium {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
