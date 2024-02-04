use crate::common::{Price, Volume};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candle {
    pub open: Price,
    pub high: Price,
    pub low: Price,
    pub close: Price,
    pub volume: Volume,
}

impl Candle {
    pub fn new(open: Price, high: Price, low: Price, close: Price, volume: Volume) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
        }
    }

    pub fn open(&self) -> Price {
        self.open
    }

    pub fn high(&self) -> Price {
        self.high
    }

    pub fn low(&self) -> Price {
        self.low
    }

    pub fn close(&self) -> Price {
        self.close
    }

    pub fn volume(&self) -> Volume {
        self.volume
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_new() {
        let candle = Candle::new(dec!(1.0), dec!(2.0), dec!(0.5), dec!(1.5), dec!(100));
        assert_eq!(candle.open, dec!(1.0));
        assert_eq!(candle.high, dec!(2.0));
        assert_eq!(candle.low, dec!(0.5));
        assert_eq!(candle.close, dec!(1.5));
        assert_eq!(candle.volume, dec!(100));
    }
}
