use crate::selenium::Selenium;
use crate::EMAIL;
use std::time::Duration;
use thirtyfour::By;

async fn handle_google_sign_in(selenium: Selenium) {
    let driver = &selenium.driver;
    fatal_unwrap_e!(driver.goto("https://www.linkedin.com/").await, "Failed to go to linkedin {}");
    let element = fatal_unwrap_e!(
        driver.find(By::XPath("//*[@title='Sign in with Google Button']")).await,
        "Failed to find google sign in text {}"
    );
    let parent = fatal_unwrap_e!(element.parent().await, "failed to get parent {}");
    let button = fatal_unwrap_e!(parent.find(By::XPath("./child::*[1]")).await, "Failed to find children elements {}");
    let window_handles = fatal_unwrap_e!(driver.windows().await, "Failed to get window handles {}");
    let main_window_handle = &window_handles[0];
    fatal_unwrap_e!(button.click().await, "Failed to click google sign in button {}");
    let window_handles = fatal_unwrap_e!(driver.windows().await, "Failed to get window handles {}");
    let mut second_window_handle = None;
    for handle in window_handles.iter() {
        if handle != main_window_handle {
            second_window_handle = Some(handle);
        }
    }
    let second_window_handle_found = fatal_unwrap!(second_window_handle, "Failed to find sign in window");
    fatal_unwrap_e!(
        driver.switch_to_window(second_window_handle_found.clone()).await,
        "Failed to switch to window {}"
    );
    let email_element = fatal_unwrap_e!(driver.find(By::Id("identifierId")).await, "Failed to find email element {}");
    fatal_unwrap_e!(email_element.send_keys(EMAIL).await, "Failed to send email {}");
    let next_button = fatal_unwrap_e!(
        driver
            .find(By::XPath(
                "/html/body/div[1]/div[1]/div[2]/c-wiz/div/div[3]/div/div[1]/div/div/button"
            ))
            .await,
        "Failed to find next button {}"
    );
    next_button.click().await.unwrap();
    tokio::time::sleep(Duration::from_secs(15)).await;
}
