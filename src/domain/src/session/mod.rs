mod entities;
mod value_objects;

pub use entities::{Account, Portfolio};
pub use value_objects::SessionId;

use crate::market::MarketId;

pub struct Session {
    id: SessionId,
    market: MarketId,
    account: Account,
    portfolio: Portfolio,
}
