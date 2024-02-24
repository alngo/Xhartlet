use std::error::Error;

use async_trait::async_trait;

use super::user::create::CreateUserError;

#[derive(Debug)]
pub enum ApplicationError {
    CreateUserError(CreateUserError),
}

impl From<CreateUserError> for ApplicationError {
    fn from(e: CreateUserError) -> Self {
        ApplicationError::CreateUserError(e)
    }
}

#[async_trait(?Send)]
pub trait UseCase {
    type Request;
    type Response;

    async fn execute(&self, request: Self::Request) -> Result<Self::Response, ApplicationError>;
}
