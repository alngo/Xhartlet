use core::fmt;
use std::error::Error;

use async_trait::async_trait;

#[derive(Debug)]
pub struct ApplicationError {
    pub message: String,
}

impl Error for ApplicationError {}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApplicationError: {}", self.message)
    }
}

#[async_trait(?Send)]
pub trait UseCase {
    type Request;
    type Response;

    async fn execute(&self, request: Self::Request) -> Result<Self::Response, ApplicationError>;
}
