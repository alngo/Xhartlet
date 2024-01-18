// Trade entity for trading application in clean architecture

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use super::{account::Account, symbol::Symbol, trade_kind::TradeKind, Error, Result};

pub struct Trade {
    account: Account,
    symbol: Symbol,
    quantity: Decimal,
    price: Decimal,
    date: DateTime<Utc>,
    kind: TradeKind,
}

impl Trade {
    pub fn new(
        account: Account,
        symbol: Symbol,
        quantity: Decimal,
        price: Decimal,
        date: DateTime<Utc>,
        kind: TradeKind,
    ) -> Result<Self> {
        if quantity <= Decimal::ZERO {
            return Err(Error::InvalidQuantity(quantity));
        }
        if price <= Decimal::ZERO {
            return Err(Error::InvalidPrice(price));
        }
        Ok(Self {
            account,
            symbol,
            quantity,
            price,
            date,
            kind,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_new_trade() {
        let account = Account::new("John".to_string(), dec!(1000.0)).unwrap();
        let trade = Trade::new(
            account,
            Symbol::EURUSD,
            dec!(100.0),
            dec!(1.0),
            Utc::now(),
            TradeKind::Buy,
        );
        assert!(trade.is_ok());
    }

    #[test]
    fn test_new_trade_invalid_quantity() {
        let account = Account::new("John".to_string(), dec!(1000.0)).unwrap();
        let trade = Trade::new(
            account,
            Symbol::EURUSD,
            dec!(-100.0),
            dec!(1.0),
            Utc::now(),
            TradeKind::Buy,
        );
        assert!(trade.is_err());
        assert_eq!(trade.err().unwrap(), Error::InvalidQuantity(dec!(-100.0)));
    }

    #[test]
    fn test_new_trade_invalid_price() {
        let account = Account::new("John".to_string(), dec!(1000.0)).unwrap();
        let trade = Trade::new(
            account,
            Symbol::EURUSD,
            dec!(100.0),
            dec!(0.0),
            Utc::now(),
            TradeKind::Buy,
        );
        assert!(trade.is_err());
        assert_eq!(trade.err().unwrap(), Error::InvalidPrice(dec!(0.0)));
    }
}
