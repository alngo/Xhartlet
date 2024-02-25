use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait Identifier<Id> {
    async fn new_id(&self) -> Result<Id, NewIdError>;
}

#[derive(Debug)]
pub struct NewIdError;
