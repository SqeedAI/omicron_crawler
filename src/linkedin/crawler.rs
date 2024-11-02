use crate::driver_ext::WebDriverExt;
use crate::linkedin::enums::{Functions, SeniorityLevel};
use crate::linkedin::parse::{parse_sales_profile, parse_search, set_function_search, set_geography_search, set_job_title_search};
use crate::linkedin::profiles::{Profile, SearchResult};
use std::result;
use std::time::Duration;
use thirtyfour::{By, WindowType};

pub struct Crawler {
    driver_ext: WebDriverExt,
}

impl Crawler {
    pub async fn new(port: String) -> Self {
        let driver_ext = WebDriverExt::new(port, "./drivers/chromedriver.exe").await;
        Self { driver_ext }
    }
    pub async fn load_linkedin(&self) {
        let driver_ext = &self.driver_ext;
        fatal_unwrap_e!(
            driver_ext.driver.goto("https://www.linkedin.com/").await,
            "Failed to go to linkedin {}"
        );
    }
    pub async fn perform_search(
        &self,
        function: Functions,
        job_title: String,
        geography: Option<String>,
        seniority_level: Option<SeniorityLevel>,
    ) {
        let driver_ext = &self.driver_ext;
        fatal_unwrap_e!(
            driver_ext.driver.goto("https://www.linkedin.com/sales/search/people").await,
            "Failed to go to linkedin {}"
        );
        set_function_search(driver_ext, function).await;
        set_job_title_search(driver_ext, job_title).await;
        if let Some(geography) = geography {
            set_geography_search(driver_ext, geography).await;
        }
    }
    pub async fn test_detection(&self) {
        let driver_ext = &self.driver_ext;
        driver_ext.driver.goto("https://demo.fingerprint.com/playground").await.unwrap();
        tokio::time::sleep(Duration::from_secs(15)).await;
    }
    pub async fn parse_search(&self) -> Vec<SearchResult> {
        let driver_ext = &self.driver_ext;
        parse_search(driver_ext).await
    }

    pub async fn parse_profile(&self, sales_url: &str) -> Profile {
        let driver_ext = &self.driver_ext;
        let original_tab = driver_ext.driver.window().await.unwrap();
        let new_window_handle = driver_ext.driver.new_tab().await.unwrap();
        driver_ext.driver.switch_to_window(new_window_handle).await.unwrap();
        let result = parse_sales_profile(driver_ext, sales_url).await;
        driver_ext.driver.close_window().await.unwrap();
        driver_ext.driver.switch_to_window(original_tab).await.unwrap();
        result
    }
}
