use xhartlet_application::{
    common::{abstract_cryptography::Cryptography, abstract_use_case::UseCase},
    use_cases::user::{
        abstract_repository::Repository,
        register::{Register, Request},
    },
};
use xhartlet_domain::user::Password;

use super::model;
use crate::common::abstract_present::Present;

pub struct Controller<'r, 'p, 'c, R, P, C> {
    pub repository: &'r R,
    pub cryptography: &'c C,
    pub presenter: &'p P,
}

impl<'r, 'p, 'c, R, P, C> Controller<'r, 'p, 'c, R, P, C>
where
    R: Repository,
    C: Cryptography,
    P: Present<model::register::Result>,
{
    pub fn new(repository: &'r R, cryptography: &'c C, presenter: &'p P) -> Self {
        Self {
            repository,
            cryptography,
            presenter,
        }
    }

    pub async fn register(&self, email: &str, name: &str, password: &str) {
        let request = Request {
            email: email.to_string(),
            username: name.to_string(),
            password: Password(password.to_string()),
        };
        let interactor = Register::new(self.repository, self.cryptography);
        let response = interactor.execute(request).await;
        self.presenter.present(response);
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_register() {
    }
}

