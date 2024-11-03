use log::{LevelFilter, Metadata, Record};
use std::sync::Once;

pub struct Logger;
impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        println!("src is: {}", metadata.target());
        metadata.level() <= log::max_level() // Compare against the max level we set
    }

    fn log(&self, record: &Record) {
        println!("[{}] {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;
static INIT: Once = Once::new();

impl Logger {
    pub fn init(logging_level: LevelFilter) {
        INIT.call_once(|| {
            log::set_logger(&LOGGER).expect("Failed to set logger");
            log::set_max_level(logging_level);
        });
    }
}
