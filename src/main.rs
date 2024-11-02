#[macro_use]
extern crate log;

use omicron_crawler::linkedin::crawler::Crawler;
use omicron_crawler::linkedin::profiles::SearchResult;
use omicron_crawler::logger::Logger;

//TODO
// 1. Create a generic error handler macro that will generically handle cases like not found / stale element / etc
//    - In case of stale, we should refetch the element and retry until timeout
//    - In case of not found, we retry with a timeout
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    Logger::init(log::LevelFilter::Trace);
    let selenium = Crawler::new("8888".to_string()).await;
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
