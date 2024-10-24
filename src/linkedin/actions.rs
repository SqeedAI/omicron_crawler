use crate::driver_ext::WebDriverExt;
use crate::linkedin::enums::Functions;
use std::time::Duration;
use thirtyfour::common::action::KeyAction::KeyDown;
use thirtyfour::prelude::{ElementQueryable, ElementWaitable};
use thirtyfour::{By, Key, WebDriver};

pub async fn set_function_search(driver: &WebDriverExt, function: Functions) {
    let function_button = fatal_unwrap_e!(
        driver
            .find_until_loaded(
                By::XPath("/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[2]/div/fieldset[1]/div/button"),
                Duration::from_secs(5)
            )
            .await,
        "Failed to find function button {}"
    );
    fatal_unwrap_e!(function_button.click().await, "Failed to click function button {}");
    let input_field = fatal_unwrap_e!(
        driver
            .driver
            .find(By::XPath(
                "/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[2]/div/fieldset[1]/div[3]/div[1]/div[1]/div/input"
            ))
            .await,
        "Failed to find input field {}"
    );

    input_field.send_keys(function.as_str()).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    input_field.send_keys(Key::Down).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    input_field.send_keys(Key::Enter).await.unwrap();
}

pub async fn set_job_title_search(driver: &WebDriverExt, job_title: String) {
    let job_title_button = fatal_unwrap_e!(
        driver
            .find_until_loaded(
                By::XPath("/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[2]/div/fieldset[2]/div/button"),
                Duration::from_secs(5)
            )
            .await,
        "Failed to find job title button {}"
    );
    fatal_unwrap_e!(job_title_button.click().await, "Failed to click job title button {}");
    let input_field = fatal_unwrap_e!(
        driver
            .driver
            .find(By::XPath(
                "/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[2]/div/fieldset[2]/div[3]/div[1]/div[1]/div/input"
            ))
            .await,
        "Failed to find input field {}"
    );

    input_field.send_keys(job_title).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    input_field.send_keys(Key::Down).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    input_field.send_keys(Key::Enter).await.unwrap();
}

pub async fn set_geography_search(driver: &WebDriverExt, geography: String) {
    let geography_button = fatal_unwrap_e!(
        driver
            .find_until_loaded(
                By::XPath("/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[3]/div/fieldset[1]/div/button"),
                Duration::from_secs(5)
            )
            .await,
        "Failed to find geography button {}"
    );
    fatal_unwrap_e!(geography_button.click().await, "Failed to click job title button {}");
    let input_field = fatal_unwrap_e!(
        driver
            .driver
            .find(By::XPath(
                "/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[3]/div/fieldset[1]/div[3]/div[1]/div[1]/div/input"
            ))
            .await,
        "Failed to find input field {}"
    );

    input_field.send_keys(geography).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    input_field.send_keys(Key::Down).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    input_field.send_keys(Key::Enter).await.unwrap();
}
