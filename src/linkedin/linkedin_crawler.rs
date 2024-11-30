use crate::driver::session_manager::SessionProxy;
use crate::errors::CrawlerError::DriverError;
use crate::errors::CrawlerResult;
use crate::linkedin::parse_linkedin::{
    parse_search, search_set_current_company, search_set_filter, search_set_industry, search_set_keywords, search_set_past_company,
    search_set_services, try_press_filter_button,
};
use crate::linkedin::profiles::SearchResult;
use std::time::Duration;
use thirtyfour::By;

pub struct LinkedinCrawler<'a> {
    pub proxy: SessionProxy<'a>,
}

impl<'a> LinkedinCrawler<'a> {
    pub async fn new(proxy: SessionProxy<'a>) -> Self {
        Self { proxy }
    }

    pub async fn parse_search(&self, page_count: u8) -> CrawlerResult<Vec<SearchResult>> {
        let driver_ext = self.proxy.session.as_ref().unwrap();
        parse_search(driver_ext, page_count).await
    }

    pub async fn set_search_filters(
        &self,
        keywords: Option<&str>,
        locations: Option<&[String]>,
        current_company: Option<&[String]>,
        past_company: Option<&[String]>,
        industry: Option<&[String]>,
        services: Option<&[String]>,
    ) -> CrawlerResult<()> {
        let driver_ext = self.proxy.session.as_ref().unwrap();
        if let Err(e) = driver_ext.driver.goto("https://www.linkedin.com/search/results/people/").await {
            return Err(DriverError(format!("Failed to go to linkedin {}", e)));
        }

        if let Some(keywords) = keywords {
            search_set_keywords(driver_ext, keywords).await?;
        }

        try_press_filter_button(driver_ext).await?;

        if let Some(locations) = locations {
            search_set_filter(driver_ext, locations).await?;
        }
        if let Some(current_company) = current_company {
            search_set_current_company(driver_ext, current_company).await?;
        }
        if let Some(past_company) = past_company {
            search_set_past_company(driver_ext, past_company).await?;
        }
        if let Some(industries) = industry {
            search_set_industry(driver_ext, industries).await?;
        }
        if let Some(services) = services {
            search_set_services(driver_ext, services).await?;
        }

        let show_results_button = match driver_ext
            .find_until_loaded(By::XPath("//span[normalize-space()='Show results']/.."), Duration::from_secs(5))
            .await
        {
            Ok(show_results_button) => show_results_button,
            Err(e) => return Err(DriverError(format!("Failed to find show results button {}", e))),
        };

        if let Err(e) = show_results_button.click().await {
            return Err(DriverError(format!("Failed to click show results button {}", e)));
        }

        Ok(())
    }
}
