use core::fmt;
use std::ops::{Mul, Sub};

use rust_decimal::Decimal;

use super::price::Price;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub struct Quantity(pub u32);

impl Mul<Price> for Quantity {
    type Output = Price;
    fn mul(self, rhs: Price) -> Self::Output {
        Price(rhs.0 * Decimal::from(self.0))
    }
}

impl Sub<Quantity> for Quantity {
    type Output = Quantity;
    fn sub(self, rhs: Quantity) -> Self::Output {
        Quantity(self.0 - rhs.0)
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quantity({})", self.0)
    }
}

impl Quantity {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
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

    #[test]
    fn test_is_zero() {
        let quantity = Quantity(2);
        assert_ne!(quantity.is_zero(), true);

        let quantity = Quantity(0);
        assert_eq!(quantity.is_zero(), true);
    }
}
