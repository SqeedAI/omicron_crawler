#[macro_use]
extern crate log;
#[macro_use]
mod macros;
pub mod driver_ext;
mod linkedin;
mod logger;
mod selenium;

use crate::linkedin::enums::Functions::Engineering;
use crate::selenium::SeleniumLinkedin;
use logger::Logger;
use std::io::{BufRead, Read};
use std::time::Duration;
use tokio::io::AsyncReadExt;

pub const EMAIL: &str = "jotogi2299@gmail.com";
pub const PASS: &str = "CR3RnozvZydacGVGGsaR";
//"./drivers/chromedriver.exe"
#[tokio::main]
async fn main() {
    Logger::init(log::LevelFilter::Trace);
    let selenium = SeleniumLinkedin::new("8888".to_string()).await;
    selenium
        .perform_search(Engineering, "Software Engineer".to_string(), Some("Slovakia".to_string()), None)
        .await;
    tokio::time::sleep(Duration::from_secs(30)).await;
}
