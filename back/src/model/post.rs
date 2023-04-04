use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::posts;

#[derive(Debug, Queryable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
