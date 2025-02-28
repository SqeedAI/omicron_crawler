pub trait ApiConfig {
    const API_URL: &'static str;
}

struct TestConfig;
impl ApiConfig for TestConfig {
    const API_URL: &'static str = "127.0.0.1:9090";
}
struct ProdConfig;
impl ApiConfig for ProdConfig {
    const API_URL: &'static str = "https://api.2captcha.com";
}
