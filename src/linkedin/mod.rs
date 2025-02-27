pub mod crawler;
pub mod json;
pub mod rate_limits;
mod tracking_client;
mod utils;

use crate::cookies::cookie_save;
use crate::errors::ClientError::{CookieError, HeaderError, IoError, RequestError, ResponseError};
use crate::errors::IoError::ParseError;
use crate::errors::{ClientError, ClientResult};
use crate::linkedin::json::res::SignupChallenge;
use crate::linkedin::json::{req, res, AuthenticateResponse, FetchCookiesResponse, Profile, SearchParams, SearchResult, Skill, SkillView};
use crate::linkedin::tracking_client::{
    default_li_user_agent, default_requested_with, default_user_agent, default_webview_user_agent, new_native_tracking_headers,
    new_webview_tracking_headers, DeviceInfo,
};
use crate::linkedin::utils::{cookies_session_id, generate_jsessionid};
use actix_web::cookie::CookieJar;
use actix_web::web::Json;
use chrono::format;
use cookie::Cookie;
use http::{HeaderMap, HeaderValue, StatusCode};
use regex::Regex;
use reqwest::cookie::CookieStore as CookieStoreTrait;
use reqwest::{Proxy, Url};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde::de::Unexpected::Str;
use std::error::Error;
use std::fmt::format;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use urlencoding::encode;

pub struct Client {
    client: reqwest::Client,
    pub native_headers: HeaderMap,
    pub webview_headers: HeaderMap,
    pub native_device_info: DeviceInfo,
    cookie_store: Arc<CookieStoreMutex>,
}

impl Client {
    const API_DOMAIN: &'static str = "https://www.linkedin.com";
    const COOKIE_DOMAIN: &'static str = "www.linkedin.com";
    const VOYAGER_URL: &'static str = "https://www.linkedin.com/voyager/api";
    const COOKIE_FOLDER: &'static str = "cookies/";

    pub fn new_proxy(endpoint: &str, username: &str, password: &str, cookie_store: Arc<CookieStoreMutex>) -> Client {
        let https_proxy = Proxy::https(endpoint).unwrap().basic_auth(username, password);
        let http_proxy = Proxy::http(endpoint).unwrap().basic_auth(username, password);

        let client = fatal_unwrap_e!(
            reqwest::Client::builder()
                .cookie_store(true)
                .cookie_provider(cookie_store.clone())
                .proxy(https_proxy)
                .proxy(http_proxy)
                .build(),
            "Failed to create client {}"
        );
        Self::new_with_client(client, cookie_store)
    }

    pub fn new_from_existing_proxy(
        endpoint: &str,
        username: &str,
        password: &str,
        cookie_store: Arc<CookieStoreMutex>,
        native_device_info: DeviceInfo,
        user_agent: &str,
        li_user_agent: &str,
        webview_user_agent: &str,
        requested_with: &str,
    ) -> ClientResult<Client> {
        let https_proxy = Proxy::https(endpoint).unwrap().basic_auth(username, password);
        let http_proxy = Proxy::http(endpoint).unwrap().basic_auth(username, password);

        let client = fatal_unwrap_e!(
            reqwest::Client::builder()
                .cookie_store(true)
                .cookie_provider(cookie_store.clone())
                .proxy(https_proxy)
                .proxy(http_proxy)
                .build(),
            "Failed to create client {}"
        );
        Self::new_from_existing_with_client(
            client,
            cookie_store,
            native_device_info,
            user_agent,
            li_user_agent,
            webview_user_agent,
            requested_with,
        )
    }

    pub fn new_from_existing(
        cookie_store: Arc<CookieStoreMutex>,
        native_device_info: DeviceInfo,
        user_agent: &str,
        li_user_agent: &str,
        webview_user_agent: &str,
        requested_with: &str,
    ) -> ClientResult<Client> {
        let client = fatal_unwrap_e!(
            reqwest::Client::builder()
                .cookie_store(true)
                .cookie_provider(cookie_store.clone())
                .build(),
            "Failed to create client {}"
        );
        Self::new_from_existing_with_client(
            client,
            cookie_store,
            native_device_info,
            user_agent,
            li_user_agent,
            webview_user_agent,
            requested_with,
        )
    }

