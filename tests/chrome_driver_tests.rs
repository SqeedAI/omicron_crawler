use omicron_crawler::driver::service::ChromeDriverService;
use omicron_crawler::driver::session_manager::SessionManager;
use omicron_crawler::env::get_env;
use omicron_crawler::linkedin::web_driver::sales_crawler::SalesCrawler;
use omicron_crawler::logger::Logger;

#[tokio::test(flavor = "multi_thread")]
async fn test_connection() {
    Logger::init(log::LevelFilter::Trace);
    fatal_unwrap_e!(dotenvy::from_filename("test_chrome.env"), "Failed to load .env file {}");
    let env = get_env().await;
    let manager: SessionManager<ChromeDriverService> = SessionManager::new(
        env.driver_host.as_str(),
        env.driver_port,
        1,
        env.driver_path.as_str(),
        env.profile_path.as_str(),
        env.browser_binary_path.as_deref(),
    )
    .await;
    let pool = &manager.pool;
    let proxy = pool.acquire().unwrap();
    let driver = proxy.session.as_ref().unwrap();
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
    fatal_unwrap_e!(dotenvy::from_filename("test_chrome.env"), "Failed to load .env file {}");
    let env = get_env().await;
    let manager: SessionManager<ChromeDriverService> = SessionManager::new(
        env.driver_host.as_str(),
        env.driver_port,
        2,
        env.driver_path.as_str(),
        env.profile_path.as_str(),
        env.browser_binary_path.as_deref(),
    )
    .await;
    let pool = &manager.pool;
    let profile_url =
        "https://www.linkedin.com/sales/lead/ACwAAAWs1dABZXg7RDqKugFxlSeo7gasFL1FPHQ,NAME_SEARCH,cypw?_ntb=xTZht7tmSNWO81Egbmk6Xg%3D%3D";

    async_scoped::TokioScope::scope_and_block(|scope| {
        for _ in 0..2 {
            scope.spawn(async {
                let proxy = pool.acquire().unwrap();
                let crawler = SalesCrawler::new(proxy).await;
                fatal_unwrap_e!(crawler.parse_profile(profile_url).await, "{}");
            });
        }
    });
}
