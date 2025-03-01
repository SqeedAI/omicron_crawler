use crate::driver::session::DriverSession;
use crate::errors::CrawlerError::{DriverError, InteractionError, ParseError};
use crate::errors::{CrawlerError, CrawlerResult};
use crate::linkedin::web_driver::profiles;
use crate::linkedin::web_driver::profiles::{Education, Experience, Interval, Language, Profile, SearchResult, Skill};
use crate::utils::get_domain_url;
use regex::Regex;
use std::fmt::format;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use thirtyfour::common::action::KeyAction::KeyDown;
use thirtyfour::error::WebDriverResult;
use thirtyfour::prelude::{ElementQueryable, ElementWaitable};
use thirtyfour::{By, Key, WebDriver, WebElement};
use tokio::time::Sleep;
//Optimize think about cases where String is moved but can be passed as a reference

pub async fn set_subject(mail_form: &WebElement, subject: &str) -> CrawlerResult<()> {
    info!("Detected in-mail input.");
    let subject_element = match mail_form.find(By::XPath("./div/input")).await {
        Ok(subject) => subject,
        Err(_) => return Err(ParseError(String::from_str("Failed to find subject input").unwrap())),
    };

    if let Err(_) = subject_element.send_keys(subject.to_string()).await {
        return Err(InteractionError(String::from_str("Failed to send keys to subject input").unwrap()));
    }
    tokio::time::sleep(Duration::from_millis(200)).await;
    Ok(())
}

pub async fn set_message(mail_form: &WebElement, body: &str) -> CrawlerResult<()> {
    info!("Detected connection message input.");
    let text_area = match mail_form.find(By::XPath("./div/section/textarea")).await {
        Ok(text_area) => text_area,
        Err(_) => return Err(ParseError(String::from_str("Failed to find text area").unwrap())),
    };
    text_area.send_keys(body.to_string()).await.unwrap();
    tokio::time::sleep(Duration::from_millis(200)).await;
    Ok(())
}
pub async fn send_message(driver: &DriverSession, profile_sales_url: &str, subject: &str, body: &str) -> CrawlerResult<()> {
    if let Err(result) = driver.driver.get(profile_sales_url).await {
        return Err(DriverError(format!("Failed to get sales page: {}", result)));
    }
    let message_button = match driver
        .find_until_loaded(
            By::XPath("/html/body/main/div[1]/div[3]/div/div/div[1]/div/div/section[1]/section[1]/div[2]/section/div[1]/div[2]/button"),
            Duration::from_secs(5),
        )
        .await
    {
        Ok(message_button) => message_button,
        Err(_) => return Err(ParseError(String::from_str("Failed to find message button").unwrap())),
    };
    match message_button.click().await {
        Ok(_) => {}
        Err(_) => return Err(InteractionError(String::from_str("Failed to click message button").unwrap())),
    }

    let mail_form = match driver
        .find_until_loaded(
            By::XPath("/html/body/div[9]/section/div[2]/section/div[2]/form[1]"),
            Duration::from_secs(5),
        )
        .await
    {
        Ok(mail_form) => mail_form,
        Err(_) => return Err(ParseError(String::from_str("Failed to find mail form").unwrap())),
    };

    if let Err(_) = set_subject(&mail_form, subject).await {
        info!("No subject input found");
    };

    set_message(&mail_form, body).await?;

    let send_message_button = match mail_form.find(By::XPath("./fieldset/section/div/button[2]")).await {
        Ok(send_message_button) => send_message_button,
        Err(_) => return Err(ParseError(String::from_str("Failed to find send message button").unwrap())),
    };
    if let Err(e) = send_message_button.click().await {
        return Err(InteractionError(format!("Failed to click send message button {}", e)));
    }
    /// TODO More robust way to wait for message to be sent
    tokio::time::sleep(Duration::from_millis(700)).await;

    Ok(())
}
pub async fn set_keyword_search(driver_session: &DriverSession, keywords: String) -> CrawlerResult<()> {
    let input_element = match driver_session
        .find_until_loaded(By::XPath("//*[@id='global-typeahead-search-input']"), Duration::from_secs(5))
        .await
    {
        Ok(input_element) => input_element,
        Err(_) => return Err(ParseError(String::from_str("Failed to find input element").unwrap())),
    };
    match input_element.click().await {
        Ok(_) => {}
        Err(_) => return Err(InteractionError(String::from_str("Failed to click input element").unwrap())),
    }
    input_element.send_keys(keywords.to_string()).await.unwrap();
    tokio::time::sleep(Duration::from_millis(700)).await;
    input_element.send_keys(Key::Enter).await.unwrap();
    Ok(())
}

