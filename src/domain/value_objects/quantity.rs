#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Quantity {
    value: i32,
}

impl Quantity {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
