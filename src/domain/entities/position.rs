use super::common::{Error, Result};
use super::entities::order::Order;
use super::value_objects::order::Kind;
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
        // TODO: Maybe the check has to be done at higher level to avoid direct dependency with Order
        // TODO: Check Market Price
        if stop_loss.kind() != Kind::STOP(self.direction.opposite()) {
            return Err(Error::InvalidDirection(*stop_loss.kind().direction()));
        }
        if stop_loss.quantity() != self.quantity {
            return Err(Error::InvalidQuantity(stop_loss.quantity()));
        }
        if self.direction == Direction::LONG && stop_loss.price() >= self.entry_price {
            return Err(Error::InvalidPrice(stop_loss.price()));
        }
        if self.direction == Direction::SHORT && stop_loss.price() <= self.entry_price {
            return Err(Error::InvalidPrice(stop_loss.price()));
        }
        self.stop_loss = Some(stop_loss);
        Ok(())
    }

    pub fn set_take_profit(&mut self, take_profit: Order) -> Result<()> {
        // TODO: Maybe the check has to be done at higher level to avoid direct dependency with Order
        // TODO: Check Market Price
        if take_profit.kind() != Kind::LIMIT(self.direction.opposite()) {
            return Err(Error::InvalidDirection(*take_profit.kind().direction()));
        }
        if take_profit.quantity() != self.quantity {
            return Err(Error::InvalidQuantity(take_profit.quantity()));
        }
        if self.direction == Direction::LONG && take_profit.price() <= self.entry_price {
            return Err(Error::InvalidPrice(take_profit.price()));
        }
        if self.direction == Direction::SHORT && take_profit.price() >= self.entry_price {
            return Err(Error::InvalidPrice(take_profit.price()));
        }
        self.take_profit = Some(take_profit);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rust_decimal_macros::dec;

    use crate::domain::value_objects::order::Kind;

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

    #[test]
    fn test_invalid_stop_loss_for_long_position() {
        let id = Id(1);
        let mut position = Position::new(
            id,
            id,
            id,
            id,
            Ticker::EURUSD,
            Quantity(5),
            Direction::LONG,
            Price(dec!(1.1234)),
        )
        .unwrap();

        let mut orders_expectations: HashMap<Order, std::result::Result<_, Error>> = HashMap::new();
        // Order is in the same direction as the position
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(5),
                Price(dec!(1.1234)),
                Kind::STOP(Direction::LONG),
            )
            .unwrap(),
            Err(Error::InvalidDirection(Direction::LONG)),
        );
        // Order quantity is different from the position quantity
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(2),
                Price(dec!(1.1234)),
                Kind::STOP(Direction::SHORT),
            )
            .unwrap(),
            Err(Error::InvalidQuantity(Quantity(2))),
        );
        // Order price is higher than the entry price for a long position
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(5),
                Price(dec!(1.1235)),
                Kind::STOP(Direction::SHORT),
            )
            .unwrap(),
            Err(Error::InvalidPrice(Price(dec!(1.1235)))),
        );

        for (order, expectation) in orders_expectations {
            let result = position.set_stop_loss(order);
            assert!(result.is_err());
            assert_eq!(result, expectation);
        }
    }

    #[test]
    fn test_invalid_stop_loss_for_short_position() {
        let id = Id(1);
        let mut position = Position::new(
            id,
            id,
            id,
            id,
            Ticker::EURUSD,
            Quantity(5),
            Direction::SHORT,
            Price(dec!(1.1234)),
        )
        .unwrap();

        let mut orders_expectations: HashMap<Order, std::result::Result<_, Error>> = HashMap::new();
        // Order is in the same direction as the position
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(5),
                Price(dec!(1.1234)),
                Kind::STOP(Direction::SHORT),
            )
            .unwrap(),
            Err(Error::InvalidDirection(Direction::SHORT)),
        );
        // Order quantity is different from the position quantity
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(2),
                Price(dec!(1.1234)),
                Kind::STOP(Direction::LONG),
            )
            .unwrap(),
            Err(Error::InvalidQuantity(Quantity(2))),
        );
        // Order price is lower than the entry price for a short position
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(5),
                Price(dec!(1.1233)),
                Kind::STOP(Direction::LONG),
            )
            .unwrap(),
            Err(Error::InvalidPrice(Price(dec!(1.1233)))),
        );

        for (order, expectation) in orders_expectations {
            let result = position.set_stop_loss(order);
            assert!(result.is_err());
            assert_eq!(result, expectation);
        }
    }

    #[test]
    fn test_invalid_take_profit_for_long_position() {
        let id = Id(1);
        let mut position = Position::new(
            id,
            id,
            id,
            id,
            Ticker::EURUSD,
            Quantity(5),
            Direction::LONG,
            Price(dec!(1.1234)),
        )
        .unwrap();

        let mut orders_expectations: HashMap<Order, std::result::Result<_, Error>> = HashMap::new();
        // Order is in the same direction as the position
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(5),
                Price(dec!(1.1234)),
                Kind::LIMIT(Direction::LONG),
            )
            .unwrap(),
            Err(Error::InvalidDirection(Direction::LONG)),
        );
        // Order quantity is different from the position quantity
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(2),
                Price(dec!(1.1234)),
                Kind::LIMIT(Direction::SHORT),
            )
            .unwrap(),
            Err(Error::InvalidQuantity(Quantity(2))),
        );
        // Order price is lower than the entry price for a long position
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(5),
                Price(dec!(1.1233)),
                Kind::LIMIT(Direction::SHORT),
            )
            .unwrap(),
            Err(Error::InvalidPrice(Price(dec!(1.1233)))),
        );

        for (order, expectation) in orders_expectations {
            let result = position.set_take_profit(order);
            assert!(result.is_err());
            assert_eq!(result, expectation);
        }
    }

    #[test]
    fn test_invalid_take_profit_for_short_position() {
        let id = Id(1);
        let mut position = Position::new(
            id,
            id,
            id,
            id,
            Ticker::EURUSD,
            Quantity(5),
            Direction::SHORT,
            Price(dec!(1.1234)),
        )
        .unwrap();

        let mut orders_expectations: HashMap<Order, std::result::Result<_, Error>> = HashMap::new();
        // Order is in the same direction as the position
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(5),
                Price(dec!(1.1234)),
                Kind::LIMIT(Direction::SHORT),
            )
            .unwrap(),
            Err(Error::InvalidDirection(Direction::SHORT)),
        );
        // Order quantity is different from the position quantity
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(2),
                Price(dec!(1.1234)),
                Kind::LIMIT(Direction::LONG),
            )
            .unwrap(),
            Err(Error::InvalidQuantity(Quantity(2))),
        );
        // Order price is higher than the entry price for a short position
        orders_expectations.insert(
            Order::new(
                id,
                id,
                id,
                id,
                Ticker::EURUSD,
                Quantity(5),
                Price(dec!(1.1235)),
                Kind::LIMIT(Direction::LONG),
            )
            .unwrap(),
            Err(Error::InvalidPrice(Price(dec!(1.1235)))),
        );

        for (order, expectation) in orders_expectations {
            let result = position.set_take_profit(order);
            assert!(result.is_err());
            assert_eq!(result, expectation);
        }
    }
}
