use async_trait::async_trait;

use xhartlet_domain::user::{Email, User};

#[derive(Debug)]
pub enum Error {
    NotFound,
    ConnectionError,
}

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait Gateway {
    async fn create(&self, user: &User) -> Result<(), Error>;
    async fn find_by_email(&self, email: Email) -> Result<Option<User>, Error>;
}
