use xhartlet-domain::user::value_objects::{UserId, UserName, Password};

pub struct Request {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct Response {
    pub id: UserId,
}
