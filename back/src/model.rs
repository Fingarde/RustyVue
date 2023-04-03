use diesel::prelude::*;

use crate::schema::posts;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}