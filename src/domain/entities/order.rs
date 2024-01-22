use super::value_objects::order::{Kind, Status};
use super::value_objects::{id::Id, price::Price, quantity::Quantity, ticker::Ticker};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Order {
    id: Id,
    account_id: Id,
    portfolio_id: Id,
    market_id: Id,
    ticker: Ticker,
    quantity: Quantity,
    price: Price,
    kind: Kind,
    status: Status,
}