pub async fn set_function_search(driver: &DriverSession, function: String) -> CrawlerResult<()> {
    let function_button = match driver
        .find_until_loaded(
            By::XPath("/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[2]/div/fieldset[1]/div/button"),
            Duration::from_secs(5),
        )
        .await
    {
        Ok(function_button) => function_button,
        Err(_) => return Err(InteractionError(String::from_str("Failed to find function filter button").unwrap())),
    };

    if let Err(_) = function_button.click().await {
        return Err(InteractionError(
            String::from_str("Failed to click function filter button").unwrap(),
        ));
    }

    let input_field = match driver
        .driver
        .find(By::XPath(
            "/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[2]/div/fieldset[1]/div[3]/div[1]/div[1]/div/input",
        ))
        .await
    {
        Ok(input_field) => input_field,
        Err(_) => {
            return Err(InteractionError(
                String::from_str("Failed to find input field for function filter").unwrap(),
            ));
        }
    };

    input_field.send_keys(function.to_string()).await.unwrap();
    tokio::time::sleep(Duration::from_millis(700)).await;
    input_field.send_keys(Key::Down).await.unwrap();
    tokio::time::sleep(Duration::from_millis(700)).await;
    input_field.send_keys(Key::Enter).await.unwrap();
    Ok(())
}

pub async fn set_job_title_search(driver: &DriverSession, job_title: String) -> CrawlerResult<()> {
    let job_title_button = match driver
        .find_until_loaded(
            By::XPath("/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[2]/div/fieldset[2]/div/button"),
            Duration::from_secs(5),
        )
        .await
    {
        Ok(job_title_button) => job_title_button,
        Err(_) => {
            return Err(InteractionError(
                String::from_str("Failed to find job title filter button").unwrap(),
            ))
        }
    };
    if let Err(_) = job_title_button.click().await {
        return Err(InteractionError(
            String::from_str("Failed to click job title filter button").unwrap(),
        ));
    }

    let input_field = match driver
        .driver
        .find(By::XPath(
            "/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[2]/div/fieldset[2]/div[3]/div[1]/div[1]/div/input",
        ))
        .await
    {
        Ok(input_field) => input_field,
        Err(_) => {
            return Err(InteractionError(
                String::from_str("Failed to find input field for job title filter").unwrap(),
            ));
        }
    };
    input_field.send_keys(job_title).await.unwrap();
    tokio::time::sleep(Duration::from_millis(700)).await;
    input_field.send_keys(Key::Down).await.unwrap();
    tokio::time::sleep(Duration::from_millis(700)).await;
    input_field.send_keys(Key::Enter).await.unwrap();
    Ok(())
}

