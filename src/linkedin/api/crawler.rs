use crate::azure::json::{CrawledProfiles, ProfileIds};
use crate::config::Config;
use crate::errors::CrawlerError::{NoFreeSession, SessionError};
use crate::errors::CrawlerResult;
use crate::linkedin::api::json::{Profile, SearchParams, SearchResult};
use crate::linkedin::api::rate_limits::RateLimiter;
use crate::linkedin::api::LinkedinClient;
use crate::session_pool::{SessionPool, SessionProxy};
use crossbeam::channel::Receiver;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use tokio::task::JoinHandle;

pub struct LinkedinSessionManager {
    session_pool: SessionPool<LinkedinClient>,
    rate_limits: RateLimiter,
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
            match linkedin_client.authenticate(entry.username.as_str(), entry.password.as_str(), false) {
                Err(e) => {
                    continue;
                    warn!("user {} failed to authenticate", entry.username);
                }
                Ok(_) => {}
            }
            sessions.push(linkedin_client);
        }

        let session_pool = SessionPool::new(sessions);
        Ok(Self { session_pool, rate_limits })
    }

    async fn try_acquire_session(&self) -> CrawlerResult<SessionProxy<LinkedinClient>> {
        match self.session_pool.acquire() {
            Some(session) => Ok(session),
            None => Err(NoFreeSession("No free session".to_string())),
        }
    }

    async fn busy_acquire_session(&self) -> SessionProxy<LinkedinClient> {
        loop {
            match self.session_pool.acquire() {
                Some(session) => return session,
                None => {
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }
            }
        }
    }

    pub async fn search_people(&self, params: SearchParams) -> CrawlerResult<JoinHandle<CrawlerResult<SearchResult>>> {
        let session = self.try_acquire_session().await?;
        let linked_in_session = session.session.unwrap();
        let handle = tokio::task::spawn(async move {
            let result = linked_in_session.search_people(params).await;
            result
        });
        Ok(handle)
    }

    ///TODO Using channels is not efficient because of constant cache sync
    /// This is an MPSC case, so ArrayQueue would be better, but we have no way to yield from tasks with tokio.
    pub async fn profiles(&self, ids: Vec<String>, interrupt_signal: Option<&AtomicBool>) -> Receiver<CrawlerResult<Profile>> {
        let (tx, rx) = crossbeam::channel::unbounded();
        /// TODO Large overhead. Consider batching
        ///
        let rate_limiter = &self.rate_limits;
        tokio::task::spawn(async move {
            for i in ids.iter() {
                let session = self.busy_acquire_session().await;
                tokio::task::spawn(async move {
                    let client = session.session.as_ref().unwrap();
                    let profile = match client.profile(i.as_str()).await {
                        Ok(profile) => profile,
                        Err(e) => {
                            tx.send(Err(e)).unwrap();
                            continue;
                        }
                    };
                    let skills = match client.skills(i.as_str()).await {
                        Ok(skills) => skills,
                        Err(e) => {
                            tx.send(Err(e)).unwrap();
                            continue;
                        }
                    };

                    let mut profile = profile;
                    profile.skill_view = skills;
                    tx.send(Ok(profile)).unwrap();
                    let wait_time = rate_limiter.next().unwrap();
                    info!("Sleeping. Rate limit: {}", wait_time.as_secs());
                    tokio::time::sleep(wait_time).await;
                });
            }
        });
        rx

        // let linked_in_session = session.session.as_ref().unwrap();
        // let mut crawled_profiles = Vec::with_capacity(ids.len());
        // for profile in ids.iter() {
        //     if let Some(signal) = interrupt_signal {
        //         if signal.load(Relaxed) == true {
        //             break;
        //         }
        //     }
        //     let mut parsed_profile = match linked_in_session.profile(profile.as_str()).await {
        //         Ok(profile) => profile,
        //         Err(e) => {
        //             error!("Failed to crawl profile {} reason: {}", profile, e);
        //             continue;
        //         }
        //     };
        //     let skills = match linked_in_session.skills(profile.as_str()).await {
        //         Ok(skills) => skills,
        //         Err(e) => {
        //             error!("Failed to crawl skills {} reason:{}", profile, e);
        //             continue;
        //         }
        //     };
        //     parsed_profile.skill_view = skills;
        //     crawled_profiles.push(parsed_profile);
        //     let wait_time = self.rate_limits.next().unwrap();
        //     info!("Sleeping. Rate limit: {}", wait_time.as_secs());
        //     tokio::time::sleep(wait_time).await;
        // }
        // Ok(crawled_profiles)
    }
}
