#[macro_use]
extern crate log;
#[macro_use]
mod macros;
mod chrome_driver_launcher;
mod logger;

use crate::chrome_driver_launcher::ChromeDriverLauncher;
use logger::Logger;
use std::io::{BufRead, Read};
use std::thread::sleep;
use std::time::Duration;
use thirtyfour::{DesiredCapabilities, WebDriver};
use tokio::io::AsyncReadExt;

async fn launch() {
    let capabilities = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", capabilities)
        .await
        .expect("Failed to create driver");
    driver.goto("https://www.google.com/").await.expect("Failed to go to google");
}

// async fn launch_chrome_driver(){
//     const EXPECTED_OUTPUT: &str = "ChromeDriver was started successfully on port 9515";
//     let mut cmd = Command::new("chromedriver-win64/chromedriver.exe");
//     cmd.args(["--port=9515"]);
//     let mut child = fatal_unwrap_e!(cmd.spawn(), "Failed to start chromedriver {}");
//     let mut stdout = fatal_unwrap_e!(child.stdout.take(), "Failed to get stdout {}");
//     let mut stderr = fatal_unwrap_e!(child.stderr.take(), "Failed to get stderr {}");
//     let mut stdout_str = String::new();
//     let mut stderr_str = String::new();
//
//     loop {
//         fatal_unwrap_e!(stdout.read_to_string(&mut stdout_str), "Failed to read stdout {}");
//         fatal_unwrap_e!(stderr.read_to_string(&mut stderr_str), "Failed to read stderr {}");
//         if stderr_str.len() > 0 {
//             error!("{}", stderr_str);
//             stderr_str.clear();
//         }
//
//         if stdout_str.len() > 0 {
//             info!("{}", stdout_str);
//             let found = stdout_str.find(EXPECTED_OUTPUT);
//             if found.is_some() {
//                 // return ok future
//             }
//         }
//     }
//     // return incomplete future
// }

#[tokio::main]
async fn main() {
    Logger::init(log::LevelFilter::Trace);
    let launcher = ChromeDriverLauncher::launch("8888".to_string());
    let handle = tokio::spawn(launcher);
    fatal_unwrap_e!(handle.await, "Failed to launch chrome driver {}");
}
