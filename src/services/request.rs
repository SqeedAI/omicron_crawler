use crate::get_linkedin_session;
use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse};
use log::{error, info, warn};
use omicron_crawler::azure::json::{CrawledProfiles, ProfileIds};
use omicron_crawler::driver::session_manager::{SessionManager, SessionPool};
use omicron_crawler::errors::CrawlerError;
use omicron_crawler::linkedin::api::json::SearchParams;
use omicron_crawler::linkedin::sales_crawler::SalesCrawler;
use std::cmp::min;
use std::thread;

#[post("/search")]
pub async fn search(search_params: Json<SearchParams>) -> HttpResponse {
    let linkedin_session = get_linkedin_session().await;
    let mut search_params = search_params.into_inner();
    let results = match linkedin_session.search_people(&mut search_params).await {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to perform search {}", e)),
    };
    HttpResponse::Ok().json(results)
}

#[post("/profiles")]
pub async fn profiles(url_requests: Json<ProfileIds>) -> HttpResponse {
    let linkedin_session = get_linkedin_session().await;
    let mut profiles = url_requests.into_inner();
    let mut crawled_profiles = Vec::with_capacity(profiles.ids.len());
    for profile in profiles.ids.iter() {
        let mut crawled_profile = match linkedin_session.profile(profile.as_str()).await {
            Ok(profile) => profile,
            Err(e) => {
                error!("Failed to crawl profile {} reason: {}", profile, e);
                continue;
            }
        };

        let skills = match linkedin_session.skills(profile.as_str()).await {
            Ok(skills) => Some(skills),
            Err(e) => {
                error!("Failed to crawl skills {} reason:{}", profile, e);
                None
            }
        };
        if let Some(skills) = skills {
            crawled_profile.skill_view = skills;
        }
        crawled_profiles.push(crawled_profile);
    }

    let crawled_profiles = CrawledProfiles {
        profiles: crawled_profiles,
        request_metadata: profiles.request_metadata.take(),
    };
    HttpResponse::Ok().json(crawled_profiles)
}
