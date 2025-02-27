use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, Clone)]
pub enum ProxyType {
    Http,
    Https,
}
impl Display for ProxyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyType::Http => write!(f, "http"),
            ProxyType::Https => write!(f, "https"),
        }
    }
}

#[derive(serde::Serialize)]
pub enum Task {
    FunCaptchaTaskProxyless(FunCaptchaTaskProxyless),
    FunCaptchaTask(FunCaptchaTask),
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoCaptcha {
    pub client_key: String,
    pub task: Task,
}
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunCaptchaTaskProxyless {
    pub website_url: String,
    pub website_public_key: String,
    pub funcaptcha_api_js_subdomain: Option<String>,
    pub user_agent: String,
}
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunCaptchaTask {
    pub website_url: String,
    pub website_public_key: String,
    pub funcaptcha_api_jssubdomain: Option<String>,
    pub user_agent: String,
    pub proxy_type: ProxyType,
    pub proxy_address: String,
    pub proxy_port: String,
    pub proxy_login: String,
    pub proxy_password: String,
}
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCheck {
    pub client_key: String,
    pub task_id: u32,
}
