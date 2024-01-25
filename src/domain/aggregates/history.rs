use super::{Error, Result};

use super::candle::Candle;
use super::timeframe::Timeframe;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct History {
    timeframe: Timeframe,
    candles: Vec<Candle>,
}

impl History {
    pub fn new(timeframe: Timeframe) -> Self {
        Self {
            timeframe,
            candles: Vec::new(),
        }
    }

    pub fn add_candle(&mut self, candle: Candle) -> Result<()> {
        if candle.timeframe() != self.timeframe {
            return Err(Error::InvalidTimeframe(candle.timeframe()));
        }
        self.candles.push(candle);
        Ok(())
    }

    pub fn candles(&self) -> &[Candle] {
        &self.candles
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::domain::value_objects::price::Price;

    use super::*;

    #[test]
    fn test_history_new() {
        let timeframe = Timeframe::M1;
        let history = History::new(timeframe);
        assert_eq!(history.timeframe, timeframe);
        assert_eq!(history.candles, Vec::new());
    }

    #[test]
    fn test_add_valid_candle() {
        let price = Price(dec!(1.0000));
        let timeframe = Timeframe::M1;
        let mut history = History::new(timeframe);
        let candle = Candle::new(timeframe, price, price, price, price);
        let result = history.add_candle(candle);
        assert!(result.is_ok());
        assert_eq!(history.candles.len(), 1);
        assert_eq!(history.candles()[0], candle);
    }

    #[test]
    fn test_add_invalid_candle() {
        let price = Price(dec!(1.0000));
        let timeframe = Timeframe::M1;
        let mut history = History::new(timeframe);
        let candle = Candle::new(Timeframe::M5, price, price, price, price);
        let result = history.add_candle(candle);
        assert!(result.is_err());
        assert_eq!(history.candles.len(), 0);
    }
}
