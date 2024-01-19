use super::{
    order::Order, position_kind::PositionKind, position_status::PositionStatus, price::Price,
    quantity::Quantity, symbol::Symbol,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    pub id: i32,
    pub symbol: Symbol,
    pub quantity: Quantity,
    pub entry_price: Price,
    pub exit_price: Price,
    pub take_profit: Order,
    pub stop_loss: Order,
    pub kind: PositionKind,
    pub status: PositionStatus,
}
