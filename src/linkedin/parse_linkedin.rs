use crate::driver::session::DriverSession;
use crate::errors::CrawlerError::{InteractionError, ParseError};
use crate::errors::CrawlerResult;
use crate::linkedin::profiles::SearchResult;
use std::time::Duration;
use thirtyfour::{By, Key, WebElement};

pub async fn try_press_filter_button(driver: &DriverSession) -> CrawlerResult<()> {
    let filter_button = match driver
        .find_until_loaded(By::XPath("//button[normalize-space()='All filters']"), Duration::from_secs(5))
        .await
    {
        Ok(all_filters_button) => all_filters_button,
        Err(e) => return Err(ParseError(format!("Failed to find all filters button {}", e))),
    };
    if let Err(result) = filter_button.click().await {
        return Err(InteractionError(format!("Failed to click all filters button {}", result)));
    }
    Ok(())
}

pub async fn search_set_filter(driver: &DriverSession, locations: &[String]) -> CrawlerResult<()> {
    for location in locations {
        let filter_add_button = match driver
            .find_until_loaded(By::XPath("//span[normalize-space()='Add a location']/.."), Duration::from_secs(5))
            .await
        {
            Ok(all_filters_button) => all_filters_button,
            Err(e) => return Err(ParseError(format!("Failed to find location add filter button {}", e))),
        };

        if let Err(e) = filter_add_button.click().await {
            return Err(InteractionError(format!("Failed to click add location filters button {}", e)));
        }

        let location_input = match driver
            .find_until_loaded(By::XPath("//input[@placeholder='Add a location']"), Duration::from_secs(5))
            .await
        {
            Ok(all_filters_button) => all_filters_button,
            Err(_) => return Err(ParseError("Failed to find location input".to_string())),
        };
        location_input.send_keys(location).await.unwrap();
        tokio::time::sleep(Duration::from_millis(700)).await;
        location_input.send_keys(Key::Down).await.unwrap();
        tokio::time::sleep(Duration::from_millis(200)).await;
        location_input.send_keys(Key::Enter).await.unwrap();
    }
    Ok(())
}
pub async fn search_set_current_company(driver: &DriverSession, companies: &[String]) -> CrawlerResult<()> {
    for company in companies {
        let filter_add_button = match driver
            .find_until_loaded(
                By::XPath("(//span[normalize-space()='Add a company'])[1]/.."),
                Duration::from_secs(5),
            )
            .await
        {
            Ok(all_filters_button) => all_filters_button,
            Err(e) => return Err(InteractionError(format!("Failed to find company add filter button {}", e))),
        };

        if let Err(e) = filter_add_button.click().await {
            return Err(InteractionError(format!("Failed to click add company add filters button {}", e)));
        }

        let location_input = match driver
            .find_until_loaded(By::XPath("//input[@placeholder='Add a company']"), Duration::from_secs(5))
            .await
        {
            Ok(all_filters_button) => all_filters_button,
            Err(_) => return Err(ParseError("Failed to find locations input button".to_string())),
        };
        location_input.send_keys(company).await.unwrap();
        tokio::time::sleep(Duration::from_millis(700)).await;
        location_input.send_keys(Key::Down).await.unwrap();
        tokio::time::sleep(Duration::from_millis(200)).await;
        location_input.send_keys(Key::Enter).await.unwrap();
    }
    Ok(())
}
pub async fn search_set_past_company(driver: &DriverSession, companies: &[String]) -> CrawlerResult<()> {
    for company in companies {
        let filter_add_button = match driver
            .find_until_loaded(
                By::XPath("(//span[normalize-space()='Add a company'])[2]/.."),
                Duration::from_secs(5),
            )
            .await
        {
            Ok(all_filters_button) => all_filters_button,
            Err(e) => return Err(ParseError(format!("Failed to find Past Add a company filter button {}", e))),
        };

        if let Err(e) = filter_add_button.scroll_into_view().await {
            return Err(InteractionError(format!(
                "Failed to scroll to Past Add a company filter button {}",
                e
            )));
        }

        if let Err(e) = filter_add_button.click().await {
            return Err(InteractionError(format!("Failed to click past Add a company filters button {}", e)));
        }

        let location_input = match driver
            .find_until_loaded(By::XPath("//input[@placeholder='Add a company']"), Duration::from_secs(5))
            .await
        {
            Ok(all_filters_button) => all_filters_button,
            Err(_) => return Err(ParseError("Failed to find Add a company input button".to_string())),
        };
        location_input.send_keys(company).await.unwrap();
        tokio::time::sleep(Duration::from_millis(700)).await;
        location_input.send_keys(Key::Down).await.unwrap();
        tokio::time::sleep(Duration::from_millis(200)).await;
        location_input.send_keys(Key::Enter).await.unwrap();
    }
    Ok(())
}
pub async fn search_set_industry(driver: &DriverSession, industries: &[String]) -> CrawlerResult<()> {
    for industry in industries {
        let filter_add_button = match driver
            .find_until_loaded(By::XPath("//span[normalize-space()='Add an industry']/.."), Duration::from_secs(5))
            .await
        {
            Ok(all_filters_button) => all_filters_button,
            Err(e) => return Err(ParseError(format!("Failed to find Add an industry filter button {}", e))),
        };

        if let Err(e) = filter_add_button.scroll_into_view().await {
            return Err(InteractionError(format!("Failed to scroll to Add an industry filter button {}", e)));
        }

        if let Err(e) = filter_add_button.click().await {
            return Err(InteractionError(format!("Failed to click Add an industry filters button {}", e)));
        }

        let location_input = match driver
            .find_until_loaded(By::XPath("//input[@placeholder='Add an industry']"), Duration::from_secs(5))
            .await
        {
            Ok(all_filters_button) => all_filters_button,
            Err(_) => return Err(ParseError("Failed to find Add an industry input button".to_string())),
        };
        location_input.send_keys(industry).await.unwrap();
        tokio::time::sleep(Duration::from_millis(700)).await;
        location_input.send_keys(Key::Down).await.unwrap();
        tokio::time::sleep(Duration::from_millis(200)).await;
        location_input.send_keys(Key::Enter).await.unwrap();
    }
    Ok(())
}
pub async fn search_set_services(driver: &DriverSession, services: &[String]) -> CrawlerResult<()> {
    for service in services {
        let filter_add_button = match driver
            .find_until_loaded(By::XPath("//span[normalize-space()='Add a service']/.."), Duration::from_secs(5))
            .await
        {
            Ok(all_filters_button) => all_filters_button,
            Err(e) => return Err(ParseError(format!("Failed to find Add a service filter button {}", e))),
        };

        if let Err(e) = filter_add_button.scroll_into_view().await {
            return Err(InteractionError(format!("Failed to scroll to Add a service filter button {}", e)));
        }

        if let Err(e) = filter_add_button.click().await {
            return Err(InteractionError(format!("Failed to click Add a service filters button {}", e)));
        }

        let location_input = match driver
            .find_until_loaded(By::XPath("//input[@placeholder='Add a service']"), Duration::from_secs(5))
            .await
        {
            Ok(all_filters_button) => all_filters_button,
            Err(_) => return Err(ParseError("Failed to find Add a service input button".to_string())),
        };
        location_input.send_keys(service).await.unwrap();
        tokio::time::sleep(Duration::from_millis(700)).await;
        location_input.send_keys(Key::Down).await.unwrap();
        tokio::time::sleep(Duration::from_millis(200)).await;
        location_input.send_keys(Key::Enter).await.unwrap();
    }
    Ok(())
}
pub async fn search_set_keywords(driver: &DriverSession, keywords: &str) -> CrawlerResult<()> {
    let search_input = match driver
        .find_until_loaded(By::XPath("//input[@placeholder='Search']"), Duration::from_secs(5))
        .await
    {
        Ok(all_filters_button) => all_filters_button,
        Err(e) => return Err(ParseError(format!("Failed to find Add a service filter button {}", e))),
    };
    if let Err(e) = search_input.click().await {
        return Err(InteractionError(format!("Failed to click search input {}", e)));
    }

    if let Err(e) = search_input.send_keys(keywords).await {
        return Err(InteractionError(format!("Failed to send keys to search input {}", e)));
    }
    if let Err(e) = search_input.send_keys(Key::Enter).await {
        return Err(InteractionError(format!("Failed to send keys to search input {}", e)));
    }
    Ok(())
}

