pub mod value_objects {
    pub type UserId = uuid::Uuid;
    pub type Username = String;
    pub type Password = String;
}

use value_objects::{Password, UserId, Username};

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub password: Password,
}
