mod json;
mod utils;

use crate::errors::CrawlerError::SessionError;
use crate::errors::CrawlerResult;
use crate::linkedin::api::json::{AuthenticateResponse, FetchCookiesResponse, Profile};
use crate::linkedin::api::utils::{cookies_session_id, load_cookies, save_cookies};
use actix_web::cookie::CookieJar;
use http::{HeaderMap, HeaderValue};
use regex::Regex;
use reqwest::cookie::CookieStore as CookieStoreTrait;
use reqwest::{Client, Url};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde::de::Unexpected::Str;
use std::ops::Deref;
use std::sync::Arc;
use tokio::io::AsyncReadExt;

pub struct LinkedinSession {
    session_id: String,
    client: Client,
    cookie_store: Arc<CookieStoreMutex>,
}
//TODO Replace crawler result with api result

impl LinkedinSession {
    const LINKEDIN_URL: &'static str = "https://www.linkedin.com";
    const COOKIE_DOMAIN: &'static str = "www.linkedin.com";
    const API_URL: &'static str = "https://www.linkedin.com/voyager/api";

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
            info!("Using csrf token {}", token);
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
            return Err(SessionError(format!("Failed to authenticate {}", auth_response.status)));
        }
        let cookies = self.cookie_store.lock().unwrap();
        let session_id = cookies.get(Self::COOKIE_DOMAIN, "/", "JSESSIONID").unwrap();
        self.session_id = session_id.value().to_string();
        Ok(())
    }

    pub async fn authenticate(&mut self, username: &str, password: &str) -> CrawlerResult<()> {
        self.obtain_session_id().await?;
        info!("Obtained session id");
        let headers = Self::create_default_headers(Some(&self.session_id));
        let form = vec![
            ("session_key", username),
            ("session_password", password),
            ("JSESSIONID", &self.session_id),
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
            return Err(SessionError(format!("Failed to authenticate {}", response.status())));
        }

        let response_data = match response.json::<AuthenticateResponse>().await {
            Ok(response_data) => response_data,
            Err(e) => {
                return Err(SessionError(format!("Failed to parse authenticate response {}", e)));
            }
        };

        if response_data.login_result != "PASS" {
            return Err(SessionError(format!("Failed to authenticate {}", response_data.login_result)));
        }
        let url = Url::parse(Self::LINKEDIN_URL).unwrap();
        let cookies = self.cookie_store.cookies(&url).unwrap();
        let bytes = cookies.as_bytes();
        save_cookies(bytes);
        info!("Authenticated successfully");
        Ok(())
    }

    pub async fn profile(&self, profile_id: String) -> CrawlerResult<Profile> {
        let endpoint = format!("{}/identity/profiles/{}/profileView", Self::API_URL, profile_id);
        let headers = Self::create_default_headers(Some(&self.session_id));
        match self.client.get(endpoint).headers(headers).send().await {
            Ok(response) => Ok(response.json::<Profile>().await.unwrap()),
            Err(e) => Err(SessionError(format!("Failed to get profile {}", e))),
        }
    }

    /// TODO Optimize string usage here. Maybe just a slice is needed for session_id instead of a copy
    pub fn new() -> LinkedinSession {
        let mut cookie_store = CookieStore::new(None);
        let mut session_id = "".to_string();

        if let Some(cookies) = load_cookies() {
            if let Some(found_session_id) = cookies_session_id(&cookies) {
                info!("Found cookies, using them");
                session_id = found_session_id;
                let linkedin_url = Url::parse(Self::LINKEDIN_URL).unwrap();
                let cookie_list = cookies.split(";").collect::<Vec<&str>>();
                for cookie in cookie_list {
                    if let Err(code) = cookie_store.parse(cookie, &linkedin_url) {
                        error!("Failed to parse cookie {}", code);
                    }
                }
            }
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
        }
    }
}
