use actix_web::{web, HttpResponse};

use crate::service::*;

mod parameter {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Register {
        pub username: String,
        pub email: String,
        pub password: String,
    }

    pub struct Login {
        username: String,
        password: String,
    }
}

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Login")
}

pub async fn register(json: web::Json<parameter::Register>) -> HttpResponse {
    let param = auth::parameter::Register {
        username: json.username.clone(),
        email: json.email.clone(),
        password: json.password.clone(),
    };

    auth::register(param).await;

    HttpResponse::Ok().body("Register")
}
