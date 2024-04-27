use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct DomainError {
    pub message: String,
}

impl Error for DomainError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type Result<T> = std::result::Result<T, DomainError>;
