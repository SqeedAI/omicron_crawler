use crate::driver_ext::WebDriverExt;
use crate::errors::CrawlerError::{InteractionError, ParseError};
use crate::errors::{CrawlerError, CrawlerResult};
use crate::linkedin::enums::Functions;
use crate::linkedin::profiles;
use crate::linkedin::profiles::{Education, Experience, Interval, Language, Profile, SearchResult, Skill};
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

pub async fn set_function_search(driver: &WebDriverExt, function: Functions) -> CrawlerResult<()> {
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

pub async fn set_job_title_search(driver: &WebDriverExt, job_title: String) -> CrawlerResult<()> {
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

pub async fn set_geography_search(driver: &WebDriverExt, geography: String) -> CrawlerResult<()> {
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

pub async fn parse_search(driver: &WebDriverExt) -> CrawlerResult<Vec<SearchResult>> {
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
        Ok(title_span) => title_span,
        Err(_) => {
            error!("Failed to find title span");
            return Err(ParseError(String::from_str("Failed to find title span").unwrap()));
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

    results.push(SearchResult {
        name: name_span.text().await.unwrap(),
        title: title_span.text().await.unwrap(),
        sales_url: url,
    });
    Ok(())
}

pub async fn parse_about(driver: &WebDriverExt) -> Option<String> {
    let possible_about_title = driver.driver.find(By::XPath("//h1[normalize-space()='About']")).await;
    if let Err((err)) = possible_about_title {
        info!("No about section found.");
        return None;
    }
    let about_title = possible_about_title.unwrap();
    about_title.scroll_into_view().await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;

    //Check if show more is needed to click

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
pub async fn parse_experience(driver: &WebDriverExt) -> Option<Vec<Experience>> {
    let experience_section = match driver
        .driver
        .find(By::XPath("//section[@data-sn-view-name='feature-lead-experience']"))
        .await
    {
        Ok(experience_section) => {
            tokio::time::sleep(Duration::from_millis(100)).await;
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
            tokio::time::sleep(Duration::from_millis(100)).await;
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

    let time_text = time.text().await.unwrap();
    let interval = Interval::from_str(time_text.as_str(), "–").unwrap();
    let title = job_title.text().await.unwrap();
    result.push(Experience { position: title, interval });
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

    let about = parse_about(driver).await;
    let experience = parse_experience(driver).await;
    let education = parse_education(driver).await;
    let skills = parse_skills(driver).await;
    let languages = parse_languages(driver).await;
    let profile_picture_url = parse_profile_picture(driver).await;

    Profile {
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
    }
}

pub async fn parse_education(driver: &WebDriverExt) -> Option<Vec<Education>> {
    let possible_education_title = driver
        .driver
        .find(By::XPath(
            "/html/body/main/div[1]/div[3]/div/div/div[1]/div/div/div/section[2]/div/h2",
        ))
        .await;
    if let Err((err)) = possible_education_title {
        info!("No education section found.");
        return None;
    }
    let education_title = possible_education_title.unwrap();
    let parent = fatal_unwrap_e!(education_title.parent().await, "Failed to find education parent {}");
    let education_ul = fatal_unwrap_e!(parent.find(By::XPath("./ul")).await, "Failed to find education ul {}");
    let education_list = fatal_unwrap_e!(education_ul.find_all(By::XPath("./li")).await, "Failed to find education list {}");
    let mut result = Vec::with_capacity(education_list.len());
    for education_entry in education_list {
        parse_education_entry(education_entry, &mut result).await;
    }
    Some(result)
}

pub async fn parse_education_entry(education_entry: WebElement, education_array: &mut Vec<Education>) {
    let education_main_div = fatal_unwrap_e!(
        education_entry.find(By::XPath("./div")).await,
        "Failed to find education main div {}"
    );
    let education_title = fatal_unwrap_e!(
        education_main_div.find(By::XPath("./h3")).await,
        "Failed to find education title {}"
    );
    let title = education_title.text().await.unwrap();

    let result_degree = education_main_div.find(By::XPath("./p[1]/span[1]")).await;
    let result_field = education_main_div.find(By::XPath("./p[1]/span[2]")).await;
    let result_timeline = education_main_div.find(By::XPath("./p[2]/span[2]")).await;

    let interval = if let Ok(timeline) = result_timeline {
        Interval::from_str(timeline.text().await.unwrap().as_str(), "–").unwrap()
    } else {
        info!("No education timeline found.");
        Interval {
            start: "".to_string(),
            end: "".to_string(),
        }
    };

    let degree = if let Ok(degree) = result_degree {
        degree.text().await.unwrap()
    } else {
        info!("No degree found.");
        "".to_string()
    };

    let field = if let Ok(field) = result_field {
        field.text().await.unwrap()
    } else {
        info!("No field found.");
        "".to_string()
    };
    education_array.push(Education {
        title,
        degree,
        field,
        interval,
    });
}

pub async fn parse_skills(driver: &WebDriverExt) -> Option<Vec<Skill>> {
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
            tokio::time::sleep(Duration::from_millis(100)).await;
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

pub async fn parse_languages(driver: &WebDriverExt) -> Option<Vec<Language>> {
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
            error!("Failed to find language fluency element");
            return;
        }
    };
    language_array.push(Language { language, fluency });
}

pub async fn parse_profile_picture(driver: &WebDriverExt) -> String {
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
