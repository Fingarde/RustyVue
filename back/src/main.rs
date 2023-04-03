use log::{ info, warn, debug };
use actix_web::{get, App, HttpServer, Responder};

mod error;
mod config;

use crate::error::Error;
use crate::config::Config;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    // load .env file and initialize logger
    let dotenv = dotenvy::dotenv();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // if .env file is not found, warn the user
    if dotenv.is_err() {
        warn!("No .env file found");
    }

    // print all env vars in debug mode
    for (key, value) in std::env::vars() {
        debug!("{}: {}", key, value);
    }

    // read server config from env vars
    let config = envy::prefixed("APP_").from_env::<Config>()?;

    info!("Starting server on http://{}:{}/", config.address, config.port);

   
    HttpServer::new(|| {
        App::new()
        // enable logger - always register actix-web Logger middleware last because it is called in reverse order
        .wrap(actix_web::middleware::Logger::default())
        .service(index)
    })
    .bind((config.address, config.port))?
    .run()
    .await?;


    info!("Server stopped");

    Ok(())
}


#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}