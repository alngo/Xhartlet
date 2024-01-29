#[derive(Debug)]
pub enum CommonError {
    InvalidInput,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommonError::InvalidInput => write!(f, "Invalid input!"),
        }
    }
}
