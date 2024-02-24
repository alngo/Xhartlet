use async_trait::async_trait;

#[async_trait(?Send)]
pub trait NewId<Id> {
    async fn new_id(&self) -> Result<Id, NewIdError>;
}

#[derive(Debug)]
pub struct NewIdError;
