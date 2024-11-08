#[macro_use]
extern crate log;

use log::LevelFilter;
use omicron_crawler::driver::driver_pool::{get_driver_session_pool, DriverSessionManager};
use omicron_crawler::driver::driver_service::{chrome_driver_service, gecko_driver_service};
use omicron_crawler::driver::init;
use omicron_crawler::fatal_assert;
use omicron_crawler::fatal_unwrap_e;
use omicron_crawler::linkedin::crawler::Crawler;
use omicron_crawler::linkedin::enums::Functions::Engineering;
use omicron_crawler::logger::Logger;
use omicron_crawler::utils::{
    browser_from_env, chrome_driver_path_from_env, driver_host_from_env, driver_port_from_env, driver_session_count_from_env,
    log_level_from_env,
};
use std::time::Duration;

//TODO
// 1. Create a generic error handler macro that will generically handle cases like not found / stale element / etc
//    - In case of stale, we should refetch the element and retry until timeout
//    - In case of not found, we retry with a timeout
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    Logger::init(LevelFilter::Trace);
    if let Err(e) = dotenvy::from_filename(".env") {
        warn!("Failed to load .env file, will use defaults!{}", e);
    }
    let browser = browser_from_env();
    let pool = init(browser.as_str()).await;
    {
        let session = pool.acquire().unwrap();
        info!("Acquired session, starting crawler...");
        let crawler = Crawler::new(session).await;

        fatal_unwrap_e!(
            crawler
                .set_search_filters(Engineering, "Software Engineer".to_string(), Some("Slovakia".to_string()))
                .await,
            "{}"
        );
        let results = fatal_unwrap_e!(crawler.parse_search().await, "{}");
        let first = results.first().unwrap();
        let profile = crawler.parse_profile(&first.sales_url).await;
        println!("{}", fatal_unwrap_e!(profile, "{}"));
    }
}
