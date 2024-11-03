use omicron_crawler::driver_service::DriverService;
use omicron_crawler::driver_session::DriverSession;
use omicron_crawler::utils::{driver_host_from_env, driver_path_from_env, driver_port_from_env};

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
