mod entities;
mod value_objects;

use core::fmt;
use std::error::Error;

pub use entities::Candle;
pub use value_objects::Timeframe;

use crate::common::{Price, Result, Ticker, Volume};

#[derive(Debug)]
pub enum InstrumentError {
    InvalidTimeframe(Timeframe, Timeframe),
}

impl Error for InstrumentError {}

impl fmt::Display for InstrumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidTimeframe(actual, expected) => write!(
                f,
                "Invalid timeframe: {}, expected: {} or greater",
                actual, expected
            ),
        }
    }
}

pub struct Instrument {
    ticker: Ticker,
    decimal_point: u32,
    timeframe: Timeframe,
    history: Vec<Candle>,
}

impl Instrument {
    pub fn new(ticker: Ticker, decimal_point: u32, timeframe: Timeframe) -> Self {
        Self {
            ticker,
            decimal_point,
            timeframe,
            history: Vec::new(),
        }
    }

    pub fn add(&mut self, open: Price, high: Price, low: Price, close: Price, volume: Volume) {
        let candle = Candle::new(
            open.round_dp(self.decimal_point),
            high.round_dp(self.decimal_point),
            low.round_dp(self.decimal_point),
            close.round_dp(self.decimal_point),
            volume,
        );
        self.history.push(candle);
    }

    pub fn history(&self, timeframe: Timeframe) -> Result<&[Candle]> {
        if self.timeframe > timeframe {
            return Err(Box::new(InstrumentError::InvalidTimeframe(
                timeframe,
                self.timeframe,
            )));
        }
        if self.timeframe == timeframe {
            return Ok(&self.history);
        } else {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_new() {
        let instrument = Instrument::new(Ticker::EURUSD, 2, Timeframe::M5);
        assert_eq!(instrument.ticker, Ticker::EURUSD);
        assert_eq!(instrument.decimal_point, 2);
        assert_eq!(instrument.timeframe, Timeframe::M5);
        assert_eq!(instrument.history.len(), 0);
    }

    #[test]
    fn test_add() {
        let mut instrument = Instrument::new(Ticker::EURUSD, 2, Timeframe::M5);
        instrument.add(
            dec!(1.2345),
            dec!(1.2346),
            dec!(1.2344),
            dec!(1.2345),
            dec!(100),
        );
        assert_eq!(instrument.history.len(), 1);
        assert_eq!(instrument.history[0].open, dec!(1.23));
        assert_eq!(instrument.history[0].high, dec!(1.23));
        assert_eq!(instrument.history[0].low, dec!(1.23));
        assert_eq!(instrument.history[0].close, dec!(1.23));
        assert_eq!(instrument.history[0].volume, dec!(100));
    }

    #[test]
    fn test_invalid_history() {
        let mut instrument = Instrument::new(Ticker::EURUSD, 2, Timeframe::M5);
        instrument.add(
            dec!(1.2345),
            dec!(1.2346),
            dec!(1.2344),
            dec!(1.2345),
            dec!(100),
        );
        let history = instrument.history(Timeframe::M1);
        assert!(history.is_err());
    }

    #[test]
    fn test_same_history() {
        let mut instrument = Instrument::new(Ticker::EURUSD, 2, Timeframe::M5);
        instrument.add(
            dec!(1.2345),
            dec!(1.2346),
            dec!(1.2344),
            dec!(1.2345),
            dec!(100),
        );
        let history = instrument.history(Timeframe::M5);
        assert!(history.is_ok());
    }
}
