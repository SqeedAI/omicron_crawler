use std::fmt::{Display, Formatter};
pub type CrawlerResult<T> = Result<T, CrawlerError>;

pub enum CrawlerError {
    ParseError(String),
    InteractionError(String),
    DriverError(String),
}

impl Display for CrawlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CrawlerError::ParseError(e) => write!(f, "ParseError: {}", e),
            CrawlerError::InteractionError(e) => write!(f, "InteractionError: {}", e),
            CrawlerError::DriverError(e) => write!(f, "DriverError: {}", e),
        }
    }
}
