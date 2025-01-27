use crate::driver::session::DriverSession;
use crate::errors::CrawlerError::DriverError;
use crate::errors::CrawlerResult;
use crate::linkedin::web_driver::profiles::{Profile, SearchResult};
use crate::linkedin::web_driver::sales::{
    parse_sales_profile, parse_search, send_message, set_function_search, set_geography_search, set_job_title_search, set_keyword_search,
};
use crate::session_pool::SessionProxy;
use std::time::Duration;

pub struct SalesCrawler<'a> {
    pub proxy: SessionProxy<'a, DriverSession>,
}

impl<'a> SalesCrawler<'a> {
    pub async fn new(proxy: SessionProxy<'a, DriverSession>) -> Self {
        Self { proxy }
    }
    pub async fn set_search_filters(
        &self,
        keywords: Option<String>,
        function: Option<String>,
        job_title: Option<String>,
        geography: Option<String>,
    ) -> CrawlerResult<()> {
        let driver_ext = self.proxy.session.as_ref().unwrap();
        match driver_ext.driver.goto("https://www.linkedin.com/sales/search/people").await {
            Ok(_) => {}
            Err(e) => return Err(DriverError(format!("Failed to go to linkedin {}", e))),
        }

        if let Some(function) = function {
            set_function_search(driver_ext, function).await?;
        }
        if let Some(job_title) = job_title {
            set_job_title_search(driver_ext, job_title).await?;
        }

        if let Some(geography) = geography {
            set_geography_search(driver_ext, geography).await?
        };

        if let Some(key_words) = keywords {
            set_keyword_search(driver_ext, key_words).await?;
        }
        Ok(())
    }
    pub async fn send_message(&self, sales_url: &str, subject: &str, body: &str) -> CrawlerResult<()> {
        let driver_ext = self.proxy.session.as_ref().unwrap();
        send_message(driver_ext, sales_url, subject, body).await
    }
    pub async fn test_detection(&self) {
        let driver_ext = self.proxy.session.as_ref().unwrap();
        driver_ext.driver.goto("https://demo.fingerprint.com/playground").await.unwrap();
        tokio::time::sleep(Duration::from_secs(100)).await;
    }
    pub async fn parse_search(&self) -> CrawlerResult<Vec<SearchResult>> {
        let driver_ext = self.proxy.session.as_ref().unwrap();
        parse_search(driver_ext).await
    }

    pub async fn parse_profile(&self, sales_url: &str) -> CrawlerResult<Profile> {
        let driver_ext = self.proxy.session.as_ref().unwrap();
        let original_tab = driver_ext.driver.window().await.unwrap();
        let new_window_handle = driver_ext.driver.new_tab().await.unwrap();
        driver_ext.driver.switch_to_window(new_window_handle).await.unwrap();
        let result = parse_sales_profile(driver_ext, sales_url).await;
        driver_ext.driver.close_window().await.unwrap();
        driver_ext.driver.switch_to_window(original_tab).await.unwrap();
        result
    }
}
