use crate::common::{Price, Volume};

pub struct Candle {
    open: Price,
    high: Price,
    low: Price,
    close: Price,
    volume: Volume,
}
