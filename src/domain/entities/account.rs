use super::{id::Id, Error, Result};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Account {
    id: Id,
    user_id: Id,
    balance: Decimal,
}

impl Account {
    fn new(id: Id, user_id: Id, balance: Decimal) -> Result<Self> {
        if balance < Decimal::ZERO {
            return Err(Error::InvalidBalance(balance));
        }
        Ok(Self {
            id,
            user_id,
            balance: balance.round_dp(2),
        })
    }

    pub fn deposit(&mut self, amount: Decimal) -> Result<Decimal> {
        self.balance += amount.round_dp(2);
        Ok(self.balance)
    }

    pub fn withdraw(&mut self, amount: Decimal) -> Result<Decimal> {
        if amount > self.balance {
            return Err(Error::InvalidBalance(self.balance - amount.round_dp(2)));
        }
        self.balance -= amount.round_dp(2);
        Ok(self.balance)
    }

    pub fn balance(&self) -> Decimal {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_new() {
        let account = Account::new(Id(1), Id(1), dec!(0.0)).unwrap();
        assert_eq!(account.id, Id(1));
        assert_eq!(account.user_id, Id(1));
        assert_eq!(account.balance, dec!(0.0));
    }

    #[test]
    fn test_balance() {
        let account = Account::new(Id(1), Id(1), dec!(100.0)).unwrap();
        assert_eq!(account.balance(), dec!(100.0));
    }

    #[test]
    fn test_negative_balance() {
        let account = Account::new(Id(1), Id(1), dec!(-10.0));
        assert_eq!(account.is_err(), true);
        assert_eq!(account.unwrap_err(), Error::InvalidBalance(dec!(-10.0)));
    }

    #[test]
    fn test_deposit() {
        let mut account = Account::new(Id(1), Id(1), dec!(0.0)).unwrap();
        let balance = account.deposit(dec!(10.0)).unwrap();
        assert_eq!(balance, dec!(10.0));
    }

    #[test]
    fn test_withdraw() {
        let mut account = Account::new(Id(1), Id(1), dec!(10.0)).unwrap();
        let balance = account.withdraw(dec!(5.0)).unwrap();
        assert_eq!(balance, dec!(5.0));
    }

    #[test]
    fn test_invalid_withdraw() {
        let mut account = Account::new(Id(1), Id(1), dec!(10.0)).unwrap();
        let balance = account.withdraw(dec!(15.0));
        assert_eq!(balance.is_err(), true);
        assert_eq!(balance.unwrap_err(), Error::InvalidBalance(dec!(-5.0)));
    }
}
