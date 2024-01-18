use super::{Error, Result};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> Result<Self> {
        if name.is_empty() {
            return Err(Error::InvalidEmptyName);
        }
        if name.len() > 50 {
            return Err(Error::InvalidNameLength);
        }
        Ok(Self { name })
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        let user = User::new("John".to_string()).unwrap();
        assert_eq!(user.name(), "John");
    }

    #[test]
    fn test_user_eq() {
        let user1 = User::new("John".to_string()).unwrap();
        let user2 = User::new("John".to_string()).unwrap();
        assert_eq!(user1, user2);
    }

    #[test]
    fn test_user_with_empty_name() {
        let user_err = User::new("".to_string()).unwrap_err();
        assert_eq!(user_err, Error::InvalidEmptyName);
    }

    #[test]
    fn test_user_with_long_name() {
        let user_err = User::new("A".repeat(51)).unwrap_err();
        assert_eq!(user_err, Error::InvalidNameLength);
    }
}
