use actix_web::HttpResponse;

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Login")
}

pub async fn register() -> HttpResponse {
    HttpResponse::Ok().body("Register")
}
