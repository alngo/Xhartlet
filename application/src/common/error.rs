use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct ApplicationError {
    pub message: String,
}

impl Error for ApplicationError {}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApplicationError: {}", self.message)
    }
}
