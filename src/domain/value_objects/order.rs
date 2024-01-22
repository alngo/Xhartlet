#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind {
    MARKET,
    LIMIT,
    STOP,
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
    fn test_Kind() {
        assert_eq!(Kind::MARKET, Kind::MARKET);
    }

    #[test]
    fn test_Status() {
        assert_eq!(Status::PENDING, Status::PENDING);
    }
}
