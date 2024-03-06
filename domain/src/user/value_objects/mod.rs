use core::fmt;

pub type UserId = uuid::Uuid;
pub type Email = String;
pub type Username = String;

#[derive(Debug, Clone)]
pub struct Password(pub String);

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "***")
    }
}
