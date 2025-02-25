use std::sync::atomic::AtomicBool;
static SHUTDOWN_SIGNAL: AtomicBool = AtomicBool::new(false);
#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    Ok(())
}
