use super::common::{Error, Result};
use super::entities::order::Order;
use super::value_objects::position::Direction;
use super::value_objects::{id::Id, price::Price, quantity::Quantity, ticker::Ticker};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    id: Id,
    account_id: Id,
    portfolio_id: Id,
    market_id: Id,
    ticker: Ticker,
    quantity: Quantity,
    direction: Direction,
    entry_price: Price,
    exit_price: Option<Price>,
    stop_loss: Option<Order>,
    take_profit: Option<Order>,
}

impl Position {
    pub fn new(
        id: Id,
        account_id: Id,
        portfolio_id: Id,
        market_id: Id,
        ticker: Ticker,
        quantity: Quantity,
        direction: Direction,
        entry_price: Price,
    ) -> Result<Self> {
        if quantity.is_zero() {
            return Err(Error::InvalidQuantity(quantity));
        }
        if entry_price.is_negative() {
            return Err(Error::InvalidPrice(entry_price));
        }
        Ok(Self {
            id,
            account_id,
            portfolio_id,
            market_id,
            ticker,
            quantity,
            direction,
            entry_price,
            exit_price: None,
            stop_loss: None,
            take_profit: None,
        })
    }

    pub fn exit(&mut self, price: Price) -> Result<()> {
        if self.quantity.is_zero() {
            return Err(Error::InvalidQuantity(self.quantity));
        }
        self.quantity = Quantity(0);
        self.exit_price = Some(price);
        Ok(())
    }

    pub fn partial_exit(&mut self, quantity: Quantity, price: Price) -> Result<()> {
        todo!()
    }

    pub fn set_stop_loss(&mut self, stop_loss: Order) -> Result<()> {
        todo!()
    }

    pub fn set_take_profit(&mut self, stop_loss: Order) -> Result<()> {
        todo!()
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
        let entry_price = Price(dec!(1.1394));
        let direction = Direction::SHORT;

        let position = Position::new(
            id,
            account_id,
            portfolio_id,
            market_id,
            ticker,
            quantity,
            direction,
            entry_price,
        )
        .unwrap();

        assert_eq!(position.id, id);
        assert_eq!(position.account_id, account_id);
        assert_eq!(position.portfolio_id, portfolio_id);
        assert_eq!(position.market_id, market_id);
        assert_eq!(position.ticker, ticker);
        assert_eq!(position.quantity, quantity);
        assert_eq!(position.entry_price, entry_price);
        assert_eq!(position.exit_price, None);
        assert_eq!(position.stop_loss, None);
        assert_eq!(position.take_profit, None);
    }

    #[test]
    fn test_exit() {
        let mut position = Position::new(
            Id(1),
            Id(1),
            Id(1),
            Id(1),
            Ticker::EURUSD,
            Quantity(5),
            Direction::LONG,
            Price(dec!(1.1234)),
        )
        .unwrap();

        assert_eq!(position.quantity, Quantity(5));
        assert_eq!(position.exit_price.is_none(), true);
        assert_eq!(position.exit(Price(dec!(1.0000))).is_ok(), true);
        assert_eq!(position.exit_price.is_some(), true);
        assert_eq!(position.exit_price, Some(Price(dec!(1.0000))));
    }
}
