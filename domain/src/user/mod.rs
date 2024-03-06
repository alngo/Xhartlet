pub mod value_objects;

use crate::common::{DomainError, Result};
pub use value_objects::{Email, Password, UserId, Username};

#[derive(Debug, Clone)]
pub struct User {
    pub email: Email,
    pub username: Username,
    pub password: Password,
}

pub enum UserError {
    UsernameTooShort,
    UsernameTooLong,
    PasswordTooShort,
    InvalidEmail,
}

impl From<UserError> for DomainError {
    fn from(error: UserError) -> Self {
        match error {
            UserError::UsernameTooShort => DomainError {
                message: "Username too short".to_string(),
            },
            UserError::UsernameTooLong => DomainError {
                message: "Username too long".to_string(),
            },
            UserError::PasswordTooShort => DomainError {
                message: "Password too short".to_string(),
            },
            UserError::InvalidEmail => DomainError {
                message: "Invalid email".to_string(),
            },
        }
    }
}

impl User {
    pub fn new(email: Email, username: Username, password: Password) -> Result<Self> {
        if username.len() < 3 {
            return Err(UserError::UsernameTooShort.into());
        }
        if username.len() > 100 {
            return Err(UserError::UsernameTooLong.into());
        }
        if password.0.len() < 8 {
            return Err(UserError::PasswordTooShort.into());
        }
        if !email.contains('@') {
            return Err(UserError::InvalidEmail.into());
        }
        Ok(User {
            username,
            password,
            email,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let result = User::new(
            "email@email.email".to_string(),
            "username".to_string(),
            Password("password".to_string()),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_user_invalid_username() {
        let result = User::new(
            "email@email.email".to_string(),
            "u".to_string(),
            Password("password".to_string()),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_user_invalid_password() {
        let result = User::new(
            "email@email.email".to_string(),
            "username".to_string(),
            Password("p".to_string()),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_user_invalid_email() {
        let result = User::new(
            "emailemail.email".to_string(),
            "username".to_string(),
            Password("password".to_string()),
        );
        assert!(result.is_err());
    }
}
