use async_trait::async_trait;
use xhartlet_domain::user::value_objects::{Email, Password, Username};
use xhartlet_domain::user::User;
use xhartlet_domain::DomainError;

use super::abstract_gateway::{Error as UserGatewayError, Gateway};
use crate::common::{
    abstract_cryptography::{Cryptography, CryptographyError},
    abstract_use_case::UseCase,
    error::ApplicationError,
};

#[derive(Debug, Clone)]
pub struct Request {
    pub email: Email,
    pub username: Username,
    pub password: Password,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub user: User,
}

impl From<UserGatewayError> for ApplicationError {
    fn from(e: UserGatewayError) -> Self {
        ApplicationError {
            message: format!("Create user error: {:?}", e),
        }
    }
}

impl From<DomainError> for ApplicationError {
    fn from(e: DomainError) -> Self {
        ApplicationError {
            message: format!("Create user error: {:?}", e),
        }
    }
}

impl From<CryptographyError> for ApplicationError {
    fn from(e: CryptographyError) -> Self {
        ApplicationError {
            message: format!("Create user error: {:?}", e),
        }
    }
}

pub struct Register<'g, 'c, G, C> {
    gateway: &'g G,
    cryptography: &'c C,
}

impl<'g, 'c, G, C> Register<'g, 'c, G, C>
where
    G: Gateway,
    C: Cryptography,
{
    pub fn new(gateway: &'g G, cryptography: &'c C) -> Self {
        Register {
            gateway,
            cryptography,
        }
    }
}

#[async_trait(?Send)]
impl<'g, 'c, G, C> UseCase for Register<'g, 'c, G, C>
where
    G: Gateway,
    C: Cryptography,
{
    type Request = Request;
    type Response = Response;

    async fn execute(&self, request: Self::Request) -> Result<Self::Response, ApplicationError> {
        let mut user = User::new(request.email, request.username, request.password)?;
        if self.gateway.is_email_taken(&user.email).await? {
            return Err(ApplicationError {
                message: "Email already exists".to_string(),
            });
        }
        user.password = Password(self.cryptography.hash(&user.password.0).await?);
        self.gateway.create(&user).await?;
        Ok(Response { user })
    }
}

#[cfg(test)]
mod tests {
    use xhartlet_domain::user::UserId;

    use super::*;
    use crate::common::abstract_cryptography::MockCryptography;
    use crate::use_cases::user::abstract_gateway::MockGateway;

    #[tokio::test]
    async fn test_register() {
        let email = "a@valid.email".to_string();
        let username = "username".to_string();
        let password = Password("password".to_string());
        let request = Request {
            email,
            username,
            password,
        };

        let mut gateway = MockGateway::new();
        gateway
            .expect_create()
            .times(1)
            .returning(move |_| Ok(UserId::new_v4()));
        gateway
            .expect_is_email_taken()
            .times(1)
            .returning(move |_| Ok(false));

        let mut cryptography = MockCryptography::new();
        cryptography
            .expect_hash()
            .times(1)
            .returning(move |_| Ok("hashed".to_string()));

        let use_case = Register::new(&gateway, &cryptography);
        let response = use_case.execute(request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_register_with_existing_email() {
        let email = "a@valid.email".to_string();
        let username = "username".to_string();
        let password = Password("password".to_string());
        let request = Request {
            email,
            username,
            password,
        };

        let mut gateway = MockGateway::new();
        let cryptography = MockCryptography::new();
        gateway
            .expect_create()
            .times(0)
            .returning(move |_| Ok(UserId::new_v4()));
        gateway
            .expect_is_email_taken()
            .times(1)
            .returning(move |_| Ok(true));

        let use_case = Register::new(&gateway, &cryptography);
        let response = use_case.execute(request).await;
        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_register_crypto_error() {
        let email = "a@valid.email".to_string();
        let username = "username".to_string();
        let password = Password("password".to_string());
        let request = Request {
            email,
            username,
            password,
        };

        let mut gateway = MockGateway::new();
        gateway
            .expect_create()
            .times(0)
            .returning(move |_| Ok(UserId::new_v4()));
        gateway
            .expect_is_email_taken()
            .times(1)
            .returning(move |_| Ok(false));

        let mut cryptography = MockCryptography::new();
        cryptography
            .expect_hash()
            .times(1)
            .returning(move |_| Err(CryptographyError));

        let use_case = Register::new(&gateway, &cryptography);
        let response = use_case.execute(request).await;
        assert!(response.is_err());
    }
}
