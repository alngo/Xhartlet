use super::repository::{Error as RepositoryError, Repository};
use async_trait::async_trait;
use xhartlet_application::use_cases::user::gateway::{
    Error as GatewayError, Gateway as UserGateway,
};
use xhartlet_domain::user::{Email, User, UserId};

pub struct Gateway<'r, R> {
    repository: &'r R,
}

impl<'r, R> Gateway<'r, R>
where
    R: Repository,
{
    pub fn new(repository: &'r R) -> Gateway<'r, R> {
        Gateway { repository }
    }
}

impl From<RepositoryError> for GatewayError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::ConnectionError => GatewayError::ConnectionError,
        }
    }
}

#[async_trait(?Send)]
impl<'r, R> UserGateway for Gateway<'r, R>
where
    R: Repository,
{
    async fn create(&self, user: &User) -> Result<UserId, GatewayError> {
        let response = self.repository.create(user).await?;
        match response {
            Some(id) => Ok(id),
            None => Err(GatewayError::CreateError),
        }
    }

    async fn is_email_taken(&self, email: &Email) -> Result<bool, GatewayError> {
        let response = self.repository.read_by_email(email).await?;
        match response {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use xhartlet_domain::user::Password;
    use xhartlet_domain::user::User;
    use xhartlet_domain::user::UserId;

    use super::*;
    use crate::user::repository::MockRepository;

    #[tokio::test]
    async fn test_create() {
        let user = User::new(
            "a@valid.email".to_string(),
            "username".to_string(),
            Password("password".to_string()),
        )
        .unwrap();
        let id = UserId::new_v4();
        let mut repository = MockRepository::new();
        repository
            .expect_create()
            .times(1)
            .returning(move |_| Ok(Some(id)));
        let gateway = Gateway::new(&repository);
        let response = gateway.create(&user).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap(), id);
    }

    #[tokio::test]
    async fn is_email_taken() {
        let email: Email = "a@valid.email".to_string();
        let mut repository = MockRepository::new();
        repository
            .expect_read_by_email()
            .times(1)
            .returning(move |_| {
                Ok(Some(
                    User::new(
                        "a@valid.email".to_string(),
                        "username".to_string(),
                        Password("password".to_string()),
                    )
                    .unwrap(),
                ))
            });
        let gateway = Gateway::new(&repository);
        let response = gateway.is_email_taken(&email).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap(), true);
    }
}
