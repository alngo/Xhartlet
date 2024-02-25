use async_trait::async_trait;
use xhartlet_domain::user::value_objects::{Password, UserId, Username};
use xhartlet_domain::user::User;

use crate::use_cases::use_case::ApplicationError;
use crate::{
    interfaces::repositories::{
        identifier::{Identifier, NewIdError},
        user::{Error as UserRepositoryError, Record, Repository},
    },
    use_cases::use_case::UseCase,
};

#[derive(Debug)]
pub struct Request {
    pub username: Username,
    pub password: Password,
}

#[derive(Debug)]
pub struct Response {
    pub id: UserId,
}

impl From<UserRepositoryError> for ApplicationError {
    fn from(e: UserRepositoryError) -> Self {
        ApplicationError {
            message: format!("Create user error: {:?}", e),
        }
    }
}

impl From<NewIdError> for ApplicationError {
    fn from(e: NewIdError) -> Self {
        ApplicationError {
            message: format!("Create user error: {:?}", e),
        }
    }
}

pub struct SignUp<'r, 'i, R, I> {
    repository: &'r R,
    identifier: &'i I,
}

impl<'r, 'i, R, I> SignUp<'r, 'i, R, I>
where
    R: Repository,
    I: Identifier<UserId>,
{
    pub fn new(repository: &'r R, identifier: &'i I) -> Self {
        SignUp {
            repository,
            identifier,
        }
    }
}

#[async_trait(?Send)]
impl<'r, 'i, R, I> UseCase for SignUp<'r, 'i, R, I>
where
    R: Repository,
    I: Identifier<UserId>,
{
    type Request = Request;
    type Response = Response;

    async fn execute(&self, request: Self::Request) -> Result<Self::Response, ApplicationError> {
        let id = self.identifier.new_id().await?;
        let user = User {
            id,
            username: request.username,
            password: request.password,
        };
        let record = Record { user };
        self.repository.create(record).await?;
        Ok(Response { id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interfaces::repositories::identifier::MockIdentifier;
    use crate::interfaces::repositories::user::MockRepository;

    #[tokio::test]
    async fn test_signup() {
        let username = "username".to_string();
        let password = "password".to_string();
        let request = Request { username, password };

        let id = UserId::new_v4();
        let mut identifier = MockIdentifier::new();
        identifier
            .expect_new_id()
            .times(1)
            .returning(move || Ok(id.clone()));

        let mut repository = MockRepository::new();
        repository
            .expect_create()
            .times(1)
            .returning(move |_| Ok(id.clone()));

        let use_case = SignUp::new(&repository, &identifier);
        let response = use_case.execute(request).await.unwrap();
        assert_eq!(response.id, id);
    }

    #[tokio::test]
    async fn test_signup_repository_error() {
        let username = "username".to_string();
        let password = "password".to_string();
        let request = Request { username, password };

        let id = UserId::new_v4();
        let mut identifier = MockIdentifier::new();
        identifier
            .expect_new_id()
            .times(1)
            .returning(move || Ok(id.clone()));

        let mut repository = MockRepository::new();
        repository
            .expect_create()
            .times(1)
            .returning(move |_| Err(UserRepositoryError::ConnectionError));

        let use_case = SignUp::new(&repository, &identifier);
        let response = use_case.execute(request).await;
        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_signup_identifier_error() {
        let username = "username".to_string();
        let password = "password".to_string();
        let request = Request { username, password };

        let mut identifier = MockIdentifier::new();
        identifier
            .expect_new_id()
            .times(1)
            .returning(move || Err(NewIdError));

        let repository = MockRepository::new();

        let use_case = SignUp::new(&repository, &identifier);
        let response = use_case.execute(request).await;
        assert!(response.is_err());
    }
}
