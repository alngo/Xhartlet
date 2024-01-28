use std::collections::HashMap;

use super::intrument::Instrument;
use super::order_book::OrderBook;
use super::ticker::Ticker;

pub struct Market {
    name: String,
    instruments: HashMap<Ticker, Instrument>,
    order_books: HashMap<Ticker, OrderBook>,
}
