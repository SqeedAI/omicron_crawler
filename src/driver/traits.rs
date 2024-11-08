pub trait BrowserConfig {
    type Capabilities: Into<thirtyfour::Capabilities>;

    fn new(user_dir: &str) -> Self::Capabilities;
    fn create_session_dirs(session_count: u16) -> Vec<String>;
}
