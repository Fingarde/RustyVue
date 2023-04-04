use crate::controller::auth::{login, register};
use crate::router::RouterFactory;
use actix_web::web::{scope, ServiceConfig};
use actix_web::{web, Scope};

pub struct AuthRouterFactory {}

impl RouterFactory for AuthRouterFactory {
    fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            scope("/auth")
                .route("/login", web::get().to(login))
                .route("/register", web::post().to(register)),
        );
    }
}