pub async fn set_geography_search(driver: &DriverSession, geography: String) -> CrawlerResult<()> {
    let geography_button = match driver
        .find_until_loaded(
            By::XPath("/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[3]/div/fieldset[1]/div/button"),
            Duration::from_secs(5),
        )
        .await
    {
        Ok(geography_button) => geography_button,
        Err(_) => {
            return Err(InteractionError(
                String::from_str("Failed to find geography filter button").unwrap(),
            ))
        }
    };
    if let Err(_) = geography_button.click().await {
        return Err(InteractionError(
            String::from_str("Failed to click geography filter button").unwrap(),
        ));
    }

    let input_field = match driver
        .driver
        .find(By::XPath(
            "/html/body/main/div[1]/div[1]/div[2]/div[1]/form/div/div[4]/fieldset[3]/div/fieldset[1]/div[3]/div[1]/div[1]/div/input",
        ))
        .await
    {
        Ok(input_field) => input_field,
        Err(_) => {
            return Err(InteractionError(
                String::from_str("Failed to find input field for geography filter").unwrap(),
            ));
        }
    };

    input_field.send_keys(geography).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    input_field.send_keys(Key::Down).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    input_field.send_keys(Key::Enter).await.unwrap();
    Ok(())
}

pub async fn parse_search(driver: &DriverSession) -> CrawlerResult<Vec<SearchResult>> {
    let domain_url = get_domain_url(driver.driver.current_url().await.unwrap().as_str());
    let search_list = match driver
        .find_until_loaded(By::Id("search-results-container"), Duration::from_secs(5))
        .await
    {
        Ok(search_list) => search_list,
        Err(_) => return Err(ParseError(String::from_str("Failed to find search list").unwrap())),
    };

    let ol_element = match search_list.find(By::Tag("ol")).await {
        Ok(ol_element) => ol_element,
        Err(_) => return Err(ParseError(String::from_str("Failed to find ol tag").unwrap())),
    };

    let li_elements = match ol_element.find_all(By::XPath("./li")).await {
        Ok(li_elements) => li_elements,
        Err(_) => return Err(ParseError(String::from_str("Failed to find direct child li elements").unwrap())),
    };

    trace!("Found {} profiles", li_elements.len());
    let mut results = Vec::with_capacity(li_elements.len());
    for li_element in li_elements {
        parse_search_entry(li_element, &mut results, &domain_url).await?;
    }
    Ok(results)
}

pub async fn parse_search_entry(search_entry: WebElement, results: &mut Vec<SearchResult>, domain_url: &str) -> CrawlerResult<()> {
    let name_span = match search_entry.find(By::XPath(".//span[@data-anonymize='person-name']")).await {
        Ok(name_span) => name_span,
        Err(_) => {
            trace!("Failed to find name span... scrolling");
            match search_entry.scroll_into_view().await {
                Ok(_) => {}
                Err(_) => return Err(ParseError(String::from_str("Failed to scroll into view").unwrap())),
            }
            tokio::time::sleep(Duration::from_millis(250)).await;
            match search_entry.find(By::XPath(".//span[@data-anonymize='person-name']")).await {
                Ok(name_span) => name_span,
                Err(_) => return Err(ParseError(String::from_str("Failed to find name span after scrolling").unwrap())),
            }
        }
    };

    let title_span = match search_entry.find(By::XPath(".//span[@data-anonymize='title']")).await {
        Ok(title_span) => title_span.text().await.unwrap_or_else(|_| {
            warn!("Failed to get title span text");
            "".to_string()
        }),
        Err(_) => {
            warn!("Failed to find title span");
            "".to_string()
        }
    };

    let a_element = match name_span.parent().await {
        Ok(a_element) => a_element,
        Err(_) => {
            error!("Failed to find parent element");
            return Err(ParseError(String::from_str("Failed to find parent element").unwrap()));
        }
    };

    let a_href = match a_element.attr("href").await {
        Ok(a_href) => match a_href {
            Some(a_href) => a_href,
            None => {
                error!("Failed to get href attribute");
                return Err(ParseError(String::from_str("Failed to get href attribute").unwrap()));
            }
        },
        Err(_) => {
            error!("Failed to get href attribute");
            return Err(ParseError(String::from_str("Failed to get href attribute").unwrap()));
        }
    };
    let url = format!("{}{}", domain_url, a_href);
    let url = url.split(",").next().unwrap();

    results.push(SearchResult {
        name: name_span.text().await.unwrap(),
        title: title_span,
        url: url.to_string(),
    });
    Ok(())
}

