pub fn generate_random_string(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

pub fn get_domain_url(url: &str) -> String {
    let indices: Vec<(usize, &str)> = url.match_indices("/").collect();
    url.split_at(indices[2].0).0.to_string()
}

pub fn log_level_from_env() -> log::LevelFilter {
    match std::env::var("LOG_LEVEL") {
        Ok(level) => match level.as_str() {
            "TRACE" => log::LevelFilter::Trace,
            "DEBUG" => log::LevelFilter::Debug,
            "INFO" => log::LevelFilter::Info,
            "WARN" => log::LevelFilter::Warn,
            "ERROR" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        },
        Err(_) => log::LevelFilter::Info,
    }
}

pub fn host_data_from_env() -> (String, u16) {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(8080);
    (host, port)
}

pub fn chrome_driver_path_from_env() -> String {
    std::env::var("CHROME_DRIVER_PATH").unwrap_or_else(|_| "./drivers/chromedriver.exe".to_string())
}

pub fn gecko_driver_path_from_env() -> String {
    std::env::var("GECKO_DRIVER_PATH").unwrap_or_else(|_| "./drivers/geckodriver.exe".to_string())
}

pub fn driver_host_from_env() -> String {
    std::env::var("DRIVER_HOST").unwrap_or_else(|_| "localhost".to_string())
}

pub fn driver_port_from_env() -> String {
    std::env::var("DRIVER_PORT").unwrap_or_else(|_| "9515".to_string())
}
pub fn driver_session_count_from_env() -> u16 {
    std::env::var("DRIVER_SESSION_COUNT")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<u16>()
        .unwrap()
}
