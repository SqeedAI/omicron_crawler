use thirtyfour::{DesiredCapabilities, WebDriver};

async fn launch() {
    let capabilities = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", capabilities).await.expect("Failed to create driver");
    driver.goto("https://www.google.com/").await.expect("Failed to go to google");
}


#[tokio::main]
async fn main() {
    launch().await;
}


