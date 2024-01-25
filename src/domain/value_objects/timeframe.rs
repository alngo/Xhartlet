use core::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Timeframe {
    M1,
    M5,
    M15,
    M30,
    H1,
    H4,
    D1,
    W1,
    MN1,
}

impl fmt::Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Timeframe::M1 => write!(f, "M1"),
            Timeframe::M5 => write!(f, "M5"),
            Timeframe::M15 => write!(f, "M15"),
            Timeframe::M30 => write!(f, "M30"),
            Timeframe::H1 => write!(f, "H1"),
            Timeframe::H4 => write!(f, "H4"),
            Timeframe::D1 => write!(f, "D1"),
            Timeframe::W1 => write!(f, "W1"),
            Timeframe::MN1 => write!(f, "MN1"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeframe() {
        assert_eq!(Timeframe::M1, Timeframe::M1);
    }
}
