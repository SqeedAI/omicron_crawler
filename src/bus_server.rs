use log::{debug, error, info};
use omicron_crawler::azure::json::{CrawledProfiles, ProfileIds};
use omicron_crawler::azure::{AzureClient, Label};
use omicron_crawler::env::load_env;
use omicron_crawler::errors::CrawlerResult;
use omicron_crawler::fatal_assert;
use omicron_crawler::linkedin::api::json::{SearchParams, SearchResult};
use omicron_crawler::linkedin::api::LinkedinSession;
use omicron_crawler::logger::Logger;
use std::collections::VecDeque;
use std::sync::atomic::AtomicU8;
use std::sync::Arc;

async fn obtain_profiles(mut params: SearchParams, linkedin_session: Arc<LinkedinSession>, azure_client: Arc<AzureClient>) {
    let result = match linkedin_session.search_people(&mut params).await {
        Ok(result) => result,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    match azure_client.push_to_bus(&result, Label::SearchComplete).await {
        Ok(_) => {
            info!("Pushed search result to bus!");
        }
        Err(e) => error!("Failed pushing searches to the bus{}", e),
    }
}

async fn crawl_profile(profiles: &mut ProfileIds, linkedin_session: Arc<LinkedinSession>, azure_client: Arc<AzureClient>) {
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
        crawled_profiles.push(crawled_profile)
    }
    let crawled_profiles = CrawledProfiles {
        profiles: crawled_profiles,
        request_metadata: profiles.request_metadata.take(),
    };
    match azure_client.push_to_bus(&crawled_profiles, Label::ProfilesComplete).await {
        Ok(_) => {
            info!("Pushed profiles to bus!");
        }
        Err(e) => error!("Failed pushing profiles to bus {}", e),
    }
}

static MAXIMUM_SEARCHES: AtomicU8 = AtomicU8::new(2);
static CURRENT_SEARCHES: AtomicU8 = AtomicU8::new(0);

static MAXIMUM_PROFILE_CRAWLS: AtomicU8 = AtomicU8::new(2);
static CURRENT_PROFILE_CRAWLS: AtomicU8 = AtomicU8::new(0);
#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    load_env();
    Logger::init(log::LevelFilter::Trace);
    let mut linkedin_session_raw = LinkedinSession::new();
    let azure_client = Arc::new(AzureClient::new().await);
    if !linkedin_session_raw.is_auth() {
        info!("Not authenticated, trying to authenticate");
        match linkedin_session_raw
            .authenticate("erik9631@gmail.com", "soRMoaN7C2bX2mKbV9V4")
            .await
        {
            Ok(_) => {
                info!("Authenticated successfully");
            }
            Err(e) => {
                fatal_assert!("Failed to authenticate {}", e);
            }
        }
    }
    let linkedin_session = Arc::new(linkedin_session_raw);
    let poll_time = std::time::Duration::from_millis(500);

    loop {
        let azure_client_clone = azure_client.clone();
        let linkedin_session_clone = linkedin_session.clone();
        if CURRENT_SEARCHES.load(std::sync::atomic::Ordering::Relaxed) < MAXIMUM_SEARCHES.load(std::sync::atomic::Ordering::Relaxed) {
            tokio::spawn(async move {
                CURRENT_SEARCHES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let search_params = match azure_client_clone.dequeue_search().await {
                    Ok(search_params) => search_params,
                    Err(e) => {
                        error!("Failed to dequeue search params {}", e);
                        CURRENT_SEARCHES.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                        return;
                    }
                };
                let search_params = match search_params {
                    Some(search_params) => search_params,
                    None => {
                        debug!("Search queue is empty");
                        CURRENT_SEARCHES.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                        return;
                    }
                };

                info!("Searching for profiles");
                obtain_profiles(search_params, linkedin_session_clone, azure_client_clone).await;
                CURRENT_SEARCHES.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            });
        }

        let azure_client_clone = azure_client.clone();
        let linkedin_session_clone = linkedin_session.clone();
        if CURRENT_PROFILE_CRAWLS.load(std::sync::atomic::Ordering::Relaxed)
            < MAXIMUM_PROFILE_CRAWLS.load(std::sync::atomic::Ordering::Relaxed)
        {
            tokio::spawn(async move {
                CURRENT_PROFILE_CRAWLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let mut profiles = match azure_client_clone.dequeue_profile().await {
                    Ok(profiles) => match profiles {
                        Some(profiles) => profiles,
                        None => {
                            debug!("profile queue is empty");
                            CURRENT_PROFILE_CRAWLS.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                            return;
                        }
                    },
                    Err(e) => {
                        error!("Failed to dequeue profiles {}", e);
                        CURRENT_PROFILE_CRAWLS.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                        return;
                    }
                };
                crawl_profile(&mut profiles, linkedin_session_clone, azure_client_clone).await;
                CURRENT_PROFILE_CRAWLS.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            });
        }
        tokio::time::sleep(poll_time).await;
    }
    Ok(())
}
