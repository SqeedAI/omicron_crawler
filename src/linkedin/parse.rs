use crate::driver_ext::WebDriverExt;
use crate::linkedin::enums::Functions;
use crate::linkedin::profiles::{Experience, Profile, SearchResult};
use crate::utils::get_domain_url;
use std::time::Duration;
use thirtyfour::common::action::KeyAction::KeyDown;
use thirtyfour::prelude::{ElementQueryable, ElementWaitable};
use thirtyfour::{By, Key, WebDriver, WebElement};

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
    tokio::time::sleep(Duration::from_millis(700)).await;
    input_field.send_keys(Key::Down).await.unwrap();
    tokio::time::sleep(Duration::from_millis(700)).await;
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
    tokio::time::sleep(Duration::from_millis(700)).await;
    input_field.send_keys(Key::Down).await.unwrap();
    tokio::time::sleep(Duration::from_millis(700)).await;
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

pub async fn parse_search(driver: &WebDriverExt) -> Vec<SearchResult> {
    let domain_url = get_domain_url(driver.driver.current_url().await.unwrap().as_str());
    let search_list: WebElement = fatal_unwrap_e!(
        driver
            .find_until_loaded(By::Id("search-results-container"), Duration::from_secs(5))
            .await,
        "Failed to find search list {}"
    );
    let ol_element: WebElement = fatal_unwrap_e!(search_list.find(By::Tag("ol")).await, "Failed to find ol tag {}");
    // Find all li elements within the ol
    let li_elements: Vec<WebElement> = fatal_unwrap_e!(
        ol_element.find_all(By::XPath("./li")).await,
        "Failed to find direct child li elements {}"
    );
    trace!("Found {} profiles", li_elements.len());
    let mut results = Vec::with_capacity(li_elements.len());
    for li_element in li_elements {
        let name_span_result = li_element.find(By::XPath(".//span[@data-anonymize='person-name']")).await;
        let name_span: WebElement;

        if let Ok(span) = name_span_result {
            name_span = span;
        } else {
            trace!("Failed to find name span... scrolling");
            fatal_unwrap_e!(li_element.scroll_into_view().await, "Failed to scroll into view {}");
            tokio::time::sleep(Duration::from_millis(250)).await;
            name_span = fatal_unwrap_e!(
                li_element.find(By::XPath(".//span[@data-anonymize='person-name']")).await,
                "Failed to find name span after scrolling {}"
            );
        }
        let title_span = fatal_unwrap_e!(
            li_element.find(By::XPath(".//span[@data-anonymize='title']")).await,
            "Failed to find title span {}"
        );

        let a_element = fatal_unwrap_e!(name_span.parent().await, "Failed to find parent element {}");
        let a_href = fatal_unwrap_e!(a_element.attr("href").await, "Failed to get href attribute {}").unwrap();
        let url = format!("{}{}", domain_url, a_href);

        results.push(SearchResult {
            name: name_span.text().await.unwrap(),
            title: title_span.text().await.unwrap(),
            sales_url: url,
        });
    }
    results
}

pub async fn parse_profile_about(driver: &WebDriverExt) -> Option<String> {
    let possible_about_title = driver.driver.find(By::XPath("//h1[normalize-space()='About']")).await;
    if let Err((err)) = possible_about_title {
        info!("No about section found.");
        return None;
    }
    let about_title = possible_about_title.unwrap();
    about_title.scroll_into_view().await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;

    //Check if show more is needed to click
    let parent = about_title.parent().await.unwrap();
    let possible_show_more_button = parent.find(By::XPath(".//div/span/button")).await;

    // In case we have show more, then the text element is also p instead of span
    if let Ok(show_more_button) = possible_show_more_button {
        trace!("Show more button found. Clicking...");
        fatal_unwrap_e!(show_more_button.click().await, "Failed to click show more button {}");
        let about_p = fatal_unwrap_e!(parent.find(By::XPath("./p")).await, "Failed to find about p {}");
        return Some(about_p.text().await.unwrap().replace("Show less", ""));
    }
    trace!("No show more button found.");

    //In case we don't have show more, then the text element is span
    let about_span = fatal_unwrap_e!(parent.find(By::XPath(".//div/span")).await, "Failed to find about span {}");
    Some(about_span.text().await.unwrap())
}
pub async fn parse_experience(driver: &WebDriverExt) -> Option<Vec<Experience>> {
    let possible_experience_title = driver.driver.find(By::XPath("//h2[contains(., 'experience')]")).await;
    let mut results = Vec::new();
    if let Err((err)) = possible_experience_title {
        info!("No experience section found.");
        return None;
    }
    let experience = possible_experience_title.unwrap();
    let experience_parent = fatal_unwrap_e!(experience.parent().await, "Failed to find experience parent {}");
    tokio::time::sleep(Duration::from_millis(100)).await;
    experience_parent.scroll_into_view().await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    let experience_ul = fatal_unwrap_e!(experience_parent.find(By::XPath("./ul")).await, "Failed to find experience ul {}");
    let experience_list = fatal_unwrap_e!(experience_ul.find_all(By::XPath("./li")).await, "Failed to find experience list {}");
    for experience_entry in experience_list {
        parse_experience_entry(experience_entry, &mut results).await;
    }

    Some(results)
}

