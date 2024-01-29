use crate::user::value_objects::Amount;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Account {
    balance: Amount,
}
