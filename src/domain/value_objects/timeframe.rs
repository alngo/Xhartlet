#[derive(Debug, Clone, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeframe() {
        assert_eq!(Timeframe::M1, Timeframe::M1);
    }
}
