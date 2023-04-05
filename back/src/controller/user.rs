use actix_web::HttpResponse;

use crate::service::user as user_service;

pub async fn list() -> HttpResponse {
    HttpResponse::Ok().json(user_service::list().await)
}
