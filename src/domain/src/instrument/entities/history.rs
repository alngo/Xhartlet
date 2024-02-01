use super::Candle;
use crate::instrument::Timeframe;

pub struct History {
    timeframe: Timeframe,
    history: Vec<Candle>,
}
