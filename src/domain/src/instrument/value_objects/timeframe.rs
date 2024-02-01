use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Timeframe {
    M1,
    M5,
    M30,
}

impl fmt::Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::M1 => write!(f, "1m"),
            Self::M5 => write!(f, "5m"),
            Self::M30 => write!(f, "30m"),
        }
    }
}
