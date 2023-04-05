use diesel::prelude::*;

use crate::database;
use crate::model::user::User;

use crate::database::schema::users::dsl::*;

pub async fn insert(user: User) -> Result<User, diesel::result::Error> {
    let mut conn = database::DATABASE_POOL.get().unwrap().get().unwrap();

    diesel::insert_into(users)
        .values(&user)
        .get_result::<User>(&mut conn)
}
