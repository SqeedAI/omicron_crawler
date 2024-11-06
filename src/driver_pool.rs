use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Weak};
use crossbeam::queue::ArrayQueue;
use fs_extra::dir::CopyOptions;
use uuid::Uuid;
use crate::driver_session::DriverSession;

struct SessionProxy<'a> {
    driver_session_pool: &'a ArrayQueue<DriverSession>,
    session: Option<DriverSession>,
}

impl<'a> SessionProxy<'a> {
    pub fn new(session: DriverSession, driver_session_pool: &ArrayQueue<DriverSession>) -> Self {
        Self { session:Some(session), driver_session_pool }
    }
}


impl<'a> Drop for SessionProxy<'a> {
    fn drop(&mut self) {
        let session = self.session.take().unwrap();
        fatal_unwrap_e!(self.driver_session_pool.push(session), "failed to push: {}");
    }
}

pub struct DriverSessionPool {
    available_sessions: ArrayQueue<DriverSession>,
}

impl DriverSessionPool {
    pub async fn new(host: &str, port: &str, session_count: u16) -> Self {
        let session_pool = DriverSessionPool {
            available_sessions: ArrayQueue::new(session_count as usize),
        };

        let session_dirs = create_sessions_dirs(session_count);

        for session_dir in session_dirs {
            let driver_session = DriverSession::new(host, port, session_dir).await;
            fatal_unwrap_e!(session_pool.available_sessions.push(driver_session), "Failed to add session to pool: {}");
        }
        session_pool
    }
    pub fn session(&mut self) -> Option<SessionProxy> {
        match self.available_sessions.pop() {
            Some(session) => Some(SessionProxy::new(session, &self.available_sessions)),
            None => None,
        }
    }
}

pub fn create_sessions_dirs(session_count: u16) -> Vec<PathBuf> {
    let user_dir = current_dir().unwrap();
    let mut session_dir = current_dir().unwrap();
    session_dir.push("sessions");
    if !session_dir.exists() {
        fatal_unwrap_e!(fs::create_dir_all(session_dir.clone()), "Failed to create user directory {}");

    }

    let mut session_folders = Vec::with_capacity(session_count as usize);
    let existing_session_folders = fatal_unwrap_e!(fs::read_dir(session_dir.clone()), "Failed to read user directory {}");
    let mut folder_count: u16 = 0;
    for Ok(dir) in existing_session_folders {
        folder_count += 1;
        session_folders.push(dir.path());
    }

    for i in folder_count..session_count {
        let mut target_dir = session_dir.clone();
        target_dir.push(i.to_string());
        let copy_options = CopyOptions::default();
        info!("Copying user directory {} to {}", user_dir.to_str().unwrap(), target_dir.to_str().unwrap());
        fatal_unwrap_e!(fs_extra::dir::copy(user_dir.clone(), target_dir.clone(), &copy_options), "Failed to copy user directory {}");
        session_dir.push(target_dir);
    }
    session_folders
}