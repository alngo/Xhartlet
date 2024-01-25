use super::position::Direction;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Kind {
    MARKET(Direction),
    LIMIT(Direction),
    STOP(Direction),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Status {
    PENDING,
    FILLED,
    CANCELLED,
}

impl Kind {
    pub fn direction(&self) -> &Direction {
        match self {
            Self::MARKET(direction) => direction,
            Self::LIMIT(direction) => direction,
            Self::STOP(direction) => direction,
        }
    }
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
