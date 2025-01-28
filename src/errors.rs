use std::fmt::{Display, Formatter};
pub type CrawlerResult<T> = Result<T, CrawlerError>;
pub enum CrawlerError {
    ParseError(String),
    InteractionError(String),
    DriverError(String),
    SessionError(String),
    NoFreeSession(String),
    LinkedinError(String),
    BusError(String),
    QueueError(String),
    FileError(String),
}

impl Display for CrawlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CrawlerError::ParseError(e) => write!(f, "ParseError: {}", e),
            CrawlerError::InteractionError(e) => write!(f, "InteractionError: {}", e),
            CrawlerError::DriverError(e) => write!(f, "DriverError: {}", e),
            CrawlerError::SessionError(e) => write!(f, "SessionError: {}", e),
            CrawlerError::BusError(e) => write!(f, "BusError {}", e),
            CrawlerError::QueueError(e) => write!(f, "QueueError {}", e),
            CrawlerError::LinkedinError(e) => write!(f, "LinkedinError {}", e),
            CrawlerError::FileError(e) => write!(f, "FileError {}", e),
            CrawlerError::NoFreeSession(e) => write!(f, "NoFreeSession {}", e),
        }
    }
}
