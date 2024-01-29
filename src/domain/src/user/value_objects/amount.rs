use crate::common::value_objects::Price;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Amount {
    value: Price,
}

impl Amount {
    pub fn new(amount: Price) -> Self {
        Self {
            value: amount.round_dp(2),
        }
    }

    pub fn value(&self) -> &Price {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_new() {
        let amount = Amount::new(dec!(100.23));
        assert_eq!(*amount.value(), dec!(100.23));

        let amount = Amount::new(dec!(100.2334));
        assert_eq!(*amount.value(), dec!(100.23));
    }
}
