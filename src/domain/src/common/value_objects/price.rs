use rust_decimal::Decimal;

pub type Price = Decimal;

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_eq() {
        let price1: Price = dec!(1.2345);
        let price2: Price = dec!(1.2345);
        assert_eq!(price1, price2);
    }

    #[test]
    fn test_mul() {
        let price1: Price = dec!(2.0);
        let price2: Price = dec!(1.2345);
        let price3: Price = price1 * price2;
        assert_eq!(price3, dec!(2.4690));
    }
}
