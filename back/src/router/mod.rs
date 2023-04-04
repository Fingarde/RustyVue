use actix_web::web::ServiceConfig;

pub mod auth;
pub mod post;

pub trait RouterFactory {
    fn config(cfg: &mut ServiceConfig);
}