    fn new_from_existing_with_client(
        client: reqwest::Client,
        cookie_store: Arc<CookieStoreMutex>,
        native_device_info: DeviceInfo,
        user_agent: &str,
        li_user_agent: &str,
        webview_user_agent: &str,
        requested_with: &str,
    ) -> ClientResult<Client> {
        let cookie_lock = cookie_store.lock().unwrap();
        let session_id = match cookie_lock.get(Self::COOKIE_DOMAIN, "/", "JSESSIONID") {
            Some(cookie) => cookie.value().to_string(),
            None => return Err(CookieError("No JSESSIONID cookie found".to_string())),
        };
        drop(cookie_lock);

        let native_headers = new_native_tracking_headers(&session_id, &native_device_info, user_agent, li_user_agent);
        let webview_headers = new_webview_tracking_headers(webview_user_agent, requested_with);
        Ok(Self {
            client,
            native_headers,
            webview_headers,
            native_device_info,
            cookie_store,
        })
    }

    pub fn new(cookie_store: Arc<CookieStoreMutex>) -> Client {
        let client = fatal_unwrap_e!(
            reqwest::Client::builder()
                .cookie_store(true)
                .cookie_provider(cookie_store.clone())
                .build(),
            "Failed to create client {}"
        );
        Self::new_with_client(client, cookie_store)
    }

    fn new_with_client(client: reqwest::Client, cookie_store: Arc<CookieStoreMutex>) -> Client {
        let native_device_info = DeviceInfo::default();
        let session_id = generate_jsessionid();
        let native_headers = new_native_tracking_headers(&session_id, &native_device_info, default_user_agent(), default_li_user_agent());
        let webview_headers = new_webview_tracking_headers(default_webview_user_agent(), default_requested_with());

        Self {
            client,
            cookie_store,
            native_headers,
            webview_headers,
            native_device_info,
        }
    }

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

