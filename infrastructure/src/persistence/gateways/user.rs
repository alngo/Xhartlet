use async_trait::async_trait;
use xhartlet_application::interfaces::gateways::user::{Error, Gateway as UserGateway};
use xhartlet_domain::user::{Email, User};

pub struct Gateway<'r, R> {
    repository: &'r R,
}

impl<'r, R> Gateway<'r, R> {
    pub fn new(repository: &'r R) -> Gateway<'r, R> {
        Gateway { repository }
    }
}

#[async_trait(?Send)]
impl<'r, R> UserGateway for Gateway<'r, R> {
    async fn create(&self, user: &User) -> Result<(), Error> {
        todo!()
    }
    async fn find_by_email(&self, email: Email) -> Result<Option<User>, Error> {
        todo!()
    }
}
