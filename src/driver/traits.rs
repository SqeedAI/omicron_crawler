use crate::driver::driver_session::DriverSession;

pub trait BrowserConfig {
    type Capabilities: Into<thirtyfour::Capabilities>;
    fn new(profile_path: &str, binary_path: Option<&str>) -> Self::Capabilities;
}

pub trait DriverService {
    type BrowserConfigType: BrowserConfig;
    type Param;
    type SessionInitializerType: SessionInitializer<Service = Self>;
    async fn new(port: u16, session_count: u16, driver_path: &str, profile_path: &str) -> Self;
    async fn session_params(&self) -> Self::Param;
}

pub trait SessionInitializer {
    type Service: DriverService;
    async fn create_sessions(
        host: &str,
        port: u16,
        param: <Self::Service as DriverService>::Param,
        session_count: u16,
        binary_path: Option<&str>,
    ) -> Vec<DriverSession>;
}
