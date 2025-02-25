pub mod services;

use omicron_crawler::env::{get_env, load_env};
use omicron_crawler::linkedin::api::crawler::LinkedinSessionManager;
use tokio::sync::OnceCell;
static CRAWLER: OnceCell<LinkedinSessionManager> = OnceCell::const_new();

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    load_env();
    let env = get_env().await;
    Ok(())
}
