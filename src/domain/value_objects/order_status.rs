#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OrderStatus {
    New,
    Filled,
    Cancelled,
    Rejected,
    Expired,
}
