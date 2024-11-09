use crate::driver::driver_service::{ChromeDriverService, GeckoDriverService};
use crate::driver::driver_session::DriverSession;
use crate::driver::traits::{BrowserConfig, DriverService, SessionInitializer};

pub struct ChromeSessionInitializer;
impl SessionInitializer for ChromeSessionInitializer {
    type Service = ChromeDriverService;
    async fn create_sessions(
        host: &str,
        port: u16,
        param: <Self::Service as DriverService>::Param,
        session_count: u16,
        binary_path: Option<&str>,
    ) -> Vec<DriverSession> {
        let params = param;
        let mut results = Vec::with_capacity(session_count as usize);
        let port = port.to_string();
        for i in params.iter() {
            let driver =
                DriverSession::new::<<Self::Service as DriverService>::BrowserConfigType>(host, port.as_str(), i.as_str(), binary_path)
                    .await;
            results.push(driver);
        }
        results
    }
}

pub struct FirefoxSessionInitializer;
impl SessionInitializer for FirefoxSessionInitializer {
    type Service = GeckoDriverService;
    async fn create_sessions(
        host: &str,
        port: u16,
        param: <Self::Service as DriverService>::Param,
        session_count: u16,
        binary_path: Option<&str>,
    ) -> Vec<DriverSession> {
        let (base64_profile, ports) = param;
        let mut results = Vec::with_capacity(session_count as usize);
        let port = port.to_string();
        for i in ports {
            let driver = DriverSession::new::<<Self::Service as DriverService>::BrowserConfigType>(
                host,
                port.as_str(),
                base64_profile.as_str(),
                binary_path,
            )
            .await;
            results.push(driver);
        }
        results
    }
}
