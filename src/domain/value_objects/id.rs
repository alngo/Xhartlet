#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Id(pub u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        assert_eq!(Id(1), Id(1));
    }
}
