use std::fmt::{Display, Formatter};
pub type ClientResult<T> = Result<T, ClientError>;

pub type IoResult<T> = Result<T, IoError>;
pub enum ClientError {
    UrlError(String),
    RequestError(String),
    ResponseError(String),
    SerializationError(String),
    IoError(IoError),
    HeaderError(String),
    CookieError(String),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::UrlError(e) => write!(f, "UrlError: {}", e),
            ClientError::RequestError(e) => write!(f, "RequestError: {}", e),
            ClientError::ResponseError(e) => write!(f, "ResponseError: {}", e),
            ClientError::SerializationError(e) => write!(f, "SerializationError: {}", e),
            ClientError::IoError(e) => write!(f, "IoError: {}", e),
            ClientError::HeaderError(e) => write!(f, "HeaderError: {}", e),
            ClientError::CookieError(e) => write!(f, "CookieError: {}", e),
        }
    }
}

pub enum IoError {
    FileError(String),
    ParseError(String),
}

impl Display for IoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError::FileError(e) => write!(f, "FileError: {}", e),
            IoError::ParseError(e) => write!(f, "ParseError: {}", e),
        }
    }
}
