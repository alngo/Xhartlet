use super::{user::User, Result};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserAccount {
    user: User,
    balance: Decimal,
}

impl UserAccount {
    pub fn new(user: User, balance: Decimal) -> Result<Self> {
        Ok(Self {
            user,
            balance: balance.round_dp(2),
        })
    }

    pub fn user(&self) -> &User {
        &self.user
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
    fn test_user_account() {
        let user = User::new("John".to_string()).unwrap();
        let user_account = UserAccount::new(user, dec!(100.001)).unwrap();
        assert_eq!(user_account.user().name(), "John");
        assert_eq!(user_account.balance(), &dec!(100.00));
    }

    #[test]
    fn test_user_account_eq() {
        let user1 = User::new("John".to_string()).unwrap();
        let user2 = User::new("John".to_string()).unwrap();
        let user_account1 = UserAccount::new(user1, dec!(100.001)).unwrap();
        let user_account2 = UserAccount::new(user2, dec!(100.001)).unwrap();
        assert_eq!(user_account1, user_account2);
    }

    #[test]
    fn test_user_account_set_balance() {
        let user = User::new("John".to_string()).unwrap();
        let mut user_account = UserAccount::new(user, dec!(100.00)).unwrap();
        user_account.set_balance(dec!(200.001));
        assert_eq!(user_account.balance(), &dec!(200.00));
    }
}
