use crate::driver::browser_config::{Chrome, Firefox};
use crate::driver::session_initializer::{ChromeSessionInitializer, GeckoSessionInitializer};
use crate::driver::traits::DriverService;
use crate::env::get_env;
use crate::utils::{generate_random_string, patch_binary_with_random};
use fs_extra::dir::CopyOptions;
use std::env::current_dir;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::ops::Range;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Condvar, Mutex};
use std::{fs, mem, thread};
use thirtyfour::support::base64_encode;
use tokio::sync::{oneshot, OnceCell};

pub struct ChromeDriverService {
    driver_service: Child,
    profiles: Vec<String>, // PATHS to various folders
}

impl ChromeDriverService {
    fn create_session_dirs(session_count: u16, profile_path: &str) -> Vec<String> {
        let mut work_dir = current_dir().unwrap();
        let profile_dir = profile_path;
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
            if folder_count == session_count {
                info!("Found enough session folders to reuse.");
                return session_folders;
            }
            let file_type = match dir.file_type() {
                Ok(file_type) => file_type,
                Err(_) => {
                    fatal_assert!("Failed to get file type")
                }
            };
            if file_type.is_file() {
                continue;
            }
            session_folders.push(dir.path().to_str().unwrap().to_string());
            folder_count += 1;
        }

