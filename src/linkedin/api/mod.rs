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
use cookie::Cookie;
use http::{HeaderMap, HeaderValue};
use regex::Regex;
use reqwest::cookie::CookieStore as CookieStoreTrait;
use reqwest::{Client, Url};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde::de::Unexpected::Str;
use std::error::Error;
use std::fmt::format;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use urlencoding::encode;

pub struct LinkedinSession {
    session_id: String,
    client: Client,
    cookie_store: Arc<CookieStoreMutex>,
    is_auth: bool,
}

impl LinkedinSession {
    const LINKEDIN_URL: &'static str = "https://www.linkedin.com";
    const COOKIE_DOMAIN: &'static str = "www.linkedin.com";
    const API_URL: &'static str = "https://www.linkedin.com/voyager/api";

    pub fn is_auth(&self) -> bool {
        self.is_auth
    }
    //TODO This should be a static place in the memory. Shouldn't be created every time

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
    async fn obtain_session_id(&mut self) -> CrawlerResult<()> {
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
        self.session_id = cookie_raw.replace("\"", "").to_string();
        Ok(())
    }
    pub async fn authenticate(&mut self, username: &str, password: &str) -> CrawlerResult<()> {
        info!("Authenticating and generating cookies...");
        self.obtain_session_id().await?;
        info!("Obtained session id");
        tokio::time::sleep(Duration::from_secs(1)).await;
        let headers = Self::create_default_headers(Some(&self.session_id));
        let formatted_session_id = format!("\"{}\"", self.session_id);
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
        let url = Url::parse(Self::LINKEDIN_URL).unwrap();
        let cookies = self.cookie_store.cookies(&url).unwrap();
        let bytes = cookies.as_bytes();
        save_cookies(bytes);
        info!("Wrote cookies");
        tokio::time::sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    pub async fn profile(&self, profile_id: &str) -> CrawlerResult<Profile> {
        info!("Getting profile {}", profile_id);
        let endpoint = format!("{}/identity/profiles/{}/profileView", Self::API_URL, profile_id);
        let headers = Self::create_default_headers(Some(&self.session_id));
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
            let headers = Self::create_default_headers(Some(&self.session_id));

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
        let endpoint = format!("{}/identity/profiles/{}/skills?count=100&start=0", Self::API_URL, profile_id);
        let headers = Self::create_default_headers(Some(&self.session_id));
        let skills = match self.client.get(endpoint).headers(headers).send().await {
            Ok(response) => match response.json::<SkillView>().await {
                Ok(skills) => skills,
                Err(e) => return Err(SessionError(format!("Failed to parse skills {:?}", e))),
            },
            Err(e) => return Err(SessionError(format!("Failed to get skills {:?}", e))),
        };
        Ok(skills)
    }

    /// TODO Optimize string usage here. Maybe just a slice is needed for session_id instead of a copy
    pub fn new() -> LinkedinSession {
        let mut cookie_store = CookieStore::new(None);
        let mut session_id = "".to_string();
        let mut is_auth = false;
        let linkedin_url = Url::parse(Self::LINKEDIN_URL).unwrap();

        if let Some(cookies) = load_cookies() {
            let cookie_list = cookies.split(";").collect::<Vec<&str>>();
            for cookie in cookie_list {
                if let Err(code) = cookie_store.parse(cookie, &linkedin_url) {
                    error!("Failed to parse cookie {}", code);
                }
            }
        };
        if let Some(cookie_session_id) = cookie_store.get(Self::COOKIE_DOMAIN, "/", "JSESSIONID") {
            let session_id_raw = cookie_session_id.value().to_string();
            session_id = session_id_raw.replace("\"", "");
            is_auth = true;
        }
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);
        let client = fatal_unwrap_e!(
            Client::builder().cookie_store(true).cookie_provider(cookie_store.clone()).build(),
            "Failed to create client {}"
        );
        Self {
            session_id,
            client,
            cookie_store,
            is_auth,
        }
    }
}
