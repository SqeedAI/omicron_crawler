pub mod traits;
use crossbeam::queue::ArrayQueue;
use crossbeam::thread;
use std::sync::{Condvar, Mutex};
use tokio::runtime::Builder;

pub struct SessionProxy<'a, SessionType>
where
    SessionType: Send,
{
    driver_session_pool: &'a SessionPool<SessionType>,
    pub session: Option<SessionType>,
}

impl<'a, SessionType> SessionProxy<'a, SessionType>
where
    SessionType: Send,
{
    pub fn new(session: SessionType, driver_session_pool: &'a SessionPool<SessionType>) -> Self {
        Self {
            session: Some(session),
            driver_session_pool,
        }
    }
}

impl<'a, SessionType> Drop for SessionProxy<'a, SessionType>
where
    SessionType: Send,
{
    fn drop(&mut self) {
        self.driver_session_pool.release(self);
    }
}

pub struct SessionPool<SessionType>
where
    SessionType: Send,
{
    sessions_available_signal: Condvar,
    sessions_available_signal_lock: Mutex<()>,
    available_sessions: ArrayQueue<SessionType>,
}
impl<SessionType> SessionPool<SessionType>
where
    SessionType: Send,
{
    pub fn new(sessions: Vec<SessionType>) -> Self {
        let available_sessions = ArrayQueue::new(sessions.len());
        for session in sessions {
            fatal_unwrap__!(available_sessions.push(session), "Failed to push session");
        }
        let session_pool = SessionPool {
            sessions_available_signal: Condvar::new(),
            sessions_available_signal_lock: Mutex::new(()),
            available_sessions,
        };
        session_pool
    }
    pub fn acquire(&self) -> Option<SessionProxy<SessionType>> {
        match self.available_sessions.pop() {
            Some(session) => Some({
                info!("Acquiring session, {} available", self.available_sessions.len());
                SessionProxy::new(session, &self)
            }),
            None => None,
        }
    }

    pub fn wait_for_all_sessions_to_be_released(&self) {
        let signal_lock = self.sessions_available_signal_lock.lock().unwrap();
        let _guard = self
            .sessions_available_signal
            .wait_while(signal_lock, |_| self.available_sessions.len() != self.available_sessions.capacity());
    }

    pub fn release(&self, session: &mut SessionProxy<SessionType>) {
        fatal_unwrap__!(self.available_sessions.push(session.session.take().unwrap()), "failed to push");
        if self.available_sessions.len() == self.available_sessions.capacity() {
            self.sessions_available_signal.notify_all();
        }
        info!("Releasing session, {} available", self.available_sessions.len());
    }
}

impl<SessionType> Drop for SessionPool<SessionType>
where
    SessionType: Send,
{
    fn drop(&mut self) {
        self.wait_for_all_sessions_to_be_released();
        let sessions = &self.available_sessions;
        thread::scope(|s| {
            s.spawn(|_| {
                let runtime = Builder::new_current_thread().enable_all().build().unwrap();
                runtime.block_on(async move { while let Some(_) = sessions.pop() {} });
            });
        })
        .unwrap();
    }
}
