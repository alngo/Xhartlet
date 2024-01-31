mod entities;
mod value_objects;

use crate::session::SessionId;
use value_objects::{UserId, UserName};

pub struct User {
    id: UserId,
    name: UserName,
    sessions: Vec<SessionId>,
}
