use std::{collections::HashMap, sync::RwLock};

use async_trait::async_trait;
use xhartlet_application::use_cases::user::abstract_repository::{
    Error, Repository as UserRepository,
};
use xhartlet_domain::user::{Email, User, UserId};

#[derive(Debug, Default)]
struct Repository {
    users: RwLock<HashMap<UserId, User>>,
}

impl Repository {
    pub fn generate_id(&self) -> UserId {
        UserId::new_v4()
    }
}

#[async_trait(?Send)]
impl UserRepository for Repository {
    async fn create(&self, user: &User) -> Result<Option<UserId>, Error> {
        let id = self.generate_id();
        self.users.write().unwrap().insert(id, user.clone());
        Ok(Some(id))
    }

    async fn read_by_email(&self, email: &Email) -> Result<Option<User>, Error> {
        let res = self
            .users
            .read()
            .unwrap()
            .values()
            .find(|u| u.email == *email)
            .cloned();
        Ok(res)
    }

    async fn is_email_taken(&self, email: &Email) -> Result<bool, Error> {
        let response = self.read_by_email(email).await?;
        match response {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn update(&self, id: UserId, user: &User) -> Result<Option<UserId>, Error> {
        self.users.write().unwrap().insert(id, user.clone());
        Ok(Some(id))
    }
    async fn delete(&self, id: UserId) -> Result<(), Error> {
        self.users.write().unwrap().remove(&id);
        Ok(())
    }
}
