pub mod json;

// TODO Refactor, this crate should be generic, no LINKEDIN dependencies
use crate::api_client::ApiClient;
use crate::azure::json::ProfileIds;
use crate::env::get_env;
use crate::errors::ClientError::{RequestError, ResponseError, SerializationError};
use crate::errors::ClientResult;
use crate::linkedin::json::SearchParams;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::Serialize;
use sha2::Sha256;
use std::fmt::{Display, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding::encode;

fn create_sas_token(resource_uri: &str, sas_key_name: &str, sas_key: &str) -> Result<String, &'static str> {
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
    manager_bus_uri: &'static str,
    manager_bus_api: &'static str,
    manager_bus_key: &'static str,
    sas_key_name: &'static str,
    sas_profile_key: &'static str,
    sas_search_key: &'static str,
    search_uri: &'static str,
    search_dequeue_api: &'static str,
    search_queue_api: &'static str,
    profile_uri: &'static str,
    profile_dequeue_api: &'static str,
    manager_search_api: &'static str,
    manager_profile_api: &'static str,
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
    pub async fn new() -> Self {
        let manager_bus_uri = get_env().await.azure_manager_bus_uri.as_str();
        let manager_bus_api = get_env().await.azure_manager_bus_api.as_str();
        let manager_bus_key = get_env().await.azure_manager_bus_key.as_str();
        let sas_key_name = get_env().await.azure_sas_key_name.as_str();
        let sas_profile_key = get_env().await.azure_sas_profile_key.as_str();
        let sas_search_key = get_env().await.azure_sas_search_key.as_str();
        let search_uri = get_env().await.azure_search_uri.as_str();
        let search_dequeue_api = get_env().await.azure_search_dequeue_api.as_str();
        let search_queue_api = get_env().await.azure_profile_queue_api.as_str();
        let profile_uri = get_env().await.azure_profile_uri.as_str();
        let profile_dequeue_api = get_env().await.azure_profile_dequeue_api.as_str();
        let manager_profile_api = get_env().await.manager_profile_api.as_str();
        let manager_search_api = get_env().await.manager_search_api.as_str();

        Self {
            manager_bus_uri,
            manager_bus_api,
            manager_bus_key,
            sas_key_name,
            sas_profile_key,
            sas_search_key,
            search_uri,
            search_dequeue_api,
            search_queue_api,
            profile_uri,
            profile_dequeue_api,
            manager_search_api,
            manager_profile_api,
            client: Client::new(),
        }
    }
    pub async fn dequeue_profile(&self) -> ClientResult<Option<ProfileIds>> {
        let sas_token = match create_sas_token(self.profile_uri, self.sas_key_name, self.sas_profile_key) {
            Ok(token) => token,
            Err(e) => return Err(RequestError(e.to_string())),
        };
        match self
            .client
            .delete(self.profile_dequeue_api)
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
                    Err(e) => Err(ResponseError(format!("Failed to dequeue profile {:?}", e))),
                }
            }
            Err(e) => Err(ResponseError(format!("Failed to dequeue profile {}", e))),
        }
    }
    pub async fn dequeue_search(&self) -> ClientResult<Option<SearchParams>> {
        let sas_token = match create_sas_token(self.search_uri, self.sas_key_name, self.sas_search_key) {
            Ok(token) => token,
            Err(e) => return Err(RequestError(e.to_string())),
        };

        match self
            .client
            .delete(self.search_dequeue_api)
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
                    Err(e) => Err(ResponseError(format!("Failed to dequeue profile {}", e))),
                }
            }
            Err(e) => Err(ResponseError(format!("Failed to dequeue profile {}", e))),
        }
    }

    pub async fn push_to_queue<T>(&self, data: &T) -> ClientResult<()>
    where
        T: Serialize,
    {
        let sas_token = match create_sas_token(self.profile_uri, self.sas_key_name, self.sas_profile_key) {
            Ok(sas) => sas,
            Err(e) => return Err(RequestError(format!("Failed to push search result {}", e))),
        };
        let json_body = match serde_json::to_string(data) {
            Ok(json_body) => json_body,
            Err(e) => return Err(SerializationError(format!("Failed to push search result {}", e))),
        };
        match self
            .client
            .post(self.search_queue_api)
            .header("Authorization", sas_token)
            .header("Content-Type", "application/json")
            .body(json_body)
            .send()
            .await
        {
            Ok(request) => {
                if !request.status().is_success() {
                    return Err(ResponseError(format!(
                        "Failed to push to queue with status {} body {}",
                        request.status().as_u16(),
                        request.text().await.unwrap()
                    )));
                }
            }
            Err(e) => {
                return Err(ResponseError(format!("Failed to push to bus {:?}", e)));
            }
        }
        Ok(())
    }

    pub async fn push_to_bus<T>(&self, data: &T, label: Label) -> ClientResult<()>
    where
        T: Serialize,
    {
        let sas = match create_sas_token(self.manager_bus_uri, self.sas_key_name, self.manager_bus_key) {
            Ok(sas) => sas,
            Err(e) => return Err(RequestError(format!("Failed to push search result {}", e))),
        };
        let json_body = match serde_json::to_string(data) {
            Ok(json_body) => json_body,
            Err(e) => return Err(SerializationError(format!("Failed to push search result {}", e))),
        };

        let request = match self
            .client
            .post(self.manager_bus_api)
            .header("Authorization", sas)
            .header("Content-Type", "application/json")
            .header("BrokerProperties", format!("{{\"Label\":\"{}\"}}", label))
            .body(json_body)
            .send()
            .await
        {
            Ok(request) => request,
            Err(e) => return Err(ResponseError(format!("Failed to push search result {}", e))),
        };
        if !request.status().is_success() {
            return Err(ResponseError(format!(
                "Failed to push search result {}",
                request.text().await.unwrap()
            )));
        }
        Ok(())
    }

    pub async fn push_to_manager<T>(&self, data: T, label: Label)
    where
        T: Serialize + Sized + Send + Sync,
    {
        let api = match label {
            Label::SearchComplete => self.manager_search_api,
            Label::ProfilesComplete => self.manager_profile_api,
        };
        let json_body = match serde_json::to_string(&data) {
            Ok(json_body) => json_body,
            Err(e) => {
                error!("Failed to serialize data: {}", e);
                return;
            }
        };

        match self.client.post(api).body(json_body).send().await {
            Ok(request) => {
                if !request.status().is_success() {
                    error!(
                        "Failed to push to manager with status {} body {}",
                        request.status().as_u16(),
                        request.text().await.unwrap()
                    );
                }
            }
            Err(e) => {
                error!("Failed to push to manager {:?}", e);
            }
        }
    }
}
