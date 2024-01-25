use super::history::History;
use super::ticker::Ticker;

pub struct Instrument {
    ticker: Ticker,
    history: History,
}

impl Instrument {
    pub fn new(ticker: Ticker, history: History) -> Self {
        Self { ticker, history }
    }
}
