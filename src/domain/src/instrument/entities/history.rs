use core::fmt;
use std::error::Error;

use crate::common::Result;
use crate::{
    common::{Price, Volume},
    instrument::Timeframe,
};

use super::Candle;

#[derive(Debug)]
pub enum HistoryError {
    InvalidTimeframe(Timeframe, Timeframe),
}

impl Error for HistoryError {}

impl fmt::Display for HistoryError {
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

pub struct History {
    timeframe: Timeframe,
    data: Vec<Candle>,
}

impl History {
    pub fn new(timeframe: Timeframe) -> Self {
        Self {
            timeframe,
            data: Vec::new(),
        }
    }

    pub fn timeframe(&self) -> Timeframe {
        self.timeframe
    }

    pub fn history(&self) -> &[Candle] {
        &self.data
    }

    pub fn add(&mut self, open: Price, high: Price, low: Price, close: Price, volume: Volume) {
        let candle = Candle::new(open, high, low, close, volume);
        self.data.push(candle);
    }

    pub fn history_timeframe(&self, timeframe: Timeframe) -> Result<&[Candle]> {
        if self.timeframe < timeframe {
            return Err(Box::new(HistoryError::InvalidTimeframe(
                self.timeframe,
                timeframe,
            )));
        }
        if self.timeframe == timeframe {
            return Ok(self.history());
        }
        let multiplier = timeframe / self.timeframe;
        let result = self
            .history()
            .chunks_exact(multiplier as usize)
            .map(|chunk| {
                let open = chunk.first().unwrap().open();
                let high = chunk.iter().map(|c| c.high()).max().unwrap();
                let low = chunk.iter().map(|c| c.low()).min().unwrap();
                let close = chunk.last().unwrap().close();
                let volume = chunk.iter().map(|c| c.volume()).sum();
                Candle::new(open, high, low, close, volume)
            })
            .collect::<Vec<Candle>>();
        return Ok(&result);
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_new() {
        let history = History::new(Timeframe::M5);
        assert_eq!(history.timeframe(), Timeframe::M5);
        assert_eq!(history.history().len(), 0);
    }

    #[test]
    fn test_add() {
        let mut history = History::new(Timeframe::M5);
        history.add(dec!(1.0), dec!(2.0), dec!(0.5), dec!(1.5), dec!(100));
        assert_eq!(history.history().len(), 1);
    }

    #[test]
    fn test_invalid_timeframe() {
        let mut history = History::new(Timeframe::M5);
        history.add(dec!(1.0), dec!(2.0), dec!(0.5), dec!(1.5), dec!(100));
        let history = history.history_timeframe(Timeframe::M1);
        assert!(history.is_err());
    }

    #[test]
    fn test_history_timeframe() {
        let mut history = History::new(Timeframe::M5);
        history.add(dec!(1.0), dec!(2.0), dec!(0.5), dec!(1.5), dec!(100));
        history.add(dec!(1.0), dec!(2.0), dec!(0.5), dec!(1.5), dec!(100));
        history.add(dec!(1.0), dec!(2.0), dec!(0.5), dec!(1.5), dec!(100));
        history.add(dec!(1.0), dec!(2.0), dec!(0.5), dec!(1.5), dec!(100));
        history.add(dec!(1.0), dec!(2.0), dec!(0.5), dec!(1.5), dec!(100));
        history.add(dec!(1.0), dec!(2.0), dec!(0.5), dec!(1.5), dec!(100));
        history.add(dec!(1.0), dec!(2.0), dec!(0.5), dec!(1.5), dec!(100));
        history.add(dec!(1.0), dec!(2.0), dec!(0.5), dec!(1.5), dec!(100));

        let history = history.history_timeframe(Timeframe::M5).unwrap();
        assert_eq!(history.len(), 8);
    }
}
