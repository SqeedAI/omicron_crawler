use actix_web::web::get;
use log::{debug, error, info};
use omicron_crawler::azure::json::{CrawledProfiles, ProfileIds};
use omicron_crawler::azure::{AzureClient, Label};
use omicron_crawler::env::{get_env, load_env};
use omicron_crawler::errors::CrawlerResult;
use omicron_crawler::linkedin::api::crawler::{Commands, LinkedinSessionManager};
use omicron_crawler::linkedin::api::json::{SearchParams, SearchResult};
use omicron_crawler::linkedin::api::rate_limits::RateLimiter;
use omicron_crawler::linkedin::api::LinkedinClient;
use omicron_crawler::logger::Logger;
use omicron_crawler::{fatal_assert, fatal_unwrap, fatal_unwrap_e};
use std::collections::VecDeque;
use std::mem;
use std::process::exit;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicU8};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::mpsc::{Sender, UnboundedSender};

async fn init_crawl_message_loop(azure_client: Arc<AzureClient>) -> UnboundedSender<Commands> {
    info!("Starting crawling tasks");
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::task::spawn(async move {
        while let Ok(command) = rx.recv() {
            match command {
                Commands::ProfileReady(profile) => match profile {
                    Ok(profile) => {
                        tokio::task::spawn(async move {
                            let crawled_profiles = CrawledProfiles {
                                profiles: vec![profile],
                                request_metadata: None,
                            };
                            azure_client.push_to_manager(&crawled_profiles, Label::ProfilesComplete).await;
                        });
                    }
                    Err(e) => {
                        error!("Failed to crawl profile {}", e);
                    }
                },
                Commands::SearchReady(search) => match search {
                    Ok(search) => {
                        tokio::task::spawn(async move {
                            azure_client.push_to_manager(search, Label::SearchComplete).await;
                        });
                    }
                    Err(e) => {
                        error!("Failed to crawl search {}", e);
                    }
                },
                End => break,
            }
        }
    });
    tx
}

static SHUTDOWN_SIGNAL: AtomicBool = AtomicBool::new(false);
#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    load_env();
    let env = get_env().await;
    Logger::init(env.log_level);
    let config_path = "config/proxies.yaml";
    let mut session_manager = match LinkedinSessionManager::new(RateLimiter::new(100, 800), config_path).await {
        Ok(crawler_result) => crawler_result,
        Err(e) => {
            error!("{}", e);
            return Ok(());
        }
    };

    let azure_client = Arc::new(AzureClient::new().await);
    let tx = init_crawl_message_loop(azure_client.clone()).await;

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
                Some(search_params) => session_manager.search_people_stream(search_params, tx.clone()).await,
                None => debug!("Search queue is empty"),
            },
            Err(e) => {
                error!("Failed to dequeue search! {}", e);
            }
        }

        let azure_client_clone = azure_client.clone();
        match azure_client_clone.dequeue_profile().await {
            Ok(profiles) => match profiles {
                Some(profiles) => session_manager.profiles_stream(profiles, None, tx.clone()).await,
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
