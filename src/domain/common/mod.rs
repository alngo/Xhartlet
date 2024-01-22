use core::fmt;

use rust_decimal::Decimal;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    InvalidEmptyName,
    InvalidNameLength(usize, usize),
    InvalidBalance(Decimal),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidEmptyName => write!(f, "User name cannot be empty"),
            Error::InvalidNameLength(len, max) => {
                write!(f, "Invalid name length: {}, max: {}", len, max)
            }
            Error::InvalidBalance(balance) => write!(f, "Invalid Balance {}", balance),
        }
    }
}
