use rust_decimal::Decimal;

use super::{symbol::Symbol, user_account::UserAccount, Error, Result};

#[derive(Debug, Clone, Eq, PartialEq)]
enum OrderKind {
    Market,
    Limit,
    Stop,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum OrderStatus {
    New,
    Filled,
    Cancelled,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Order {
    account: UserAccount,
    symbol: Symbol,
    quantity: Decimal,
    price: Decimal,
    kind: OrderKind,
    status: OrderStatus,
}

impl Order {
    pub fn new(
        account: UserAccount,
        symbol: Symbol,
        quantity: Decimal,
        price: Decimal,
        kind: OrderKind,
    ) -> Result<Self> {
        if quantity <= Decimal::ZERO {
            return Err(Error::InvalidQuantity(quantity));
        }
        if price == Decimal::ZERO {
            return Err(Error::InvalidPrice(price));
        }
        if price * quantity > *account.balance() {
            return Err(Error::InvalidOrder(price * quantity, *account.balance()));
        }
        Ok(Self {
            account,
            symbol,
            quantity,
            price,
            kind,
            status: OrderStatus::New,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::entities::user::User;

    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_new_order() {
        let user = User::new("John".to_string()).unwrap();
        let account = UserAccount::new(user, dec!(1000.0)).unwrap();
        let order = Order::new(
            account,
            Symbol::EURUSD,
            dec!(100),
            dec!(1.0),
            OrderKind::Market,
        );
        assert!(order.is_ok());
    }

    #[test]
    fn test_new_order_zero_quantity() {
        let user = User::new("John".to_string()).unwrap();
        let account = UserAccount::new(user, dec!(1000.0)).unwrap();
        let order = Order::new(
            account,
            Symbol::EURUSD,
            dec!(0),
            dec!(1.0),
            OrderKind::Market,
        )
        .unwrap_err();
        assert_eq!(order, Error::InvalidQuantity(Decimal::ZERO));
    }

    #[test]
    fn test_new_order_zero_price() {
        let user = User::new("John".to_string()).unwrap();
        let account = UserAccount::new(user, dec!(1000.0)).unwrap();
        let order = Order::new(
            account,
            Symbol::EURUSD,
            dec!(100),
            dec!(0),
            OrderKind::Market,
        )
        .unwrap_err();
        assert_eq!(order, Error::InvalidPrice(Decimal::ZERO));
    }

    #[test]
    fn test_new_order_invalid_order() {
        let user = User::new("John".to_string()).unwrap();
        let account = UserAccount::new(user, dec!(1000.0)).unwrap();
        let order = Order::new(
            account,
            Symbol::EURUSD,
            dec!(100),
            dec!(1000.0),
            OrderKind::Market,
        )
        .unwrap_err();
        assert_eq!(
            order,
            Error::InvalidOrder(dec!(1000.0) * dec!(100), dec!(1000.0))
        );
    }
}
