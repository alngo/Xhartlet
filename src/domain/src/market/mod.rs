mod entities;
mod value_objects;

pub use value_objects::MarketId;

use crate::instrument::InstrumentId;

pub struct Market {
    id: MarketId,
    instruments: Vec<InstrumentId>,
}
