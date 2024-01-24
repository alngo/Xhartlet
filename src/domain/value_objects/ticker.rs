#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Ticker {
    EURUSD,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker() {
        assert_eq!(Ticker::EURUSD, Ticker::EURUSD);
    }
}
