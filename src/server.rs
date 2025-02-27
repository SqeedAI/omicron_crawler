use omicron_crawler::env::{get_env, load_env};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    load_env();
    let env = get_env().await;
    Ok(())
}
