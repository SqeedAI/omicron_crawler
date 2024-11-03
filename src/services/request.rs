use actix_web::rt::task;
use actix_web::web::Json;
use actix_web::{get, post, HttpResponse};
use log::warn;
use omicron_crawler::driver_service::driver_service;
use omicron_crawler::linkedin::crawler::Crawler;
use omicron_crawler::linkedin::enums::Functions;
use omicron_crawler::utils::{driver_host_from_env, driver_port_from_env};

#[derive(serde::Deserialize, Debug)]
pub struct Search {
    function: Functions,
    job_title: String,
    geography: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Url {
    sales_url: String,
}

#[get("/search")]
pub async fn search(search_params: Json<Search>) -> HttpResponse {
    driver_service().await;
    let search_request = search_params.into_inner();
    let host = driver_host_from_env();
    let port = driver_port_from_env();
    let crawler = Crawler::new(host, port).await;

    match crawler
        .set_search_filters(search_request.function, search_request.job_title, search_request.geography)
        .await
    {
        Ok(results) => results,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to perform search {}", e)),
    };
    let results = match crawler.parse_search().await {
        Ok(results) => results,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to parse search results {}", e)),
    };
    crawler.quit().await;
    HttpResponse::Ok().json(results)
}

#[get("/profiles")]
pub async fn profiles(url_requests: Json<Vec<Url>>) -> HttpResponse {
    driver_service().await;
    let url_request = url_requests.into_inner();
    let mut tasks = Vec::with_capacity(url_request.len());
    let mut response_profiles = Vec::new();

    // Create tasks for parallel execution
    for url in url_request {
        let sales_url = url.sales_url.clone();
        tasks.push(task::spawn_blocking(move || async move {
            let host = driver_host_from_env();
            let port = driver_port_from_env();
            let crawler = Crawler::new(host, port).await;
            let result = crawler.parse_profile(sales_url.as_str()).await;
            crawler.quit().await;
            result
        }));
    }

    for task in tasks.into_iter() {
        let url = task.await.unwrap().await;
        match url {
            Ok(url) => response_profiles.push(url),
            Err(e) => {
                warn!("{}", e);
            }
        }
    }

    HttpResponse::Ok().json(response_profiles)
}
