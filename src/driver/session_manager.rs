use crate::driver::session::DriverSession;
use crate::driver::traits::{BrowserConfig, DriverService, SessionInitializer};
use crate::session_pool::SessionPool;

pub struct SessionManager<ServiceType>
where
    ServiceType: DriverService,
{
    pub pool: SessionPool<DriverSession>,
    driver_service: ServiceType,
}

impl<ServiceType> SessionManager<ServiceType>
where
    ServiceType: DriverService,
{
    pub async fn new(
        driver_host: &str,
        driver_port: u16,
        session_count: u16,
        driver_path: &str,
        profile_path: &str,
        browser_binary_path: Option<&str>,
    ) -> Self {
        let service = ServiceType::new(driver_port, session_count, driver_path, profile_path, browser_binary_path).await;
        let params = service.session_params().await;
        let sessions =
            ServiceType::SessionInitializerType::create_sessions(driver_host, driver_port, params, session_count, browser_binary_path)
                .await;
        let pool = SessionPool::new(sessions);
        let session_pool = SessionManager {
            pool,
            driver_service: service,
        };
        session_pool
    }
}
