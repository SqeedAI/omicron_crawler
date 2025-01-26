use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::time::Duration;

pub struct RateLimiter {
    profiles_per_hour: u32,
    avg_response_time_ms: u32,
    waits: Vec<u64>,
    current: AtomicUsize,
    end: usize,
}

impl RateLimiter {
    pub fn new(profiles_per_hour: u32, avg_response_time_ms: u32) -> Self {
        let mut waits = Self::generate_random_waits(profiles_per_hour, avg_response_time_ms);
        let current = AtomicUsize::new(0);
        let end = waits.len();
        Self {
            profiles_per_hour,
            waits,
            current,
            end,
            avg_response_time_ms,
        }
    }

    pub fn generate_random_waits(profiles_per_hour: u32, avg_response_time_ms: u32) -> Vec<u64> {
        if profiles_per_hour == 0 {
            return vec![0];
        }

        let total_request_time_sec = (profiles_per_hour * avg_response_time_ms) as f32 / 1000f32;
        let total_wait_time = 3600f32 - total_request_time_sec;
        let set_max = (((1f32 + 8f32 * total_wait_time).sqrt() - 1f32) / 2f32) as usize;
        // Create sequential array from 0 to set_max
        let mut waits: Vec<u64> = (0..=set_max).map(|x| x as u64).collect();
        waits.shuffle(&mut thread_rng());
        waits
    }

    pub fn next(&self) -> Option<Duration> {
        let end = self.end;
        let current_local = match self.current.fetch_update(Release, Acquire, |current| {
            let new = current + 1;
            if new >= end {
                Some(0)
            } else {
                Some(new)
            }
        }) {
            Ok(current) => current,
            Err(_) => return None,
        };
        let wait_time = unsafe { self.waits.get_unchecked(current_local) };

        unsafe { Some(Duration::from_secs(*wait_time)) }
    }
}
