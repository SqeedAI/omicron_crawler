#[macro_use]
extern crate log;

use omicron_crawler::fatal_assert;
use omicron_crawler::fatal_unwrap_e;
use omicron_crawler::linkedin::crawler::Crawler;
use omicron_crawler::linkedin::enums::Functions::Engineering;
use omicron_crawler::logger::Logger;

//TODO
// 1. Create a generic error handler macro that will generically handle cases like not found / stale element / etc
//    - In case of stale, we should refetch the element and retry until timeout
//    - In case of not found, we retry with a timeout
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    Logger::init(log::LevelFilter::Trace);
    let selenium = Crawler::new("8888".to_string()).await;
    fatal_unwrap_e!(
        selenium
            .perform_search(Engineering, "Software Engineer".to_string(), Some("Slovakia".to_string()))
            .await,
        "{}"
    );
    let results = fatal_unwrap_e!(selenium.parse_search().await, "{}");
    let first = results.first().unwrap();
    let profile = selenium.parse_profile(&first.sales_url).await;
    println!("{}", fatal_unwrap_e!(profile, "{}"));
}
