#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction {
    LONG,
    SHORT,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction() {
        assert_eq!(Direction::LONG, Direction::LONG);
        assert_ne!(Direction::LONG, Direction::SHORT);
    }
}
