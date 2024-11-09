use crate::driver::session_manager::SessionProxy;
use crate::errors::CrawlerError::DriverError;
use crate::errors::CrawlerResult;
use crate::linkedin::enums::Functions;
use crate::linkedin::parse_sales::{parse_sales_profile, parse_search, set_function_search, set_geography_search, set_job_title_search};
use crate::linkedin::profiles::{Profile, SearchResult};
use std::time::Duration;

pub struct Crawler<'a> {
    pub proxy: SessionProxy<'a>,
}

impl<'a> Crawler<'a> {
    pub async fn new(proxy: SessionProxy<'a>) -> Self {
        Self { proxy }
    }
    pub async fn set_search_filters(&self, function: Functions, job_title: String, geography: Option<String>) -> CrawlerResult<()> {
        let driver_ext = self.proxy.session.as_ref().unwrap();
        match driver_ext.driver.goto("https://www.linkedin.com/sales/search/people").await {
            Ok(_) => {}
            Err(e) => return Err(DriverError(format!("Failed to go to linkedin {}", e))),
        }
        set_function_search(driver_ext, function).await?;
        set_job_title_search(driver_ext, job_title).await?;
        if let Some(geography) = geography {
            set_geography_search(driver_ext, geography).await?
        };
        Ok(())
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
