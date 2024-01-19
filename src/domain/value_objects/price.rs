use rust_decimal::Decimal;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Price {
    value: Decimal,
}

impl Price {
    pub fn new(value: Decimal) -> Self {
        Self { value }
    }

    pub fn value(&self) -> Decimal {
        self.value
    }
}
