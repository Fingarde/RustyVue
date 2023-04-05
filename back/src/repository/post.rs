use diesel::prelude::*;

use crate::database;
use crate::model::post::Post;

use crate::database::schema::posts::dsl::*;

pub fn insert(post: Post) -> Result<Post, diesel::result::Error> {
    let mut conn = database::DATABASE_POOL.get().unwrap().get().unwrap();

    diesel::insert_into(posts)
        .values(&post)
        .get_result::<Post>(&mut conn)
}

pub fn list() -> Result<Vec<Post>, diesel::result::Error> {
    let mut conn = database::DATABASE_POOL.get().unwrap().get().unwrap();

    posts.load::<Post>(&mut conn)
}

// get next id incremented
pub fn get_next_id() -> Result<i32, diesel::result::Error> {
    let mut conn = database::DATABASE_POOL.get().unwrap().get().unwrap();

    posts
        .select(diesel::dsl::max(id))
        .first::<Option<i32>>(&mut conn)
        .map(|max_id| max_id.unwrap_or(0) + 1)
}

pub fn update(post: Post) -> Result<Post, diesel::result::Error> {
    let mut conn = database::DATABASE_POOL.get().unwrap().get().unwrap();

    diesel::update(posts.find(post.id))
        .set(&post)
        .get_result::<Post>(&mut conn)
}
