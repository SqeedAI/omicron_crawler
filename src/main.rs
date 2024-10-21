#[macro_use]
extern crate log;
#[macro_use]
mod macros;
mod linkedin_crawl;
mod logger;
mod selenium;

use crate::selenium::Selenium;
use logger::Logger;
use std::io::{BufRead, Read};
use tokio::io::AsyncReadExt;
pub const EMAIL: &str = "jotogi2299@gmail.com";
pub const PASS: &str = "CR3RnozvZydacGVGGsaR";

#[tokio::main]
async fn main() {
    Logger::init(log::LevelFilter::Trace);
    let selenium = Selenium::new("8888".to_string()).await;
}
