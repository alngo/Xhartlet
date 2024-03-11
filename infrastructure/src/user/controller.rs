use xhartlet_application::{
    interfaces::common::{cryptography::Cryptography, use_case::UseCase},
    use_cases::user::{
        gateway::Gateway,
        register::{Register, Request},
    },
};
use xhartlet_domain::user::Password;

use super::model;
use crate::interfaces::common::present::Present;

pub struct Controller<'g, 'p, 'c, G, P, C> {
    pub gateway: &'g G,
    pub cryptography: &'c C,
    pub presenter: &'p P,
}

impl<'g, 'p, 'c, G, P, C> Controller<'g, 'p, 'c, G, P, C>
where
    G: Gateway,
    C: Cryptography,
    P: Present<model::register::Result>,
{
    pub fn new(gateway: &'g G, cryptography: &'c C, presenter: &'p P) -> Self {
        Self {
            gateway,
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
        let interactor = Register::new(self.gateway, self.cryptography);
        let response = interactor.execute(request).await;
        self.presenter.present(response);
    }
}
