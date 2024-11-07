use crate::driver::driver_session::DriverSession;
use crate::utils::{driver_host_from_env, driver_port_from_env, driver_session_count_from_env};
use crossbeam::queue::ArrayQueue;
use crossbeam::thread;
use fs_extra::dir::CopyOptions;
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::AtomicU16;
use std::sync::{Arc, Condvar, Mutex, Weak};
use tokio::sync::{OnceCell, Semaphore};

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
    pub async fn new(host: &str, port: &str, session_count: u16) -> Self {
        let session_pool = DriverSessionPool {
            sessions_available_signal: Condvar::new(),
            sessions_available_signal_lock: Mutex::new(()),
            available_sessions: ArrayQueue::new(session_count as usize),
        };

        let session_dirs = create_sessions_dirs(session_count);
        let zipped_iter = session_dirs.into_iter().zip(0..session_count);

        for (session_dir, _) in zipped_iter.into_iter() {
            let driver_session = DriverSession::new(host, port, session_dir).await;
            fatal_unwrap__!(
                session_pool.available_sessions.push(driver_session),
                "Failed to add session to pool"
            );
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
            fatal_unwrap_e!(session.quit().await, "Failed to quit the WebDriver: {}");
        }
    }
}

pub fn create_sessions_dirs(session_count: u16) -> Vec<PathBuf> {
    let mut work_dir = current_dir().unwrap();
    work_dir.push("../../user_data");
    let user_dir = work_dir.clone();
    let mut session_dir = current_dir().unwrap();
    session_dir.push("../../sessions");
    if !session_dir.exists() {
        fatal_unwrap_e!(fs::create_dir_all(session_dir.clone()), "Failed to create user directory {}");
    }

    let mut session_folders = Vec::with_capacity(session_count as usize);
    let existing_session_folders = fatal_unwrap_e!(fs::read_dir(session_dir.clone()), "Failed to read user directory {}");
    let mut folder_count: u16 = 0;
    for dir in existing_session_folders.filter_map(Result::ok) {
        folder_count += 1;
        session_folders.push(dir.path());
    }
    if folder_count >= session_count {
        info!("Found enough session folders to reuse.");
        return session_folders;
    }

    // OPTIMIZE use async as this is an IO bound operation
    let result = thread::scope(|s| {
        let mut join_handles = Vec::with_capacity(folder_count as usize);
        for i in folder_count..session_count {
            let mut target_dir = session_dir.clone();
            let user_dir_ref = &user_dir;
            join_handles.push(s.spawn(move |_| {
                target_dir.push(i.to_string());
                let copy_options = CopyOptions {
                    copy_inside: true,
                    ..Default::default()
                };
                info!(
                    "Copying user directory {} to {}",
                    user_dir_ref.to_str().unwrap(),
                    target_dir.to_str().unwrap()
                );
                fatal_unwrap_e!(
                    fs_extra::dir::copy(user_dir_ref.clone(), target_dir.clone(), &copy_options),
                    "Failed to copy user directory {}"
                );
                return target_dir;
            }));
        }
        for handle in join_handles {
            session_folders.push(handle.join().unwrap());
        }
    });
    fatal_unwrap__!(result, "Failed to create session folders");
    session_folders
}

static DRIVER_POOL: OnceCell<DriverSessionPool> = OnceCell::const_new();

pub async fn driver_session_pool() -> &'static DriverSessionPool {
    DRIVER_POOL
        .get_or_init(|| async {
            let host = driver_host_from_env();
            let port = driver_port_from_env();
            let count = driver_session_count_from_env();
            DriverSessionPool::new(host.as_str(), port.as_str(), count).await
        })
        .await
}
