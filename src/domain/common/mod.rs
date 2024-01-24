use core::fmt;

use rust_decimal::Decimal;

use super::value_objects::{id::Id, position::Direction, price::Price, quantity::Quantity};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    InvalidEmptyName,
    InvalidNameLength(usize, usize),
    InvalidBalance(Decimal),
    InvalidQuantity(Quantity),
    InvalidPrice(Price),
    InvalidDirection(Direction),
    OrderNotPending(Id),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidEmptyName => write!(f, "User name cannot be empty"),
            Error::InvalidNameLength(len, max) => {
                write!(f, "Invalid name length: {}, max: {}", len, max)
            }
            Error::InvalidBalance(balance) => write!(f, "Invalid Balance {}", balance),
            Error::InvalidQuantity(quantity) => write!(f, "Invalid Quantity {}", quantity),
            Error::InvalidPrice(price) => write!(f, "Invalid Price {}", price),
            Error::InvalidDirection(direction) => write!(f, "Invalid direction {}", direction),
            Error::OrderNotPending(id) => write!(f, "Order {} is not pending", id),
        }
    }
}
