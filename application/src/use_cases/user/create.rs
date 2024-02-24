use core::fmt;
use std::error::Error;

use async_trait::async_trait;
use xhartlet_domain::user::value_objects::{Password, UserId, Username};
use xhartlet_domain::user::User;

use crate::{
    repositories::{
        identifier::{NewId, NewIdError},
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

#[derive(Debug)]
pub enum CreateUserError {
    RepositoryError(UserRepositoryError),
    NewIdError(NewIdError),
}

impl From<UserRepositoryError> for CreateUserError {
    fn from(e: UserRepositoryError) -> Self {
        CreateUserError::RepositoryError(e)
    }
}

impl From<NewIdError> for CreateUserError {
    fn from(e: NewIdError) -> Self {
        CreateUserError::NewIdError(e)
    }
}

impl fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreateUserError::RepositoryError(e) => write!(f, "RepositoryError: {:?}", e),
            CreateUserError::NewIdError(e) => write!(f, "NewIdError: {:?}", e),
        }
    }
}

impl Error for CreateUserError {}

pub struct CreateUser<'r, 'g, R, G> {
    repository: &'r R,
    id_generator: &'g G,
}

impl<'r, 'g, R, G> CreateUser<'r, 'g, R, G>
where
    R: Repository,
    G: NewId<UserId>,
{
    pub fn new(repository: &'r R, id_generator: &'g G) -> Self {
        CreateUser {
            repository,
            id_generator,
        }
    }
}

#[async_trait(?Send)]
impl<'r, 'g, R, G> UseCase for CreateUser<'r, 'g, R, G>
where
    R: Repository,
    G: NewId<UserId>,
{
    type Request = Request;
    type Response = Response;

    async fn execute(&self, request: Self::Request) -> Result<Self::Response, CreateUserError> {
        let id = self.id_generator.new_id().await?;
        let user = User {
            id: id.clone(),
            username: request.username,
            password: request.password,
        };
        let record = Record { user };
        self.repository.create(record).await?;
        Ok(Response { id })
    }
}
