use core::fmt;
use std::ops::Mul;

use rust_decimal::Decimal;

use super::price::Price;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Quantity(pub u32);

impl Mul<Price> for Quantity {
    type Output = Price;
    fn mul(self, rhs: Price) -> Self::Output {
        Price(rhs.0 * Decimal::from(self.0))
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quantity({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_quantity_mul_price() {
        let quantity = Quantity(2);
        let price = Price(dec!(21.21));
        let expected = Price(dec!(42.42));
        assert_eq!(quantity * price, expected);
    }
}
