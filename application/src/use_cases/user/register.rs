use async_trait::async_trait;
use xhartlet_domain::user::value_objects::{Email, Password, Username};
use xhartlet_domain::user::User;
use xhartlet_domain::DomainError;

use crate::use_cases::use_case::ApplicationError;
use crate::{
    interfaces::gateways::user::{Error as UserGatewayError, Gateway},
    use_cases::use_case::UseCase,
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

pub struct Register<'g, G> {
    gateway: &'g G,
}

impl<'g, G> Register<'g, G>
where
    G: Gateway,
{
    pub fn new(gateway: &'g G) -> Self {
        Register { gateway }
    }
}

#[async_trait(?Send)]
impl<'g, G> UseCase for Register<'g, G>
where
    G: Gateway,
{
    type Request = Request;
    type Response = Response;

    async fn execute(&self, request: Self::Request) -> Result<Self::Response, ApplicationError> {
        let user = User::new(request.email, request.username, request.password)?;
        let find = self.gateway.find_by_email(user.email.clone()).await?;
        if find.is_some() {
            return Err(ApplicationError {
                message: "Email already exists".to_string(),
            });
        }
        // hash password
        self.gateway.create(&user).await?;
        Ok(Response { user })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interfaces::gateways::user::MockGateway;

    #[tokio::test]
    async fn test_signup() {
        let email = "a@valid.email".to_string();
        let username = "username".to_string();
        let password = Password("password".to_string());
        let request = Request {
            email,
            username,
            password,
        };

        let mut gateway = MockGateway::new();
        gateway.expect_create().times(1).returning(move |_| Ok(()));
        gateway
            .expect_find_by_email()
            .times(1)
            .returning(move |_| Ok(None));

        let use_case = Register::new(&gateway);
        let response = use_case.execute(request).await;
        assert!(response.is_ok());
    }
}
