use actix_web::web::get;
use log::{debug, error, info};
use omicron_crawler::azure::json::{CrawledProfiles, ProfileIds};
use omicron_crawler::azure::{AzureClient, Label};
use omicron_crawler::env::{get_env, load_env};
use omicron_crawler::errors::CrawlerResult;
use omicron_crawler::linkedin::api::crawler::Crawler;
use omicron_crawler::linkedin::api::json::{SearchParams, SearchResult};
use omicron_crawler::linkedin::api::rate_limits::RateLimits;
use omicron_crawler::linkedin::api::LinkedinSession;
use omicron_crawler::logger::Logger;
use omicron_crawler::{fatal_assert, fatal_unwrap, fatal_unwrap_e};
use std::collections::VecDeque;
use std::process::exit;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicU8};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;

async fn obtain_profiles(params: SearchParams, crawler: &Crawler, azure_client: Arc<AzureClient>) {
    let profiles = match crawler.search_people(params).await {
        Ok(profiles) => profiles,
        Err(e) => {
            error!("Failed to search people {}", e);
            return;
        }
    };
    //TODO Retry in case of failure
    tokio::spawn(async move {
        info!("Pushing {} search result to manager", profiles.elements.len());
        azure_client.push_to_manager(&profiles, Label::SearchComplete).await;
    });
}

async fn crawl_profile(mut ids: ProfileIds, crawler: &mut Crawler, azure_client: Arc<AzureClient>) {
    info!("Crawling {} profiles", ids.ids.len());
    const PROFILES_PER_REQUEST: usize = 10;
    let chunks = ids.ids.chunks(PROFILES_PER_REQUEST);
    let request_metadata = ids.request_metadata.take();
    let mut current_profile = 0;
    for chunk in chunks {
        let crawled_profiles = match crawler.profiles(chunk, Some(&SHUTDOWN_SIGNAL)).await {
            Ok(profiles) => profiles,
            Err(e) => {
                error!("Failed to crawl profiles {}", e);
                return;
            }
        };
        current_profile += crawled_profiles.len();
        let metadata = request_metadata.clone();
        let azure_client_clone = azure_client.clone();
        tokio::task::spawn(async move {
            info!("Pushing {} profiles to manager", crawled_profiles.len());
            let crawled_profiles = CrawledProfiles {
                profiles: crawled_profiles,
                request_metadata: metadata,
            };
            //TODO Retry in case of failure
            azure_client_clone.push_to_manager(crawled_profiles, Label::ProfilesComplete).await
        });
        if SHUTDOWN_SIGNAL.load(Relaxed) == true {
            break;
        }
    }

    if SHUTDOWN_SIGNAL.load(Relaxed) == true {
        let remaining_profiles: Vec<String> = ids.ids.iter().skip(current_profile).map(|p| p.to_string()).collect();
        info!(
            "Pushing {} unfinished profiles from {} to queue...",
            remaining_profiles.len(),
            ids.ids.len()
        );
        let profile_ids = ProfileIds {
            ids: remaining_profiles,
            request_metadata,
        };
        if let Err(e) = azure_client.push_to_queue(&profile_ids).await {
            error!("Failed to push profiles to queue! {}", e);
        }
    }
}

static SHUTDOWN_SIGNAL: AtomicBool = AtomicBool::new(false);
#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    load_env();
    let env = get_env().await;
    Logger::init(env.log_level);
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();
    let mut crawler = Crawler::new(RateLimits::new(100, 800), username, password).await;
    let azure_client = Arc::new(AzureClient::new().await);

    // Spawn CTRL+C handler task
    tokio::spawn(async move {
        fatal_unwrap_e!(signal::ctrl_c().await, "Failed to listen for CTRL+C {}");
        info!("Received CTRL+C, initiating shutdown...");
        SHUTDOWN_SIGNAL.store(true, Relaxed);
    });

    loop {
        let azure_client_clone = azure_client.clone();
        match azure_client_clone.dequeue_search().await {
            Ok(search_params) => match search_params {
                /// TODO log search params
                Some(search_params) => obtain_profiles(search_params, &crawler, azure_client_clone).await,
                None => debug!("Search queue is empty"),
            },
            Err(e) => {
                error!("Failed to dequeue search! {}", e);
            }
        }

        let azure_client_clone = azure_client.clone();
        match azure_client_clone.dequeue_profile().await {
            Ok(profiles) => match profiles {
                Some(profiles) => crawl_profile(profiles, &mut crawler, azure_client_clone).await,
                None => debug!("profile queue is empty"),
            },
            Err(e) => {
                error!("Failed to dequeue profile! {}", e);
            }
        }
        if SHUTDOWN_SIGNAL.load(Relaxed) {
            break;
        }
    }
    Ok(())
}
