use core::fmt;
use std::ops::Div;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Timeframe {
    M1 = 1,
    M5 = 5,
    M30 = 30,
}

impl Timeframe {
    pub fn from_minutes(minutes: u32) -> Option<Self> {
        match minutes {
            1 => Some(Self::M1),
            5 => Some(Self::M5),
            30 => Some(Self::M30),
            _ => None,
        }
    }
}

impl Div<Timeframe> for Timeframe {
    type Output = u32;

    fn div(self, rhs: Timeframe) -> u32 {
        self as u32 / rhs as u32
    }
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
