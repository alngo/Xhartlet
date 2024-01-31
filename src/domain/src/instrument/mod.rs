mod entities;
mod value_objects;

pub use entities::{Candle, History};
pub use value_objects::{InstrumentId, Timeframe};

pub struct Instrument {
    id: InstrumentId,
    decimal_point: u8,
    history: History,
}
