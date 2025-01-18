pub mod json;

// TODO Refactor services into another crate
use crate::azure::json::ProfileIds;
use crate::errors::CrawlerError::{BusError, QueueError};
use crate::errors::CrawlerResult;
use crate::linkedin::api::json::SearchParams;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::Serialize;
use sha2::Sha256;
use std::fmt::{Display, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding::encode;

const SEARCH_URI: &str = "https://sqeed-dev-bus.servicebus.windows.net/search/";
const SEARCH_DEQUEUE_API: &str = "https://sqeed-dev-bus.servicebus.windows.net/search/messages/head";
const PROFILE_URI: &str = "https://sqeed-dev-bus.servicebus.windows.net/profile/";
const PROFILE_DEQUEUE_API: &str = "https://sqeed-dev-bus.servicebus.windows.net/profile/messages/head";
const MANAGER_BUS_URI: &str = "https://sqeed-dev-bus.servicebus.windows.net/manager/";
const MANAGER_BUS_API: &str = "https://sqeed-dev-bus.servicebus.windows.net/manager/messages";
const MANAGER_BUS_KEY: &str = "bC3swcT8ywbPHpNgSx4eJVG6tkhBtlC8b+ASbLzwa+4=";
const SAS_KEY_NAME: &str = "rw";
const SAS_PROFILE_KEY: &str = "xIf1mAf1YIRFq8WoUk4me4yG2XILTDUH7+ASbJl066Y=";
const SAS_SEARCH_KEY: &str = "kZxuQJeumvq2r7n+s7uhSENCDJIEdYjf6+ASbD/itM4=";
fn create_service_bus_sas_token(resource_uri: &str, sas_key_name: &str, sas_key: &str) -> Result<String, &'static str> {
    let encoded_uri = encode(resource_uri);

    let ttl = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 60;

    let signature_str = format!("{}\n{}", encoded_uri, ttl);
    let key_bytes = sas_key.as_bytes();

    let mut mac = match Hmac::<Sha256>::new_from_slice(&key_bytes) {
        Ok(mac) => mac,
        Err(_) => return Err("HMAC on SAS key failed"),
    };
    mac.update(signature_str.as_bytes());
    let hash = BASE64_STANDARD.encode(mac.finalize().into_bytes());

    Ok(format!(
        "SharedAccessSignature sr={}&sig={}&se={}&skn={}",
        encoded_uri,
        encode(&hash),
        ttl,
        sas_key_name
    ))
}

pub struct AzureClient {
    client: Client,
}

pub enum Label {
    ProfilesComplete,
    SearchComplete,
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Label::ProfilesComplete => write!(f, "profiles_complete"),
            Label::SearchComplete => write!(f, "search_complete"),
        }
    }
}

impl AzureClient {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }
    pub async fn dequeue_profile(&self) -> CrawlerResult<Option<ProfileIds>> {
        let sas_token = match create_service_bus_sas_token(PROFILE_URI, SAS_KEY_NAME, SAS_PROFILE_KEY) {
            Ok(token) => token,
            Err(e) => return Err(QueueError(e.to_string())),
        };
        match self
            .client
            .delete(PROFILE_DEQUEUE_API)
            .header("Authorization", sas_token)
            .send()
            .await
        {
            Ok(response) => {
                if response.status() == 204 {
                    return Ok(None);
                }
                match response.json::<ProfileIds>().await {
                    Ok(profile) => Ok(Some(profile)),
                    Err(e) => Err(QueueError(format!("Failed to dequeue profile {}", e))),
                }
            }
            Err(e) => Err(QueueError(format!("Failed to dequeue profile {}", e))),
        }
    }
    pub async fn dequeue_search(&self) -> CrawlerResult<Option<SearchParams>> {
        let sas_token = match create_service_bus_sas_token(SEARCH_URI, SAS_KEY_NAME, SAS_SEARCH_KEY) {
            Ok(token) => token,
            Err(e) => return Err(QueueError(e.to_string())),
        };

        match self
            .client
            .delete(SEARCH_DEQUEUE_API)
            .header("Authorization", sas_token)
            .send()
            .await
        {
            Ok(response) => {
                if response.status() == 204 {
                    return Ok(None);
                }
                match response.json::<SearchParams>().await {
                    Ok(params) => Ok(Some(params)),
                    Err(e) => Err(QueueError(format!("Failed to dequeue profile {}", e))),
                }
            }
            Err(e) => Err(QueueError(format!("Failed to dequeue profile {}", e))),
        }
    }

    pub async fn push_to_bus<T>(&self, search_result: &T, label: Label) -> CrawlerResult<()>
    where
        T: Serialize + Sized,
    {
        let sas = match create_service_bus_sas_token(MANAGER_BUS_URI, SAS_KEY_NAME, MANAGER_BUS_KEY) {
            Ok(sas) => sas,
            Err(e) => return Err(BusError(format!("Failed to push search result {}", e))),
        };
        let json_body = match serde_json::to_string(search_result) {
            Ok(json_body) => json_body,
            Err(e) => return Err(BusError(format!("Failed to push search result {}", e))),
        };

        let request = match self
            .client
            .post(MANAGER_BUS_API)
            .header("Authorization", sas)
            .header("Content-Type", "application/json")
            .header("BrokerProperties", format!("{{\"Label\":\"{}\"}}", label))
            .body(json_body)
            .send()
            .await
        {
            Ok(request) => request,
            Err(e) => return Err(BusError(format!("Failed to push search result {}", e))),
        };
        if !request.status().is_success() {
            return Err(BusError(format!("Failed to push search result {}", request.text().await.unwrap())));
        }
        Ok(())
    }
}
