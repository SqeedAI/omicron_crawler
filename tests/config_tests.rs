use omicron_crawler::config::Config;

#[test]
fn test_no_config() {
    let config_path = "test_config/config.yaml";

    // Check if file exists and remove it
    if std::path::Path::new(config_path).exists() {
        std::fs::remove_file(config_path).expect("Failed to delete existing config file");
    }

    match Config::load_config(config_path) {
        Ok(_) => {
            assert!(false);
        }
        Err(e) => {
            println!("{}", e);
            assert!(true);
        }
    }
}

#[test]
fn test_create_example_config() {
    let config_path = "test_config/config.yaml";
    match Config::create_example_config(config_path) {
        Ok(_) => {
            assert!(true);
        }
        Err(e) => {
            println!("{}", e);
            assert!(false);
        }
    }
}

#[test]
fn test_load_config() {
    let config_path = "test_config/config.yaml";
    // Check if file exists and remove it
    if std::path::Path::new(config_path).exists() {
        std::fs::remove_file(config_path).expect("Failed to delete existing config file");
    }

    match Config::create_example_config(config_path) {
        Ok(_) => {
            assert!(true);
        }
        Err(e) => {
            println!("{}", e);
            assert!(false);
        }
    }

    let config = match Config::load_config(config_path) {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            assert!(false);
            return;
        }
    };

    assert_eq!(config.sessions[0].username, "<username1>");
    assert_eq!(config.sessions[0].password, "<password1>");
    assert_eq!(config.sessions[0].proxy, "<proxy1>");
    assert_eq!(config.sessions[0].proxy_username, "<proxy_username1>");
    assert_eq!(config.sessions[0].proxy_password, "<proxy_password1>");
    assert_eq!(config.sessions[1].username, "<username2>");
    assert_eq!(config.sessions[1].password, "<password2>");
    assert_eq!(config.sessions[1].proxy, "<proxy2>");
    assert_eq!(config.sessions[1].proxy_username, "<proxy_username2>");
    assert_eq!(config.sessions[1].proxy_password, "<proxy_password2>");
}
