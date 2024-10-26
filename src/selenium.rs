use crate::driver_ext::WebDriverExt;
use crate::linkedin::actions::{parse_search, set_function_search, set_geography_search, set_job_title_search};
use crate::linkedin::enums::{Functions, SeniorityLevel};
use crate::EMAIL;
use std::time::Duration;
use thirtyfour::By;

pub struct SeleniumLinkedin {
    driver_ext: WebDriverExt,
}

impl SeleniumLinkedin {
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
    pub async fn handle_google_sign_in(&self) {
        let driver_ext = &self.driver_ext;
        let element = fatal_unwrap_e!(
            driver_ext.driver.find(By::XPath("//*[@title='Sign in with Google Button']")).await,
            "Failed to find google sign in text {}"
        );
        let parent = fatal_unwrap_e!(element.parent().await, "failed to get parent {}");
        let button = fatal_unwrap_e!(parent.find(By::XPath("./child::*[1]")).await, "Failed to find children elements {}");
        let window_handles = fatal_unwrap_e!(driver_ext.driver.windows().await, "Failed to get window handles {}");
        let main_window_handle = &window_handles[0];
        fatal_unwrap_e!(button.click().await, "Failed to click google sign in button {}");
        let window_handles = fatal_unwrap_e!(driver_ext.driver.windows().await, "Failed to get window handles {}");
        let mut second_window_handle = None;
        for handle in window_handles.iter() {
            if handle != main_window_handle {
                second_window_handle = Some(handle);
            }
        }
        let second_window_handle_found = fatal_unwrap!(second_window_handle, "Failed to find sign in window");
        fatal_unwrap_e!(
            driver_ext.driver.switch_to_window(second_window_handle_found.clone()).await,
            "Failed to switch to window {}"
        );
        let email_element = fatal_unwrap_e!(
            driver_ext.driver.find(By::Id("identifierId")).await,
            "Failed to find email element {}"
        );
        fatal_unwrap_e!(email_element.send_keys(EMAIL).await, "Failed to send email {}");
        let next_button = fatal_unwrap_e!(
            driver_ext
                .driver
                .find(By::XPath(
                    "/html/body/div[1]/div[1]/div[2]/c-wiz/div/div[3]/div/div[1]/div/div/button"
                ))
                .await,
            "Failed to find next button {}"
        );
        next_button.click().await.unwrap();
        tokio::time::sleep(Duration::from_secs(15)).await;
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
    pub async fn parse_profiles(&self) {
        let driver_ext = &self.driver_ext;
        parse_search(driver_ext).await;
    }
}
