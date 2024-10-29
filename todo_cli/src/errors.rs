use std::io;
use serde_json;

#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
    JsonError(serde_json::Error),
    ParseError(String),
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::JsonError(err)
    }
}
