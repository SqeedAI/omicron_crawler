use log::{debug, error, info};
use omicron_crawler::azure::json::{CrawledProfiles, ProfileIds};
use omicron_crawler::azure::{AzureClient, Label};
use omicron_crawler::env::{get_env, load_env};
use omicron_crawler::errors::CrawlerResult;
use omicron_crawler::fatal_assert;
use omicron_crawler::linkedin::api::crawler::Crawler;
use omicron_crawler::linkedin::api::json::{SearchParams, SearchResult};
use omicron_crawler::linkedin::api::rate_limits::RateLimits;
use omicron_crawler::linkedin::api::LinkedinSession;
use omicron_crawler::logger::Logger;
use std::collections::VecDeque;
use std::sync::atomic::AtomicU8;
use std::sync::Arc;

async fn obtain_profiles(mut params: SearchParams, crawler: &Crawler, azure_client: Arc<AzureClient>) {
    let profiles = match crawler.search_people(params).await {
        Ok(profiles) => profiles,
        Err(e) => {
            error!("Failed to search people {}", e);
            return;
        }
    };
    tokio::spawn(async move {
        info!("Pushing search result to manager");
        azure_client.push_to_manager(&profiles, Label::SearchComplete).await;
    });
}

// TODO profile rate limits
// Goal is 500 profiles per hour
// That means 5 cooldown periods, each one is 12 minutes
// Every 10th profile shall have a micro cooldown of 5 seconds (So 50 seconds - 12 minutes)
async fn crawl_profile(ids: ProfileIds, crawler: &mut Crawler, azure_client: Arc<AzureClient>) {
    let crawled_profiles = match crawler.profiles(ids).await {
        Ok(profiles) => profiles,
        Err(e) => {
            error!("Failed to crawl profiles {}", e);
            return;
        }
    };
    tokio::task::spawn(async move {
        info!("Pushing profiles to manager");
        azure_client.push_to_manager(crawled_profiles, Label::ProfilesComplete).await
    });
}
#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    load_env();
    Logger::init(log::LevelFilter::Trace);
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();
    let mut crawler = Crawler::new(RateLimits::new(100), username, password).await;
    let azure_client = Arc::new(AzureClient::new().await);

    loop {
        let azure_client_clone = azure_client.clone();
        match azure_client_clone.dequeue_search().await {
            Ok(search_params) => match search_params {
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
    }
    Ok(())
}
