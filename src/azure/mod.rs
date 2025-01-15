mod json;

// TODO Refactor services into another crate
use crate::azure::json::ProfileIds;
use crate::linkedin::api::json::SearchParams;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use hmac::{Hmac, Mac};
use reqwest::Client;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding::encode;

const SEARCH_URI: &str = "https://sqeed-dev-bus.servicebus.windows.net/search/";
const SEARCH_DEQUEUE_API: &str = "https://sqeed-dev-bus.servicebus.windows.net/search/messages/head";
const PROFILE_URI: &str = "https://sqeed-dev-bus.servicebus.windows.net/profile/";
const PROFILE_DEQUEUE_API: &str = "https://sqeed-dev-bus.servicebus.windows.net/profile/messages/head";
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

impl AzureClient {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }
    pub async fn dequeue_profile(&self) -> Result<ProfileIds, String> {
        let sas_token = match create_service_bus_sas_token(PROFILE_URI, SAS_KEY_NAME, SAS_PROFILE_KEY) {
            Ok(token) => token,
            Err(e) => return Err(e.to_string()),
        };
        match self
            .client
            .delete(PROFILE_DEQUEUE_API)
            .header("Authorization", sas_token)
            .send()
            .await
        {
            Ok(response) => match response.json::<ProfileIds>().await {
                Ok(profile) => Ok(profile),
                Err(e) => Err(format!("Failed to dequeue profile {}", e)),
            },
            Err(e) => Err(format!("Failed to dequeue profile {}", e)),
        }
    }

    pub async fn dequeue_search(&self) -> Result<SearchParams, String> {
        let sas_token = match create_service_bus_sas_token(SEARCH_URI, SAS_KEY_NAME, SAS_SEARCH_KEY) {
            Ok(token) => token,
            Err(e) => return Err(e.to_string()),
        };

        match self
            .client
            .delete(SEARCH_DEQUEUE_API)
            .header("Authorization", sas_token)
            .send()
            .await
        {
            Ok(response) => match response.json::<SearchParams>().await {
                Ok(params) => Ok(params),
                Err(e) => Err(format!("Failed to dequeue profile {}", e)),
            },
            Err(e) => Err(format!("Failed to dequeue profile {}", e)),
        }
    }

    pub async fn push_search_result() {}
}
