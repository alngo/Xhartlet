use core::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    LONG,
    SHORT,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Direction({})", self)
    }
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Self::LONG => Self::SHORT,
            Self::SHORT => Self::LONG,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction() {
        assert_eq!(Direction::LONG, Direction::LONG);
        assert_ne!(Direction::LONG, Direction::SHORT);
    }

    #[test]
    fn test_direction_opposite() {
        assert_eq!(Direction::LONG.opposite(), Direction::SHORT);
        assert_eq!(Direction::SHORT.opposite(), Direction::LONG);
    }
}
