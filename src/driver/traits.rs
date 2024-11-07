pub trait Capabilities {
    type Capabilities: Into<thirtyfour::Capabilities>;

    fn new(user_dir: &str) -> Self::Capabilities;
}
