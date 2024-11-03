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
    // let search_request = search_params.into_inner();
    // let crawler = get_crawler().await;
    // match crawler
    //     .set_search_filters(search_request.function, search_request.job_title, search_request.geography)
    //     .await
    // {
    //     Ok(results) => results,
    //     Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to perform search {}", e)),
    // };
    // let results = match crawler.parse_search().await {
    //     Ok(results) => results,
    //     Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to parse search results {}", e)),
    // };
    // let first = results.first().unwrap();
    // let profile = match crawler.parse_profile(&first.sales_url).await {
    //     Ok(profile) => profile,
    //     Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to parse profile {}", e)),
    // };
    // println!("{}", profile);
    HttpResponse::Ok().body("Performing search...") // TODO
}
