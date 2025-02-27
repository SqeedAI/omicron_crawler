use crate::errors::{IoError, IoResult};
use crate::utils::{load_file_as_str, save_to_file};
use reqwest::cookie::CookieStore as CookieStoreTrait;
use reqwest::Url;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use std::sync::Arc;

pub fn new_cookie_jar() -> Arc<CookieStoreMutex> {
    let cookies = CookieStore::new(None);
    let cookie_store_mutex = CookieStoreMutex::new(cookies);
    Arc::new(cookie_store_mutex)
}

pub fn cookie_save(cookie_store: &Arc<CookieStoreMutex>, url: &Url, file_path: &str) -> IoResult<()> {
    let cookies = match cookie_store.cookies(url) {
        Some(cookies) => cookies,
        None => {
            return Err(IoError::FileError(format!("Failed to get cookies. No cookie for url {}", url)));
        }
    };
    let bytes = cookies.as_bytes();
    save_to_file(bytes, file_path)
}

pub fn cookie_load(cookie_store: Arc<CookieStoreMutex>, url: &Url, file_path: &str) -> IoResult<()> {
    let cookie_str = load_file_as_str(file_path)?;
    let mut store = cookie_store.lock().unwrap();
    let cookie_list = cookie_str.split(";").collect::<Vec<&str>>();
    for cookie in cookie_list {
        store.parse(cookie, url).map_err(|e| IoError::ParseError(e.to_string()))?;
    }
    Ok(())
}
