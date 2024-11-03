pub mod request;

use actix_web::get;
use omicron_crawler::linkedin::crawler::Crawler;
use tokio::sync::OnceCell;

static DRIVER: OnceCell<Crawler> = OnceCell::const_new();

async fn get_crawler() -> &'static Crawler {
    DRIVER.get_or_init(|| async { Crawler::new("8888".to_string()).await }).await
}
#[get("/")]
pub async fn hello() -> &'static str {
    "Hello World!"
}
