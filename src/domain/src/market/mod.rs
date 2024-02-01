mod entities;
mod value_objects;

use std::collections::HashSet;

pub use value_objects::MarketId;

use crate::instrument::Ticker;

pub struct Market {
    id: MarketId,
    instruments: HashSet<Ticker>,
}
