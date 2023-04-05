use actix_web::web;
use actix_web::web::scope;

use actix_web_httpauth::middleware::HttpAuthentication;

use crate::auth::validator;
use crate::controller::user as user_controller;
use crate::router::RouterFactory;

pub struct UserRouterFactory;

impl RouterFactory for UserRouterFactory {
    fn config(cfg: &mut web::ServiceConfig) {
        let auth = HttpAuthentication::bearer(validator);

        cfg.service(
            scope("/users")
                .wrap(auth)
                .route("/", web::get().to(user_controller::list)),
        );
    }
}
