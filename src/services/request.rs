use actix_web::web::Json;
use actix_web::{post, HttpResponse};
use omicron_crawler::azure::json::ProfileIds;
use omicron_crawler::linkedin::api::json::SearchParams;

#[post("/search")]
pub async fn search(search_params: Json<SearchParams>) -> HttpResponse {
    HttpResponse::Ok().body("")
}

#[post("/profiles")]
pub async fn profiles(url_requests: Json<ProfileIds>) -> HttpResponse {
    HttpResponse::Ok().body("")
}
