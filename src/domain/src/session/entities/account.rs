use core::fmt;
use std::error::Error;

use rust_decimal::Decimal;

use crate::common::Result;

type Amount = Decimal;

#[derive(Debug)]
pub enum AccountError {
    InvalidDeposit(Amount),
    InvalidWithdrawal(Amount, Amount),
}

impl Error for AccountError {}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountError::InvalidDeposit(deposit) => write!(f, "Invalid deposit: {}", deposit),
            AccountError::InvalidWithdrawal(amount, balance) => write!(
                f,
                "Invalid withdrawal, amount: {}, balance: {}",
                amount, balance
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Account {
    balance: Amount,
}

impl Account {
    pub fn new(initial_deposit: Amount) -> Result<Self> {
        if initial_deposit.is_sign_negative() {
            return Err(Box::new(AccountError::InvalidDeposit(initial_deposit)));
        }
        Ok(Self {
            balance: initial_deposit.round_dp(2),
        })
    }

    pub fn balance(&self) -> Amount {
        self.balance
    }

    pub fn deposit(&mut self, deposit: Amount) -> Result<()> {
        if deposit.is_sign_negative() {
            return Err(Box::new(AccountError::InvalidDeposit(deposit)));
        }
        self.balance += deposit.round_dp(2);
        Ok(())
    }

    pub fn withdraw(&mut self, amount: Amount) -> Result<()> {
        if amount.is_sign_negative() {
            return Err(Box::new(AccountError::InvalidWithdrawal(
                amount,
                self.balance,
            )));
        }
        if amount > self.balance {
            return Err(Box::new(AccountError::InvalidWithdrawal(
                amount,
                self.balance,
            )));
        }
        self.balance -= amount;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_new() {
        let account = Account::new(dec!(100.23)).unwrap();
        assert_eq!(account.balance(), dec!(100.23));

        let account = Account::new(dec!(100.2334)).unwrap();
        assert_eq!(account.balance(), dec!(100.23));
    }

    #[test]
    fn test_invalid_initial_deposit() {
        let account = Account::new(dec!(-100.23));
        assert!(account.is_err());
        assert_eq!(
            account.unwrap_err().to_string(),
            "Invalid deposit: -100.23".to_string()
        );
    }

    #[test]
    fn test_deposit() {
        let mut account = Account::new(dec!(100)).unwrap();
        let _ = account.deposit(dec!(50));
        assert_eq!(account.balance(), dec!(150.00));
    }

    #[test]
    fn test_invalid_deposit() {
        let mut account = Account::new(dec!(0)).unwrap();
        let result = account.deposit(dec!(-100));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid deposit: -100".to_string()
        );
    }

    #[test]
    fn test_withdraw() {
        let mut account = Account::new(dec!(100)).unwrap();
        let _ = account.withdraw(dec!(50));
        assert_eq!(account.balance(), dec!(50.00));
    }

    #[test]
    fn test_invalid_negative_withdraw() {
        let mut account = Account::new(dec!(0)).unwrap();
        let result = account.withdraw(dec!(-100));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid withdrawal, amount: -100, balance: 0".to_string()
        );
    }

    #[test]
    fn test_invalid_withdraw() {
        let mut account = Account::new(dec!(0)).unwrap();
        let result = account.withdraw(dec!(100));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid withdrawal, amount: 100, balance: 0".to_string()
        );
    }
}