pub async fn parse_timeline(timeline: WebElement, results: &mut Vec<Experience>) {
    let experience_entries = fatal_unwrap_e!(timeline.find_all(By::XPath("./li")).await, "Failed to find experience list {}");
    for experience_entry in experience_entries {
        let title = fatal_unwrap_e!(
            experience_entry.find(By::XPath(".//h3")).await,
            "Failed to find experience title {}"
        );
        let time = fatal_unwrap_e!(
            experience_entry.find(By::XPath("./div/p[1]/span")).await,
            "Failed to find experience duration {}"
        );
        let time_text = time.text().await.unwrap();
        let (start, end) = time_text.split_once("–").unwrap();
        results.push(Experience {
            position: title.text().await.unwrap(),
            start: start.to_string(),
            end: end.to_string(),
        });
    }
}

pub async fn parse_experience_entry(experience_entry: WebElement, result: &mut Vec<Experience>) {
    let possible_timeline = experience_entry.find(By::XPath("./ul")).await;
    if let Ok(timeline) = possible_timeline {
        trace!("Timeline found.");
        parse_timeline(timeline, result).await;
        return;
    }

    let job_title = fatal_unwrap_e!(
        experience_entry.find(By::XPath(".//h2")).await,
        "Failed to find experience title {}"
    );
    let time = fatal_unwrap_e!(
        experience_entry.find(By::XPath(".//p/span")).await,
        "Failed to find experience duration {}"
    );
    let time_text = time.text().await.unwrap();
    let (start, end) = time_text.split_once("–").unwrap();
    let title = job_title.text().await.unwrap();
    result.push(Experience {
        position: title,
        start: start.to_string(),
        end: end.to_string(),
    });
}

pub async fn parse_sales_profile(driver: &WebDriverExt, sales_profile_url: &str) -> Profile {
    driver.driver.goto(sales_profile_url).await.unwrap();
    let name_span: WebElement = fatal_unwrap_e!(
        driver
            .find_until_loaded(By::XPath(".//h1[@data-anonymize='person-name']"), Duration::from_secs(5))
            .await,
        "Failed to find name span after scrolling {}"
    );
    let profile_options = fatal_unwrap_e!(
        driver.driver.find(By::XPath("//*[@id='hue-menu-trigger-ember51']")).await,
        "Failed to find profile options {}"
    );

    fatal_unwrap_e!(profile_options.click().await, "Failed to click profile options {}");

    let linkedin_url_element = fatal_unwrap_e!(
        driver
            .find_until_loaded(By::XPath("/html/body/div[1]/div[2]/ul/li[2]/a"), Duration::from_secs(5))
            .await,
        "Failed to find linkedin url element {}"
    );
    let linkedin_url = linkedin_url_element.attr("href").await.unwrap().unwrap();
    // To close the menu
    driver.driver.action_chain().send_keys(Key::Escape).perform().await.unwrap();
    let description_element = fatal_unwrap_e!(
        driver.driver.find(By::XPath(".//span[@data-anonymize='headline']")).await,
        "Failed to find name span after scrolling {}"
    );

    let location = fatal_unwrap_e!(
        driver
            .driver
            .find(By::XPath(
                "/html/body/main/div[1]/div[3]/div/div/div[1]/div/div/section[1]/section[1]/div[1]/div[4]/div[1]"
            ))
            .await,
        "Failed to find location span after scrolling {}"
    );

    let about = parse_profile_about(driver).await;
    let experience = parse_experience(driver).await;

    Profile {
        name: name_span.text().await.unwrap().to_string(),
        url: linkedin_url,
        description: description_element.text().await.unwrap(),
        about,
        location: location.text().await.unwrap(),
        connections: String::new(),
        experience,
        education: None,
        skills: Vec::new(),
        languages: Vec::new(),
    }
}
