use xhartlet_application::interfaces::repositories::{
    identifier::Identifier, user::Repository as UserRepository,
};

use xhartlet_domain::user::value_objects::UserId;

pub trait Database: Identifier<UserId> + UserRepository + 'static {}
