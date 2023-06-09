use actix_web::{App, HttpServer};

use crate::config::database::DatabaseConfig;
use crate::config::server::ServerConfig;
use log::info;

mod auth;
mod config;
mod controller;
mod database;
mod dto;
mod error;
mod model;
mod repository;
mod router;
mod service;
mod utils;

use crate::error::Error;

use crate::router::RouterFactory;
use crate::router::{auth::AuthRouterFactory, post::PostRouterFactory, user::UserRouterFactory};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    // load env + initialize logger
    utils::load_env();

    // read configs from env vars
    let server_config = ServerConfig::from_env()?;
    let database_config = DatabaseConfig::from_env()?;

    // create database connection pool
    database::init(database_config);

    info!(
        "Starting server on http://{}:{}/",
        server_config.address, server_config.port
    );

    HttpServer::new(move || {
        App::new()
            // enable logger - always register actix-web Logger middleware last because it is called in reverse order
            .wrap(actix_web::middleware::Logger::default())
            // register routers
            .configure(AuthRouterFactory::config)
            .configure(PostRouterFactory::config)
            .configure(UserRouterFactory::config)
    })
    .bind((server_config.address, server_config.port))?
    .run()
    .await?;

    info!("Server stopped");

    Ok(())
}
