use super::common::{Error, Result};
use super::value_objects::id::Id;

const MAXIMUM_NAME_LEN: usize = 50;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct User {
    id: Id,
    name: String,
}

impl User {
    fn new(id: Id, name: String) -> Result<Self> {
        if name.is_empty() {
            return Err(Error::InvalidEmptyName);
        }
        if name.len() > MAXIMUM_NAME_LEN {
            return Err(Error::InvalidNameLength(name.len(), MAXIMUM_NAME_LEN));
        }
        Ok(Self { id, name })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let user = User::new(Id(1), "Alice".to_string()).unwrap();
        assert_eq!(user.id, Id(1));
        assert_eq!(user.name, user.name);
    }

    #[test]
    fn test_empty_name() {
        let user = User::new(Id(1), "".to_string());
        assert_eq!(user.is_err(), true);
        assert_eq!(user.unwrap_err(), Error::InvalidEmptyName);
    }

    #[test]
    fn test_lenghty_name() {
        let user = User::new(Id(1), "A".repeat(51));
        assert_eq!(user.is_err(), true);
        assert_eq!(user.unwrap_err(), Error::InvalidNameLength(51, 50));
    }

    #[test]
    fn test_eq() {
        let user1 = User::new(Id(2), "Alice".to_string()).unwrap();
        let user2 = User::new(Id(2), "Alice".to_string()).unwrap();
        assert_eq!(user1, user2);
    }
}
