use crate::azure::json::{CrawledProfiles, ProfileIds};
use crate::errors::CrawlerResult;
use crate::linkedin::api::json::{Profile, SearchParams, SearchResult};
use crate::linkedin::api::rate_limits::RateLimiter;
use crate::linkedin::api::LinkedinClient;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;

pub struct Crawler {
    linked_in_session: LinkedinClient,
    rate_limits: RateLimiter,
}

impl Crawler {
    pub async fn new(rate_limits: RateLimiter, username: &str, password: &str) -> Self {
        let mut linked_in_session = LinkedinClient::new();
        if let Err(e) = linked_in_session.authenticate(username, password, false).await {
            fatal_assert!("Failed to authenticate {}", e);
        }
        Self {
            linked_in_session: LinkedinClient::new(),
            rate_limits,
        }
    }

    pub async fn search_people(&self, params: SearchParams) -> CrawlerResult<SearchResult> {
        self.linked_in_session.search_people(params).await
    }
    pub async fn profiles(&self, ids: &[String], interrupt_signal: Option<&AtomicBool>) -> CrawlerResult<Vec<Profile>> {
        let mut crawled_profiles = Vec::with_capacity(ids.len());
        for profile in ids.iter() {
            if let Some(signal) = interrupt_signal {
                if signal.load(Relaxed) == true {
                    break;
                }
            }
            let mut parsed_profile = match self.linked_in_session.profile(profile.as_str()).await {
                Ok(profile) => profile,
                Err(e) => {
                    error!("Failed to crawl profile {} reason: {}", profile, e);
                    continue;
                }
            };
            let skills = match self.linked_in_session.skills(profile.as_str()).await {
                Ok(skills) => skills,
                Err(e) => {
                    error!("Failed to crawl skills {} reason:{}", profile, e);
                    continue;
                }
            };
            parsed_profile.skill_view = skills;
            crawled_profiles.push(parsed_profile);
            let wait_time = self.rate_limits.next().unwrap();
            info!("Sleeping. Rate limit: {}", wait_time.as_secs());
            tokio::time::sleep(wait_time).await;
        }
        Ok(crawled_profiles)
    }
}
