pub trait Capabilities {
    type Capabilities;
    fn new(user_dir: &str) -> Self::Capabilities;
}
