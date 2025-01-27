use crate::errors::CrawlerError::FileError;
use crate::errors::CrawlerResult;
use regex::Regex;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

pub fn load_cookies(path: &str) -> Option<String> {
    let mut file = match fs::File::open(path) {
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

pub fn save_cookies(cookies: &[u8], path: &str) -> CrawlerResult<()> {
    let path_sys = Path::new(path);

    if let Some(parent) = path_sys.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return Err(FileError(format!("Failed to create directory: {}", e)));
        }
    }

    let mut file = match fs::File::create(path_sys) {
        Ok(file) => file,
        Err(e) => {
            return Err(FileError(format!("Failed to open cookies file {}", e)));
        }
    };
    if let Err(e) = file.write_all(cookies) {
        return Err(FileError(format!("Failed to write cookies file {}", e)));
    }
    info!("Cookies saved to {}", path);
    Ok(())
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
