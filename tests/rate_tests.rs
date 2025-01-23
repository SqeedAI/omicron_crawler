use omicron_crawler::linkedin::api::rate_limits::RateLimits;

#[test]
fn test_rate_limits1() {
    let profiles_per_hour = 100;
    let total_request_time_sec = (profiles_per_hour * RateLimits::RESPONSE_TIME_MS) as f32 / 1000f32;
    let total_wait_time = 3600f32 - total_request_time_sec;
    let waits = RateLimits::generate_random_waits(profiles_per_hour);
    let mut total = 0;
    for i in waits {
        total += i;
    }
    let accuracy = total as f32 / total_wait_time;
    assert!(accuracy > 0.9);
    println!("accuracy, total: {}, {}", accuracy, total)
}

#[test]
fn test_rate_limits2() {
    let profiles_per_hour = 400;
    let total_request_time_sec = (profiles_per_hour * RateLimits::RESPONSE_TIME_MS) as f32 / 1000f32;
    let total_wait_time = 3600f32 - total_request_time_sec;
    let waits = RateLimits::generate_random_waits(profiles_per_hour);
    let mut total = 0;
    for i in waits {
        total += i;
    }
    let accuracy = total as f32 / total_wait_time;
    assert!(accuracy > 0.9);
    println!("accuracy, total: {}, {}", accuracy, total)
}

#[test]
fn test_rate_limits3() {
    let profiles_per_hour = 100;
    let rate_limits = RateLimits::new(profiles_per_hour);
    let mut count = 100;
    for i in rate_limits {
        if count == 0 {
            break;
        }
        println!("{}", i.as_secs());
        count -= 1;
    }
}

#[test]
fn test_rate_limits4() {
    let profiles_per_hour = 0;
    let rate_limits = RateLimits::new(profiles_per_hour);
    let mut count = 100;
    for i in rate_limits {
        if count == 0 {
            break;
        }
        println!("{}", i.as_secs());
        count -= 1;
    }
}
