use log::{error, info};
use omicron_crawler::azure::{AzureClient, Label};
use omicron_crawler::errors::CrawlerResult;
use omicron_crawler::fatal_assert;
use omicron_crawler::linkedin::api::json::{SearchParams, SearchResult};
use omicron_crawler::linkedin::api::LinkedinSession;
use omicron_crawler::logger::Logger;
use std::collections::VecDeque;
use std::sync::Arc;

async fn obtain_profiles(params: SearchParams, linkedin_session: Arc<LinkedinSession>, azure_client: Arc<AzureClient>) {
    let result = match linkedin_session.search_people(&params).await {
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
        Err(e) => error!("{}", e),
    }
}
#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    Logger::init(log::LevelFilter::Trace);
    let mut linkedin_session_raw = LinkedinSession::new();
    let azure_client = Arc::new(AzureClient::new());
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
    let error_wait_time = std::time::Duration::from_millis(1000);

    loop {
        let search_params = match azure_client.dequeue_search().await {
            Ok(search_params) => search_params,
            Err(e) => {
                error!("Failed to dequeue search params {}", e);
                tokio::time::sleep(error_wait_time).await;
                continue;
            }
        };
        let search_params = match search_params {
            Some(search_params) => search_params,
            None => {
                info!("Search queue is empty, retrying");
                continue;
            }
        };

        let azure_client_clone = azure_client.clone();
        let linkedin_session_clone = linkedin_session.clone();
        info!("Searching for profiles");
        tokio::spawn(async move { obtain_profiles(search_params, linkedin_session_clone, azure_client_clone).await });
    }
    Ok(())
}
