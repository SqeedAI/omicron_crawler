use crate::driver::driver_service::{ChromeDriverService, GeckoDriverService};
use crate::driver::driver_session::DriverSession;
use crate::driver::traits::{BrowserConfig, DriverService, SessionInitializer};

struct ChromeSessionInitializer;
impl SessionInitializer for ChromeSessionInitializer {
    type Service = ChromeDriverService;
    async fn create_sessions(
        host: &str,
        port: &str,
        param: Self::Service::Param,
        session_count: u16,
        binary_path: Option<&str>,
    ) -> Vec<DriverSession> {
        let params = param;
        let mut results = Vec::with_capacity(session_count as usize);
        for i in params.iter() {
            let driver = DriverSession::new::<Self::Service::Capabilities>(host, port, i, binary_path).await;
            results.push(driver);
        }
        results
    }
}

struct FirefoxSessionInitializer;

impl SessionInitializer for FirefoxSessionInitializer {
    type Service = GeckoDriverService;
    async fn create_sessions(
        host: &str,
        port: &str,
        param: Self::Service::Param,
        session_count: u16,
        binary_path: Option<&str>,
    ) -> Vec<DriverSession> {
        let (base64_profile, ports) = param;
        let mut results = Vec::with_capacity(session_count as usize);
        for i in ports {
            let driver = DriverSession::new::<Self::Service::Capabilities>(host, port, base64_profile, binary_path).await;
            results.push(driver);
        }
        results
    }
}
