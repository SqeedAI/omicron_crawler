mod json;
mod utils;

use crate::errors::CrawlerError::SessionError;
use crate::errors::CrawlerResult;
use crate::linkedin::api::json::{AuthenticateResponse, FetchCookiesResponse};
use actix_web::cookie::CookieJar;
use http::{HeaderMap, HeaderValue};
use regex::Regex;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::{Client, Url};
use serde::de::Unexpected::Str;
use std::sync::Arc;
use tokio::io::AsyncReadExt;

static LINKEDIN_URL: &str = "https://www.linkedin.com";
static API_URL: &str = "https://www.linkedin.com/voyager/api";

//TODO Split the apis into separate functions

pub struct LinkedinSession {
    pub session_id: String,
    pub client: Client,
    pub cookie_store: Arc<Jar>,
}

impl LinkedinSession {
    fn create_default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Li-User-Agent",
            HeaderValue::from_static("LIAuthLibrary:0.0.3 com.linkedin.android:4.1.881 Asus_ASUS_Z01QD:android_9"),
        );
        headers.insert("User-Agent", HeaderValue::from_static("ANDROID OS"));
        headers.insert("X-User-Language", HeaderValue::from_static("en"));
        headers.insert("X-User-Locale", HeaderValue::from_static("en_US"));
        headers.insert("Accept-Language", HeaderValue::from_static("en-us"));
        headers
    }
    async fn obtain_session_id(client: &Client, cookie_store: &Jar) -> CrawlerResult<String> {
        let auth_url = format!("{}{}", LINKEDIN_URL, "/uas/authenticate");
        let default_headers = Self::create_default_headers();
        let response = fatal_unwrap_e!(
            client.get(auth_url).headers(default_headers).send().await,
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
        let linkedin_url = Url::parse(LINKEDIN_URL).unwrap();
        let session_id = match cookie_store.cookies(&linkedin_url) {
            Some(cookie) => {
                let str_cookie = match cookie.to_str() {
                    Ok(str_cookie) => str_cookie,
                    Err(_) => {
                        return Err(SessionError("Failed to convert cookie to str".to_string()));
                    }
                };
                let re = Regex::new(r#"JSESSIONID="(.*?)"(?:;|$)"#).unwrap();
                info!("Checking cookie {}", str_cookie);
                match re.captures(str_cookie) {
                    Some(captures) => captures.get(1).unwrap().as_str().to_string(),
                    None => {
                        return Err(SessionError("Failed to find JSESSIONID cookie".to_string()));
                    }
                }
            }
            None => {
                return Err(SessionError("No cookie found".to_string()));
            }
        };
        Ok(session_id)
    }
    pub async fn new(username: &str, password: &str) -> CrawlerResult<LinkedinSession> {
        let cookie_store = Arc::new(Jar::default());
        let client = fatal_unwrap_e!(
            Client::builder().cookie_store(true).cookie_provider(cookie_store.clone()).build(),
            "Failed to create client {}"
        );

        let session_id = Self::obtain_session_id(&client, &cookie_store).await?;
        info!("Obtained session id");
        let headers = Self::create_default_headers();
        let form = vec![
            ("session_key", username),
            ("session_password", password),
            ("JSESSIONID", &session_id),
        ];
        let response = match client
            .post(format!("{}{}", LINKEDIN_URL, "/uas/authenticate"))
            .headers(headers)
            .header("csrf-token", &session_id)
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
        println!("{}", response_data.login_result);
        info!("Authenticated successfully");

        Ok(LinkedinSession {
            session_id,
            client,
            cookie_store,
        })
    }
}
