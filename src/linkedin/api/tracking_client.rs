use http::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn new_webview_tracking_headers(webview_user_agent: &str, requested_with: &str) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    header_map.insert("User-Agent", HeaderValue::from_str(webview_user_agent.as_str()));
    header_map.insert("x-requested-with", HeaderValue::from_str(requested_with).unwrap());
    header_map.insert("sec-fetch-site", HeaderValue::from_str("none"));
    header_map.insert("sec-fetch-mode", HeaderValue::from_str("navigate"));
    header_map.insert("sec-fetch-user", HeaderValue::from_str("?1"));
    header_map.insert("sec-fetch-dest", HeaderValue::from_str("document"));
    header_map.insert("accept-language", HeaderValue::from_str("en-US,en;q=0.9"));
}

pub fn new_native_tracking_headers(jsessionid: &str, device_info: &DeviceInfo, user_agent: &str, li_user_agent: &str) -> HeaderMap {
    let mut header_map = HeaderMap::new();

    let host = "www.linkedin.com";
    let user_agent = user_agent.to_string();
    let x_udid = device_info.device_id.clone();
    let accept_language = "en-US";
    let csrf_token = jsessionid.to_string();
    let x_li_track = device_info;
    let x_li_lang = "en-US";
    let x_li_user_agent = li_user_agent.to_string();

    let tracking_json = serde_json::to_string(&x_li_track).unwrap();
    header_map.insert("Host", HeaderValue::from_static(host));
    header_map.insert("User-Agent", HeaderValue::from_str(user_agent.as_str()).unwrap());
    header_map.insert("x-udid", HeaderValue::from_str(x_udid.as_str()).unwrap());
    header_map.insert("accept-language", HeaderValue::from_static(accept_language));
    header_map.insert("csrf-token", HeaderValue::from_str(csrf_token.as_str()).unwrap());
    header_map.insert("x-li-track", HeaderValue::from_str(tracking_json.as_str()).unwrap());
    header_map.insert("x-li-lang", HeaderValue::from_static(x_li_lang));
    header_map.insert("x-li-user-agent", HeaderValue::from_str(x_li_user_agent.as_str()).unwrap());
    header_map
}

pub fn default_requested_with() -> &'static str {
    "com.linkedin.android"
}

pub fn default_webview_user_agent() -> &'static str {
    "Mozilla/5.0 (Linux; Android 14; Android SDK built for x86_64 Build/SE1A.220826.006.A1; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/91.0.4472.114 Mobile Safari/537.36"
}
pub fn default_user_agent() -> &'static str {
    "ANDROID OS"
}
pub fn default_li_user_agent() -> &'static str {
    "LIAuthLibrary:0.0.3 com.linkedin.android:4.1.1022 unknown_Android SDK built for x86_64:android_12"
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    os_name: String,
    os_version: String,
    client_version: String,
    client_minor_version: i32,
    model: String,
    display_density: f64,
    display_width: i32,
    display_height: i32,
    dpi: String,
    device_type: String,
    app_id: String,
    device_id: String,
    timezone_offset: i32,
    timezone: String,
    store_id: String,
    is_ad_tracking_limited: bool,
    mp_name: String,
    mp_version: String,
}

impl DeviceInfo {
    pub fn new_timezone(timezone: &str) -> Self {
        Self {
            os_name: "Android OS".to_string(),
            os_version: "31".to_string(),
            client_version: "4.1.1022".to_string(),
            client_minor_version: 193000,
            model: "unknown_Android SDK built for x86_64".to_string(),
            display_density: 2.625,
            display_width: 1080,
            display_height: 2274,
            dpi: "xhdpi".to_string(),
            device_type: "android".to_string(),
            app_id: "com.linkedin.android".to_string(),
            device_id: Uuid::new_v4().to_string(),
            timezone_offset: 1,
            timezone: timezone.to_string(),
            store_id: "us_googleplay".to_string(),
            is_ad_tracking_limited: false,
            mp_name: "voyager-android".to_string(),
            mp_version: "1.100.118".to_string(),
        }
    }
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            os_name: "Android OS".to_string(),
            os_version: "31".to_string(),
            client_version: "4.1.1022".to_string(),
            client_minor_version: 193000,
            model: "unknown_Android SDK built for x86_64".to_string(),
            display_density: 2.625,
            display_width: 1080,
            display_height: 2274,
            dpi: "xhdpi".to_string(),
            device_type: "android".to_string(),
            app_id: "com.linkedin.android".to_string(),
            device_id: Uuid::new_v4().to_string(),
            timezone_offset: 1,
            timezone: "Europe/Bratislava".to_string(),
            store_id: "us_googleplay".to_string(),
            is_ad_tracking_limited: false,
            mp_name: "voyager-android".to_string(),
            mp_version: "1.100.118".to_string(),
        }
    }
}
