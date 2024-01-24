use rust_decimal::Decimal;

use super::common::{Error, Result};
use super::value_objects::position::Direction;
use super::value_objects::{id::Id, price::Price, quantity::Quantity, ticker::Ticker};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    id: Id,
    account_id: Id,
    portfolio_id: Id,
    market_id: Id,
    ticker: Ticker,
    quantity: Quantity,
    entry_price: Price,
    direction: Direction,
}

impl Position {
    pub fn new(
        id: Id,
        account_id: Id,
        portfolio_id: Id,
        market_id: Id,
        ticker: Ticker,
        quantity: Quantity,
        entry_price: Price,
        direction: Direction,
    ) -> Result<Self> {
        if quantity.0 == 0 {
            return Err(Error::InvalidQuantity(quantity));
        }
        if entry_price.0 < Decimal::ZERO {
            return Err(Error::InvalidPrice(entry_price));
        }
        Ok(Self {
            id,
            account_id,
            portfolio_id,
            market_id,
            ticker,
            quantity,
            entry_price,
            direction,
        })
    }
}
