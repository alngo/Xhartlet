use async_trait::async_trait;

use super::error::ApplicationError;

#[async_trait(?Send)]
pub trait UseCase {
    type Request;
    type Response;

    async fn execute(&self, request: Self::Request) -> Result<Self::Response, ApplicationError>;
}
