use crate::dto::user::UserDto;

use crate::repository::user as user_repository;

pub async fn list() -> Vec<UserDto> {
    user_repository::list()
        .await
        .unwrap()
        .into_iter()
        .map(UserDto::from)
        .collect::<Vec<UserDto>>()
}
