use crate::azure::json::{CrawledProfiles, ProfileIds};
use crate::errors::CrawlerResult;
use crate::linkedin::api::json::{SearchParams, SearchResult};
use crate::linkedin::api::rate_limits::RateLimits;
use crate::linkedin::api::LinkedinSession;
use std::time::Duration;

pub struct Crawler {
    linked_in_session: LinkedinSession,
    rate_limits: RateLimits,
}

impl Crawler {
    pub async fn new(rate_limits: RateLimits, username: &str, password: &str) -> Self {
        let mut linked_in_session = LinkedinSession::new();
        if !linked_in_session.is_auth {
            match linked_in_session.authenticate(username, password).await {
                Ok(_) => {}
                Err(e) => panic!("Failed to authenticate {}", e),
            }
        }
        Self {
            linked_in_session: LinkedinSession::new(),
            rate_limits,
        }
    }

    pub async fn search_people(&self, params: SearchParams) -> CrawlerResult<SearchResult> {
        self.linked_in_session.search_people(params).await
    }

    /// TODO Add splitting
    pub async fn profiles(&mut self, mut ids: ProfileIds) -> CrawlerResult<CrawledProfiles> {
        let mut crawled_profiles = Vec::with_capacity(ids.ids.len());
        for profile in ids.ids.iter() {
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
        Ok(CrawledProfiles {
            profiles: crawled_profiles,
            request_metadata: ids.request_metadata.take(),
        })
    }
}
