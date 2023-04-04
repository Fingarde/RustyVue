use actix_web::{HttpResponse};

pub async fn create() -> HttpResponse {
    HttpResponse::Ok().body("Create")
}

pub async fn list() -> HttpResponse {
    HttpResponse::Ok().body("List")
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
