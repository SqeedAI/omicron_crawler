use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::time::Duration;

pub struct RateLimits {
    profiles_per_hour: u32,
    waits: Vec<u64>,
    current: *const u64,
    end: *const u64,
}

impl RateLimits {
    pub const RESPONSE_TIME_MS: u32 = 428;
    pub fn new(profiles_per_hour: u32) -> Self {
        let waits = Self::generate_random_waits(profiles_per_hour);
        let current = waits.as_ptr();
        let end = unsafe { current.add(waits.len()) };
        Self {
            profiles_per_hour,
            waits,
            current,
            end,
        }
    }

    pub fn generate_random_waits(profiles_per_hour: u32) -> Vec<u64> {
        if profiles_per_hour == 0 {
            return vec![0];
        }

        let total_request_time_sec = (profiles_per_hour * Self::RESPONSE_TIME_MS) as f32 / 1000f32;
        let total_wait_time = 3600f32 - total_request_time_sec;
        let set_max = (((1f32 + 8f32 * total_wait_time).sqrt() - 1f32) / 2f32) as usize;
        // Create sequential array from 0 to set_max
        let mut waits: Vec<u64> = (0..=set_max).map(|x| x as u64).collect();
        waits.shuffle(&mut thread_rng());
        waits
    }
}

impl Iterator for RateLimits {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = unsafe { self.current.add(1) };
        if self.current >= self.end {
            self.current = self.waits.as_ptr();
        }
        unsafe { Some(Duration::from_secs(*self.current)) }
    }
}
