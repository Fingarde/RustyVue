use actix_web::web;
use actix_web::web::scope;

use actix_web_httpauth::middleware::HttpAuthentication;

use crate::auth::validator;
use crate::controller::post::{create, delete, get, list, update};
use crate::router::RouterFactory;

pub struct PostRouterFactory;

impl RouterFactory for PostRouterFactory {
    fn config(cfg: &mut web::ServiceConfig) {
        let auth = HttpAuthentication::bearer(validator);

        cfg.service(
            scope("/posts")
                .wrap(auth)
                .route("/", web::get().to(list))
                .route("/", web::post().to(create))
                .route("/{id}", web::get().to(get))
                .route("/{id}", web::put().to(update))
                .route("/{id}", web::delete().to(delete)),
        );
    }
}
