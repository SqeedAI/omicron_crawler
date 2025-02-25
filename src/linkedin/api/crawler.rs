use crate::azure::json::{CrawledProfiles, ProfileIds};
use crate::config::Config;
use crate::errors::CrawlerError::{NoFreeSession, SessionError};
use crate::errors::CrawlerResult;
use crate::linkedin::api::crawler::Commands::ProfileReady;
use crate::linkedin::api::json::{Profile, SearchParams, SearchResult};
use crate::linkedin::api::rate_limits::RateLimiter;
use crate::linkedin::api::LinkedinClient;
use crate::session_pool::{SessionPool, SessionProxy};
use crossbeam::channel::{Receiver, Sender};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

pub struct LinkedinSessionManager {
    session_pool: Arc<SessionPool<LinkedinClient>>,
    rate_limits: RateLimiter,
}

pub enum Commands {
    SearchReady(CrawlerResult<SearchResult>),
    ProfileReady(CrawlerResult<Profile>),
    ProfileUnparsed(String),
    End,
}

impl LinkedinSessionManager {
    pub async fn new(rate_limits: RateLimiter, config_path: &str) -> CrawlerResult<Self> {
        let config = match Config::load_config(config_path) {
            Ok(config) => config,
            Err(e) => {
                return Err(SessionError(format!(
                    "Failed to load config, creating example config {}, please fill it out and try again",
                    config_path
                )));
            }
        };

        let mut sessions = Vec::with_capacity(config.sessions.len());
        for entry in config.sessions {
            let mut linkedin_client =
                LinkedinClient::new_proxy(entry.proxy.as_str(), entry.proxy_username.as_str(), entry.proxy_password.as_str());
            match linkedin_client
                .authenticate(entry.username.as_str(), entry.password.as_str(), false)
                .await
            {
                Err(e) => {
                    continue;
                    warn!("user {} failed to authenticate", entry.username);
                }
                Ok(_) => {}
            }
            sessions.push(linkedin_client);
        }

        let raw_session_pool = SessionPool::new(sessions);
        let session_pool = Arc::new(raw_session_pool);
        Ok(Self { session_pool, rate_limits })
    }
}
