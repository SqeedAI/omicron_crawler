use regex::Regex;
use std::io::{Read, Write};

pub fn load_cookies() -> Option<String> {
    let mut file = match std::fs::File::open("cookie.dat") {
        Ok(file) => file,
        Err(_) => {
            info!("Failed to open cookies file");
            return None;
        }
    };
    let mut cookies = String::new();
    if let Err(error_code) = file.read_to_string(&mut cookies) {
        error!("Failed to read cookies file {}", error_code);
        return None;
    }
    Some(cookies)
}

pub fn save_cookies(cookies: &[u8]) {
    let mut file = match std::fs::File::create("cookie.dat") {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open cookies file {}", e);
            return;
        }
    };
    if let Err(e) = file.write_all(cookies) {
        error!("Failed to write cookies file {}", e);
        return;
    }
}

pub fn cookies_session_id(cookies: &str) -> Option<String> {
    let re = Regex::new(r#"JSESSIONID="(.*?)"(?:;|$)"#).unwrap();
    info!("Checking cookie {}", cookies);
    match re.captures(cookies) {
        Some(captures) => Some(captures.get(1).unwrap().as_str().to_string()),
        None => {
            error!("Failed to find JSESSIONID cookie");
            None
        }
    }
}
