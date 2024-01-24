use core::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Id(pub u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        assert_eq!(Id(1), Id(1));
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id({})", self.0)
    }
}
