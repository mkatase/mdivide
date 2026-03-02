// src/error.rs

use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    InvalidTag(String),
    Config(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "I/O error: {}", err),
            AppError::InvalidTag(msg) => write!(f, "Tag error: {}", msg),
            AppError::Config(msg) => write!(f, "Config error: {}", msg),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}