pub async fn parse_about(driver: &DriverSession) -> Option<String> {
    let possible_about_title = driver.driver.find(By::XPath("//h1[normalize-space()='About']")).await;
    if let Err((err)) = possible_about_title {
        info!("No about section found.");
        return None;
    }
    let about_title = possible_about_title.unwrap();
    about_title.scroll_into_view().await.unwrap();

    //In case we have show more, then the text element is also p instead of span. This is the first branch
    let about_section = about_title.parent().await.unwrap();
    if let Some(about_p) = parse_about_show_more(&about_section).await {
        return Some(about_p);
    }

    //In case we don't have show more, then the text element is span. This is another branch / case
    match about_section.find(By::XPath(".//div/span")).await {
        Ok(about_span) => match about_span.text().await {
            Ok(about_span) => return Some(about_span),
            Err(_) => warn!("Failed to get about span text"),
        },
        Err(_) => warn!("Failed to find about span"),
    }
    None
}
pub async fn parse_about_show_more(about_section: &WebElement) -> Option<String> {
    let show_more_button = match about_section.find(By::XPath(".//div/span/button")).await {
        Ok(show_more_button) => show_more_button,
        Err(_) => {
            warn!("Failed to find show more button");
            return None;
        }
    };

    trace!("Show more button found. Clicking...");
    match show_more_button.click().await {
        Ok(_) => match about_section.find(By::XPath("./p")).await {
            Ok(about_p) => match about_p.text().await {
                Ok(about_p) => return Some(about_p.replace("Show less", "")),
                Err(_) => warn!("Failed to get about p text"),
            },
            Err(_) => warn!("Failed to find about p"),
        },
        Err(_) => warn!("Failed to click about show more button"),
    }
    None
}
pub async fn parse_experience(driver: &DriverSession) -> Option<Vec<Experience>> {
    let experience_section = match driver
        .driver
        .find(By::XPath("//section[@data-sn-view-name='feature-lead-experience']"))
        .await
    {
        Ok(experience_section) => {
            experience_section.scroll_into_view().await.unwrap();
            experience_section
        }
        Err(_) => {
            info!("No experience section found.");
            return None;
        }
    };

    match experience_section.find(By::XPath("./button")).await {
        Ok(show_more_button) => {
            show_more_button.scroll_into_view().await.unwrap();
            show_more_button.click().await.unwrap();
        }
        Err(_) => {
            info!("No show more button for experience found.");
        }
    };

    let experience_list = match experience_section.find_all(By::XPath("./div/ul/li")).await {
        Ok(experience_list) => experience_list,
        Err(_) => {
            info!("No experience list found.");
            return None;
        }
    };
    let mut results = Vec::new();
    for experience_entry in experience_list {
        parse_experience_entry(experience_entry, &mut results).await;
    }

    Some(results)
}

pub async fn parse_timeline(timeline: WebElement, results: &mut Vec<Experience>) {
    let experience_entries = match timeline.find_all(By::XPath("./li")).await {
        Ok(experience_entries) => experience_entries,
        Err(_) => {
            warn!("Failed to find experience list");
            return;
        }
    };

    for experience_entry in experience_entries {
        let title = match experience_entry.find(By::XPath(".//h3")).await {
            Ok(title) => title,
            Err(_) => {
                warn!("Failed to find experience title");
                continue;
            }
        };
        let time = match experience_entry.find(By::XPath("./div/p[1]/span")).await {
            Ok(time) => time,
            Err(_) => {
                warn!("Failed to find experience duration");
                continue;
            }
        };

        let time_text = time.text().await.unwrap();
        let interval = Interval::from_str(time_text.as_str(), "–").unwrap();
        results.push(Experience {
            position: title.text().await.unwrap(),
            interval,
        });
    }
}

