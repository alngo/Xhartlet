use super::{id::Id, Result, Error};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Account {
    id: Id,
    user_id: Id,
    balance: Decimal,
}

impl Account {
    fn new(id: Id, user_id: Id, balance: Decimal) -> Result<Self> {
    }
    

}
