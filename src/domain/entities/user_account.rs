use super::user::User;
use rust_decimal::Decimal;

#[derive(Default)]
pub struct UserAccount {
    user: User,
    balance: Decimal
}

impl UserAccount {
    pub fn new(user: User, balance: Decimal) -> Self {
        Self {
            user,
            balance: balance.round_dp(2)
        }
    }

    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn get_balance(&self) -> &Decimal {
        &self.balance
    }

    pub fn set_balance(&mut self, balance: Decimal) {
        self.balance = balance;
    }
}