pub async fn parse_experience_entry(experience_entry: WebElement, result: &mut Vec<Experience>) {
    //We either parse as a timeline or as a basic experience. Two different branches
    match experience_entry.find(By::XPath("./ul")).await {
        Ok(timeline) => {
            parse_timeline(timeline, result).await;
            return;
        }
        Err(_) => {
            trace!("No experience timeline found");
        }
    };

    let job_title = match experience_entry.find(By::XPath(".//h2")).await {
        Ok(job_title) => job_title,
        Err(_) => {
            warn!("Failed to find job title");
            return;
        }
    };

    let time = match experience_entry.find(By::XPath(".//p/span")).await {
        Ok(time) => time,
        Err(_) => {
            warn!("Failed to find experience duration");
            return;
        }
    };

    let time_text = match time.text().await {
        Ok(time_text) => time_text,
        Err(_) => {
            warn!("Failed to get experience duration text");
            return;
        }
    };
    let interval = match Interval::from_str(time_text.as_str(), "–") {
        Ok(interval) => interval,
        Err(_) => {
            warn!("Failed to parse experience duration");
            return;
        }
    };
    let title = match job_title.text().await {
        Ok(title) => title,
        Err(_) => {
            warn!("Failed to get job title text");
            return;
        }
    };
    result.push(Experience { position: title, interval });
}

pub async fn parse_sales_profile(driver: &DriverSession, sales_profile_url: &str) -> CrawlerResult<Profile> {
    info!("Going to sales profile {}", sales_profile_url);
    driver.driver.goto(sales_profile_url).await.unwrap();
    let name_span = match driver
        .find_until_loaded(By::XPath(".//h1[@data-anonymize='person-name']"), Duration::from_secs(5))
        .await
    {
        Ok(name_span) => name_span,
        Err(_) => return Err(ParseError(String::from_str("Failed to find name").unwrap())),
    };

    let profile_options = match driver
        .driver
        .find(By::XPath(
            "/html/body/main/div[1]/div[3]/div/div/div[1]/div/div/section[1]/section[1]/div[2]/section/div[2]/button",
        ))
        .await
    {
        Ok(profile_options) => profile_options,
        Err(_) => return Err(ParseError(String::from_str("Failed to find profile options").unwrap())),
    };

    if let Err(_) = profile_options.click().await {
        return Err(ParseError(String::from_str("Failed to click profile options").unwrap()));
    };

    let linkedin_url = match driver
        .find_until_loaded(By::XPath("/html/body/div[1]/div[2]/ul//a"), Duration::from_secs(5))
        .await
    {
        Ok(linkedin_url_element) => match linkedin_url_element.attr("href").await {
            Ok(linkedin_url) => match linkedin_url {
                Some(linkedin_url) => linkedin_url,
                None => {
                    return Err(ParseError(String::from_str("Failed to obtain linkedin href content").unwrap()));
                }
            },
            Err(_) => return Err(ParseError(String::from_str("Failed to get linkedin url href").unwrap())),
        },
        Err(_) => return Err(ParseError(String::from_str("Failed to find linkedin url element").unwrap())),
    };

    // To close the menu
    driver.driver.action_chain().send_keys(Key::Escape).perform().await.unwrap();

    let description_element = match driver.driver.find(By::XPath(".//span[@data-anonymize='headline']")).await {
        Ok(description_element) => description_element,
        Err(_) => return Err(ParseError(String::from_str("Failed to find name span").unwrap())),
    };

    let location = match driver
        .driver
        .find(By::XPath(
            "/html/body/main/div[1]/div[3]/div/div/div[1]/div/div/section[1]/section[1]/div[1]/div[4]/div[1]",
        ))
        .await
    {
        Ok(location) => location,
        Err(_) => return Err(ParseError(String::from_str("Failed to find location span").unwrap())),
    };

    let about = parse_about(driver).await;
    let experience = parse_experience(driver).await;
    let education = parse_education(driver).await;
    let skills = parse_skills(driver).await;
    let languages = parse_languages(driver).await;
    let profile_picture_url = parse_profile_picture(driver).await;

    Ok(Profile {
        sales_url: Some(sales_profile_url.to_string()),
        profile_picture_url,
        name: name_span.text().await.unwrap().to_string(),
        url: linkedin_url,
        description: description_element.text().await.unwrap(),
        about,
        location: location.text().await.unwrap(),
        experience,
        education,
        skills,
        languages,
    })
}

