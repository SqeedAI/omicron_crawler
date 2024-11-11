use crate::driver::service::{ChromeDriverService, GeckoDriverService};
use crate::driver::session::DriverSession;
use crate::driver::traits::{BrowserConfig, DriverService, SessionInitializer};

pub struct ChromeSessionInitializer;
impl SessionInitializer for ChromeSessionInitializer {
    type Service = ChromeDriverService;
    async fn create_sessions<'a>(
        host: &str,
        port: u16,
        param: <Self::Service as DriverService>::Param<'a>,
        session_count: u16,
        browser_binary_path: Option<&str>,
    ) -> Vec<DriverSession> {
        let params = param;
        let mut results = Vec::with_capacity(session_count as usize);
        let port = port.to_string();
        for i in params.iter() {
            let driver = DriverSession::new::<<Self::Service as DriverService>::BrowserConfigType>(
                host,
                port.as_str(),
                i.as_str(),
                browser_binary_path,
            )
            .await;
            results.push(driver);
        }
        results
    }
}

pub struct GeckoSessionInitializer;
impl SessionInitializer for GeckoSessionInitializer {
    type Service = GeckoDriverService;
    async fn create_sessions<'a>(
        host: &str,
        port: u16,
        param: <Self::Service as DriverService>::Param<'a>,
        session_count: u16,
        browser_binary_path: Option<&str>,
    ) -> Vec<DriverSession> {
        let (base64_profile, ports) = param;
        let mut results = Vec::with_capacity(session_count as usize);
        for i in ports {
            let port_str = i.to_string();
            let driver = DriverSession::new::<<Self::Service as DriverService>::BrowserConfigType>(
                host,
                port_str.as_str(),
                base64_profile,
                browser_binary_path,
            )
            .await;
            results.push(driver);
        }
        results
    }
}