        // OPTIMIZE use async as this is an IO bound operation
        let _ = thread::scope(|s| {
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
impl DriverService for ChromeDriverService {
    type BrowserConfigType = Chrome;
    type Param<'a> = &'a Vec<String>;
    type SessionInitializerType = ChromeSessionInitializer;

    async fn new(port: u16, session_count: u16, driver_path: &str, profile_path: &str) -> Self {
        patch_binary_with_random(driver_path, b"cdc_", 22);
        let mut cmd = Command::new(driver_path);
        cmd.arg(format!("--port={}", port));
        cmd.stdout(Stdio::piped());

        let mut driver_service: Child = fatal_unwrap_e!(cmd.spawn(), "Failed to start chromedriver {}");
        let stdout = driver_service.stdout.take().expect("Failed to get stdout");
        let signal = Arc::new(Condvar::new());
        let signal_lock = Arc::new(Mutex::new(true));
        let tokio_signal = signal.clone();
        let tokio_signal_lock = signal_lock.clone();

        ///REFACTOR reuse the same code for both chrome and gecko
        tokio::spawn(async move {
            const EXPECTED_OUTPUT: &str = "success";

            let mut reader = BufReader::new(stdout);
            let mut out_str = String::new();
            fatal_unwrap_e!(reader.read_line(&mut out_str), "Failed to read line {}");
            loop {
                out_str.clear();
                match reader.read_line(&mut out_str) {
                    Ok(0) => {
                        println!("[CHROME-DRIVER] Process ended");
                        break;
                    }
                    Ok(_) => {
                        if out_str.contains(EXPECTED_OUTPUT) {
                            let mut guard = tokio_signal_lock.lock().unwrap();
                            *guard = false;
                            tokio_signal.notify_all();
                            break;
                        }
                        println!("[CHROME-DRIVER] {}", out_str);
                    }
                    Err(e) => {
                        println!("[CHROME-DRIVER] Error reading output: {}", e);
                        break;
                    }
                }
            }
            // No need for an if in a loop
            loop {
                out_str.clear();
                match reader.read_line(&mut out_str) {
                    Ok(0) => {
                        // EOF reached - pipe closed
                        println!("[CHROME-DRIVER] Process ended");
                        break;
                    }
                    Ok(_) => {
                        println!("[CHROME-DRIVER]{}", out_str);
                    }
                    Err(e) => {
                        println!("[CHROME-DRIVER] Error reading output: {}", e);
                        break;
                    }
                }
            }
        });

        let guard = signal_lock.lock().unwrap();
        fatal_unwrap_e!(signal.wait_while(guard, |val| *val), "Failed to wait for geckodriver service: {}");

        info!("Preparing chrome session profiles...");
        let profiles = Self::create_session_dirs(session_count, profile_path);

        Self { driver_service, profiles }
    }

    async fn session_params<'a>(&'a self) -> Self::Param<'a> {
        &self.profiles
    }
}

impl Drop for ChromeDriverService {
    fn drop(&mut self) {
        info!("Killing driver service");
        self.driver_service.kill().unwrap();
    }
}

pub struct GeckoDriverService {
    driver_services: Vec<Child>,
    base64_profile: String,
    ports: Range<u16>,
}

impl GeckoDriverService {
    fn create_session_dirs(profile_path: &str) -> String {
        let current_dir = current_dir().unwrap();
        let mut b64_file_path = current_dir.clone();
        b64_file_path.push("sessions");
        if !b64_file_path.exists() {
            fatal_unwrap_e!(fs::create_dir(b64_file_path.clone()), "Failed to create sessions dir {}");
        }
        b64_file_path.push("encoded.b64");
        let encoded;
        if !b64_file_path.exists() {
            let target_file = current_dir.join(profile_path);
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
        encoded
    }
}
impl DriverService for GeckoDriverService {
    type BrowserConfigType = Firefox;
    type Param<'a> = (&'a str, Range<u16>);
    type SessionInitializerType = GeckoSessionInitializer;
    async fn new(port: u16, session_count: u16, driver_path: &str, profile_path: &str) -> Self {
        const EXPECTED_OUTPUT: &str = "Listening on";
        let path_str = driver_path;
        let signal = Arc::new(Condvar::new());
        let signal_lock = Arc::new(Mutex::new(session_count));
        let mut results = Vec::with_capacity(session_count as usize);
        for i in 0..session_count {
            let port = port + i;
            let mut cmd = Command::new(path_str);
            let mut gecko_driver = cmd.arg("--port").arg(port.to_string()).stdout(Stdio::piped()).spawn().unwrap();
            let stdout = gecko_driver.stdout.take().unwrap();
            results.push(gecko_driver);

            let tokio_signal = signal.clone();
            let tokio_signal_lock = signal_lock.clone();
            tokio::spawn(async move {
                let mut buff_reader = BufReader::new(stdout);
                let mut out_str = String::new();
                loop {
                    match buff_reader.read_line(&mut out_str) {
                        Ok(0) => {
                            // EOF reached - pipe closed
                            println!("[GECKO-DRIVER {}] Process ended", i);
                            break;
                        }
                        Ok(_) => {
                            if out_str.contains(EXPECTED_OUTPUT) {
                                println!("[GECKO-DRIVER {}] {}", i, out_str);
                                let mut guard = tokio_signal_lock.lock().unwrap();
                                *guard -= 1;
                                tokio_signal.notify_all();
                                break;
                            }
                        }
                        Err(e) => {
                            println!("[GECKO-DRIVER {}] Error reading output: {}", i, e);
                            break;
                        }
                    }
                }

                // Second loop that exits when pipe is closed. No need for an if in a loop
                loop {
                    out_str.clear();
                    match buff_reader.read_line(&mut out_str) {
                        Ok(0) => {
                            // EOF reached - pipe closed
                            println!("[GECKO-DRIVER {}] Process ended", i);
                            break;
                        }
                        Ok(_) => {
                            println!("[GECKO-DRIVER {}]{}", i, out_str);
                        }
                        Err(e) => {
                            println!("[GECKO-DRIVER {}] Error reading output: {}", i, e);
                            break;
                        }
                    }
                }
            });
        }

        let guard = signal_lock.lock().unwrap();
        fatal_unwrap_e!(
            signal.wait_while(guard, |val| *val > 0),
            "Failed to wait for geckodriver service: {}"
        );
        info!(
            "{} Geckodriver started successfully on port {} - {}",
            session_count,
            port,
            port + session_count
        );
        info!("Preparing gecko session base64 profile...");
        let base64_profile = Self::create_session_dirs(profile_path);
        Self {
            ports: port..port + session_count,
            driver_services: results,
            base64_profile,
        }
    }
    async fn session_params<'a>(&'a self) -> Self::Param<'a> {
        (self.base64_profile.as_str(), self.ports.clone())
    }
}

impl Drop for GeckoDriverService {
    fn drop(&mut self) {
        info!("Killing driver service");
        for mut driver in self.driver_services.iter_mut() {
            driver.kill().unwrap();
        }
    }
}
