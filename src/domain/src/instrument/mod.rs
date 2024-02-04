mod entities;
mod value_objects;

use core::fmt;
use std::error::Error;

pub use entities::History;
pub use value_objects::Timeframe;

use crate::common::{Price, Result, Ticker, Volume};

#[derive(Debug)]
pub enum InstrumentError {}

impl Error for InstrumentError {}

impl fmt::Display for InstrumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("InstrumentError")
    }
}

pub struct Instrument {
    ticker: Ticker,
    decimal_point: u32,
    history: History,
}

impl Instrument {
    pub fn new(ticker: Ticker, decimal_point: u32, timeframe: Timeframe) -> Self {
        Self {
            ticker,
            decimal_point,
            history: History::new(timeframe),
        }
    }

    pub fn add_candle(
        &mut self,
        open: Price,
        high: Price,
        low: Price,
        close: Price,
        volume: Volume,
    ) {
        self.history.add(
            open.round_dp(self.decimal_point),
            high.round_dp(self.decimal_point),
            low.round_dp(self.decimal_point),
            close.round_dp(self.decimal_point),
            volume,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let instrument = Instrument::new(Ticker::EURUSD, 2, Timeframe::M5);
        assert_eq!(instrument.ticker, Ticker::EURUSD);
        assert_eq!(instrument.decimal_point, 2);
        assert_eq!(instrument.history.timeframe(), Timeframe::M5);
    }
}
