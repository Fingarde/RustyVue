use crate::dto::post::PostDto;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::model::post::Post;
use crate::repository::post as post_repository;

#[derive(Deserialize)]
pub struct CreatePostRequest {
    title: String,
    body: String,
}

pub async fn create(post: web::Json<CreatePostRequest>) -> HttpResponse {
    let post = Post {
        id: post_repository::get_next_id().unwrap(),
        title: post.title.clone(),
        body: post.body.clone(),
        published: false,
    };

    match post_repository::insert(post) {
        Ok(post) => HttpResponse::Ok().json(PostDto::from(post)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn list() -> HttpResponse {
    match post_repository::list() {
        Ok(posts) => HttpResponse::Ok().json(
            posts
                .into_iter()
                .map(PostDto::from)
                .collect::<Vec<PostDto>>(),
        ),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get() -> HttpResponse {
    HttpResponse::Ok().body("Get")
}

pub async fn update() -> HttpResponse {
    HttpResponse::Ok().body("Update")
}

pub async fn delete() -> HttpResponse {
    HttpResponse::Ok().body("Delete")
}
