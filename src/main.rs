#[macro_use]
extern crate log;
#[macro_use]
mod macros;
pub mod driver_ext;
mod linkedin;
mod logger;
mod selenium;
mod utils;

use crate::linkedin::profiles::SearchResult;
use crate::selenium::SeleniumLinkedin;
use logger::Logger;
use std::io::{BufRead, Read};
use tokio::io::AsyncReadExt;

pub const EMAIL: &str = "jotogi2299@gmail.com";
pub const PASS: &str = "CR3RnozvZydacGVGGsaR";
//"./drivers/chromedriver.exe"

//TODO
// 1. Create a generic error handler macro that will generically handle cases like not found / stale element / etc
//    - In case of stale, we should refetch the element and retry until timeout
//    - In case of not found, we retry with a timeout
#[tokio::main]
async fn main() {
    Logger::init(log::LevelFilter::Trace);
    let selenium = SeleniumLinkedin::new("8888".to_string()).await;
    // selenium
    //     .perform_search(Engineering, "Software Engineer".to_string(), Some("Slovakia".to_string()), None)
    //     .await;
    let results = vec![SearchResult {
        name: "Matus Chochlik".to_string(),
        title: "Software Engineer".to_string(),
        sales_url:
            "https://www.linkedin.com/sales/lead/ACwAAAWs1dABZXg7RDqKugFxlSeo7gasFL1FPHQ,NAME_SEARCH,cypw?_ntb=xTZht7tmSNWO81Egbmk6Xg%3D%3D"
                .to_string(),
    }];
    // let results = selenium.parse_search().await;
    let first = results.first().unwrap();
    let profile = selenium.parse_profile(&first.sales_url).await;
    println!("{}", profile);
}
