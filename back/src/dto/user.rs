use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}
