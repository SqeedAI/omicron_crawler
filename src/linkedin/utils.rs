use crate::errors::CrawlerError::FileError;
use crate::errors::CrawlerResult;
use rand::random;
use regex::Regex;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

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

pub fn generate_jsessionid() -> String {
    // Generate a random positive 64-bit number
    let random_long: u64 = random();

    // Format to 19 digits with leading zeros
    let formatted_number = format!("{:019}", random_long);

    // Return in format "ajax:XXXXXXXXXXXXXXXXXXX"
    format!("ajax:{}", formatted_number)
}
