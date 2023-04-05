use diesel::prelude::*;
use uuid::Uuid;

use crate::database::schema::users;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}
