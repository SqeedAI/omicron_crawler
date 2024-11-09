use crate::driver::driver_session::DriverSession;
use crate::driver::traits::{BrowserConfig, DriverService, SessionInitializer};
use crate::env::get_env;
use crossbeam::queue::ArrayQueue;
use crossbeam::thread;
use fs_extra::dir::CopyOptions;
use std::env::current_dir;
use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::atomic::AtomicU16;
use std::sync::{Arc, Condvar, Mutex, Weak};
use thirtyfour::{ChromeCapabilities, FirefoxCapabilities};
use tokio::sync::OnceCell;

pub struct DriverSessionProxy<'a, ServiceType>
where
    ServiceType: DriverService,
{
    driver_session_pool: &'a DriverSessionManager<ServiceType>,
    pub session: Option<DriverSession>,
}

impl<'a, ServiceType> DriverSessionProxy<'a, ServiceType>
where
    ServiceType: DriverService,
{
    pub fn new(session: DriverSession, driver_session_pool: &'a DriverSessionManager<ServiceType>) -> Self {
        Self {
            session: Some(session),
            driver_session_pool,
        }
    }
}

impl<'a, ServiceType> Drop for DriverSessionProxy<'a, ServiceType>
where
    ServiceType: DriverService,
{
    fn drop(&mut self) {
        self.driver_session_pool.release(self);
    }
}

pub struct DriverSessionManager<ServiceType>
where
    ServiceType: DriverService,
{
    sessions_available_signal: Condvar,
    sessions_available_signal_lock: Mutex<()>,
    driver_service: ServiceType,
    available_sessions: ArrayQueue<DriverSession>,
}

impl<ServiceType> DriverSessionManager<ServiceType>
where
    ServiceType: DriverService,
{
    pub async fn new(
        driver_host: &str,
        driver_port: u16,
        session_count: u16,
        driver_path: &str,
        profile_path: &str,
        binary_path: Option<&str>,
    ) -> Self {
        let service = ServiceType::new(driver_port, session_count, driver_path, profile_path).await;
        let params = service.session_params().await;
        let sessions =
            ServiceType::SessionInitializerType::create_sessions(driver_host, driver_port, params, session_count, binary_path).await;
        let queue = ArrayQueue::new(session_count as usize);
        for session in sessions {
            fatal_unwrap__!(queue.push(session), "Failed to push session");
        }

        let session_pool = DriverSessionManager {
            sessions_available_signal: Condvar::new(),
            sessions_available_signal_lock: Mutex::new(()),
            driver_service: service,
            available_sessions: queue,
        };
        session_pool
    }
    pub fn acquire(&self) -> Option<DriverSessionProxy<ServiceType>> {
        match self.available_sessions.pop() {
            Some(session) => Some({
                info!("Acquiring session, {} available", self.available_sessions.len());
                DriverSessionProxy::new(session, &self)
            }),
            None => None,
        }
    }

    pub async fn wait_for_all_sessions_to_be_released(&self) {
        let signal_lock = self.sessions_available_signal_lock.lock().unwrap();
        let _guard = self
            .sessions_available_signal
            .wait_while(signal_lock, |_| self.available_sessions.len() != self.available_sessions.capacity());
    }

    pub fn release(&self, session: &mut DriverSessionProxy<ServiceType>) {
        fatal_unwrap__!(self.available_sessions.push(session.session.take().unwrap()), "failed to push");
        if self.available_sessions.len() == self.available_sessions.capacity() {
            self.sessions_available_signal.notify_all();
        }
        info!("Releasing session, {} available", self.available_sessions.len());
    }

    pub async fn quit(&self) {
        self.wait_for_all_sessions_to_be_released().await;
        while let Some(session) = self.available_sessions.pop() {
            match session.quit().await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Failed to quit the WebDriver: {}", e);
                }
            }
        }
    }
}
