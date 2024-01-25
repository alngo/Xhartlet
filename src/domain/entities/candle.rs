use super::value_objects::{price::Price, timeframe::Timeframe};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Candle {
    timeframe: Timeframe,
    open: Price,
    high: Price,
    low: Price,
    close: Price,
}

impl Candle {
    pub fn new(timeframe: Timeframe, open: Price, high: Price, low: Price, close: Price) -> Self {
        Self {
            timeframe,
            open,
            high,
            low,
            close,
        }
    }

    pub fn timeframe(&self) -> Timeframe {
        self.timeframe
    }
}
