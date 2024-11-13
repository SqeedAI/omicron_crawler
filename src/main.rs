#[macro_use]
extern crate log;

use log::LevelFilter;
use omicron_crawler::driver::service::GeckoDriverService;
use omicron_crawler::driver::session_manager::SessionManager;
use omicron_crawler::env::{get_env, load_env};
use omicron_crawler::linkedin::crawler::Crawler;
use omicron_crawler::logger::Logger;

//TODO
// 1. Create a generic error handler macro that will generically handle cases like not found / stale element / etc
//    - In case of stale, we should refetch the element and retry until timeout
//    - In case of not found, we retry with a timeout
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    Logger::init(LevelFilter::Trace);
    load_env();

    let env = get_env().await;
    let manager: SessionManager<GeckoDriverService> = SessionManager::new(
        env.driver_host.as_str(),
        env.driver_port,
        env.driver_session_count,
        env.driver_path.as_str(),
        env.profile_path.as_str(),
        env.browser_binary_path.as_deref(),
    )
    .await;
    let pool = &manager.pool;
    let session = pool.acquire().unwrap();
    let crawler = Crawler::new(session).await;

    // let result = crawler
    //     .parse_profile("https://www.linkedin.com/sales/lead/ACwAABtsCsMBHH4i-dKLOpQqrcqE4H3YDX8CbxE")
    //     .await;
    // println!("{}", result.ok().unwrap());

    //
    // fatal_unwrap_e!(
    //     crawler
    //         .set_search_filters(Some("Java".to_string()), None, None, Some("Slovakia".to_string()))
    //         .await,
    //     "{}"
    // );
    // let results = fatal_unwrap_e!(crawler.parse_search().await, "{}");
    // let first = results.first().unwrap();
    // let profile = crawler.parse_profile(&first.sales_url).await;
    // println!("{}", fatal_unwrap_e!(profile, "{}"));
}
