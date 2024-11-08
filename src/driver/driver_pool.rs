use crate::driver::driver_session::DriverSession;
use crate::driver::traits::BrowserConfig;
use crate::utils::{driver_host_from_env, driver_port_from_env, driver_session_count_from_env};
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
    driver_session_pool: &'a DriverSessionPool,
    pub session: Option<DriverSession>,
}

impl<'a> DriverSessionProxy<'a> {
    pub fn new(session: DriverSession, driver_session_pool: &'a DriverSessionPool) -> Self {
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

pub struct DriverSessionPool {
    sessions_available_signal: Condvar,
    sessions_available_signal_lock: Mutex<()>,
    available_sessions: ArrayQueue<DriverSession>,
}

impl DriverSessionPool {
    pub async fn new<T>(host: &str, port: &str, session_count: u16) -> Self
    where
        T: BrowserConfig,
    {
        let session_pool = DriverSessionPool {
            sessions_available_signal: Condvar::new(),
            sessions_available_signal_lock: Mutex::new(()),
            available_sessions: ArrayQueue::new(session_count as usize),
        };

        let session_dirs = T::create_session_dirs(session_count);
        let zipped_iter = session_dirs.into_iter().zip(0..session_count);

        for (session_dir, _) in zipped_iter.into_iter() {
            let dir_path = session_dir.as_str();
            trace!("Creating session");
            let driver_session = DriverSession::new::<T>(host, port, dir_path).await;
            fatal_unwrap__!(
                session_pool.available_sessions.push(driver_session),
                "Failed to add session to pool"
            );
            trace!("Added session to pool");
        }
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

static DRIVER_SESSION_POOL: OnceCell<DriverSessionPool> = OnceCell::const_new();
pub(super) async fn create_driver_session_pool<T>() -> &'static DriverSessionPool
where
    T: BrowserConfig,
{
    DRIVER_SESSION_POOL
        .get_or_init(|| async {
            let host = driver_host_from_env();
            let port = driver_port_from_env();
            let count = driver_session_count_from_env();
            DriverSessionPool::new::<T>(host.as_str(), port.as_str(), count).await
        })
        .await
}
pub fn get_driver_session_pool() -> &'static DriverSessionPool {
    fatal_unwrap!(DRIVER_SESSION_POOL.get(), "Driver session pool not initialized!")
}
