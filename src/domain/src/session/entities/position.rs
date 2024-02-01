use crate::common::{Price, Ticker};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    ticker: Ticker,
    shares: u32,
    price: Price,
}
