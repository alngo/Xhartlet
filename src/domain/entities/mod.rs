pub use super::value_objects::*;
use rust_decimal::Decimal;
use std::fmt;

mod account;
mod order;
mod trade;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Error {
    InvalidEmptyName,
    InvalidNameLength,
    InvalidQuantity(Decimal),
    InvalidPrice(Decimal),
    InvalidOrder(Decimal, Decimal),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidEmptyName => write!(f, "User name cannot be empty"),
            Error::InvalidNameLength => write!(f, "User name cannot be longer than 50 characters"),
            Error::InvalidQuantity(quantity) => write!(f, "Invalid quantity: {}", quantity),
            Error::InvalidPrice(price) => write!(f, "Invalid price: {}", price),
            Error::InvalidOrder(order_price, balance) => write!(
                f,
                "Invalid order: order price {} is greater than balance {}",
                order_price, balance
            ),
        }
    }
}
