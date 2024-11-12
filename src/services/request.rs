use crate::get_driver_session_manager;
use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse};
use log::warn;
use omicron_crawler::driver::session_manager::{SessionManager, SessionPool};
use omicron_crawler::errors::CrawlerError;
use omicron_crawler::linkedin::crawler::Crawler;
use std::cmp::min;
use std::thread;

#[derive(serde::Deserialize, Debug)]
pub struct Search {
    keywords: Option<String>,
    function: Option<String>,
    job_title: Option<String>,
    geography: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Url {
    sales_url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Message {
    sales_url: String,
    subject: String,
    body: String,
}

// TODO Chunk it like with search and use parallelism
// TODO Shouldn't fail if one fails
#[post("/message")]
pub async fn message(message: Json<Vec<Message>>) -> HttpResponse {
    let driver_session_manager = get_driver_session_manager().await;
    let pool = &driver_session_manager.pool;
    let session = pool.acquire();
    let message_data = message.into_inner();
    let crawler = match session {
        Some(session) => Crawler::new(session).await,
        None => {
            return HttpResponse::ServiceUnavailable().body("No free crawlers available, try again later");
        }
    };

    for message in message_data {
        let result = crawler
            .send_message(message.sales_url.as_str(), message.subject.as_str(), message.body.as_str())
            .await;
        if let Err(e) = result {
            return HttpResponse::InternalServerError().body(format!("Failed to send mail {}", e));
        }
    }
    HttpResponse::Ok().body("")
}

#[post("/search")]
pub async fn search(search_params: Json<Search>) -> HttpResponse {
    let driver_session_manager = get_driver_session_manager().await;
    let pool = &driver_session_manager.pool;
    let search_request = search_params.into_inner();
    let session = pool.acquire();
    let crawler = match session {
        Some(session) => Crawler::new(session).await,
        None => {
            return HttpResponse::ServiceUnavailable().body("No free crawlers available, try again later");
        }
    };

    match crawler
        .set_search_filters(
            search_request.keywords,
            search_request.function,
            search_request.job_title,
            search_request.geography,
        )
        .await
    {
        Ok(results) => results,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to perform search {}", e)),
    };
    let results = match crawler.parse_search().await {
        Ok(results) => results,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to parse search results {}", e)),
    };
    HttpResponse::Ok().json(results)
}

#[post("/profiles")]
pub async fn profiles(url_requests: Json<Vec<Url>>) -> HttpResponse {
    let url_request = url_requests.into_inner();
    let parsed_profiles = thread::scope(|s| {
        let mut response_profiles = Vec::new();
        let chunk_size = 2;
        let mut offset = 0;
        let end = url_request.len();
        let mut tasks = Vec::with_capacity(chunk_size);
        while offset < end {
            let current_iter_end = min(offset + chunk_size, end);
            for i in offset..current_iter_end {
                let url = &url_request[i];
                let rt = tokio::runtime::Runtime::new().unwrap();
                tasks.push(s.spawn(move || {
                    rt.block_on(async move {
                        let driver_session_manager = get_driver_session_manager().await;
                        let pool = &driver_session_manager.pool;
                        let session = pool.acquire();
                        let crawler = match session {
                            Some(session) => Crawler::new(session).await,
                            None => {
                                return Err(CrawlerError::DriverError("No free crawlers available, try again later".to_string()));
                            }
                        };
                        let result = crawler.parse_profile(url.sales_url.as_str()).await;
                        result
                    })
                }));
            }

            while tasks.len() > 0 {
                let task = tasks.pop().unwrap();
                let result = task.join().unwrap();
                match result {
                    Ok(result) => response_profiles.push(result),
                    Err(e) => {
                        warn!("{}", e);
                    }
                }
            }
            offset = current_iter_end;
        }
        response_profiles
    });

    HttpResponse::Ok().json(parsed_profiles)
}
