use core::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    InvalidEmptyName,
    InvalidNameLength(usize, usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidEmptyName => write!(f, "User name cannot be empty"),
            Error::InvalidNameLength(len, max) => write!(f, "Something {}, {}", len, max),
        }
    }
}