    fn get_session_id(&self) -> ClientResult<String> {
        let session_id = self
            .native_headers
            .get("JSESSIONID")
            .ok_or(HeaderError("JSESSIONID not found".to_string()))?;
        Ok(session_id.to_str().unwrap().to_string())
    }
    pub async fn authenticate(&mut self, username: &str, password: &str) -> ClientResult<()> {
        info!("Authenticating and generating cookies...");
        self.tracking().await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        let session_id = self.get_session_id()?;
        let formatted_session_id = format!("\"{}\"", session_id);
        let form = vec![
            ("session_key", username),
            ("session_password", password),
            ("JSESSIONID", formatted_session_id.as_ref()),
        ];
        let response = match self
            .client
            .post(format!("{}{}", Self::API_DOMAIN, "/uas/authenticate"))
            .headers(self.native_headers.clone())
            .form(&form)
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                return Err(ResponseError(format!("Failed authenticate request {}", e)));
            }
        };

        if !response.status().is_success() {
            return Err(ResponseError(format!(
                "Failed to authenticate {} {}",
                response.status(),
                response.text().await.unwrap()
            )));
        }

        let response_data = match response.json::<AuthenticateResponse>().await {
            Ok(response_data) => response_data,
            Err(e) => {
                return Err(ClientError::SerializationError(format!(
                    "Failed to parse authenticate response {}",
                    e
                )));
            }
        };

        if response_data.login_result != "PASS" {
            return Err(ResponseError(format!(
                "Failed to authenticate {} {}",
                response_data.login_result, response_data.challenge_url
            )));
        }
        info!("Authenticated successfully");
        let url = Url::parse(Self::API_DOMAIN).unwrap();
        cookie_save(&self.cookie_store, &url, username).map_err(|e| IoError(e))?;
        Ok(())
    }

    pub async fn profile(&self, profile_id: &str) -> ClientResult<Profile> {
        info!("Getting profile {}", profile_id);

        let endpoint = format!("{}/identity/profiles/{}/profileView", Self::VOYAGER_URL, profile_id);
        let profile = match self.client.get(endpoint).headers(self.native_headers.clone()).send().await {
            Ok(response) => {
                if !response.status().is_success() {
                    return Err(ResponseError(format!("Failed to get profile {}", response.text().await.unwrap())));
                }
                match response.json::<Profile>().await {
                    Ok(profile) => profile,
                    Err(e) => return Err(ClientError::SerializationError(format!("Failed to parse profile {:?}", e))),
                }
            }
            Err(e) => return Err(ClientError::SerializationError(format!("Failed to get profile {}", e))),
        };

        Ok(profile)
    }

    pub async fn search_people(&self, mut params: SearchParams) -> ClientResult<SearchResult> {
        let session_id = self.get_session_id()?;

        if params.page > params.end {
            return Err(RequestError("Start page cannot be greater than end page".to_string()));
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
                Self::VOYAGER_URL,
                current_offset,
                encoded_keywords,
                filter_params
            );

            let response = match self.client.get(endpoint).headers(self.native_headers.clone()).send().await {
                Ok(response) => match response.json::<SearchResult>().await {
                    Ok(result) => result,
                    Err(e) => return Err(ResponseError(format!("search people parse failed: {:?}", e))),
                },
                Err(e) => return Err(ResponseError(format!("search people failed {:?}", e))),
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

    pub async fn skills(&self, profile_id: &str) -> ClientResult<SkillView> {
        let endpoint = format!("{}/identity/profiles/{}/skills?count=100&start=0", Self::VOYAGER_URL, profile_id);
        let skills = match self.client.get(endpoint).headers(self.native_headers.clone()).send().await {
            Ok(response) => match response.json::<SkillView>().await {
                Ok(skills) => skills,
                Err(e) => return Err(ClientError::SerializationError(format!("Failed to parse skills {:?}", e))),
            },
            Err(e) => return Err(ResponseError(format!("Failed to get skills {:?}", e))),
        };
        Ok(skills)
    }

    pub async fn tracking(&self) -> ClientResult<String> {
        let url = fatal_unwrap_e!(
            Url::parse(format!("{}/mob/tracking", Self::API_DOMAIN).as_str()),
            "Failed to parse tracking url {}"
        );
        let tracking_data = serde_json::to_string(&self.native_device_info).unwrap();
        let response = self
            .client
            .get(url)
            .header("User-Agent", tracking_data)
            .send()
            .await
            .map_err(|e| ResponseError(format!("Failed to send tracking request {}", e)))?;
        Ok(response.text().await.unwrap())
    }

    pub async fn signup(&self, signup_data: req::Signup) -> ClientResult<res::Signup> {
        self.tracking().await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        let url = Url::parse(Self::API_DOMAIN).unwrap();
        let response = self
            .client
            .post(format!(
                "{}{}",
                Self::API_DOMAIN,
                "/signup/api/createAccount?trk=native_voyager_join"
            ))
            .headers(self.native_headers.clone())
            .json(&signup_data);
        let response = match response.send().await {
            Ok(response) => response,
            Err(e) => return Err(ResponseError(format!("Failed to send signup request {}", e))),
        };

        let result = match response.status() {
            StatusCode::OK => match response.json::<res::Signup>().await {
                Ok(response) => Ok(response),
                Err(e) => Err(ResponseError(format!("Failed to parse signup response {}", e))),
            },
            _code => {
                let response = match response.text().await {
                    Ok(response) => response,
                    Err(e) => return Err(ResponseError(format!("Failed to get signup response {}", e))),
                };
                Err(ResponseError(format!("Failed to signup {} {}", _code, response)))
            }
        };
        cookie_save(&self.cookie_store, &url, &signup_data.email_address).map_err(|e| IoError(e))?;
        result
    }

    pub async fn challenge(&self, challenge: &SignupChallenge) -> ClientResult<String> {
        let url = fatal_unwrap_e!(
            Url::parse(format!("{}{}", Self::API_DOMAIN, challenge.challenge_url).as_str()),
            "Failed to parse challenge url {}"
        );

        let res = self
            .client
            .get(url)
            .headers(self.webview_headers.clone())
            .send()
            .await
            .map_err(|e| ResponseError(format!("Failed to send challenge request {}", e)))?;
        let text = match res.status() {
            StatusCode::OK => match res.text().await {
                Ok(text) => text,
                Err(e) => return Err(ResponseError(format!("Failed to get challenge text {}", e))),
            },
            _code => {
                let response = match res.text().await {
                    Ok(response) => response,
                    Err(e) => return Err(ResponseError(format!("Failed to get challenge response {}", e))),
                };
                return Err(ResponseError(format!("Failed to challenge {} {}", _code, response)));
            }
        };
        Ok(text)
    }
}
