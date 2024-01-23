use rust_decimal::Decimal;

use super::common::{Error, Result};
use super::value_objects::order::{Kind, Status};
use super::value_objects::{id::Id, price::Price, quantity::Quantity, ticker::Ticker};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Order {
    id: Id,
    account_id: Id,
    portfolio_id: Id,
    market_id: Id,
    ticker: Ticker,
    quantity: Quantity,
    price: Price,
    kind: Kind,
    status: Status,
}

impl Order {
    fn new(
        id: Id,
        account_id: Id,
        portfolio_id: Id,
        market_id: Id,
        ticker: Ticker,
        quantity: Quantity,
        price: Price,
        kind: Kind,
    ) -> Result<Self> {
        if quantity.0 == 0 {
            return Err(Error::InvalidQuantity(quantity));
        }
        if price.0 < Decimal::ZERO {
            return Err(Error::InvalidPrice(price));
        }
        Ok(Self {
            id,
            account_id,
            portfolio_id,
            market_id,
            ticker,
            quantity,
            price,
            kind,
            status: Status::PENDING,
        })
    }

    pub fn cancel(&mut self) -> Result<()> {
        if self.status == Status::PENDING {
            self.status = Status::CANCELLED;
            Ok(())
        } else {
            Err(Error::OrderNotPending(self.id.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_new() {
        let id = Id(1);
        let account_id = Id(1);
        let portfolio_id = Id(1);
        let market_id = Id(1);
        let ticker = Ticker::EURUSD;
        let quantity = Quantity(1000);
        let price = Price(dec!(1.1394));
        let kind = Kind::MARKET;
        let status = Status::PENDING;

        let order = Order::new(
            id.clone(),
            account_id.clone(),
            portfolio_id.clone(),
            market_id.clone(),
            ticker.clone(),
            quantity.clone(),
            price.clone(),
            kind.clone(),
        )
        .unwrap();

        assert_eq!(order.id, id);
        assert_eq!(order.account_id, account_id);
        assert_eq!(order.portfolio_id, portfolio_id);
        assert_eq!(order.market_id, market_id);
        assert_eq!(order.ticker, ticker);
        assert_eq!(order.quantity, quantity);
        assert_eq!(order.price, price);
        assert_eq!(order.kind, kind);
        assert_eq!(order.status, status);
    }

    #[test]
    fn test_new_invalid_quantity() {
        let id = Id(1);
        let quantity = Quantity(0);
        let order = Order::new(
            id.clone(),
            id.clone(),
            id.clone(),
            id.clone(),
            Ticker::EURUSD,
            quantity.clone(),
            Price(dec!(1.1394)),
            Kind::MARKET,
        );

        assert_eq!(order, Err(Error::InvalidQuantity(quantity)));
    }

    #[test]
    fn test_new_invalid_price() {
        let id = Id(1);
        let price = Price(dec!(-1.1394));
        let order = Order::new(
            id.clone(),
            id.clone(),
            id.clone(),
            id.clone(),
            Ticker::EURUSD,
            Quantity(1000),
            price.clone(),
            Kind::MARKET,
        );

        assert_eq!(order, Err(Error::InvalidPrice(price)));
    }

    #[test]
    fn test_cancel() {
        let id = Id(1);
        let mut order = Order::new(
            id.clone(),
            id.clone(),
            id.clone(),
            id.clone(),
            Ticker::EURUSD,
            Quantity(1000),
            Price(dec!(1.1394)),
            Kind::MARKET,
        )
        .unwrap();
        assert_eq!(order.status, Status::PENDING);
        order.cancel().unwrap();
        assert_eq!(order.status, Status::CANCELLED);
    }
}
