use std::fmt::format;
use std::fs;
use std::fs::File;
use std::path::Path;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SessionConfig {
    pub username: String,
    pub password: String,
    pub proxy: String,
    pub proxy_username: String,
    pub proxy_password: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub sessions: Vec<SessionConfig>,
}

impl Config {
    pub fn load_config(path: &str) -> Result<Config, String> {
        let data = match fs::read_to_string(path) {
            Ok(val) => {
                info!("loaded config from: {}", path);
                val
            }
            Err(e) => {
                return Err(format!("Error reading file {}", e));
            }
        };
        let config = match serde_yaml::from_str::<Config>(data.as_str()) {
            Ok(serialized) => serialized,
            Err(e) => return Err(format!("failed to read yaml config: {}", e)),
        };

        Ok(config)
    }

    pub fn create_example_config(path: &str) -> Result<(), String> {
        let config = Config {
            sessions: vec![
                SessionConfig {
                    proxy: "<proxy1>".to_string(),
                    username: "<username1>".to_string(),
                    proxy_password: "<proxy_password1>".to_string(),
                    proxy_username: "<proxy_username1>".to_string(),
                    password: "<password1>".to_string(),
                },
                SessionConfig {
                    proxy: "<proxy2>".to_string(),
                    username: "<username2>".to_string(),
                    proxy_password: "<proxy_password2>".to_string(),
                    proxy_username: "<proxy_username2>".to_string(),
                    password: "<password2>".to_string(),
                },
            ],
        };
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            match fs::create_dir_all(parent) {
                Ok(_) => {}
                Err(e) => {
                    return Err(format!("Failed to create directory {}", e));
                }
            }
        }

        let serialized_config = match serde_yaml::to_string(&config) {
            Ok(serialized_config) => serialized_config,
            Err(e) => {
                return Err(format!("Failed to serialize config {}", e));
            }
        };
        match fs::write(path, serialized_config) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write config {}", e)),
        }
    }
}
