use crate::driver::traits::BrowserConfig;
use crate::utils::{chrome_profile_path_from_env, firefox_profile_path_from_env};
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

    fn new(user_dir_path: &str) -> Self::Capabilities {
        let mut caps = DesiredCapabilities::chrome();
        let initial_args = get_chrome_args();
        for arg in initial_args.iter() {
            fatal_unwrap_e!(caps.add_arg(*arg), "Failed to add arg {}");
        }
        fatal_unwrap_e!(
            caps.add_experimental_option("excludeSwitches", ["enable-automation"]),
            "Failed to add experimental excludeSwitches option {}"
        );

        let arg = format!("user-data-dir={}", user_dir_path);
        caps.add_arg(arg.as_str()).unwrap();
        caps
    }

    fn create_session_dirs(session_count: u16) -> Vec<String> {
        let mut work_dir = current_dir().unwrap();
        let profile_dir = chrome_profile_path_from_env();
        work_dir.push(profile_dir);
        let user_dir = work_dir.clone();
        let mut session_dir = current_dir().unwrap();
        session_dir.push("sessions");
        if !session_dir.exists() {
            fatal_unwrap_e!(fs::create_dir_all(session_dir.clone()), "Failed to create user directory {}");
        }

        let mut session_folders = Vec::with_capacity(session_count as usize);
        let existing_session_folders = fatal_unwrap_e!(fs::read_dir(session_dir.clone()), "Failed to read user directory {}");
        let mut folder_count: u16 = 0;
        for dir in existing_session_folders.filter_map(Result::ok) {
            folder_count += 1;
            session_folders.push(dir.path().to_str().unwrap().to_string());
        }
        if folder_count >= session_count {
            info!("Found enough session folders to reuse.");
            return session_folders;
        }

        // OPTIMIZE use async as this is an IO bound operation
        let result = thread::scope(|s| {
            let mut join_handles = Vec::with_capacity(folder_count as usize);
            for i in folder_count..session_count {
                let mut target_dir = session_dir.clone();
                let user_dir_ref = &user_dir;
                join_handles.push(s.spawn(move || {
                    target_dir.push(i.to_string());
                    let copy_options = CopyOptions {
                        copy_inside: true,
                        ..Default::default()
                    };
                    info!(
                        "Copying user directory {} to {}",
                        user_dir_ref.to_str().unwrap(),
                        target_dir.to_str().unwrap()
                    );
                    fatal_unwrap_e!(
                        fs_extra::dir::copy(user_dir_ref.clone(), target_dir.clone(), &copy_options),
                        "Failed to copy user directory {}"
                    );
                    return target_dir.to_str().unwrap().to_string();
                }));
            }
            for handle in join_handles {
                session_folders.push(handle.join().unwrap());
            }
        });
        session_folders
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
impl BrowserConfig for Firefox {
    type Capabilities = FirefoxCapabilities;

    fn new(user_dir_path: &str) -> Self::Capabilities {
        let mut caps = DesiredCapabilities::firefox();
        let mut initial_args = get_firefox_args();

        fatal_unwrap_e!(caps.set_encoded_profile(user_dir_path), "Failed to set profile {}");
        let mut prefs = FirefoxPreferences::new();
        fatal_unwrap_e!(
            caps.set_firefox_binary("C:\\Program Files\\Mozilla Firefox\\firefox.exe"),
            "Failed to set firefox binary {}"
        );
        // fatal_unwrap_e!(caps.set_headless(), "Failed to set headless {}");

        // Hide WebDriver
        prefs.set("dom.webdriver.enabled", false)?;
        prefs.set("webdriver_enable_native_events", false)?;
        prefs.set("webdriver.load.strategy", "unstable")?;

        // Hardware/Platform consistency
        prefs.set("webgl.disabled", false)?;
        prefs.set("canvas.capturestream.enabled", true)?;
        prefs.set("media.navigator.enabled", true)?;
        prefs.set("media.hardware-video-decoding.enabled", true)?;
        prefs.set("dom.webgl.enabled", true)?;
        prefs.set("layout.css.font-visibility.level", 1)?; // For consistent font fingerprinting

        // Audio settings
        prefs.set("media.webaudio.enabled", true)?;
        prefs.set("javascript.options.wasm", true)?;

        // Timezone/Location
        prefs.set("privacy.resistFingerprinting", false)?; // Can interfere with fingerprint consistency
        prefs.set("intl.accept_languages", "en-US, en")?;
        prefs.set("general.useragent.locale", "en-US")?;

        // Additional stealth
        prefs.set("network.http.referer.spoofSource", false)?;
        prefs.set("security.ssl.disable_session_identifiers", false)?;
        prefs.set("privacy.trackingprotection.enabled", false)?;

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
        caps
    }

    // OPTIMIZE Think about how to not return a vec, but rather, maybe let the config obtain a free folder instead (Be abstract)
    fn create_session_dirs(session_count: u16) -> Vec<String> {
        let mut paths = Vec::with_capacity(session_count as usize);
        let current_dir = current_dir().unwrap();
        let b64_file_path = current_dir.join("sessions\\encoded.b64");
        let encoded;
        if !b64_file_path.exists() {
            let target_file = current_dir.join(firefox_profile_path_from_env());
            let file_content = fatal_unwrap_e!(fs::read(target_file), "Failed to read file {}");
            if file_content.len() >= 4 && file_content[0..4] == [0x50, 0x4B, 0x03, 0x04] {
                info!("Found zipped profile file! size: {}", file_content.len());
            } else {
                fatal_assert!("Invalid zip file");
            }
            encoded = base64_encode(file_content.as_slice());
            fatal_unwrap_e!(fs::write(b64_file_path.clone(), encoded.clone()), "Failed to write file {}");
        } else {
            encoded = fatal_unwrap_e!(fs::read_to_string(b64_file_path.clone()), "Failed to read file {}");
            info!("Found encoded profile file! size: {}", b64_file_path.metadata().unwrap().len());
        }

        for _ in 0..session_count {
            paths.push(encoded.clone());
        }
        paths
    }
}

pub fn get_firefox_args() -> Vec<&'static str> {
    vec![]
}
