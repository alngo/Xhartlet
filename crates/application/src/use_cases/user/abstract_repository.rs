use async_trait::async_trait;

use xhartlet_domain::user::{Email, User, UserId};

#[derive(Debug)]
pub enum Error {
    ConnectionError,
    CreateError,
}

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait Repository {
    async fn create(&self, user: &User) -> Result<Option<UserId>, Error>;
    async fn is_email_taken(&self, email: &Email) -> Result<bool, Error>;
    async fn read_by_email(&self, email: &Email) -> Result<Option<User>, Error>;
    async fn update(&self, id: UserId, user: &User) -> Result<Option<UserId>, Error>;
    async fn delete(&self, id: UserId) -> Result<(), Error>;
}
