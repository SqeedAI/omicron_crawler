use omicron_crawler::linkedin::enums::Functions;

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

// #[get("/search")]
// pub async fn search(search_params: Json<Search>) -> HttpResponse {
//     let search_request = search_params.into_inner();
//     let session = driver_session_pool().await.acquire();
//     let crawler = match session {
//         Some(session) => Crawler::new(session).await,
//         None => {
//             return HttpResponse::ServiceUnavailable().body("No free crawlers available, try again later");
//         }
//     };
//
//     match crawler
//         .set_search_filters(search_request.function, search_request.job_title, search_request.geography)
//         .await
//     {
//         Ok(results) => results,
//         Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to perform search {}", e)),
//     };
//     let results = match crawler.parse_search().await {
//         Ok(results) => results,
//         Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to parse search results {}", e)),
//     };
//     HttpResponse::Ok().json(results)
// }
//
// #[get("/profiles")]
// pub async fn profiles(url_requests: Json<Vec<Url>>) -> HttpResponse {
//     let url_request = url_requests.into_inner();
//     let parsed_profiles = thread::scope(|s| {
//         let mut response_profiles = Vec::new();
//         let chunk_size = 2;
//         let mut offset = 0;
//         let end = url_request.len();
//         let mut tasks = Vec::with_capacity(chunk_size);
//         while offset < end {
//             let current_iter_end = min(offset + chunk_size, end);
//             for i in offset..current_iter_end {
//                 let url = &url_request[i];
//                 let rt = tokio::runtime::Runtime::new().unwrap();
//                 tasks.push(s.spawn(move || {
//                     rt.block_on(async move {
//                         let session = driver_session_pool().await.acquire();
//                         let crawler = match session {
//                             Some(session) => Crawler::new(session).await,
//                             None => {
//                                 return Err(CrawlerError::DriverError("No free crawlers available, try again later".to_string()));
//                             }
//                         };
//                         let result = crawler.parse_profile(url.sales_url.as_str()).await;
//                         result
//                     })
//                 }));
//             }
//
//             while tasks.len() > 0 {
//                 let task = tasks.pop().unwrap();
//                 let result = task.join().unwrap();
//                 match result {
//                     Ok(result) => response_profiles.push(result),
//                     Err(e) => {
//                         warn!("{}", e);
//                     }
//                 }
//             }
//             offset = current_iter_end;
//         }
//         response_profiles
//     });
//
//     HttpResponse::Ok().json(parsed_profiles)
// }
