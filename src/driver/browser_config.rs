use crate::driver::traits::BrowserConfig;
use crate::utils::patch_binary_with_random;
use fs_extra::dir::CopyOptions;
use std::env::current_dir;
use std::fmt::format;
use std::path::PathBuf;
use std::{fs, thread};
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::support::base64_encode;
use thirtyfour::{BrowserCapabilitiesHelper, ChromeCapabilities, ChromiumLikeCapabilities, DesiredCapabilities, FirefoxCapabilities};

pub struct Chrome;
impl BrowserConfig for Chrome {
    type Capabilities = ChromeCapabilities;

    fn new(profile_path: &str, binary_path: Option<&str>) -> Self::Capabilities {
        let mut caps = DesiredCapabilities::chrome();
        let initial_args = get_chrome_args();
        for arg in initial_args.iter() {
            fatal_unwrap_e!(caps.add_arg(*arg), "Failed to add arg {}");
        }
        fatal_unwrap_e!(
            caps.add_experimental_option("excludeSwitches", ["enable-automation"]),
            "Failed to add experimental excludeSwitches option {}"
        );

        let arg = format!("user-data-dir={}", profile_path);
        caps.add_arg(arg.as_str()).unwrap();
        caps
    }
}

pub fn get_chrome_args() -> Vec<&'static str> {
    vec![
        "--disable-background-timer-throttling",
        "--disable-backgrounding-occluded-windows",
        "--disable-logging",
        "--no-sandbox",
        //"--headless=new", // Add headless mode
        "--disable-blink-features=AutomationControlled",
        "--disable-infobars",
        "--disable-notifications",
        "--disable-popup-blocking",
        "--disable-extensions",
        "--disable-dev-shm-usage",
        "--no-sandbox",
        "--window-size=1920,1080",
        "--start-maximized",
        "--ignore-certificate-errors",
        "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    ]
}

pub struct Firefox;

impl Firefox {
    fn patch_xul_config(binary_path: &str) {
        let mut xul_path_buf = PathBuf::from(binary_path);
        xul_path_buf.pop();
        xul_path_buf.push("xul.dll");
        let xul_path = xul_path_buf.to_str().unwrap();
        patch_binary_with_random(xul_path, b"webdriver", 9);
    }
}
impl BrowserConfig for Firefox {
    type Capabilities = FirefoxCapabilities;

    fn new(profile_path: &str, binary_path: Option<&str>) -> Self::Capabilities {
        let mut caps = DesiredCapabilities::firefox();
        let mut initial_args = get_firefox_args();

        fatal_unwrap_e!(caps.set_encoded_profile(profile_path), "Failed to set profile {}");
        let mut prefs = FirefoxPreferences::new();
        let firefox_binary = fatal_unwrap_e!(binary_path, "Failed to get firefox binary path {}");

        fatal_unwrap_e!(caps.set_firefox_binary(firefox_binary.as_str()), "Failed to set firefox binary {}");
        // fatal_unwrap_e!(caps.set_headless(), "Failed to set headless {}");

        fatal_unwrap_e!(
            prefs.set("privacy.trackingprotection.enabled", false),
            "Failed to set privacy.trackingprotection.enabled {}"
        );
        fatal_unwrap_e!(
            prefs.set("privacy.trackingprotection.pbmode.enabled", false),
            "Failed to set privacy.trackingprotection.pbmode.enabled {}"
        );
        fatal_unwrap_e!(
            prefs.set("browser.cache.memory.enable", false),
            "Failed to set browser.cache.memory.enable {}"
        );
        fatal_unwrap_e!(
            prefs.set("browser.cache.offline.enable", false),
            "Failed to set browser.cache.offline.enable {}"
        );
        fatal_unwrap_e!(
            prefs.set("network.http.use-cache", false),
            "Failed to set network.http.use-cache {}"
        );
        fatal_unwrap_e!(prefs.set("dom.webdriver.enabled", false), "Failed to set dom.webdriver.enabled {}");
        fatal_unwrap_e!(
            prefs.set_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0".to_string()),
            "Failed to set user agent {}"
        );

        caps.set_preferences(prefs).unwrap();

        for arg in initial_args.iter() {
            fatal_unwrap_e!(caps.add_arg(*arg), "Failed to add arg {}");
        }
        Self::patch_xul_config(binary_path.unwrap());
        caps
    }
}

pub fn get_firefox_args() -> Vec<&'static str> {
    vec![]
}
