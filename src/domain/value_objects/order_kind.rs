#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OrderKind {
    Market,
    Limit,
    Stop,
}