pub async fn parse_education(driver: &DriverSession) -> Option<Vec<Education>> {
    let education_ul = match driver
        .find_until_loaded(By::XPath("//h2[normalize-space()='Education']/../ul"), Duration::from_secs(5))
        .await
    {
        Ok(education_list) => education_list,
        Err(_) => {
            return {
                info!("No education section found.");
                None
            }
        }
    };

    let education_list = match education_ul.find_all(By::XPath("./li")).await {
        Ok(education_list) => education_list,
        Err(_) => {
            return {
                info!("No education lists found");
                None
            }
        }
    };

    let mut result = Vec::with_capacity(education_list.len());
    for education_entry in education_list {
        parse_education_entry(education_entry, &mut result).await;
    }
    Some(result)
}

pub async fn parse_education_entry(education_entry: WebElement, education_array: &mut Vec<Education>) {
    let education_main_div = match education_entry.find(By::XPath("./div")).await {
        Ok(education_main_div) => education_main_div,
        Err(_) => {
            warn!("Failed to find education main div");
            return;
        }
    };

    let title = match education_main_div.find(By::XPath("./h3")).await {
        Ok(education_title) => match education_title.text().await {
            Ok(education_title) => education_title,
            Err(_) => {
                warn!("Failed to get education title text");
                return;
            }
        },
        Err(_) => {
            warn!("Failed to find education title");
            return;
        }
    };

    let result_degree = education_main_div.find(By::XPath("./p[1]/span[1]")).await;
    let result_field = education_main_div.find(By::XPath("./p[1]/span[2]")).await;
    let result_timeline = education_main_div.find(By::XPath("./p[2]/span[2]")).await;

    let interval = match result_timeline {
        Ok(timeline) => Interval::from_str(timeline.text().await.unwrap().as_str(), "–").unwrap(),
        Err(_) => {
            info!("No education timeline found.");
            Interval {
                start: "".to_string(),
                end: "".to_string(),
            }
        }
    };

    let degree = match result_degree {
        Ok(degree) => degree.text().await.unwrap(),
        Err(_) => {
            info!("No degree found.");
            "".to_string()
        }
    };

    let field = match result_field {
        Ok(field) => field.text().await.unwrap(),
        Err(_) => {
            info!("No field found.");
            "".to_string()
        }
    };
    education_array.push(Education {
        title,
        degree,
        field,
        interval,
    });
}

