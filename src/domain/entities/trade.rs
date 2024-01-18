// Trade entity for trading application in clean architecture

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use super::{symbol::Symbol, user_account::UserAccount};

pub struct Trade {
    account: UserAccount,
    symbol: Symbol,
    quantity: u32,
    price: Decimal,
    date: DateTime<Utc>,
}
