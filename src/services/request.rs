use crate::get_crawler;
use actix_web::web::Json;
use actix_web::{post, HttpResponse};
use log::error;
use omicron_crawler::azure::json::{CrawledProfiles, ProfileIds};
use omicron_crawler::linkedin::api::json::SearchParams;

#[post("/search")]
pub async fn search(search_params: Json<SearchParams>) -> HttpResponse {
    let crawler = get_crawler().await;
    let search_params = search_params.into_inner();
    let results = match crawler.search_people(search_params).await {
        Ok(result) => result,
        Err(e) => {
            error!("Failed to search people {}", e);
            return HttpResponse::InternalServerError().body(format!("Failed to perform search {}", e));
        }
    };
    HttpResponse::Ok().json(results)
}

#[post("/profiles")]
pub async fn profiles(url_requests: Json<ProfileIds>) -> HttpResponse {
    let crawler = get_crawler().await;
    let mut profiles_response = url_requests.into_inner();
    let profiles = match crawler.profiles(profiles_response.ids.as_slice(), None).await {
        Ok(profiles) => profiles,
        Err(e) => {
            error!("Failed to parse profiles {}", e);
            return HttpResponse::InternalServerError().body(format!("Failed to perform profiles {}", e));
        }
    };

    let crawled_profiles = CrawledProfiles {
        profiles,
        request_metadata: profiles_response.request_metadata.take(),
    };
    HttpResponse::Ok().json(crawled_profiles)
}
