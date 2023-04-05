use serde::{Deserialize, Serialize};

use crate::model::post::Post;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostDto {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl From<Post> for PostDto {
    fn from(post: Post) -> Self {
        PostDto {
            id: post.id,
            title: post.title,
            body: post.body,
            published: post.published,
        }
    }
}