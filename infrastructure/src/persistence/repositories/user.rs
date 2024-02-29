use async_trait::async_trait;
use xhartlet_domain::user::{Email, User, UserId};

#[derive(Debug)]
pub enum Error {
    ConnectionError,
}

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait Repository {
    async fn create(&self, user: &User) -> Result<Option<UserId>, Error>;
    async fn read_by_email(&self, email: &Email) -> Result<Option<User>, Error>;
    async fn update(&self, user: &User) -> Result<Option<User>, Error>;
    async fn delete(&self, id: i32) -> Result<(), Error>;
}
