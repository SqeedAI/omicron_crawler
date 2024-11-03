pub mod request;

use actix_web::get;

#[get("/")]
pub async fn hello() -> &'static str {
    "Hello World!"
}
