// Trade entity for trading application in clean architecture

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use super::{account::Account, symbol::Symbol};

pub struct Trade {
    account: Account,
    symbol: Symbol,
    quantity: u32,
    price: Decimal,
    date: DateTime<Utc>,
}
