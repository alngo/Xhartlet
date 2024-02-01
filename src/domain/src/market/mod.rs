mod entities;
mod value_objects;

use std::collections::HashMap;

pub use value_objects::MarketId;

use crate::{common::Ticker, instrument::Instrument};

pub struct Market {
    id: MarketId,
    instruments: HashMap<Ticker, Instrument>,
}
