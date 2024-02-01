mod entities;
mod value_objects;

pub use entities::{Candle, History};
pub use value_objects::{Ticker, Timeframe};

pub struct Instrument {
    ticker: Ticker,
    decimal_point: u8,
    history: History,
}
