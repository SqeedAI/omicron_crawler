use omicron_crawler::driver_ext::WebDriverExt;

#[tokio::test]
async fn test_connection() {
    let driver = WebDriverExt::new("9115".to_string(), "./drivers/chromedriver.exe").await;
    let profile_url =
        "https://www.linkedin.com/sales/lead/ACwAAAWs1dABZXg7RDqKugFxlSeo7gasFL1FPHQ,NAME_SEARCH,cypw?_ntb=xTZht7tmSNWO81Egbmk6Xg%3D%3D";

    match driver.driver.goto(profile_url).await {
        Ok(_) => {}
        Err(e) => {
            assert!(false, "Failed to go to test1 {}", e);
        }
    }
    driver.cleanup().await;
}
