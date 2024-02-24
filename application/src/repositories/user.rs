use async_trait::async_trait;

use xhartlet_domain::user::{value_objects::UserId, User};

#[derive(Debug)]
pub enum Error {
    NotFound,
    ConnectionError,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub user: User,
}

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait Repository: Send + Sync {
    async fn create(&self, record: Record) -> Result<UserId, Error>;
    async fn read(&self, id: UserId) -> Result<Record, Error>;
    async fn update(&self, id: UserId, record: Record) -> Result<Record, Error>;
    async fn delete(&self, id: UserId) -> Result<(), Error>;
}
