use super::position::Direction;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind {
    MARKET(Direction),
    LIMIT(Direction),
    STOP(Direction),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Status {
    PENDING,
    FILLED,
    CANCELLED,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind() {
        assert_eq!(
            Kind::MARKET(Direction::SHORT),
            Kind::MARKET(Direction::SHORT)
        );
    }

    #[test]
    fn test_status() {
        assert_eq!(Status::PENDING, Status::PENDING);
    }
}
