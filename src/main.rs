#[macro_use]
extern crate log;

use actix_web::web::get;
use log::LevelFilter;
use omicron_crawler::driver::driver_pool::DriverSessionManager;
use omicron_crawler::driver::driver_service::{ChromeDriverService, GeckoDriverService};
use omicron_crawler::env::get_env;
use omicron_crawler::fatal_assert;
use omicron_crawler::fatal_unwrap_e;
use omicron_crawler::linkedin::crawler::Crawler;
use omicron_crawler::linkedin::enums::Functions::Engineering;
use omicron_crawler::logger::Logger;
use std::time::Duration;

//TODO
// 1. Create a generic error handler macro that will generically handle cases like not found / stale element / etc
//    - In case of stale, we should refetch the element and retry until timeout
//    - In case of not found, we retry with a timeout
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    Logger::init(LevelFilter::Trace);
    if let Err(e) = dotenvy::dotenv() {
        warn!("Failed to load .env file, will use defaults!{}", e);
    }
    let env = get_env().await;
    let pool: DriverSessionManager<GeckoDriverService> = DriverSessionManager::new(
        env.host.as_str(),
        env.port,
        1,
        env.driver_path.as_str(),
        env.profile_path.as_str(),
        env.browser_binary_path.as_deref(),
    )
    .await;
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
