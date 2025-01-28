pub mod crawler;
pub mod json;
pub mod rate_limits;
mod utils;

use crate::errors::CrawlerError::{LinkedinError, SessionError};
use crate::errors::CrawlerResult;
use crate::linkedin::api::json::{AuthenticateResponse, FetchCookiesResponse, Profile, SearchParams, SearchResult, Skill, SkillView};
use crate::linkedin::api::utils::{cookies_session_id, load_cookies, save_cookies};
use actix_web::cookie::CookieJar;
use actix_web::web::Json;
use chrono::format;
use cookie::Cookie;
use http::{HeaderMap, HeaderValue};
use regex::Regex;
use reqwest::cookie::CookieStore as CookieStoreTrait;
use reqwest::{Client, Proxy, Url};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde::de::Unexpected::Str;
use std::error::Error;
use std::fmt::format;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use urlencoding::encode;

pub struct LinkedinClient {
    session_id: Option<String>,
    client: Client,
    cookie_store: Arc<CookieStoreMutex>,
}

impl LinkedinClient {
    const LINKEDIN_URL: &'static str = "https://www.linkedin.com";
    const COOKIE_DOMAIN: &'static str = "www.linkedin.com";
    const API_URL: &'static str = "https://www.linkedin.com/voyager/api";
    const COOKIE_FOLDER: &'static str = "cookies/";

