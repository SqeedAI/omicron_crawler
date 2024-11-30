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

pub async fn parse_search_entry(search_entry: &WebElement, results: &mut Vec<SearchResult>) -> CrawlerResult<()> {
    let link = match search_entry
        .find(By::XPath("div/div/div/div[2]/div[1]/div[1]/div/span[1]/span/a"))
        .await
    {
        Ok(link) => link,
        Err(e) => return Err(ParseError(format!("Failed to find link {}", e))),
    };
    let url = match link.attr("href").await {
        Ok(link_string) => link_string.unwrap(),
        Err(_) => return Err(ParseError("Failed to get link string".to_string())),
    };

    let name = match link.find(By::XPath("span/span[1]")).await {
        Ok(name) => match name.text().await {
            Ok(name) => name,
            Err(_) => return Err(ParseError("Failed to get name text".to_string())),
        },
        Err(_) => return Err(ParseError("Failed to find name".to_string())),
    };

    let title = match search_entry.find(By::XPath("div/div/div/div[2]/div[1]/div[2]")).await {
        Ok(title) => match title.text().await {
            Ok(title) => title,
            Err(_) => return Err(ParseError("Failed to get title text".to_string())),
        },
        Err(_) => return Err(ParseError("Failed to find title".to_string())),
    };

    results.push(SearchResult { name, title, url });
    Ok(())
}
//TODO Make a retry macro for stale element issues
pub async fn parse_search(driver: &DriverSession, page_count: u8) -> CrawlerResult<Vec<SearchResult>> {
    let mut results = Vec::new();

    // FIRST PAGE contains sales nav so the path to the initial UL is different for the first page.
    // We could make the code not duplicate but the logic would require an if inside the loop which is slow.
    let ul = match driver
        .find_until_loaded(By::XPath("(//ul[@role='list'])[2]"), Duration::from_secs(5))
        .await
    {
        Ok(ul) => ul,
        Err(e) => return Err(ParseError(format!("Failed to find ul {}", e))),
    };

    let list = match ul.find_all(By::XPath("li")).await {
        Ok(list) => list,
        Err(e) => return Err(ParseError(format!("Failed to find list {}", e))),
    };

    for entry in list.iter() {
        parse_search_entry(entry, &mut results).await?;
    }

    for page in 2..page_count + 1 {
        load_results_page(driver, page).await?;
        let mut retry = 5u8;
        while retry != 0 {
            let ul = match driver
                .find_until_loaded(By::XPath("(//ul[@role='list'])[1]"), Duration::from_secs(5))
                .await
            {
                Ok(ul) => ul,
                Err(e) => return Err(ParseError(format!("Failed to find ul {}", e))),
            };

            let list = match ul.find_all(By::XPath("li")).await {
                Ok(list) => list,
                Err(e) => return Err(ParseError(format!("Failed to find list {}", e))),
            };

            for entry in list.iter() {
                match parse_search_entry(entry, &mut results).await {
                    Ok(_) => {
                        retry = 1;
                    }
                    Err(_) => {
                        tokio::time::sleep(Duration::from_millis(250)).await;
                        break;
                    }
                }
            }
            retry -= 1;
        }
    }
    Ok(results)
}

pub async fn load_results_page(driver: &DriverSession, page: u8) -> CrawlerResult<()> {
    let mut current_url = match driver.driver.current_url().await {
        Ok(url) => url,
        Err(e) => return Err(ParseError(format!("Failed to get current url {}", e))),
    };

    let mut query_pairs = current_url.query_pairs();
    let mut new_query_string = String::new();
    let mut found_page = false;
    for (key, value) in query_pairs.by_ref() {
        if !new_query_string.is_empty() {
            new_query_string.push('&');
        }
        new_query_string.push_str(key.as_ref());
        new_query_string.push('=');
        if key.as_ref() == "page" {
            new_query_string.push_str(page.to_string().as_str());
            found_page = true;
            continue;
        }
        new_query_string.push_str(value.as_ref());
    }

    if !found_page {
        if !new_query_string.is_empty() {
            new_query_string.push('&');
        }
        new_query_string.push_str(&format!("page={}", page));
    }

    current_url.set_query(Some(new_query_string.as_str()));
    if let Err(e) = driver.driver.goto(&current_url.to_string()).await {
        return Err(InteractionError(format!("Failed to load page {}", e)));
    }
    Ok(())
}
