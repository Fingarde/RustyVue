use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database::schema::posts;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