    fn create_default_headers(csrf_token: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Li-User-Agent",
            HeaderValue::from_static("LIAuthLibrary:0.0.3 com.linkedin.android:4.1.881 Asus_ASUS_Z01QD:android_9"),
        );
        headers.insert("User-Agent", HeaderValue::from_static("ANDROID OS"));
        headers.insert("X-User-Language", HeaderValue::from_static("en"));
        headers.insert("X-User-Locale", HeaderValue::from_static("en_US"));
        headers.insert("Accept-Language", HeaderValue::from_static("en-us"));
        if let Some(token) = csrf_token {
            debug!("Using csrf token {}", token);
            headers.insert("csrf-token", HeaderValue::from_str(token).unwrap());
        }
        headers
    }
    pub async fn obtain_session_id(&self) -> CrawlerResult<(String)> {
        let auth_url = format!("{}{}", Self::LINKEDIN_URL, "/uas/authenticate");
        let default_headers = Self::create_default_headers(None);

        let response = fatal_unwrap_e!(
            self.client.get(auth_url).headers(default_headers).send().await,
            "Failed to obtain linkedin set-cookies {}"
        );
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap();
            return Err(SessionError(format!("Failed to obtain linkedin set-cookies {} {}", status, text)));
        }

        let auth_response = match response.json::<FetchCookiesResponse>().await {
            Ok(response) => response,
            Err(e) => {
                return Err(SessionError(format!("Failed to parse auth response {}", e)));
            }
        };
        if auth_response.status != "success" {
            return Err(SessionError(format!("Failed to obtain session_id {}", auth_response.status)));
        }

        let cookies = self.cookie_store.lock().unwrap();
        let session_id_cookie = cookies.get(Self::COOKIE_DOMAIN, "/", "JSESSIONID").unwrap();

        let cookie_raw = session_id_cookie.value();
        debug!("{}", cookie_raw);
        let session_id = cookie_raw.replace("\"", "").to_string();
        Ok(session_id)
    }
    pub async fn authenticate(&mut self, username: &str, password: &str, ignore_cookies: bool) -> CrawlerResult<()> {
        let cookie_path = format!("{}{}", Self::COOKIE_FOLDER, username);
        if !ignore_cookies {
            match load_cookies(cookie_path.as_str()) {
                Some(cookies) => {
                    let mut cookies_guard = self.cookie_store.lock().unwrap();
                    let cookies_store = cookies_guard.deref_mut();
                    match Self::parse_cookies(cookies_store, cookies) {
                        Ok(session_id) => {
                            info!("Found cookies for {}, using them", username);
                            self.session_id = Some(session_id);
                            return Ok(());
                        }
                        Err(e) => {
                            error!("{}", e);
                        }
                    }
                }
                _ => {}
            };
        }

        info!("Authenticating and generating cookies...");
        let session_id = self.obtain_session_id().await?;
        info!("Obtained session id");
        tokio::time::sleep(Duration::from_secs(1)).await;
        let headers = Self::create_default_headers(Some(session_id.as_str()));
        let formatted_session_id = format!("\"{}\"", session_id);
        let form = vec![
            ("session_key", username),
            ("session_password", password),
            ("JSESSIONID", formatted_session_id.as_ref()),
        ];
        let response = match self
            .client
            .post(format!("{}{}", Self::LINKEDIN_URL, "/uas/authenticate"))
            .headers(headers)
            .form(&form)
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                return Err(SessionError(format!("Failed authenticate request {}", e)));
            }
        };

        if !response.status().is_success() {
            return Err(SessionError(format!(
                "Failed to authenticate {} {}",
                response.status(),
                response.text().await.unwrap()
            )));
        }

        let response_data = match response.json::<AuthenticateResponse>().await {
            Ok(response_data) => response_data,
            Err(e) => {
                return Err(SessionError(format!("Failed to parse authenticate response {}", e)));
            }
        };

        if response_data.login_result != "PASS" {
            return Err(SessionError(format!(
                "Failed to authenticate {} {}",
                response_data.login_result, response_data.challenge_url
            )));
        }
        info!("Authenticated successfully");
        self.session_id = Some(session_id);
        let url = Url::parse(Self::LINKEDIN_URL).unwrap();
        let cookies = self.cookie_store.cookies(&url).unwrap();
        let bytes = cookies.as_bytes();
        save_cookies(bytes, cookie_path.as_str())?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    pub async fn profile(&self, profile_id: &str) -> CrawlerResult<Profile> {
        info!("Getting profile {}", profile_id);
        let session_id = match self.session_id.as_ref() {
            Some(id) => id,
            None => return Err(SessionError("Session is not initialized".to_string())),
        };

        let endpoint = format!("{}/identity/profiles/{}/profileView", Self::API_URL, profile_id);
        let headers = Self::create_default_headers(Some(session_id.as_str()));
        let profile = match self.client.get(endpoint).headers(headers).send().await {
            Ok(response) => {
                if !response.status().is_success() {
                    return Err(LinkedinError(format!("Failed to get profile {}", response.text().await.unwrap())));
                }
                match response.json::<Profile>().await {
                    Ok(profile) => profile,
                    Err(e) => return Err(SessionError(format!("Failed to parse profile {:?}", e))),
                }
            }
            Err(e) => return Err(SessionError(format!("Failed to get profile {}", e))),
        };

        Ok(profile)
    }

    pub async fn search_people(&self, mut params: SearchParams) -> CrawlerResult<SearchResult> {
        let session_id = match self.session_id.as_ref() {
            Some(session_id) => session_id,
            None => return Err(SessionError("No session id found".to_string())),
        };

        if params.page > params.end {
            return Err(SessionError("Start page cannot be greater than end page".to_string()));
        }

        let mut filters = Vec::<String>::new();
        info!("Performing search with params {}", params);
        const ITEM_PER_PAGE: u16 = 10;
        filters.push(String::from("(key:resultType,value:List(PEOPLE))".to_string()));
        if let Some(keyword_first_name) = params.keyword_first_name.as_ref() {
            filters.push(format!("(key:firstName,value:List({}))", keyword_first_name))
        }
        if let Some(keyword_last_name) = params.keyword_last_name.as_ref() {
            filters.push(format!("(key:lastName,value:List({}))", keyword_last_name))
        }
        if let Some(keyword_title) = params.keyword_title.as_ref() {
            filters.push(format!("(key:title,value:List({}))", keyword_title))
        }
        if let Some(keyword_company) = params.keyword_company.as_ref() {
            filters.push(format!("(key:company,value:List({}))", keyword_company))
        }
        if let Some(keyword_school) = params.keyword_school.as_ref() {
            filters.push(format!("(key:school,value:List({}))", keyword_school))
        }
        if let Some(countries) = params.countries.as_ref() {
            filters.push(format!(
                "(key:geoUrn,value:List({}))",
                countries.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(",")
            ))
        }
        if let Some(network_depth) = params.network_depth.as_ref() {
            filters.push(format!(
                "(key:network,value:List({}))",
                network_depth.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(",")
            ))
        }
        if let Some(profile_language) = params.profile_language.as_ref() {
            filters.push(format!("(key:profileLanguage,value:List({}))", profile_language.join(",")))
        }
        let filter_params = format!("List({})", filters.join(","));
        let keywords = match params.keywords.as_ref() {
            Some(keywords) => keywords,
            None => "",
        };
        let mut current_offset = if params.page == 0 { 0 } else { params.page * ITEM_PER_PAGE };
        let mut total_offset = params.end * ITEM_PER_PAGE;
        let mut search_response = SearchResult {
            elements: Vec::with_capacity((total_offset - current_offset) as usize),
            request_metadata: params.request_metadata.take(),
            total: 0,
            total_lookup: 0,
        };
        info!("searching {} from total {}", current_offset, total_offset);
        let encoded_keywords = encode(&keywords);
        while current_offset < total_offset {
            let endpoint = format!(
                "{}/graphql?variables=(start:{},origin:GLOBAL_SEARCH_HEADER,query:(keywords:{},flagshipSearchIntent:SEARCH_SRP,queryParameters:{},includeFiltersInResponse:false))&queryId=voyagerSearchDashClusters.b0928897b71bd00a5a7291755dcd64f0",
                Self::API_URL,
                current_offset,
                encoded_keywords,
                filter_params
            );
            let headers = Self::create_default_headers(Some(session_id.as_str()));

            let response = match self.client.get(endpoint).headers(headers).send().await {
                Ok(response) => match response.json::<SearchResult>().await {
                    Ok(result) => result,
                    Err(e) => return Err(SessionError(format!("search people parse failed: {:?}", e))),
                },
                Err(e) => return Err(SessionError(format!("search people failed {:?}", e))),
            };
            for item in response.elements.iter() {
                search_response.elements.push(item.clone());
            }
            if total_offset > response.total_lookup {
                total_offset = response.total_lookup;
            }

            search_response.total = response.total;
            search_response.total_lookup = total_offset;
            /// offset: 110, total: 119
            info!("offset: {}, total: {}", current_offset, total_offset);
            current_offset += ITEM_PER_PAGE;
        }
        Ok(search_response)
    }

    pub async fn skills(&self, profile_id: &str) -> CrawlerResult<SkillView> {
        let session_id = match self.session_id.as_ref() {
            Some(session_id) => session_id,
            None => return Err(SessionError("No session id found".to_string())),
        };

        let endpoint = format!("{}/identity/profiles/{}/skills?count=100&start=0", Self::API_URL, profile_id);
        let headers = Self::create_default_headers(Some(session_id.as_ref()));
        let skills = match self.client.get(endpoint).headers(headers).send().await {
            Ok(response) => match response.json::<SkillView>().await {
                Ok(skills) => skills,
                Err(e) => return Err(SessionError(format!("Failed to parse skills {:?}", e))),
            },
            Err(e) => return Err(SessionError(format!("Failed to get skills {:?}", e))),
        };
        Ok(skills)
    }

    fn parse_cookies(cookie_store: &mut CookieStore, cookies: String) -> CrawlerResult<String> {
        let linkedin_url = Url::parse(Self::LINKEDIN_URL).unwrap();
        {
            let cookie_list = cookies.split(";").collect::<Vec<&str>>();
            for cookie in cookie_list {
                if let Err(code) = cookie_store.parse(cookie, &linkedin_url) {
                    error!("Failed to parse cookie {}", code);
                }
            }

            match cookie_store.get(Self::COOKIE_DOMAIN, "/", "JSESSIONID") {
                Some(cookies) => Ok(cookies.value().to_string().replace("\"", "")),
                None => Err(SessionError("Failed to find JSESSIONID".to_string())),
            }
        }
    }
    pub fn new_proxy(endpoint: &str, username: &str, password: &str) -> LinkedinClient {
        let cookie_store = CookieStore::new(None);

        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);

        let https_proxy = Proxy::https(endpoint).unwrap().basic_auth(username, password);
        let http_proxy = Proxy::http(endpoint).unwrap().basic_auth(username, password);

        let client = fatal_unwrap_e!(
            Client::builder()
                .cookie_store(true)
                .cookie_provider(cookie_store.clone())
                .proxy(https_proxy)
                .proxy(http_proxy)
                .build(),
            "Failed to create client {}"
        );
        Self {
            session_id: None,
            client,
            cookie_store,
        }
    }

    pub fn new() -> LinkedinClient {
        let cookie_store = CookieStore::new(None);
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);

        let client = fatal_unwrap_e!(
            Client::builder().cookie_store(true).cookie_provider(cookie_store.clone()).build(),
            "Failed to create client {}"
        );
        Self {
            session_id: None,
            client,
            cookie_store,
        }
    }
}
