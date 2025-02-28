pub trait ApiConfig {
    const API_URL: &'static str;
    const COOKIE_DOMAIN: &'static str;
    const COOKIE_FOLDER: &'static str;
}

struct TestConfig;
impl ApiConfig for TestConfig {
    const API_URL: &'static str = "127.0.0.1:9090";
    const COOKIE_DOMAIN: &'static str = "127.0.0.1";
    const COOKIE_FOLDER: &'static str = "cookies/";
}
struct ProdConfig;
impl ApiConfig for ProdConfig {
    const API_URL: &'static str = "https://www.linkedin.com";
    const COOKIE_DOMAIN: &'static str = "www.linkedin.com";
    const COOKIE_FOLDER: &'static str = "cookies/";
}
