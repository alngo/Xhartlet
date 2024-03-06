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
pub trait Gateway {
    async fn create(&self, user: &User) -> Result<UserId, Error>;
    async fn is_email_taken(&self, email: &Email) -> Result<bool, Error>;
}
