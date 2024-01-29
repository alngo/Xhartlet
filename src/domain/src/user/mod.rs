mod entities;
mod value_objects;

use entities::{Account, Portfolio};
use value_objects::{UserId, UserName};

pub struct User {
    id: UserId,
    name: UserName,
    account: Account,
    portfolio: Portfolio,
}
