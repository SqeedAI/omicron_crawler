use crate::errors::{ClientError, ClientResult};
use crate::two_captcha::req::{FunCaptchaTask, FunCaptchaTaskProxyless, ProxyType, TaskCheck};
use crate::two_captcha::res::{Solve, TaskResult};
use crate::two_captcha::traits::TwoCaptchaClient;
pub mod req;
pub mod res;
pub mod traits;

const API_URL: &'static str = "https://api.2captcha.com";

pub struct ProxyClient {
    client: reqwest::Client,
    client_key: String,
    proxy_addr: String,
    proxy_port: String,
    username: String,
    password: String,
    user_agent: String,
    proxy_type: ProxyType,
}

impl ProxyClient {
    pub fn new(
        proxy_addr: &str,
        proxy_port: &str,
        username: &str,
        password: &str,
        proxy_type: ProxyType,
        client_key: &str,
        user_agent: &str,
    ) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            client_key: client_key.to_string(),
            proxy_addr: proxy_addr.to_string(),
            proxy_port: proxy_port.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            user_agent: user_agent.to_string(),
            proxy_type,
        }
    }
}

impl TwoCaptchaClient for ProxyClient {
    async fn solve(&self, website_public_key: &str, website_public_url: &str, subdomain: Option<String>) -> ClientResult<Solve> {
        let url = reqwest::Url::parse(format!("{}/createTask", API_URL).as_str()).map_err(|e| ClientError::UrlError(e.to_string()))?;
        let request = FunCaptchaTask {
            proxy_password: self.password.clone(),
            proxy_login: self.username.clone(),
            proxy_address: self.proxy_addr.clone(),
            proxy_port: self.proxy_port.clone(),
            website_public_key: website_public_key.to_string(),
            website_url: website_public_url.to_string(),
            user_agent: self.user_agent.clone(),
            proxy_type: self.proxy_type.clone(),
            funcaptcha_api_jssubdomain: subdomain,
        };

        let response = self
            .client
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(|e| ClientError::ResponseError(e.to_string()))?;
        response.json().await.map_err(|e| ClientError::SerializationError(e.to_string()))
    }

    async fn get_task_result(&self, task_id: u32) -> ClientResult<TaskResult> {
        let url = reqwest::Url::parse(format!("{}/getTaskResult", API_URL).as_str()).map_err(|e| ClientError::UrlError(e.to_string()))?;
        let request = TaskCheck {
            client_key: self.client_key.clone(),
            task_id,
        };
        let response = self
            .client
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(|e| ClientError::ResponseError(e.to_string()))?;

        response.json().await.map_err(|e| ClientError::SerializationError(e.to_string()))
    }
}

pub struct ProxyLessClient {
    client: reqwest::Client,
    client_key: String,
    user_agent: String,
}

impl ProxyLessClient {
    pub fn new(user_agent: &str, client_key: &str) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            client_key: client_key.to_string(),
            user_agent: user_agent.to_string(),
        }
    }
}

impl TwoCaptchaClient for ProxyLessClient {
    async fn solve(&self, website_public_key: &str, website_public_url: &str, subdomain: Option<String>) -> ClientResult<Solve> {
        let url = reqwest::Url::parse(format!("{}/createTask", API_URL).as_str()).map_err(|e| ClientError::UrlError(e.to_string()))?;
        let request = FunCaptchaTaskProxyless {
            website_public_key: website_public_key.to_string(),
            website_url: website_public_url.to_string(),
            user_agent: self.user_agent.clone(),
            funcaptcha_api_js_subdomain: subdomain,
        };

        let response = self
            .client
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(|e| ClientError::ResponseError(e.to_string()))?;

        response.json().await.map_err(|e| ClientError::SerializationError(e.to_string()))
    }

    async fn get_task_result(&self, task_id: u32) -> ClientResult<TaskResult> {
        let url = reqwest::Url::parse(format!("{}/getTaskResult", API_URL).as_str()).map_err(|e| ClientError::UrlError(e.to_string()))?;
        let request = TaskCheck {
            client_key: self.client_key.clone(),
            task_id,
        };
        let response = self
            .client
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(|e| ClientError::ResponseError(e.to_string()))?;

        response.json().await.map_err(|e| ClientError::SerializationError(e.to_string()))
    }
}
