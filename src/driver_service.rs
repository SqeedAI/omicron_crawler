use crate::utils::{driver_host_from_env, driver_path_from_env, driver_port_from_env, generate_random_string};
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::{fs, mem};
use tokio::sync::{oneshot, OnceCell};

pub struct DriverService {
    port: String,
    driver_service: Child,
}

impl DriverService {
    pub async fn new(port: String, chromedriver_path: &str) -> Self {
        let path_str = chromedriver_path;
        patch_cdc(path_str);
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
                println!("{}", out_str);
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

pub fn patch_cdc(chromedriver_path: &str) {
    const CDC_SIZE: usize = 22;
    let mut driver_binary = fatal_unwrap_e!(fs::read(chromedriver_path), "Failed to read chromedriver binary {}");
    let pattern = b"cdc_";
    let new_cdc = generate_random_string(CDC_SIZE);
    // TODO use strings instead of bytes
    let mut matches = Vec::with_capacity(3);
    for (index, window) in driver_binary.windows(pattern.len()).enumerate() {
        if window == pattern {
            matches.push(index);
        }
    }
    if matches.len() == 0 {
        info!("no cdc matches found, no need to patch!");
        return;
    }

    let first_match = unsafe { String::from_raw_parts(driver_binary.as_mut_ptr().add(matches[0]), CDC_SIZE, CDC_SIZE) };
    info!("Replacing {} with {}", first_match, new_cdc);
    mem::forget(first_match);

    for index in matches {
        let mut cdc_str = unsafe { String::from_raw_parts(driver_binary.as_mut_ptr().add(index), CDC_SIZE, CDC_SIZE) };
        cdc_str.replace_range(0..CDC_SIZE, &new_cdc);
        mem::forget(cdc_str);
    }
    fs::write(chromedriver_path, driver_binary).unwrap();
}

impl Drop for DriverService {
    fn drop(&mut self) {
        info!("Killing driver service");
        self.driver_service.kill().unwrap();
    }
}

static DRIVER_SERVICE: OnceCell<DriverService> = OnceCell::const_new();

pub async fn driver_service() -> &'static DriverService {
    let port = driver_port_from_env();
    let path = driver_path_from_env();
    DRIVER_SERVICE
        .get_or_init(|| async { DriverService::new(port, path.as_str()).await })
        .await
}
