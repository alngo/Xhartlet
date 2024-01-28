use super::entities::order::Order;

pub struct OrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>,
}
