use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

use std::fmt;

#[derive(Debug)]
pub enum Error {
    TelegramError(String),
    InvalidToken,
    TimedOut,
    Conflict(String),
    RequestError(ReqwestError),
    SerdeJsonError(JsonError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TelegramError(msg) => write!(f, "{}", msg),
            Self::RequestError(rq) => write!(f, "Failure making request: {:?}", rq),
            Self::InvalidToken => write!(f, "Invalid Token"),
            Self::Conflict(msg) => write!(f, "{}", msg),
            Self::TimedOut => write!(f, "Timed Out"),
            Self::SerdeJsonError(obj) => write!(f, "Failed to parse or construct JSON: {:?}", obj),
        }
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::TelegramError(err)
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        Self::RequestError(error)
    }
}

impl From<JsonError> for Error {
    fn from(error: JsonError) -> Self {
        Self::SerdeJsonError(error)
    }
}