use super::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Portfolio {
    positions: Vec<Position>,
}
