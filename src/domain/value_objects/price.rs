use core::fmt;
use std::ops::Mul;

use rust_decimal::Decimal;

use super::quantity::Quantity;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Price(pub Decimal);

impl Mul<Quantity> for Price {
    type Output = Price;
    fn mul(self, rhs: Quantity) -> Self::Output {
        Price(self.0 * Decimal::from(rhs.0))
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Price({})", self.0)
    }
}

impl Price {
    pub fn is_zero(&self) -> bool {
        self.0 == Decimal::ZERO
    }

    pub fn is_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_price_mul_quantity() {
        let price = Price(dec!(21.21));
        let quantity = Quantity(2);
        let expected = Price(dec!(42.42));
        assert_eq!(price * quantity, expected);
    }
}
