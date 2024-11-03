use actix_web::{post, web, HttpResponse};
use omicron_crawler::linkedin::enums::Functions;

#[derive(serde::Deserialize, Debug)]
pub struct Search {
    function: Functions,
    job_title: String,
    geography: Option<String>,
}

#[post("/search")]
pub async fn search(search_params: web::Json<Search>) -> HttpResponse {
    let search_request = search_params.into_inner();
    HttpResponse::Ok().body("Performing search...")
}
