use omicron_crawler::driver_service::DriverService;
use omicron_crawler::driver_session::DriverSession;
use omicron_crawler::linkedin::crawler::Crawler;
use omicron_crawler::logger::Logger;
use omicron_crawler::utils::{driver_host_from_env, driver_path_from_env, driver_port_from_env};
use tokio::try_join;

#[tokio::test(flavor = "multi_thread")]
async fn test_connection() {
    let host = driver_host_from_env();
    let port = driver_port_from_env();
    let path = driver_path_from_env();
    let _driver_service = DriverService::new(port.clone(), path).await;

    let driver = DriverSession::new(host, port).await;
    let profile_url =
        "https://www.linkedin.com/sales/lead/ACwAAAWs1dABZXg7RDqKugFxlSeo7gasFL1FPHQ,NAME_SEARCH,cypw?_ntb=xTZht7tmSNWO81Egbmk6Xg%3D%3D";

    match driver.driver.goto(profile_url).await {
        Ok(_) => {}
        Err(e) => {
            assert!(false, "Failed to go to webpage {}", e);
        }
    }
}
#[tokio::test(flavor = "multi_thread")]
async fn test_multiple_sessions() {
    Logger::init(log::LevelFilter::Trace);
    let host = driver_host_from_env();
    let port = driver_port_from_env();
    let path = driver_path_from_env();
    let _driver_service = DriverService::new(port.clone(), path).await;
    let profile_url =
        "https://www.linkedin.com/sales/lead/ACwAAAWs1dABZXg7RDqKugFxlSeo7gasFL1FPHQ,NAME_SEARCH,cypw?_ntb=xTZht7tmSNWO81Egbmk6Xg%3D%3D";

    let mut handles = Vec::with_capacity(4);
    for _ in 0..4 {
        let port = port.clone();
        let host = host.clone();
        handles.push(tokio::spawn(async move {
            let crawler = Crawler::new(host, port).await;
            fatal_unwrap_e!(crawler.parse_profile(profile_url.clone()).await, "{}");
            crawler.quit().await;
        }));
    }

    for handle in handles.into_iter() {
        handle.await.unwrap();
    }
}
