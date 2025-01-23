use crate::azure::json::{CrawledProfiles, ProfileIds};
use crate::errors::CrawlerResult;
use crate::linkedin::api::json::{SearchParams, SearchResult};
use crate::linkedin::api::LinkedinSession;

pub struct Crawler {
    pub linked_in_session: LinkedinSession,
}

impl Crawler {
    pub fn new() -> Self {
        Self {
            linked_in_session: LinkedinSession::new(),
        }
    }

    pub async fn search_people(&self, params: &mut SearchParams) -> CrawlerResult<SearchResult> {
        self.linked_in_session.search_people(params).await
    }

    pub async fn profiles(&self, mut ids: ProfileIds) -> CrawlerResult<CrawledProfiles> {
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
        }
        Ok(CrawledProfiles {
            profiles: crawled_profiles,
            request_metadata: ids.request_metadata.take(),
        })
    }
}
