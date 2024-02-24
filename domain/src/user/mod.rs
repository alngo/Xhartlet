pub mod value_objects {
    pub type UserId = uuid::Uuid;
    pub type Username = String;
    pub type Password = String;
}

use value_objects::{Password, UserId, Username};

#[derive(Debug)]
pub struct User {
    id: UserId,
    username: Username,
    password: Password,
}