pub async fn parse_search_entry(driver: &DriverSession, search_entry: WebElement, results: &mut Vec<SearchResult>) -> CrawlerResult<()> {
    let link = match search_entry.find(By::XPath(".//a")).await {
        Ok(link) => link,
        Err(_) => return Err(ParseError("Failed to find link".to_string())),
    };
    let url = match link.attr("href") {
        Ok(link_string) => link_string,
        Err(_) => return Err(ParseError("Failed to get link string".to_string())),
    };

    let name = match link.find(By::XPath("./span/span[1]/")).await {
        Ok(name) => match name.text() {
            Ok(name) => name,
            Err(_) => return Err(ParseError("Failed to get name text".to_string())),
        },
        Err(_) => return Err(ParseError("Failed to find name".to_string())),
    };

    let title = match search_entry.find(By::XPath("/div/div/div/[2]/div[2]/")).await {
        Ok(title) => match title.text() {
            Ok(title) => title,
            Err(_) => return Err(ParseError("Failed to get title text".to_string())),
        },
        Err(_) => return Err(ParseError("Failed to find title".to_string())),
    };

    results.push(SearchResult { name, title, url });
    Ok(())
}

pub async fn parse_search(driver: &DriverSession) -> CrawlerResult<Vec<SearchResult>> {
    let list = driver.find_all(By::XPath("//ul[@role='list']/li")).await?;
    for entry in list {
        parse_search_entry(entry, &mut Vec::new(), driver).await?;
    }
}
