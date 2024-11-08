use crate::driver::driver_session::DriverSession;
use crate::driver::traits::{BrowserConfig, DriverService};
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

pub struct DriverSessionProxy<'a> {
    driver_session_pool: &'a DriverSessionManager,
    pub session: Option<DriverSession>,
}

impl<'a> DriverSessionProxy<'a> {
    pub fn new(session: DriverSession, driver_session_pool: &'a DriverSessionManager) -> Self {
        Self {
            session: Some(session),
            driver_session_pool,
        }
    }
}

impl<'a> Drop for DriverSessionProxy<'a> {
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
    pub async fn new(host: &str, port: &str, session_count: u16) -> Self {
        let service = ServiceType::new(
            port.to_string(),
            session_count,
            get_env().driver_path.as_str(),
            get_env().profile_path.as_str(),
        )
        .await;

        for (session_dir, _) in zipped_iter.into_iter() {
            let dir_path = session_dir.as_str();
            trace!("Creating session");
            let driver_session = DriverSession::new::<BrowserType>(host, port, dir_path).await;
            fatal_unwrap__!(
                session_pool.available_sessions.push(driver_session),
                "Failed to add session to pool"
            );
            trace!("Added session to pool");
        }

        let session_pool = DriverSessionManager {
            sessions_available_signal: Condvar::new(),
            sessions_available_signal_lock: Mutex::new(()),
            available_sessions: ArrayQueue::new(session_count as usize),
        };
        session_pool
    }
    pub fn acquire(&self) -> Option<DriverSessionProxy> {
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

    pub fn release(&self, session: &mut DriverSessionProxy) {
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
