use crate::utils::{
    chrome_driver_path_from_env, driver_host_from_env, driver_port_from_env, firefox_binary_path_from_env, gecko_driver_path_from_env,
    generate_random_string,
};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Condvar, Mutex};
use std::{fs, mem};
use tokio::sync::{oneshot, OnceCell};

pub struct ChromeDriverService {
    port: String,
    driver_service: Child,
}

impl ChromeDriverService {
    pub async fn new(port: String, chromedriver_path: &str) -> Self {
        let path_str = chromedriver_path;
        patch_binary_with_random(path_str, b"cdc_", 22);
        let mut cmd = Command::new(path_str);
        cmd.arg(format!("--port={}", port));
        cmd.stdout(Stdio::piped());

        let mut driver_service: Child = fatal_unwrap_e!(cmd.spawn(), "Failed to start chromedriver {}");
        let stdout = driver_service.stdout.take().expect("Failed to get stdout");

        let (tx, rx) = oneshot::channel();
        let port_clone = port.clone();

        tokio::spawn(async move {
            let expected_output = "success";

            let mut reader = BufReader::new(stdout);
            let mut out_str = String::new();
            fatal_unwrap_e!(reader.read_line(&mut out_str), "Failed to read line {}");

            loop {
                println!("[CHROME-DRIVER] {}", out_str);
                if out_str.contains(&expected_output) {
                    fatal_unwrap_e!(tx.send(()), "Failed to notify on success! {:?}");
                    break;
                }
                out_str.clear();
                fatal_unwrap_e!(reader.read_line(&mut out_str), "Failed to read line {}");
            }
        });

        tokio::select! {
            _ = rx => {
                info!("Chromedriver started successfully on port {}", port);
            },
        }

        Self { port, driver_service }
    }
}

pub fn patch_binary_with_random(binary_path: &str, pattern: &[u8], random_string_size: usize) {
    let mut binary = fatal_unwrap_e!(fs::read(binary_path), "Failed to read target binary for patching {}");
    let pattern = pattern;
    let new_string = generate_random_string(random_string_size);
    // TODO use strings instead of bytes
    let mut matches = Vec::with_capacity(3);
    for (index, window) in binary.windows(pattern.len()).enumerate() {
        if window == pattern {
            matches.push(index);
        }
    }
    if matches.len() == 0 {
        info!("no pater matches found, no need to patch!");
        return;
    }

    let first_match = unsafe { String::from_raw_parts(binary.as_mut_ptr().add(matches[0]), random_string_size, random_string_size) };
    info!("Replacing {} with {}", first_match, new_string);
    mem::forget(first_match);

    for index in matches {
        let mut pattern_str = unsafe { String::from_raw_parts(binary.as_mut_ptr().add(index), random_string_size, random_string_size) };
        pattern_str.replace_range(0..random_string_size, &new_string);
        mem::forget(pattern_str);
    }
    fs::write(binary_path, binary).unwrap();
}

impl Drop for ChromeDriverService {
    fn drop(&mut self) {
        info!("Killing driver service");
        self.driver_service.kill().unwrap();
    }
}

static CHROME_DRIVER_SERVICE: OnceCell<ChromeDriverService> = OnceCell::const_new();

pub async fn chrome_driver_service() -> &'static ChromeDriverService {
    let port = driver_port_from_env();
    let path = chrome_driver_path_from_env();
    CHROME_DRIVER_SERVICE
        .get_or_init(|| async { ChromeDriverService::new(port, path.as_str()).await })
        .await
}

pub struct GeckoDriverService {
    port: String,
    driver_service: Child,
}

impl GeckoDriverService {
    pub async fn new(port: String, geckodriver_path: &str) -> Self {
        let path_str = geckodriver_path;
        let firefox_binary_path = firefox_binary_path_from_env();
        let mut xul_path_buf = PathBuf::from(firefox_binary_path);
        xul_path_buf.pop();
        xul_path_buf.push("xul.dll");
        let xul_path = xul_path_buf.to_str().unwrap();
        patch_binary_with_random(xul_path, b"webdriver", 9);

        let mut cmd = Command::new(path_str);
        let mut gecko_driver = cmd.arg("--port").arg(port.clone()).stdout(Stdio::piped()).spawn().unwrap();
        let stdout = gecko_driver.stdout.take().unwrap();
        let signal = Arc::new(Condvar::new());
        let signal_lock = Arc::new(Mutex::new(true));
        let tokio_signal = signal.clone();
        let tokio_signal_lock = signal_lock.clone();
        tokio::spawn(async move {
            let mut buff_reader = BufReader::new(stdout);
            let mut out_str = String::new();
            loop {
                out_str.clear();
                let _ = buff_reader.read_line(&mut out_str);
                if out_str.contains("Listening on") {
                    println!("[GECKO-DRIVER] {}", out_str);
                    let mut guard = tokio_signal_lock.lock().unwrap();
                    *guard = false;
                    tokio_signal.notify_all();
                    break;
                }
            }
            // No point to have an if in a loop
            loop {
                out_str.clear();
                if let Ok(len) = buff_reader.read_line(&mut out_str) {
                    if len == 0 {
                        continue;
                    }
                    println!("[GECKO-DRIVER]{}", out_str);
                }
            }
        });

        let guard = signal_lock.lock().unwrap();
        fatal_unwrap_e!(signal.wait_while(guard, |val| *val), "Failed to wait for geckodriver service: {}");
        info!("Geckodriver started successfully on port {}", port);
        Self {
            port,
            driver_service: gecko_driver,
        }
    }
}

static GECKO_DRIVER_SERVICE: OnceCell<GeckoDriverService> = OnceCell::const_new();
pub async fn gecko_driver_service() -> &'static GeckoDriverService {
    let port = driver_port_from_env();
    let path = gecko_driver_path_from_env();
    GECKO_DRIVER_SERVICE
        .get_or_init(|| async { GeckoDriverService::new(port, path.as_str()).await })
        .await
}
