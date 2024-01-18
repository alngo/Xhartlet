use super::{Error, Result};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Account {
    name: String,
    balance: Decimal,
}

impl Account {
    pub fn new(name: String, balance: Decimal) -> Result<Self> {
        if name.is_empty() {
            return Err(Error::InvalidEmptyName);
        }
        if name.len() > 50 {
            return Err(Error::InvalidNameLength);
        }
        Ok(Self {
            name,
            balance: balance.round_dp(2),
        })
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn balance(&self) -> &Decimal {
        &self.balance
    }

    pub fn set_balance(&mut self, balance: Decimal) {
        self.balance = balance.round_dp(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_account() {
        let account = Account::new("John".to_string(), dec!(100.001)).unwrap();
        assert_eq!(account.name(), "John");
        assert_eq!(account.balance(), &dec!(100.00));
    }

    #[test]
    fn test_account_eq() {
        let account1 = Account::new("John".to_string(), dec!(100.001)).unwrap();
        let account2 = Account::new("John".to_string(), dec!(100.001)).unwrap();
        assert_eq!(account1, account2);
    }

    #[test]
    fn test_account_set_balance() {
        let mut account = Account::new("John".to_string(), dec!(100.001)).unwrap();
        account.set_balance(dec!(200.001));
        assert_eq!(account.balance(), &dec!(200.00));
    }

    #[test]
    fn test_account_with_empty_name() {
        let account_err = Account::new("".to_string(), dec!(100.001)).unwrap_err();
        assert_eq!(account_err, Error::InvalidEmptyName);
    }

    #[test]
    fn test_account_with_long_name() {
        let account_err = Account::new("A".repeat(51), dec!(100.001)).unwrap_err();
        assert_eq!(account_err, Error::InvalidNameLength);
    }
}