pub async fn parse_skills(driver: &DriverSession) -> Option<Vec<Skill>> {
    let skills_title = match driver
        .driver
        .find(By::XPath("//h2[contains(., 'Featured skills and endorsements')]"))
        .await
    {
        Ok(skills_title) => skills_title,
        Err(_) => {
            info!("No skills section found.");
            return None;
        }
    };

    let skills_section = match skills_title.parent().await {
        Ok(skills_main_div) => match skills_main_div.parent().await {
            Ok(skills_section) => skills_section,
            Err(_) => {
                error!("No skills parent section found.");
                return None;
            }
        },
        Err(_) => {
            error!("No skills parent div found.");
            return None;
        }
    };

    match skills_section.find(By::XPath("./button")).await {
        Ok(button) => {
            if let Err(result) = button.scroll_into_view().await {
                error!("Failed to scroll to show more skills button: {}", result);
            } else {
                tokio::time::sleep(Duration::from_millis(500)).await;
                if let Err(result) = button.click().await {
                    error!("Failed to click show more skills button: {}", result);
                }
                if let Err(result) = button.scroll_into_view().await {
                    error!("Failed to scroll to show more skills button after clicking: {}", result);
                }
            }
        }
        Err(_) => {
            info!("No show more button for skills found.");
        }
    }

    let skill_list = if let Ok(skill_list) = skills_section.find_all(By::XPath("./div/ul/li")).await {
        skill_list
    } else {
        error!("No skill list found.");
        return None;
    };
    let mut result = Vec::new();
    for skill_entry in skill_list {
        parse_skill_entry(skill_entry, &mut result).await;
    }

    Some(result)
}

pub async fn parse_skill_entry(entry: WebElement, skill_entry: &mut Vec<Skill>) {
    let skill_name = match entry.find(By::XPath("./p")).await {
        Ok(skill_elem) => match skill_elem.text().await {
            Ok(text) => text,
            Err(_) => {
                error!("Failed to get skill name text");
                return;
            }
        },
        Err(_) => {
            error!("Failed to find skill name element");
            return;
        }
    };

    let endorsements = if let Ok(endorsements_elem) = entry.find(By::XPath("./div/span")).await {
        let text = endorsements_elem.text().await.unwrap();
        let re = Regex::new(r"\d+").unwrap();
        let re_found_text = re.find(&text).unwrap();
        let cleaned_text = re_found_text.as_str();
        let parsed_endorsements = if let Ok(parsed) = cleaned_text.parse::<u16>() {
            parsed
        } else {
            error!("Failed to parse endorsements to int from str: {}", text);
            0
        };
        parsed_endorsements
    } else {
        trace!("No endorsements found.");
        0
    };

    skill_entry.push(Skill {
        name: skill_name,
        endorsements,
    });
}

pub async fn parse_languages(driver: &DriverSession) -> Option<Vec<Language>> {
    let languages_title = match driver.driver.find(By::XPath("//h2[contains(., 'Languages')]")).await {
        Ok(languages_title) => languages_title,
        Err(_) => {
            info!("No languages section found.");
            return None;
        }
    };

    let language_list = match languages_title.find_all(By::XPath("../ul/li")).await {
        Ok(language_list) => language_list,
        Err(_) => {
            error!("No language list found.");
            return None;
        }
    };
    let mut language_array = Vec::new();
    for language_entry in language_list {
        parse_language_entry(language_entry, &mut language_array).await;
    }
    Some(language_array)
}

pub async fn parse_language_entry(language_entry: WebElement, language_array: &mut Vec<Language>) {
    let language = match language_entry.find(By::XPath("./p[1]")).await {
        Ok(language_name) => match language_name.text().await {
            Ok(text) => text,
            Err(_) => {
                error!("Failed to get language name text");
                return;
            }
        },
        Err(_) => {
            error!("Failed to find language name element");
            return;
        }
    };
    let fluency = match language_entry.find(By::XPath("./p[2]")).await {
        Ok(language_fluency) => match language_fluency.text().await {
            Ok(text) => text,
            Err(_) => {
                error!("Failed to get language fluency text");
                return;
            }
        },
        Err(_) => {
            trace!("Failed to find language fluency element");
            "".to_string()
        }
    };
    language_array.push(Language { language, fluency });
}

pub async fn parse_profile_picture(driver: &DriverSession) -> String {
    match driver
        .find_until_loaded(By::XPath("//div/img[@data-anonymize='headshot-photo']"), Duration::from_secs(5))
        .await
    {
        Ok(profile_img) => profile_img.attr("src").await.unwrap().unwrap(),
        Err(e) => {
            info!("Failed to find picture {}", e);
            "".to_string()
        }
    }
}
