pub use super::value_objects::*;
use rust_decimal::Decimal;
use std::fmt;

pub mod account;
pub mod order;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Error {
    InvalidEmptyName,
    InvalidNameLength,
    InvalidQuantity(Decimal),
    InvalidPrice(Decimal),
    InvalidBalance(Decimal, Decimal),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidEmptyName => write!(f, "User name cannot be empty"),
            Error::InvalidNameLength => write!(f, "User name cannot be longer than 50 characters"),
            Error::InvalidQuantity(quantity) => write!(f, "Invalid quantity: {}", quantity),
            Error::InvalidPrice(price) => write!(f, "Invalid price: {}", price),
            Error::InvalidBalance(price, balance) => write!(
                f,
                "Invalid balance: price {} is greater than balance {}",
                price, balance
            ),
        }
    }
}
